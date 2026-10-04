use bytemuck::{Pod, Zeroable};

use crate::wgpu_granny_actor_renderer::GrannyActorVertex;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DualTextureArmorUniforms {
    pub view_proj: [[f32; 4]; 4],
    pub view_matrix: [[f32; 4]; 4],
    pub bone_matrices: [[[f32; 4]; 4]; 128],
    pub specular_trans: [f32; 3],
    pub specular_power: f32,
}

pub struct WgpuDualTextureArmorPipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

pub const DUAL_TEXTURE_ARMOR_SHADER_WGSL: &str = r#"
struct DualTextureArmorUniforms {
    view_proj: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    bone_matrices: array<mat4x4<f32>, 128>,
    specular_trans: vec3<f32>,
    specular_power: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: DualTextureArmorUniforms;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@group(1) @binding(2)
var t_env: texture_2d<f32>;
@group(1) @binding(3)
var s_env: sampler;

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
    @location(1) env_coord: vec2<f32>,
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

    // Reflection calculation for env map (Sphere Mapping)
    // Equivalent of D3DTSS_TCI_CAMERASPACEREFLECTIONVECTOR
    let view_pos_vec = uniforms.view_matrix * vec4<f32>(skinned_pos.xyz, 1.0);
    let view_normal_vec = uniforms.view_matrix * vec4<f32>(normalize(skinned_normal.xyz), 0.0);
    
    let view_pos = normalize(view_pos_vec.xyz);
    let view_normal = normalize(view_normal_vec.xyz);
    
    let reflection = reflect(view_pos, view_normal);
    
    // Simple sphere map projection with specular translation
    var env_u = reflection.x * 0.5 + 0.5 + uniforms.specular_trans.x;
    var env_v = -reflection.y * 0.5 + 0.5 + uniforms.specular_trans.y;
    out.env_coord = vec2<f32>(env_u, env_v);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let diffuse_color = textureSample(t_diffuse, s_diffuse, in.tex_coord);
    let env_color = textureSample(t_env, s_env, in.env_coord);
    
    // D3DTOP_MODULATEALPHA_ADDCOLOR
    // Blend with specular_power and diffuse alpha
    let spec_factor = diffuse_color.a * uniforms.specular_power;
    let final_color = diffuse_color.rgb + (env_color.rgb * spec_factor);
    
    return vec4<f32>(final_color, diffuse_color.a);
}
"#;

impl WgpuDualTextureArmorPipeline {
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
            label: Some("DualTextureArmor Uniform Bind Group Layout"),
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("DualTextureArmor Texture Bind Group Layout"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("DualTextureArmor Render Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bind_group_layout), Some(&texture_bind_group_layout)],
            immediate_size: 0,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("DualTextureArmor Shader"),
            source: wgpu::ShaderSource::Wgsl(DUAL_TEXTURE_ARMOR_SHADER_WGSL.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("DualTextureArmor Render Pipeline"),
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
