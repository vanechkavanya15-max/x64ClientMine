use glam::{Mat4, Vec3};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct WgpuShadowMapPipeline {
    pub shadow_map_size: u16,
    pub recreate: AtomicBool,
    pub light_view_matrix: Mat4,
    pub light_proj_matrix: Mat4,
}

impl Default for WgpuShadowMapPipeline {
    fn default() -> Self {
        Self {
            shadow_map_size: 512,
            recreate: AtomicBool::new(false),
            light_view_matrix: Mat4::IDENTITY,
            light_proj_matrix: Mat4::IDENTITY,
        }
    }
}

pub const CHARACTER_SHADOW_SHADER: &str = r#"
struct ShadowUniforms {
    light_view_proj: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: ShadowUniforms;
@group(0) @binding(1) var shadow_map: texture_depth_2d;
@group(0) @binding(2) var shadow_sampler: sampler_comparison;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.light_view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) {
    // Depth-only render pass writes automatically.
}
"#;

impl WgpuShadowMapPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_shadow_texture_size(&mut self, size: u16) {
        if self.shadow_map_size != size {
            self.recreate.store(true, Ordering::SeqCst);
        }
        self.shadow_map_size = size;
    }

    pub fn create_depth_texture(device: &wgpu::Device, size: u16) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
        let size_ext = wgpu::Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Character Shadow Map Depth Texture"),
            size: size_ext,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Character Shadow PCF Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        (texture, view, sampler)
    }

    /// Calculates Light View & Proj matrices matching `CMapOutdoor::BeginRenderCharacterShadowToTexture` exactly.
    pub fn calculate_light_matrices(&mut self, target: Vec3) {
        // v3Eye(v3Target.x - 1.732f * 1250.0f,
        //       v3Target.y - 1250.0f,
        //       v3Target.z + 2.0f * 1.732f * 1250.0f);
        let eye = Vec3::new(
            target.x - 1.732 * 1250.0,
            target.y - 1250.0,
            target.z + 2.0 * 1.732 * 1250.0,
        );
        let up = Vec3::new(0.0, 0.0, 1.0);

        // D3DXMatrixLookAtRH
        self.light_view_matrix = Mat4::look_at_rh(eye, target, up);

        // D3DXMatrixOrthoRH(w=2550.0, h=2550.0, zn=1.0, zf=15000.0)
        let w = 2550.0;
        let h = 2550.0;
        let zn = 1.0;
        let zf = 15000.0;

        self.light_proj_matrix = Mat4::orthographic_rh(
            -w / 2.0, w / 2.0,
            -h / 2.0, h / 2.0,
            zn, zf,
        );
    }
}
