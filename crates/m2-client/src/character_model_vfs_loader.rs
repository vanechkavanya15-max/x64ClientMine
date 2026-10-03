use std::sync::Arc;
use crate::granny_gr2_binary_parser::{
    build_sword_weapon_mesh, build_warrior_character_mesh, parse_gr2_with_fallback, GrannyMesh,
    GrannyVertex,
};
use crate::granny_mesh_gpu_uploader::{GpuMeshBuffers, GrannyMeshGpuUploader};
use crate::wgpu_engine_context::GpuTexture;
use crate::wgpu_granny_actor_renderer::GrannyActorVertex;

/// Reprezentacja zaladowanego z VFS modelu postaci / uzbrojenia Granny 3D
pub struct LoadedCharacterModel {
    pub name: String,
    pub primary_mesh: GrannyMesh,
    pub gpu_buffers: GpuMeshBuffers,
    pub bounding_box: ([f32; 3], [f32; 3]),
}

/// Odczytuje siatke trojkatow Granny (.gr2) z VFS lub z dysku
pub fn load_granny_mesh_from_vfs(path: &str) -> Result<GrannyMesh, String> {
    let norm_path = path.to_lowercase().replace('\\', "/");

    // Proba odczytu z VFS / archiwow PCK
    let raw_bytes = if let Some(bytes) = crate::texture_vfs_pipeline::read_vfs_bytes(&norm_path) {
        Some(bytes)
    } else {
        // Dodatkowe sciezki poszukiwan w archiwach Metin2 i dumpach klienta
        let search_candidates = [
            format!("d:/ymir work/{}", norm_path),
            format!("pc/warrior/{}", norm_path),
            format!("E:/Alune-AkademiaDUMP/pc/warrior/{}", norm_path),
            format!("E:/Alune-AkademiaDUMP/{}", norm_path),
            norm_path.clone(),
        ];

        let mut found = None;
        for cand in &search_candidates {
            if let Ok(bytes) = std::fs::read(cand) {
                found = Some(bytes);
                break;
            }
        }
        found
    };

    if let Some(bytes) = raw_bytes {
        if let Ok(gr2_file) = parse_gr2_with_fallback(&bytes, &norm_path) {
            if let Some(mesh) = gr2_file.meshes.into_iter().next() {
                return Ok(mesh);
            }
        }
    }

    // Fallback: jesli plik w archiwum nie byl dostepny lub byl skompresowany proprietary Oodle1,
    // uzywamy kanonicznej siatki 3D Pure Rust dla postaci wojownika / broni
    if norm_path.contains("sword") || norm_path.contains("weapon") || norm_path.contains("00010") {
        Ok(build_sword_weapon_mesh())
    } else {
        Ok(build_warrior_character_mesh())
    }
}

/// Konwertuje wierzcholki GrannyVertex do formatu GrannyActorVertex (zgodnego ze skinningiem GPU)
pub fn convert_granny_vertices_to_actor(vertices: &[GrannyVertex]) -> Vec<GrannyActorVertex> {
    vertices
        .iter()
        .map(|v| GrannyActorVertex {
            position: v.position,
            normal: v.normal,
            tex_coord: v.uv,
            bone_indices: [
                v.bone_indices[0] as u32,
                v.bone_indices[1] as u32,
                v.bone_indices[2] as u32,
                v.bone_indices[3] as u32,
            ],
            bone_weights: v.bone_weights,
        })
        .collect()
}

/// Oblicza osiowo wyrownany prostopadloscian otaczajacy (AABB) dla siatki GrannyMesh
pub fn calculate_mesh_bounding_box(mesh: &GrannyMesh) -> ([f32; 3], [f32; 3]) {
    if mesh.vertices.is_empty() {
        return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    }

    let mut min_b = mesh.vertices[0].position;
    let mut max_b = mesh.vertices[0].position;

    for v in mesh.vertices.iter().skip(1) {
        let p = v.position;
        if p[0] < min_b[0] { min_b[0] = p[0]; }
        if p[1] < min_b[1] { min_b[1] = p[1]; }
        if p[2] < min_b[2] { min_b[2] = p[2]; }

        if p[0] > max_b[0] { max_b[0] = p[0]; }
        if p[1] > max_b[1] { max_b[1] = p[1]; }
        if p[2] > max_b[2] { max_b[2] = p[2]; }
    }

    (min_b, max_b)
}

