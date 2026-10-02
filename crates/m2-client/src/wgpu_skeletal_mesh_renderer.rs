use bytemuck::{Pod, Zeroable};
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SkeletalVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
    pub bone_indices: [u32; 4],
    pub bone_weights: [f32; 4],
}

impl SkeletalVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SkeletalVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32x4,
                },
                wgpu::VertexAttribute {
                    offset: 48,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SkeletalUniforms {
    pub view_proj: [[f32; 4]; 4],
    pub light_dir: [f32; 4], // w is ambient intensity
    pub light_color: [f32; 4],
    pub bone_matrices: [[[f32; 4]; 4]; 128],
}

pub const SKELETAL_SHADER_WGSL: &str = r#"
struct SkeletalUniforms {
    view_proj: mat4x4<f32>,
    light_dir: vec4<f32>,
    light_color: vec4<f32>,
    bone_matrices: array<mat4x4<f32>, 128>,
}

@group(0) @binding(0)
var<uniform> uniforms: SkeletalUniforms;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
    @location(3) bone_indices: vec4<u32>,
    @location(4) bone_weights: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) normal: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var skinned_pos = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var skinned_normal = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    for (var i = 0u; i < 4u; i = i + 1u) {
        let bone_index = model.bone_indices[i];
        let weight = model.bone_weights[i];
        
        let bone_matrix = uniforms.bone_matrices[bone_index];
        
        let local_pos = vec4<f32>(model.position, 1.0);
        skinned_pos = skinned_pos + (bone_matrix * local_pos) * weight;
        
        let local_normal = vec4<f32>(model.normal, 0.0);
        skinned_normal = skinned_normal + (bone_matrix * local_normal) * weight;
    }

    out.clip_position = uniforms.view_proj * vec4<f32>(skinned_pos.xyz, 1.0);
    out.tex_coord = model.tex_coord;
    out.normal = normalize(skinned_normal.xyz);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = textureSample(t_diffuse, s_diffuse, in.tex_coord);
    
    let normal = normalize(in.normal);
    let light_dir = normalize(uniforms.light_dir.xyz);
    let diffuse_factor = max(dot(normal, light_dir), 0.0);
    
    let ambient_intensity = uniforms.light_dir.w;
    let ambient_light = uniforms.light_color.xyz * ambient_intensity;
    let diffuse_light = uniforms.light_color.xyz * diffuse_factor;
    
    let final_color = base_color.xyz * (ambient_light + diffuse_light);
    
    return vec4<f32>(final_color, base_color.a);
}
"#;

pub struct WgpuSkeletalMeshRenderer {
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl WgpuSkeletalMeshRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            ],
            label: Some("Skeletal Uniform Bind Group Layout"),
        });

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Skeletal Texture Bind Group Layout"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Skeletal Render Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bind_group_layout), Some(&texture_bind_group_layout)],
            immediate_size: 0,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skeletal Shader"),
            source: wgpu::ShaderSource::Wgsl(SKELETAL_SHADER_WGSL.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skeletal Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(SkeletalVertex::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
            multiview_mask: None,
        });

        Self {
            render_pipeline,
            uniform_bind_group_layout,
            texture_bind_group_layout,
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        uniform_bind_group: &'a wgpu::BindGroup,
        texture_bind_group: &'a wgpu::BindGroup,
        num_indices: u32,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, uniform_bind_group, &[]);
        render_pass.set_bind_group(1, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
    }
}

pub fn get_attachment_bone_matrix(
    uniforms: &SkeletalUniforms,
    bone_name: &str,
    bone_names: &[&str],
) -> Option<[[f32; 4]; 4]> {
    let index = bone_names.iter().position(|&name| name == bone_name)?;
    if index < 128 {
        Some(uniforms.bone_matrices[index])
    } else {
        None
    }
}

pub fn get_weapon_attachment_matrix(
    uniforms: &SkeletalUniforms,
    bone_names: &[&str],
) -> Option<[[f32; 4]; 4]> {
    get_attachment_bone_matrix(uniforms, "Bip01 R Hand", bone_names)
}

pub fn get_hair_attachment_matrix(
    uniforms: &SkeletalUniforms,
    bone_names: &[&str],
) -> Option<[[f32; 4]; 4]> {
    get_attachment_bone_matrix(uniforms, "Bip01 Head", bone_names)
}
