use bytemuck::{Pod, Zeroable};
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GrannyActorVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
    pub bone_indices: [u32; 4],
    pub bone_weights: [f32; 4],
}

impl GrannyActorVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<GrannyActorVertex>() as wgpu::BufferAddress,
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
pub struct ActorUniforms {
    pub view_proj: [[f32; 4]; 4],
    pub bone_matrices: [[[f32; 4]; 4]; 128],
}

pub struct WgpuGrannyActorRenderer {
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

pub const ACTOR_SHADER_WGSL: &str = r#"
struct ActorUniforms {
    view_proj: mat4x4<f32>,
    bone_matrices: array<mat4x4<f32>, 128>,
}

@group(0) @binding(0)
var<uniform> uniforms: ActorUniforms;

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

    // Skinning
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
    let object_color = textureSample(t_diffuse, s_diffuse, in.tex_coord);
    return object_color;
}
"#;

impl WgpuGrannyActorRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("Actor Uniform Bind Group Layout"),
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
            label: Some("Actor Texture Bind Group Layout"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Actor Render Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bind_group_layout), Some(&texture_bind_group_layout)],
            immediate_size: 0,
                    });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Actor Shader"),
            source: wgpu::ShaderSource::Wgsl(ACTOR_SHADER_WGSL.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Actor Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(GrannyActorVertex::desc())],
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
}

impl WgpuGrannyActorRenderer {
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