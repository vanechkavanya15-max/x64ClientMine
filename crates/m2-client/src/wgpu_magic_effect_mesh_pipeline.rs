use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Quat, Vec3};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

pub const MESH_BILLBOARD_TYPE_NONE: u8 = 0;
pub const MESH_BILLBOARD_TYPE_ALL: u8 = 1;
pub const MESH_BILLBOARD_TYPE_Y: u8 = 2;
pub const MESH_BILLBOARD_TYPE_MOVE: u8 = 3;

// D3D9 Blend constants matching Metin2 codebase
pub const D3DBLEND_ZERO: u8 = 1;
pub const D3DBLEND_ONE: u8 = 2;
pub const D3DBLEND_SRCCOLOR: u8 = 3;
pub const D3DBLEND_INVSRCCOLOR: u8 = 4;
pub const D3DBLEND_SRCALPHA: u8 = 5;
pub const D3DBLEND_INVSRCALPHA: u8 = 6;
pub const D3DBLEND_DESTALPHA: u8 = 7;
pub const D3DBLEND_INVDESTALPHA: u8 = 8;
pub const D3DBLEND_DESTCOLOR: u8 = 9;
pub const D3DBLEND_INVDESTCOLOR: u8 = 10;
pub const D3DBLEND_SRCALPHASAT: u8 = 11;
pub const D3DBLEND_BOTHSRCALPHA: u8 = 12;
pub const D3DBLEND_BOTHINVSRCALPHA: u8 = 13;

pub fn d3d_blend_to_wgpu_factor(d3d_blend: u8) -> wgpu::BlendFactor {
    match d3d_blend {
        D3DBLEND_ZERO => wgpu::BlendFactor::Zero,
        D3DBLEND_ONE => wgpu::BlendFactor::One,
        D3DBLEND_SRCCOLOR => wgpu::BlendFactor::Src,
        D3DBLEND_INVSRCCOLOR => wgpu::BlendFactor::OneMinusSrc,
        D3DBLEND_SRCALPHA => wgpu::BlendFactor::SrcAlpha,
        D3DBLEND_INVSRCALPHA => wgpu::BlendFactor::OneMinusSrcAlpha,
        D3DBLEND_DESTALPHA => wgpu::BlendFactor::DstAlpha,
        D3DBLEND_INVDESTALPHA => wgpu::BlendFactor::OneMinusDstAlpha,
        D3DBLEND_DESTCOLOR => wgpu::BlendFactor::Dst,
        D3DBLEND_INVDESTCOLOR => wgpu::BlendFactor::OneMinusDst,
        D3DBLEND_SRCALPHASAT => wgpu::BlendFactor::SrcAlphaSaturated,
        _ => wgpu::BlendFactor::One, // fallback
    }
}

pub fn get_wgpu_blend_state(src_type: u8, dest_type: u8) -> wgpu::BlendState {
    wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: d3d_blend_to_wgpu_factor(src_type),
            dst_factor: d3d_blend_to_wgpu_factor(dest_type),
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: d3d_blend_to_wgpu_factor(src_type),
            dst_factor: d3d_blend_to_wgpu_factor(dest_type),
            operation: wgpu::BlendOperation::Add,
        },
    }
}

