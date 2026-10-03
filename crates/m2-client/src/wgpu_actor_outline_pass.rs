use bytemuck::{Pod, Zeroable};
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OutlineVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl OutlineVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<OutlineVertex>() as wgpu::BufferAddress,
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
            ],
        }
    }
}

// WGSL Memory Layout Rules:
// mat4x4<f32>: size 64, align 16
// vec4<f32>: size 16, align 16
// f32: size 4, align 4
// padding: added to align to 16 bytes.
//
// In WGSL:
// view_proj: offset 0 (size 64)
// model: offset 64 (size 64)
// outline_color: offset 128 (size 16)
// outline_thickness: offset 144 (size 4)
// -> total active size: 148 bytes. WGPU usually aligns uniform blocks to 16 bytes.
// So size must be 160.

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OutlineUniforms {
    pub view_proj: [[f32; 4]; 4],  // 64 bytes (offset 0)
    pub model: [[f32; 4]; 4],      // 64 bytes (offset 64)
    pub outline_color: [f32; 4],   // 16 bytes (offset 128)
    pub outline_thickness: f32,    // 4 bytes  (offset 144)
    pub padding: [f32; 3],         // 12 bytes (offset 148) -> Total 160 bytes.
}

pub const OUTLINE_SHADER_WGSL: &str = r#"
struct OutlineUniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    outline_color: vec4<f32>,
    outline_thickness: f32,
    // The padding is implicitly added to 16 bytes align the struct
}

@group(0) @binding(0)
var<uniform> uniforms: OutlineUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Extrude position along normal based on thickness
    let extruded_pos = model.position + (model.normal * uniforms.outline_thickness);
    let world_pos = uniforms.model * vec4<f32>(extruded_pos, 1.0);
    
    out.clip_position = uniforms.view_proj * world_pos;
    
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return uniforms.outline_color;
}
"#;

pub struct WgpuActorOutlinePass {
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
}

impl WgpuActorOutlinePass {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
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
            label: Some("Actor Outline Uniform Bind Group Layout"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Actor Outline Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bind_group_layout)],
            immediate_size: 0,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Actor Outline Shader"),
            source: wgpu::ShaderSource::Wgsl(OUTLINE_SHADER_WGSL.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Actor Outline Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(OutlineVertex::desc())],
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
                // Cull Front faces so we only see the back faces which form the outline
                cull_mode: Some(wgpu::Face::Front),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                // Do not write to depth buffer, to avoid interfering with subsequent passes
                depth_write_enabled: Some(false),
                // Outline should be drawn behind the object, so we check if depth is less or equal
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
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
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        uniform_bind_group: &'a wgpu::BindGroup,
        num_indices: u32,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FfiOutlinePassInfo {
    pub outlines_drawn: u32,
}

impl WgpuActorOutlinePass {
    pub fn ffi_get_info() -> FfiOutlinePassInfo {
        FfiOutlinePassInfo {
            outlines_drawn: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_outline_pass_get_info() -> FfiOutlinePassInfo {
    WgpuActorOutlinePass::ffi_get_info()
}
