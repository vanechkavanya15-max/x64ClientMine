use std::mem;

use crate::lens_sun_flare_v2::{self, FlarePiece};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct FlareVertex {
    pub position: [f32; 2],
    pub tex_coord: [f32; 2],
    pub color: [f32; 4],
}

impl FlareVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<FlareVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 2]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct LensFlareScreenPipeline {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl LensFlareScreenPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("LensFlareScreenShader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str_or_fallback())),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("LensFlareScreenPipelineLayout"),
            bind_group_layouts: &[], // Typically texture bind group here, but simplified
            immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("LensFlareScreenRenderPipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(FlareVertex::desc())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::One, // Additive blending for flares
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::Zero,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // No depth testing for screen elements
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self { render_pipeline }
    }

    pub fn generate_vertices(
        pieces: &[FlarePiece],
        screen_width: f32,
        screen_height: f32,
    ) -> Vec<FlareVertex> {
        let mut vertices = Vec::with_capacity(pieces.len() * 6); // 6 vertices per piece (2 triangles)

        for piece in pieces {
            if piece.size <= 0.0 || piece.color_a <= 0.0 {
                continue;
            }

            let half_size = piece.size / 2.0;

            // Screen coords (origin top-left)
            let x0 = piece.position_x - half_size;
            let y0 = piece.position_y - half_size;
            let x1 = piece.position_x + half_size;
            let y1 = piece.position_y + half_size;

            // Convert screen coords to NDC [-1, 1]
            let ndc_x0 = (x0 / screen_width) * 2.0 - 1.0;
            let ndc_y0 = 1.0 - (y0 / screen_height) * 2.0;
            let ndc_x1 = (x1 / screen_width) * 2.0 - 1.0;
            let ndc_y1 = 1.0 - (y1 / screen_height) * 2.0;

            let color = [piece.color_r, piece.color_g, piece.color_b, piece.color_a];

            // Top Left
            let tl = FlareVertex {
                position: [ndc_x0, ndc_y0],
                tex_coord: [0.0, 0.0],
                color,
            };
            // Bottom Left
            let bl = FlareVertex {
                position: [ndc_x0, ndc_y1],
                tex_coord: [0.0, 1.0],
                color,
            };
            // Bottom Right
            let br = FlareVertex {
                position: [ndc_x1, ndc_y1],
                tex_coord: [1.0, 1.0],
                color,
            };
            // Top Right
            let tr = FlareVertex {
                position: [ndc_x1, ndc_y0],
                tex_coord: [1.0, 0.0],
                color,
            };

            // First triangle
            vertices.push(tl);
            vertices.push(bl);
            vertices.push(br);

            // Second triangle
            vertices.push(tl);
            vertices.push(br);
            vertices.push(tr);
        }

        vertices
    }
}

fn include_str_or_fallback() -> &'static str {
    "
    struct VertexInput {
        @location(0) position: vec2<f32>,
        @location(1) tex_coord: vec2<f32>,
        @location(2) color: vec4<f32>,
    };

    struct VertexOutput {
        @builtin(position) clip_position: vec4<f32>,
        @location(0) tex_coord: vec2<f32>,
        @location(1) color: vec4<f32>,
    };

    @vertex
    fn vs_main(model: VertexInput) -> VertexOutput {
        var out: VertexOutput;
        out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
        out.tex_coord = model.tex_coord;
        out.color = model.color;
        return out;
    }

    @fragment
    fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
        // Just use color for now, since texture binding isn't fully implemented in this module
        return in.color;
    }
    "
}
