use std::sync::Arc;
use m2_client::wgpu_engine_context::{GpuTexture, WgpuEngineContext};
use m2_client::wgpu_skybox_renderer::SkyboxRenderer;
use m2_client::vfs_pck_memory_mapped::read_file_from_archive;
use m2_client::py_bridge_pack::get_mounted_packs;
use m2_client::tga_decoder::TgaImage;
use m2_client::ui_texture_mgr::UiTextureManager;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentData {
    pub b_sky_box_texture_render_mode: bool,
    pub v3_sky_box_scale: [f32; 3],
    pub by_sky_box_gradient_level_upper: u8,
    pub by_sky_box_gradient_level_lower: u8,
    pub str_sky_box_face_file_name: [String; 6], // 0: front, 1: back, 2: left, 3: right, 4: top, 5: bottom
    pub v2_cloud_scale: [f32; 2],
    pub f_cloud_height: f32,
    pub v2_cloud_texture_scale: [f32; 2],
    pub v2_cloud_speed: [f32; 2],
    pub str_cloud_texture_file_name: String,
}

impl Default for EnvironmentData {
    fn default() -> Self {
        Self {
            b_sky_box_texture_render_mode: false,
            v3_sky_box_scale: [1.0, 1.0, 1.0],
            by_sky_box_gradient_level_upper: 0,
            by_sky_box_gradient_level_lower: 0,
            str_sky_box_face_file_name: [String::new(), String::new(), String::new(), String::new(), String::new(), String::new()],
            v2_cloud_scale: [1.0, 1.0],
            f_cloud_height: 1.0,
            v2_cloud_texture_scale: [1.0, 1.0],
            v2_cloud_speed: [0.0, 0.0],
            str_cloud_texture_file_name: String::new(),
        }
    }
}

pub fn parse_environment_data(text: &str) -> Result<EnvironmentData, &'static str> {
    let mut env = EnvironmentData::default();
    
    let mut in_skybox = false;
    let lines = text.lines();
    for line in lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line == "skybox" {
            in_skybox = true;
            continue;
        } else if line == "}" || line == "{" {
            if line == "}" && in_skybox {
                in_skybox = false;
            }
            continue;
        }

        if in_skybox {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            let key = parts[0].to_lowercase();
            match key.as_str() {
                "btexturerendermode" => {
                    if parts.len() > 1 {
                        env.b_sky_box_texture_render_mode = parts[1] == "1";
                    }
                }
                "scale" => {
                    if parts.len() > 3 {
                        env.v3_sky_box_scale[0] = parts[1].parse().unwrap_or(1.0);
                        env.v3_sky_box_scale[1] = parts[2].parse().unwrap_or(1.0);
                        env.v3_sky_box_scale[2] = parts[3].parse().unwrap_or(1.0);
                    }
                }
                "gradientlevelupper" => {
                    if parts.len() > 1 {
                        env.by_sky_box_gradient_level_upper = parts[1].parse().unwrap_or(0);
                    }
                }
                "gradientlevellower" => {
                    if parts.len() > 1 {
                        env.by_sky_box_gradient_level_lower = parts[1].parse().unwrap_or(0);
                    }
                }
                "frontfacefilename" => if parts.len() > 1 { env.str_sky_box_face_file_name[0] = parts[1].replace("\"", ""); }
                "backfacefilename" => if parts.len() > 1 { env.str_sky_box_face_file_name[1] = parts[1].replace("\"", ""); }
                "leftfacefilename" => if parts.len() > 1 { env.str_sky_box_face_file_name[2] = parts[1].replace("\"", ""); }
                "rightfacefilename" => if parts.len() > 1 { env.str_sky_box_face_file_name[3] = parts[1].replace("\"", ""); }
                "topfacefilename" => if parts.len() > 1 { env.str_sky_box_face_file_name[4] = parts[1].replace("\"", ""); }
                "bottomfacefilename" => if parts.len() > 1 { env.str_sky_box_face_file_name[5] = parts[1].replace("\"", ""); }
                "cloudscale" => {
                    if parts.len() > 2 {
                        env.v2_cloud_scale[0] = parts[1].parse().unwrap_or(1.0);
                        env.v2_cloud_scale[1] = parts[2].parse().unwrap_or(1.0);
                    }
                }
                "cloudheight" => {
                    if parts.len() > 1 {
                        env.f_cloud_height = parts[1].parse().unwrap_or(1.0);
                    }
                }
                "cloudtexturescale" => {
                    if parts.len() > 2 {
                        env.v2_cloud_texture_scale[0] = parts[1].parse().unwrap_or(1.0);
                        env.v2_cloud_texture_scale[1] = parts[2].parse().unwrap_or(1.0);
                    }
                }
                "cloudspeed" => {
                    if parts.len() > 2 {
                        env.v2_cloud_speed[0] = parts[1].parse().unwrap_or(0.0);
                        env.v2_cloud_speed[1] = parts[2].parse().unwrap_or(0.0);
                    }
                }
                "cloudtexturefilename" => {
                    if parts.len() > 1 {
                        env.str_cloud_texture_file_name = parts[1].replace("\"", "");
                    }
                }
                _ => {}
            }
        }
    }
    
    Ok(env)
}

