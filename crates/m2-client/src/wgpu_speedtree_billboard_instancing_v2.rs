use bytemuck::{Pod, Zeroable};

// Constants reproducing SpeedTreeConfig.h exact values
pub const C_F_NEAR_LOD_FACTOR: f32 = 2.0;
pub const C_F_FAR_LOD_FACTOR: f32 = 9.0;
pub const C_N_DEFAULT_ALPHA_TEST_VALUE: u32 = 84;
pub const C_N_NUM_WIND_MATRICES: usize = 4;

/// Represents the SBillboardVertex from SpeedTreeWrapper.cpp
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SBillboardVertex {
    pub f_x: f32,
    pub f_y: f32,
    pub f_z: f32,
    pub f_u: f32,
    pub f_v: f32,
}

impl SBillboardVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SBillboardVertex>() as wgpu::BufferAddress,
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
            ],
        }
    }
}

/// Represents instance data for each billboard (e.g. its world position and instance-specific parameters)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SBillboardInstance {
    pub position: [f32; 3],
    pub scale: f32,
    pub tree_height: f32,
    pub wind_offset: f32, // Optional: used for wind phase
}

impl SBillboardInstance {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SBillboardInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SpeedTreeBillboardGlobals {
    pub view_proj: [f32; 16],
    pub camera_pos: [f32; 3],
    pub _pad0: f32, // Padding for 16-byte alignment
    pub wind_matrices: [[f32; 16]; 4],
}

pub struct SpeedTreeBillboardManager {
    pub instance_buffer: Option<wgpu::Buffer>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub uniform_buffer: Option<wgpu::Buffer>,
    pub num_instances: u32,
    pub num_indices: u32,
    pub render_pipeline: Option<wgpu::RenderPipeline>,
    pub bind_group: Option<wgpu::BindGroup>,
    pub uniform_bind_group: Option<wgpu::BindGroup>,
}

impl SpeedTreeBillboardManager {
    pub fn new() -> Self {
        Self {
            instance_buffer: None,
            vertex_buffer: None,
            index_buffer: None,
            uniform_buffer: None,
            num_instances: 0,
            num_indices: 0,
            render_pipeline: None,
            bind_group: None,
            uniform_bind_group: None,
        }
    }

    /// Initializes WGPU resources using the provided device and bind group layout
    pub fn init(
        &mut self,
        device: &wgpu::Device,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        texture_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) {
        use wgpu::util::DeviceExt;

        // Texture Bind Group
        self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
            label: Some("SpeedTree Billboard Texture Bind Group"),
        }));

        // Globals Uniform Buffer and Bind Group
        let initial_globals = SpeedTreeBillboardGlobals {
            view_proj: [0.0; 16],
            camera_pos: [0.0; 3],
            _pad0: 0.0,
            wind_matrices: [[0.0; 16]; 4],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("SpeedTree Billboard Globals Buffer"),
            contents: bytemuck::bytes_of(&initial_globals),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("SpeedTree Billboard Globals Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        self.uniform_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SpeedTree Billboard Globals Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        }));

        self.uniform_buffer = Some(uniform_buffer);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("SpeedTree Billboard Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("wgpu_speedtree_billboard_shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("SpeedTree Billboard Pipeline Layout"),
            bind_group_layouts: &[Some(texture_bind_group_layout), Some(&uniform_bind_group_layout)],
            immediate_size: 0,
        });

        self.render_pipeline = Some(device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("SpeedTree Billboard Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(SBillboardVertex::desc()), Some(SBillboardInstance::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
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
            multiview_mask: None,
            cache: None,
        }));
    }

    /// Uploads vertex and index data for the static billboard geometry
    pub fn upload_geometry(
        &mut self,
        device: &wgpu::Device,
        vertices: &[SBillboardVertex],
        indices: &[u16],
    ) {
        use wgpu::util::DeviceExt;
        
        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Billboard Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }));
        
        self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Billboard Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        }));
        
        self.num_indices = indices.len() as u32;
    }

    /// Uploads instance data for all currently visible billboards
    pub fn upload_instances(
        &mut self,
        device: &wgpu::Device,
        instances: &[SBillboardInstance],
    ) {
        use wgpu::util::DeviceExt;
        
        if instances.is_empty() {
            self.num_instances = 0;
            return;
        }

        self.instance_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Billboard Instance Buffer"),
            contents: bytemuck::cast_slice(instances),
            usage: wgpu::BufferUsages::VERTEX,
        }));

        self.num_instances = instances.len() as u32;
    }

    /// Updates the global variables (ViewProj matrix, Camera position, and Wind matrices)
    pub fn update_globals(
        &mut self,
        queue: &wgpu::Queue,
        view_proj: [f32; 16],
        camera_pos: [f32; 3],
        wind_matrices: [[f32; 16]; 4],
    ) {
        if let Some(uniform_buffer) = &self.uniform_buffer {
            let globals = SpeedTreeBillboardGlobals {
                view_proj,
                camera_pos,
                _pad0: 0.0,
                wind_matrices,
            };
            queue.write_buffer(uniform_buffer, 0, bytemuck::bytes_of(&globals));
        }
    }

    /// Renders the billboards using the provided render pass
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.num_instances == 0 || self.num_indices == 0 {
            return;
        }
        
        if let (Some(pipeline), Some(bind_group), Some(uniform_bind_group), Some(v_buf), Some(i_buf), Some(inst_buf)) = (
            &self.render_pipeline,
            &self.bind_group,
            &self.uniform_bind_group,
            &self.vertex_buffer,
            &self.index_buffer,
            &self.instance_buffer,
        ) {
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.set_bind_group(1, uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, v_buf.slice(..));
            render_pass.set_vertex_buffer(1, inst_buf.slice(..));
            render_pass.set_index_buffer(i_buf.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_instances);
        }
    }

    /// Calculate the LOD level (or blend factor) based on distance
    /// Translating `m_pSpeedTree->SetLodLimits(fHeight * c_fNearLodFactor, fHeight * c_fFarLodFactor);`
    pub fn calculate_lod_factor(distance: f32, tree_height: f32) -> f32 {
        let near_limit = tree_height * C_F_NEAR_LOD_FACTOR;
        let far_limit = tree_height * C_F_FAR_LOD_FACTOR;

        if distance <= near_limit {
            return 1.0; // High LOD
        } else if distance >= far_limit {
            return 0.0; // Low LOD / billboard
        } else {
            // Linear fade between near and far limit
            return 1.0 - ((distance - near_limit) / (far_limit - near_limit));
        }
    }

    /// Calculates a wind sway factor for the billboard
    pub fn calculate_wind_sway(time: f32, wind_strength: f32, wind_offset: f32) -> f32 {
        // A simple oscillation formula to simulate wind logic.
        // We use the offset so that not all trees sway in perfect unison.
        (time * 2.0 + wind_offset).sin() * wind_strength * 0.1
    }
}
