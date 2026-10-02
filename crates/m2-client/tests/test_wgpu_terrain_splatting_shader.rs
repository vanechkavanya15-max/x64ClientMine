use m2_client::wgpu_terrain_splatting_shader::{TerrainSplattingPipeline, TerrainUniforms};
use bytemuck::{Pod, Zeroable};

#[test]
fn test_terrain_uniforms_size() {
    // 4x4 matrix = 16 f32s = 64 bytes
    // sun_dir = 4 f32s = 16 bytes
    // sun_color = 4 f32s = 16 bytes
    // ambient_color = 4 f32s = 16 bytes
    // fog_color = 4 f32s = 16 bytes
    // fog_params = 4 f32s = 16 bytes
    // camera_pos = 4 f32s = 16 bytes
    // total = 160 bytes
    assert_eq!(std::mem::size_of::<TerrainUniforms>(), 160);
}

#[test]
fn test_terrain_uniforms_alignment() {
    assert_eq!(std::mem::align_of::<TerrainUniforms>(), 4);
}

#[test]
fn test_terrain_splat_blending_math() {
    let c0 = [0.2, 0.4, 0.2]; // Grass
    let c1 = [0.5, 0.3, 0.1]; // Soil
    let c2 = [0.8, 0.8, 0.8]; // Rock
    let c3 = [0.9, 0.8, 0.5]; // Sand
    
    let splat = [0.5, 0.3, 0.1, 0.1];
    
    let blended_r: f32 = c0[0] * splat[0] + c1[0] * splat[1] + c2[0] * splat[2] + c3[0] * splat[3];
    let blended_g: f32 = c0[1] * splat[0] + c1[1] * splat[1] + c2[1] * splat[2] + c3[1] * splat[3];
    let blended_b: f32 = c0[2] * splat[0] + c1[2] * splat[1] + c2[2] * splat[2] + c3[2] * splat[3];
    
    assert!((blended_r - 0.42).abs() < 1e-5);
    assert!((blended_g - 0.45).abs() < 1e-5);
    assert!((blended_b - 0.26).abs() < 1e-5);
}

#[test]
fn test_distance_fog_math() {
    let fog_start = 100.0;
    let fog_end = 200.0;
    let fog_color = [0.5, 0.6, 0.7];
    let lit_color = [0.1, 0.8, 0.2];

    // Distance exactly halfway
    let dist = 150.0;
    let fog_factor: f32 = (dist - fog_start) / (fog_end - fog_start);
    let fog_factor = fog_factor.clamp(0.0, 1.0);
    assert!((fog_factor - 0.5).abs() < 1e-5);

    let final_r: f32 = lit_color[0] * (1.0 - fog_factor) + fog_color[0] * fog_factor;
    let final_g: f32 = lit_color[1] * (1.0 - fog_factor) + fog_color[1] * fog_factor;
    let final_b: f32 = lit_color[2] * (1.0 - fog_factor) + fog_color[2] * fog_factor;

    assert!((final_r - 0.3).abs() < 1e-5);
    assert!((final_g - 0.7).abs() < 1e-5);
    assert!((final_b - 0.45).abs() < 1e-5);

    // Distance beyond fog
    let dist = 300.0;
    let mut fog_factor: f32 = (dist - fog_start) / (fog_end - fog_start); fog_factor = fog_factor.clamp(0.0, 1.0);
    assert!((fog_factor - 1.0).abs() < 1e-5);

    // Distance before fog
    let dist = 50.0;
    let mut fog_factor: f32 = (dist - fog_start) / (fog_end - fog_start); fog_factor = fog_factor.clamp(0.0, 1.0);
    assert!((fog_factor - 0.0).abs() < 1e-5);
}

#[test]
fn test_lighting_math_diffuse() {
    let normal = [0.0, 1.0, 0.0];
    let sun_dir = [0.0, -1.0, 0.0]; // Sun shining directly down
    let sun_color = [1.0, 1.0, 1.0];
    let ambient_color = [0.2, 0.2, 0.2];
    let albedo = [0.5, 0.5, 0.5];

    let dot_nl: f32 = normal[0] * (-sun_dir[0]) + normal[1] * (-sun_dir[1]) + normal[2] * (-sun_dir[2]);
    let diffuse_factor = dot_nl.max(0.0);
    assert!((diffuse_factor - 1.0).abs() < 1e-5);

    let lit_r = albedo[0] * (ambient_color[0] + sun_color[0] * diffuse_factor);
    assert!((lit_r - 0.6).abs() < 1e-5); // 0.5 * (0.2 + 1.0 * 1.0) = 0.6
}

#[test]
fn test_lighting_math_specular() {
    let normal = [0.0, 1.0, 0.0];
    let sun_dir = [0.0, -1.0, 0.0]; // pointing straight down
    let view_dir = [0.0, 1.0, 0.0]; // looking straight down
    let sun_color = [1.0, 1.0, 1.0];
    
    // Half vector
    let mut h = [(-sun_dir[0]) + view_dir[0], (-sun_dir[1]) + view_dir[1], (-sun_dir[2]) + view_dir[2]];
    let sum: f32 = h[0] * h[0] + h[1] * h[1] + h[2] * h[2]; let len: f32 = sum.sqrt();
    h[0] /= len;
    h[1] /= len;
    h[2] /= len;
    
    let dot_nh: f32 = normal[0] * h[0] + normal[1] * h[1] + normal[2] * h[2];
    let shininess = 16.0;
    let specular_factor = dot_nh.max(0.0).powf(shininess);
    assert!((specular_factor - 1.0).abs() < 1e-5);
    
    let specular_r = sun_color[0] * specular_factor * 0.1; // 0.1 is the specular strength in shader
    assert!((specular_r - 0.1).abs() < 1e-5);
}

#[test]
fn test_terrain_uniforms_default_zeroable() {
    let uniforms = TerrainUniforms::zeroed();
    assert_eq!(uniforms.view_proj, [[0.0; 4]; 4]);
    assert_eq!(uniforms.sun_dir, [0.0; 4]);
    assert_eq!(uniforms.fog_params, [0.0; 4]);
}

#[tokio::test]
async fn test_shader_compilation() {
    let instance = wgpu::Instance::default();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await;
        
        
        
    
    
    // We can only test this if an adapter is available (e.g. headless CI container might not have a GPU, 
    // but wgpu can often fallback to Lavapipe or similar if configured).
    if let Ok(adapter) = adapter {
        let (device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();
        
        // This will panic if the shader is invalid
        let _pipeline = TerrainSplattingPipeline::new(&device, wgpu::TextureFormat::Rgba8Unorm);
    }
}
