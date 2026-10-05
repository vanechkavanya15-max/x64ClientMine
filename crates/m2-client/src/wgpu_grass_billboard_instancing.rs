
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct GrassVertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
    pub wind_weight: f32,
}

impl GrassVertex {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GrassVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GrassInstance {
    pub position: [f32; 3],
    pub scale: f32,
    pub rotation: f32,
}

impl GrassInstance {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GrassInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<f32>()) as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

/// Generuje siatke wierzcholkow dla kepki trawy.
/// Zwykle to 2 lub 3 przecinajace sie quady (billboardy) w ksztalcie gwiazdy (*).
pub fn generate_grass_mesh(width: f32, height: f32) -> Vec<GrassVertex> {
    let mut vertices = Vec::new();
    let hw = width * 0.5;
    
    // Quad 1: wzdluz osi X
    let q1_bl = GrassVertex { position: [-hw, 0.0, 0.0], tex_coord: [0.0, 1.0], wind_weight: 0.0 };
    let q1_br = GrassVertex { position: [hw, 0.0, 0.0], tex_coord: [1.0, 1.0], wind_weight: 0.0 };
    let q1_tl = GrassVertex { position: [-hw, 0.0, height], tex_coord: [0.0, 0.0], wind_weight: 1.0 };
    let q1_tr = GrassVertex { position: [hw, 0.0, height], tex_coord: [1.0, 0.0], wind_weight: 1.0 };
    
    vertices.push(q1_bl); vertices.push(q1_br); vertices.push(q1_tr);
    vertices.push(q1_bl); vertices.push(q1_tr); vertices.push(q1_tl);

    // Quad 2: wzdluz osi Y
    let q2_bl = GrassVertex { position: [0.0, -hw, 0.0], tex_coord: [0.0, 1.0], wind_weight: 0.0 };
    let q2_br = GrassVertex { position: [0.0, hw, 0.0], tex_coord: [1.0, 1.0], wind_weight: 0.0 };
    let q2_tl = GrassVertex { position: [0.0, -hw, height], tex_coord: [0.0, 0.0], wind_weight: 1.0 };
    let q2_tr = GrassVertex { position: [0.0, hw, height], tex_coord: [1.0, 0.0], wind_weight: 1.0 };
    
    vertices.push(q2_bl); vertices.push(q2_br); vertices.push(q2_tr);
    vertices.push(q2_bl); vertices.push(q2_tr); vertices.push(q2_tl);

    // Quad 3: pod katem 45 stopni
    let d = hw * 0.70710678; // hw * sin(45)
    let q3_bl = GrassVertex { position: [-d, -d, 0.0], tex_coord: [0.0, 1.0], wind_weight: 0.0 };
    let q3_br = GrassVertex { position: [d, d, 0.0], tex_coord: [1.0, 1.0], wind_weight: 0.0 };
    let q3_tl = GrassVertex { position: [-d, -d, height], tex_coord: [0.0, 0.0], wind_weight: 1.0 };
    let q3_tr = GrassVertex { position: [d, d, height], tex_coord: [1.0, 0.0], wind_weight: 1.0 };
    
    vertices.push(q3_bl); vertices.push(q3_br); vertices.push(q3_tr);
    vertices.push(q3_bl); vertices.push(q3_tr); vertices.push(q3_tl);

    vertices
}

/// Generuje instancje kepkow trawy na wskazanym obszarze.
pub fn generate_grass_instances(
    patch_x: f32,
    patch_y: f32,
    width: f32,
    height: f32,
    count: usize,
    mut seed: u32,
) -> Vec<GrassInstance> {
    let mut instances = Vec::with_capacity(count);

    for _ in 0..count {
        // Prosty LCG pseudo-random
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let rx = (seed >> 16) as f32 / 65535.0; // 0.0 .. 1.0
        
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let ry = (seed >> 16) as f32 / 65535.0; // 0.0 .. 1.0
        
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let rs = (seed >> 16) as f32 / 65535.0; // 0.0 .. 1.0
        
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let rr = (seed >> 16) as f32 / 65535.0; // 0.0 .. 1.0

        let x = patch_x + rx * width;
        let y = patch_y + ry * height;
        let z = 0.0; // W pelnej implementacji pobierane z mapy wysokosci

        let scale = 0.8 + rs * 0.4; // 0.8 .. 1.2
        let rotation = rr * std::f32::consts::PI * 2.0;

        instances.push(GrassInstance {
            position: [x, y, z],
            scale,
            rotation,
        });
    }

    instances
}

pub fn get_grass_shader_wgsl() -> &'static str {
    r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
    time: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) wind_weight: f32,
};

struct InstanceInput {
    @location(3) instance_pos: vec3<f32>,
    @location(4) scale: f32,
    @location(5) rotation: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    let s = sin(instance.rotation);
    let c = cos(instance.rotation);
    
    // Rotacja wokol osi Z, skalowanie
    var pos_x = (model.position.x * c - model.position.y * s) * instance.scale;
    var pos_y = (model.position.x * s + model.position.y * c) * instance.scale;
    var pos_z = model.position.z * instance.scale;
    
    // Prosty efekt wiatru (Wind Sway) za pomoca funkcji sin i cos od czasu
    let wind_strength = 0.5;
    let wind_freq = 2.0;
    
    // Dodajemy odrobine losowosci zaleznie od pozycji swiata
    let phase = instance.instance_pos.x * 0.1 + instance.instance_pos.y * 0.1;
    
    // Przemieszczenie na wietrze tyczy sie glownie czubkow (wind_weight)
    let sway_x = sin(camera.time * wind_freq + phase) * wind_strength * model.wind_weight;
    let sway_y = cos(camera.time * wind_freq * 0.8 + phase) * wind_strength * model.wind_weight;
    
    pos_x += sway_x;
    pos_y += sway_y;
    
    let world_pos = vec3<f32>(
        pos_x + instance.instance_pos.x,
        pos_y + instance.instance_pos.y,
        pos_z + instance.instance_pos.z
    );

    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.tex_coord = model.tex_coord;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coord);
    // Odrzucenie pikseli w pelni przezroczystych (alpha test)
    if (tex_color.a < 0.5) {
        discard;
    }
    return tex_color;
}
    "#
}

pub fn create_grass_instancing_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader_source = get_grass_shader_wgsl();

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Grass Instancing Shader"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Grass Instancing Pipeline Layout"),
        bind_group_layouts: &[Some(camera_bind_group_layout), Some(texture_bind_group_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Grass Instancing Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Some(GrassVertex::desc()), Some(GrassInstance::desc())],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None, // Brak cullingu, trawa widoczna z dwoch stron
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}
