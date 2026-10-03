use m2_client::wgpu_skeletal_mesh_renderer::{
    SkeletalVertex, SkeletalUniforms, WgpuSkeletalMeshRenderer,
    get_attachment_bone_matrix, get_weapon_attachment_matrix, get_hair_attachment_matrix
};
use std::mem;

#[test]
fn test_vertex_layout() {
    let size = mem::size_of::<SkeletalVertex>();
    // 3 * 4 + 3 * 4 + 2 * 4 + 4 * 4 + 4 * 4 = 12 + 12 + 8 + 16 + 16 = 64
    assert_eq!(size, 64);
    
    let desc = SkeletalVertex::desc();
    assert_eq!(desc.array_stride, 64);
    assert_eq!(desc.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(desc.attributes.len(), 5);
}

#[test]
fn test_uniforms_layout() {
    let size = mem::size_of::<SkeletalUniforms>();
    // view_proj (16 * 4) + light_dir (4 * 4) + light_color (4 * 4) + bone_matrices (128 * 16 * 4)
    // = 64 + 16 + 16 + 8192 = 8288
    assert_eq!(size, 8288);
}

#[test]
fn test_bone_attachment_logic() {
    let mut uniforms = SkeletalUniforms {
        view_proj: [[0.0; 4]; 4],
        light_dir: [0.0; 4],
        light_color: [0.0; 4],
        bone_matrices: [[[0.0; 4]; 4]; 128],
    };

    let test_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [10.0, 20.0, 30.0, 1.0],
    ];
    uniforms.bone_matrices[5] = test_matrix;
    
    let head_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 50.0, 0.0, 1.0],
    ];
    uniforms.bone_matrices[2] = head_matrix;

    let bone_names = vec![
        "Root", "Pelvis", "Bip01 Head", "Spine", "Arm", "Bip01 R Hand"
    ];

    let result = get_attachment_bone_matrix(&uniforms, "Bip01 R Hand", &bone_names);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), test_matrix);

    let weapon_result = get_weapon_attachment_matrix(&uniforms, &bone_names);
    assert!(weapon_result.is_some());
    assert_eq!(weapon_result.unwrap(), test_matrix);

    let hair_result = get_hair_attachment_matrix(&uniforms, &bone_names);
    assert!(hair_result.is_some());
    assert_eq!(hair_result.unwrap(), head_matrix);

    let invalid_result = get_attachment_bone_matrix(&uniforms, "NonExistentBone", &bone_names);
    assert!(invalid_result.is_none());
}

#[test]
fn test_attachment_bounds_checking() {
    let mut uniforms = SkeletalUniforms {
        view_proj: [[0.0; 4]; 4],
        light_dir: [0.0; 4],
        light_color: [0.0; 4],
        bone_matrices: [[[0.0; 4]; 4]; 128],
    };

    let mut bone_names = vec![];
    for i in 0..150 {
        let name = format!("Bone{}", i);
        bone_names.push(name);
    }
    let string_slice: Vec<&str> = bone_names.iter().map(|s| s.as_str()).collect();

    // Valid index
    let result = get_attachment_bone_matrix(&uniforms, "Bone50", &string_slice);
    assert!(result.is_some());

    // Out of bounds index
    let out_of_bounds = get_attachment_bone_matrix(&uniforms, "Bone130", &string_slice);
    assert!(out_of_bounds.is_none());
}

#[tokio::test]
async fn test_pipeline_creation() {
    let instance = wgpu::Instance::default();
    let adapter = match instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
        ..Default::default()
    }).await {
        Ok(a) => a,
        Err(_) => {
            println!("Skipping wgpu test, no adapter found.");
            return;
        }
    };

    let (device, _queue) = match adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        ..Default::default()
    }).await {
        Ok(dq) => dq,
        Err(_) => {
            println!("Skipping wgpu test, device request failed.");
            return;
        }
    };

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: 800,
        height: 600,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
        color_space: wgpu::SurfaceColorSpace::Auto,
    };

    let _renderer = WgpuSkeletalMeshRenderer::new(&device, &config);
    assert!(true);
}
