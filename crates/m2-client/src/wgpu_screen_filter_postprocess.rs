use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use crate::wgpu_magic_effect_mesh_pipeline::{get_wgpu_blend_state, D3DBLEND_SRCALPHA, D3DBLEND_INVSRCALPHA};

const SCREEN_FILTER_SHADER: &str = r#"
struct Uniforms {
    color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((in_vertex_index << 1u) & 2u);
    let y = f32(in_vertex_index & 2u);
    out.position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return uniforms.color;
}
"#;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ScreenFilterUniform {
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterState {
    None,
    Flash,
    FadeIn,
    FadeOut,
}

pub struct WgpuScreenFilter {
    pub enabled: bool,
    pub src_blend: u8,
    pub dest_blend: u8,
    pub color: [f32; 4],
    
    // State management for effects
    pub state: FilterState,
    pub current_time: f32,
    pub duration: f32,
    pub start_color: [f32; 4],
    pub target_color: [f32; 4],
    pub flash_revert_color: [f32; 4],

    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    shader_module: wgpu::ShaderModule,
}

impl WgpuScreenFilter {
    fn create_pipeline(
        device: &wgpu::Device,
        shader_module: &wgpu::ShaderModule,
        bind_group_layout: &wgpu::BindGroupLayout,
        src_blend: u8,
        dest_blend: u8,
        format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ScreenFilter Pipeline Layout"),
            bind_group_layouts: &[Some(bind_group_layout)],
            immediate_size: 0,
        });

        let blend_state = get_wgpu_blend_state(src_blend, dest_blend);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ScreenFilter Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(blend_state),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        })
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ScreenFilter Shader"),
            source: wgpu::ShaderSource::Wgsl(SCREEN_FILTER_SHADER.into()),
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ScreenFilter Uniform Buffer"),
            contents: bytemuck::cast_slice(&[ScreenFilterUniform {
                color: [0.0, 0.0, 0.0, 0.0],
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ScreenFilter Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ScreenFilter Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline = Self::create_pipeline(
            device,
            &shader_module,
            &bind_group_layout,
            D3DBLEND_SRCALPHA,
            D3DBLEND_INVSRCALPHA,
            format,
        );

        Self {
            enabled: false,
            src_blend: D3DBLEND_SRCALPHA,
            dest_blend: D3DBLEND_INVSRCALPHA,
            color: [0.0, 0.0, 0.0, 0.0],
            state: FilterState::None,
            current_time: 0.0,
            duration: 0.0,
            start_color: [0.0, 0.0, 0.0, 0.0],
            target_color: [0.0, 0.0, 0.0, 0.0],
            flash_revert_color: [0.0, 0.0, 0.0, 0.0],
            pipeline,
            uniform_buffer,
            bind_group,
            bind_group_layout,
            shader_module,
        }
    }

    pub fn set_enable(&mut self, enable: bool) {
        self.enabled = enable;
    }

    pub fn set_blend_type(
        &mut self,
        src_type: u8,
        dest_type: u8,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) {
        if self.src_blend == src_type && self.dest_blend == dest_type {
            return;
        }

        self.src_blend = src_type;
        self.dest_blend = dest_type;

        self.pipeline = Self::create_pipeline(
            device,
            &self.shader_module,
            &self.bind_group_layout,
            self.src_blend,
            self.dest_blend,
            format,
        );
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = [r, g, b, a];
    }

    pub fn flash(&mut self, r: f32, g: f32, b: f32, duration: f32) {
        self.state = FilterState::Flash;
        self.current_time = 0.0;
        self.duration = duration;
        self.flash_revert_color = self.color;
        self.start_color = [r, g, b, 1.0];
        self.color = self.start_color;
        self.enabled = true;
    }

    pub fn fade_in(&mut self, r: f32, g: f32, b: f32, duration: f32) {
        self.state = FilterState::FadeIn;
        self.current_time = 0.0;
        self.duration = duration;
        self.start_color = [r, g, b, 1.0];
        self.target_color = [r, g, b, 0.0];
        self.color = self.start_color;
        self.enabled = true;
    }

    pub fn fade_out(&mut self, r: f32, g: f32, b: f32, duration: f32) {
        self.state = FilterState::FadeOut;
        self.current_time = 0.0;
        self.duration = duration;
        self.start_color = [r, g, b, 0.0];
        self.target_color = [r, g, b, 1.0];
        self.color = self.start_color;
        self.enabled = true;
    }

    pub fn fade(&mut self, r: f32, g: f32, b: f32, target_alpha: f32, progress: f32) {
        let current_alpha = self.color[3];
        let new_alpha = current_alpha + (target_alpha - current_alpha) * progress.clamp(0.0, 1.0);
        self.set_color(r, g, b, new_alpha);
    }

    pub fn tint(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.set_color(r, g, b, a);
        self.enabled = a > 0.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.state == FilterState::None {
            return;
        }

        self.current_time += delta_time;

        if self.current_time >= self.duration {
            // Effect finished
            match self.state {
                FilterState::Flash => {
                    self.color = self.flash_revert_color;
                    self.state = FilterState::None;
                }
                FilterState::FadeIn => {
                    self.color = self.target_color;
                    self.state = FilterState::None;
                    self.enabled = false;
                }
                FilterState::FadeOut => {
                    self.color = self.target_color;
                    self.state = FilterState::None;
                }
                FilterState::None => {}
            }
            return;
        }

        let progress = (self.current_time / self.duration).clamp(0.0, 1.0);

        match self.state {
            FilterState::Flash => {
                for i in 0..4 {
                    self.color[i] = self.start_color[i] + (self.flash_revert_color[i] - self.start_color[i]) * progress;
                }
            }
            FilterState::FadeIn | FilterState::FadeOut => {
                for i in 0..4 {
                    self.color[i] = self.start_color[i] + (self.target_color[i] - self.start_color[i]) * progress;
                }
            }
            FilterState::None => {}
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, queue: &wgpu::Queue) {
        if !self.enabled {
            return;
        }

        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[ScreenFilterUniform {
                color: self.color,
            }]),
        );

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
