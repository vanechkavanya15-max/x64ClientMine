#[path = "../src/skeletal_bone_palette_uploader.rs"]
mod skeletal_bone_palette_uploader;

use skeletal_bone_palette_uploader::{SkeletalBonePaletteUploader, MAX_BONE_MATRICES};
use glam::{Mat4, Vec3};

async fn setup_wgpu_env() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: Default::default(),
        backend_options: Default::default(),
        memory_budget_thresholds: Default::default(),
        display: None,
    });
    
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: None,
            apply_limit_buckets: Default::default(),
        })
        .await
        .expect("Failed to find wgpu adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
                experimental_features: Default::default(),
            }
        )
        .await
        .expect("Failed to create wgpu device");

    (device, queue)
}

#[test]
fn test_initial_state() {
    let uploader = SkeletalBonePaletteUploader::new();
    let identity = Mat4::IDENTITY.to_cols_array_2d();

    for i in 0..MAX_BONE_MATRICES {
        assert_eq!(uploader.bone_matrices[i], identity, "Matrix {} should be identity", i);
    }
}

#[test]
fn test_update_palette_partial() {
    let mut uploader = SkeletalBonePaletteUploader::new();
    
    let mat1 = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
    let mat2 = Mat4::from_rotation_x(std::f32::consts::PI / 4.0);
    
    let matrices = vec![mat1, mat2];
    
    uploader.update_palette(&matrices);
    
    assert_eq!(uploader.bone_matrices[0], mat1.to_cols_array_2d());
    assert_eq!(uploader.bone_matrices[1], mat2.to_cols_array_2d());
    
    let identity = Mat4::IDENTITY.to_cols_array_2d();
    for i in 2..MAX_BONE_MATRICES {
        assert_eq!(uploader.bone_matrices[i], identity, "Matrix {} should be identity after partial update", i);
    }
}

#[test]
fn test_update_palette_overflow() {
    let mut uploader = SkeletalBonePaletteUploader::new();
    
    let mut large_matrices = Vec::new();
    for i in 0..(MAX_BONE_MATRICES + 10) {
        large_matrices.push(Mat4::from_translation(Vec3::new(i as f32, 0.0, 0.0)));
    }
    
    uploader.update_palette(&large_matrices);
    
    for i in 0..MAX_BONE_MATRICES {
        let expected = Mat4::from_translation(Vec3::new(i as f32, 0.0, 0.0)).to_cols_array_2d();
        assert_eq!(uploader.bone_matrices[i], expected, "Matrix {} did not match expected value on overflow", i);
    }
}

#[test]
fn test_update_palette_empty() {
    let mut uploader = SkeletalBonePaletteUploader::new();
    
    let mat1 = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
    uploader.update_palette(&vec![mat1]);
    
    // update with empty slice should clear to identity
    uploader.update_palette(&[]);
    
    let identity = Mat4::IDENTITY.to_cols_array_2d();
    for i in 0..MAX_BONE_MATRICES {
        assert_eq!(uploader.bone_matrices[i], identity, "Matrix {} should be identity after empty update", i);
    }
}

#[test]
fn test_upload_to_gpu_buffer() {
    pollster::block_on(async {
        let (device, queue) = setup_wgpu_env().await;
        
        let uploader = SkeletalBonePaletteUploader::new();
        
        let buffer_size = std::mem::size_of_val(&uploader.bone_matrices) as wgpu::BufferAddress;
        
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Bone Palette Test Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        
        uploader.upload(&queue, &buffer, 0);
        
        let read_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(&buffer, 0, &read_buffer, 0, buffer_size);
        let submission_index = queue.submit(Some(encoder.finish()));
        
        let buffer_slice = read_buffer.slice(..);
        let (sender, receiver) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        
        device.poll(wgpu::PollType::Wait {
            submission_index: Some(submission_index),
            timeout: None,
        }).unwrap();
        
        receiver.await.unwrap().unwrap();
        
        {
            let data = buffer_slice.get_mapped_range().unwrap();
            let expected_data = bytemuck::cast_slice(&uploader.bone_matrices);
            
            assert_eq!(&*data, expected_data, "Buffer data did not match expected identity matrices");
        }
        read_buffer.unmap();
    });
}