fn read_vfs_bytes(path: &str) -> Option<Vec<u8>> {
    let norm = path.to_lowercase().replace('\\', "/");
    if let Ok(bytes) = std::fs::read(&norm) {
        return Some(bytes);
    }
    if let Ok(guard) = get_mounted_packs().lock() {
        for archive in guard.values() {
            if let Some(data) = read_file_from_archive(archive, &norm) {
                return Some(data);
            }
        }
    }
    None
}

fn decode_image(bytes: &[u8], path: &str) -> Option<(Vec<u8>, u32, u32)> {
    let norm = path.to_lowercase().replace('\\', "/");
    if norm.ends_with(".dds") {
        UiTextureManager::decode_dds_to_rgba(bytes).ok()
    } else if norm.ends_with(".tga") {
        let tga = TgaImage::from_bytes(bytes).ok()?;
        Some((tga.rgba_data, tga.width as u32, tga.height as u32))
    } else {
        let img = image::load_from_memory(bytes).ok()?;
        let rgba_img = img.to_rgba8();
        Some((rgba_img.into_raw(), img.width(), img.height()))
    }
}

pub struct SkyboxIntegration {
    pub faces: [Option<Arc<GpuTexture>>; 6],
    pub env_data: EnvironmentData,
    pub sampler: wgpu::Sampler,
}

impl SkyboxIntegration {
    pub fn new(device: &wgpu::Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });

        Self {
            faces: [None, None, None, None, None, None],
            env_data: EnvironmentData::default(),
            sampler,
        }
    }

    pub fn load_environment(
        &mut self,
        env_path: &str,
        ctx: &WgpuEngineContext,
        renderer: &mut SkyboxRenderer,
    ) -> Result<(), &'static str> {
        let bytes = read_vfs_bytes(env_path).ok_or("Failed to read environment file")?;
        let text = String::from_utf8_lossy(&bytes);
        
        self.env_data = parse_environment_data(&text)?;

        for i in 0..6 {
            let filename = &self.env_data.str_sky_box_face_file_name[i];
            if filename.is_empty() {
                continue;
            }
            
            if let Some(tex_bytes) = read_vfs_bytes(filename) {
                if let Some((rgba, w, h)) = decode_image(&tex_bytes, filename) {
                    if let Ok(gpu_tex) = ctx.create_texture_from_rgba(&rgba, w, h, Some(filename)) {
                        self.faces[i] = Some(Arc::new(gpu_tex));
                    }
                }
            }
        }

        // To comply with the existing SkyboxRenderer WGSL which uses a single 2D texture,
        // we bind the front face (index 0) if available, otherwise fallback.
        if let Some(front_tex) = &self.faces[0] {
            let uniform_bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Skybox Uniform Bind Group Layout (Integration)"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

            renderer.uniform_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Skybox Uniform Bind Group (Integration)"),
                layout: &uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: renderer.uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&front_tex.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
            });
        }

        Ok(())
    }

    pub fn render_skybox<'a>(&'a self, renderer: &'a SkyboxRenderer, pass: &mut wgpu::RenderPass<'a>) {
        if self.env_data.b_sky_box_texture_render_mode {
            renderer.render(pass);
        }
    }
}