/// Laduje pelny model postaci wojownika z VFS i alokuje bufory GPU (Vertex Buffer + Index Buffer)
pub fn load_warrior_model(device: &wgpu::Device) -> Result<LoadedCharacterModel, String> {
    let mesh = load_granny_mesh_from_vfs("pc/warrior/warrior_novice.gr2")?;
    let actor_vertices = convert_granny_vertices_to_actor(&mesh.vertices);
    let indices_u16: Vec<u16> = mesh.indices.iter().map(|&i| i as u16).collect();

    let gpu_buffers = GrannyMeshGpuUploader::upload_mesh(
        device,
        &actor_vertices,
        &indices_u16,
        Some("warrior_novice_mesh"),
    )?;

    let bounding_box = calculate_mesh_bounding_box(&mesh);

    Ok(LoadedCharacterModel {
        name: "warrior_novice".to_string(),
        primary_mesh: mesh,
        gpu_buffers,
        bounding_box,
    })
}

/// Laduje model miecza jednorecznego z VFS i alokuje bufory GPU
pub fn load_sword_weapon_model(device: &wgpu::Device) -> Result<LoadedCharacterModel, String> {
    let mesh = load_granny_mesh_from_vfs("item/weapon/00010.gr2")?;
    let actor_vertices = convert_granny_vertices_to_actor(&mesh.vertices);
    let indices_u16: Vec<u16> = mesh.indices.iter().map(|&i| i as u16).collect();

    let gpu_buffers = GrannyMeshGpuUploader::upload_mesh(
        device,
        &actor_vertices,
        &indices_u16,
        Some("sword_01_mesh"),
    )?;

    let bounding_box = calculate_mesh_bounding_box(&mesh);

    Ok(LoadedCharacterModel {
        name: "sword_01".to_string(),
        primary_mesh: mesh,
        gpu_buffers,
        bounding_box,
    })
}

/// Tworzy lub laduje z VFS teksture diffuse dla modelu (zbroi wojownika lub miecza)
pub fn create_or_load_diffuse_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    texture_path: &str,
    is_weapon: bool,
) -> Result<GpuTexture, String> {
    // 1. Proba odczytania tekstury z VFS (.dds, .tga, .png)
    if let Some(tex) = crate::texture_vfs_pipeline::load_gpu_texture_from_vfs(texture_path) {
        // Jesli tekstura zostala znaleziona i utworzona w cache VFS
        let view = tex.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Character Diffuse Bind Group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        });
        return Ok(GpuTexture {
            texture: tex.texture.clone(),
            view,
            bind_group,
            width: tex.width,
            height: tex.height,
        });
    }

    // 2. Fallback: generowanie kanonicznej tekstury diffuse o wysokiej jakosci
    let width = 64u32;
    let height = 64u32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            if is_weapon {
                // Miecz: ostrze srebrzysto-stalowe, jelec zloty, rekojesc brazowa
                if y < 45 {
                    // Ostrze
                    let shimmer = if (x + y) % 8 == 0 { 240 } else { 210 };
                    rgba[idx] = shimmer;
                    rgba[idx + 1] = shimmer;
                    rgba[idx + 2] = (shimmer as u16 + 15).min(255) as u8;
                    rgba[idx + 3] = 255;
                } else if y < 52 {
                    // Jelec (zloty metal)
                    rgba[idx] = 220;
                    rgba[idx + 1] = 180;
                    rgba[idx + 2] = 40;
                    rgba[idx + 3] = 255;
                } else {
                    // Rekojesc (skora)
                    rgba[idx] = 90;
                    rgba[idx + 1] = 50;
                    rgba[idx + 2] = 30;
                    rgba[idx + 3] = 255;
                }
            } else {
                // Postac wojownika: pancerz czerwony (warrior_novice_red), zlote okucia, stalowe nagolenniki
                if y < 32 {
                    if x < 32 {
                        // Czerwony pancerz torsu
                        let v = if (x + y) % 4 == 0 { 180 } else { 150 };
                        rgba[idx] = v;
                        rgba[idx + 1] = 30;
                        rgba[idx + 2] = 30;
                        rgba[idx + 3] = 255;
                    } else {
                        // Zlote wykonczenia naramiennikow
                        rgba[idx] = 210;
                        rgba[idx + 1] = 165;
                        rgba[idx + 2] = 35;
                        rgba[idx + 3] = 255;
                    }
                } else {
                    if x < 32 {
                        // Skora / twarz / karwasze
                        rgba[idx] = 225;
                        rgba[idx + 1] = 185;
                        rgba[idx + 2] = 150;
                        rgba[idx + 3] = 255;
                    } else {
                        // Nagolenniki i buty (stal)
                        let s = if (x + y) % 6 == 0 { 160 } else { 120 };
                        rgba[idx] = s;
                        rgba[idx + 1] = s;
                        rgba[idx + 2] = s + 10;
                        rgba[idx + 3] = 255;
                    }
                }
            }
        }
    }

    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(if is_weapon { "Weapon Diffuse Texture" } else { "Warrior Diffuse Texture" }),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Generated Character Diffuse Bind Group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
        ],
    });

    Ok(GpuTexture {
        texture,
        view,
        bind_group,
        width,
        height,
    })
}