pub fn calculate_effect_mesh_world_matrix(
    billboard_type: u8,
    view_matrix: Mat4,
    mut world_matrix: Mat4,
    position: Vec3,
    last_position: Vec3,
) -> Mat4 {
    match billboard_type {
        MESH_BILLBOARD_TYPE_ALL => {
            let inv_view = view_matrix.inverse();
            // D3DXMatrixRotationX(&matTemp, 90.0f);
            let rot_x_90 = Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2);
            // D3DXMatrixInverse(&m_matWorld, NULL, &CScreen::GetViewMatrix());
            // m_matWorld = matTemp * m_matWorld;
            // The original logic:
            // matTemp = RotX(90)
            // m_matWorld = InvView
            // m_matWorld = matTemp * m_matWorld
            // Then it applies position later.
            // Let's replicate this rotation behavior

            // To strip translation from inv_view, we just take the upper 3x3
            let inv_view_rot = Mat4::from_quat(inv_view.to_scale_rotation_translation().1);

            world_matrix = rot_x_90 * inv_view_rot;
        }
        MESH_BILLBOARD_TYPE_Y => {
            let inv_view = view_matrix.inverse();
            // _11, _12, _21, _22 (D3D is row-major, Mat4 is column-major)
            // D3D _11, _12, _21, _22 corresponds to X and Y axes
            // Let's map it safely.
            let mut mat = Mat4::IDENTITY;
            mat.col_mut(0)[0] = inv_view.col(0)[0];
            mat.col_mut(0)[1] = inv_view.col(0)[1];
            mat.col_mut(1)[0] = inv_view.col(1)[0];
            mat.col_mut(1)[1] = inv_view.col(1)[1];
            world_matrix = mat;
        }
        MESH_BILLBOARD_TYPE_MOVE => {
            let mut move_vec = position - last_position;
            if move_vec.length_squared() > 0.001 {
                move_vec = move_vec.normalize();
                let up = Vec3::new(0.0, -1.0, 0.0);

                // SafeRotationNormalizedArc equivalent
                let dot = up.dot(move_vec);
                let q = if dot < -0.999999 {
                    // 180 degree rotation around any orthogonal axis
                    let mut axis = up.cross(Vec3::X);
                    if axis.length_squared() < 0.001 {
                        axis = up.cross(Vec3::Y);
                    }
                    Quat::from_axis_angle(axis.normalize(), std::f32::consts::PI)
                } else if dot > 0.999999 {
                    Quat::IDENTITY
                } else {
                    let axis = up.cross(move_vec).normalize();
                    let angle = dot.acos();
                    Quat::from_axis_angle(axis, angle)
                };

                world_matrix = Mat4::from_quat(q);
            }
        }
        _ => {} // MESH_BILLBOARD_TYPE_NONE or other
    }

    world_matrix
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct EffectMeshVertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl EffectMeshVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<EffectMeshVertex>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct EffectMeshUniforms {
    pub world_matrix: [[f32; 4]; 4],
    pub view_proj_matrix: [[f32; 4]; 4],
    pub color_factor: [f32; 4],
    pub color_op_type: u32,
    pub padding: [u32; 3],
}

pub struct WgpuMagicEffectMeshPipeline {
    bind_group_layout: wgpu::BindGroupLayout,
    pipelines: HashMap<String, wgpu::RenderPipeline>,
    shader_module: wgpu::ShaderModule,
}

impl WgpuMagicEffectMeshPipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader_src = r#"
            struct EffectMeshUniforms {
                world_matrix: mat4x4<f32>,
                view_proj_matrix: mat4x4<f32>,
                color_factor: vec4<f32>,
                color_op_type: u32,
                padding: vec3<u32>,
            };

            @group(0) @binding(0)
            var<uniform> uniforms: EffectMeshUniforms;

            @group(1) @binding(0)
            var t_diffuse: texture_2d<f32>;
            @group(1) @binding(1)
            var s_diffuse: sampler;

            struct VertexInput {
                @location(0) position: vec3<f32>,
                @location(1) tex_coord: vec2<f32>,
            };

            struct VertexOutput {
                @builtin(position) clip_position: vec4<f32>,
                @location(0) tex_coord: vec2<f32>,
            };

            @vertex
            fn vs_main(model: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                let world_pos = uniforms.world_matrix * vec4<f32>(model.position, 1.0);
                out.clip_position = uniforms.view_proj_matrix * world_pos;
                out.tex_coord = model.tex_coord;
                return out;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coord);
                
                // Apply color operation equivalent (D3DTOP_MODULATE is typical default, handled by multiplying)
                // Since color_op_type is not directly 1:1 mapped to WGSL without complex branching, 
                // we assume Modulate/Multiply as the standard behavior for D3D9 effects.
                let final_color = tex_color * uniforms.color_factor;
                
                return final_color;
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("EffectMesh Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("EffectMesh Uniforms Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(
                        std::mem::size_of::<EffectMeshUniforms>() as _,
                    ),
                },
                count: None,
            }],
        });

        Self {
            bind_group_layout,
            pipelines: HashMap::new(),
            shader_module,
        }
    }

    pub fn get_or_create_pipeline(
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
        blend_state: wgpu::BlendState,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> &wgpu::RenderPipeline {
        let key = format!("{:?}_{:?}", format, blend_state);

        if !self.pipelines.contains_key(&key) {
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("EffectMesh Pipeline Layout"),
                bind_group_layouts: &[Some(&self.bind_group_layout), Some(texture_bind_group_layout)],
            });

            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("EffectMesh Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &self.shader_module,
                    entry_point: Some("vs_main"),
                    buffers: &[Some(EffectMeshVertex::desc())],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.shader_module,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(blend_state),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None, // Effects usually render double-sided or disable culling
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: depth_format,
                    depth_write_enabled: Some(false), // Effects typically don't write depth
                    depth_compare: Some(wgpu::CompareFunction::LessEqual),
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
            });

            self.pipelines.insert(key.clone(), pipeline);
        }

        self.pipelines.get(&key).unwrap()
    }
}
