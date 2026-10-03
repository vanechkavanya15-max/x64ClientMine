#[path = "../src/map_object_instanced_renderer.rs"]
mod map_object_instanced_renderer;

use map_object_instanced_renderer::{InstanceData, MapObjectInstancedRenderer, MapObjectVertex};

async fn create_test_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: None,
            ..Default::default()
        })
        .await;

    if let Ok(adapter) = adapter {
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    ..Default::default()
                },
            )
            .await
            .expect("Failed to create device");
        return (device, queue);
    }
    
    panic!("No WGPU adapter available for testing");
}

fn skip_if_no_adapter() -> bool {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        ..Default::default()
    }));
    
    adapter.is_err()
}

#[test]
fn test_renderer_initialization() {
    let renderer = MapObjectInstancedRenderer::new();
    assert!(renderer.material_groups.is_empty());
}

#[test]
fn test_add_object_type_creates_groups() {
    if skip_if_no_adapter() { return; }
    
    let (device, _) = pollster::block_on(create_test_device());
    let mut renderer = MapObjectInstancedRenderer::new();
    
    let vertices = [MapObjectVertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        tex_coord: [0.0, 0.0],
    }];
    let indices = [0, 1, 2];
    
    let material_id = 10;
    let mesh_id = 5;
    
    renderer.add_object_type(&device, mesh_id, material_id, &vertices, &indices);
    
    assert!(renderer.material_groups.contains_key(&material_id));
    let material_group = renderer.material_groups.get(&material_id).unwrap();
    assert!(material_group.meshes.contains_key(&mesh_id));
}

#[test]
fn test_add_instance_success() {
    if skip_if_no_adapter() { return; }
    
    let (device, _) = pollster::block_on(create_test_device());
    let mut renderer = MapObjectInstancedRenderer::new();
    
    let vertices = [MapObjectVertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        tex_coord: [0.0, 0.0],
    }];
    let indices = [0, 1, 2];
    
    let material_id = 10;
    let mesh_id = 5;
    
    renderer.add_object_type(&device, mesh_id, material_id, &vertices, &indices);
    
    let instance = InstanceData {
        model_matrix_0: [1.0, 0.0, 0.0, 0.0],
        model_matrix_1: [0.0, 1.0, 0.0, 0.0],
        model_matrix_2: [0.0, 0.0, 1.0, 0.0],
        model_matrix_3: [0.0, 0.0, 0.0, 1.0],
    };
    
    let result = renderer.add_instance(mesh_id, material_id, instance);
    assert!(result.is_ok());
    
    let material_group = renderer.material_groups.get(&material_id).unwrap();
    let mesh_group = material_group.meshes.get(&mesh_id).unwrap();
    assert_eq!(mesh_group.instances.len(), 1);
}

#[test]
fn test_add_instance_failure_missing_mesh_or_material() {
    let mut renderer = MapObjectInstancedRenderer::new();
    
    let instance = InstanceData {
        model_matrix_0: [1.0, 0.0, 0.0, 0.0],
        model_matrix_1: [0.0, 1.0, 0.0, 0.0],
        model_matrix_2: [0.0, 0.0, 1.0, 0.0],
        model_matrix_3: [0.0, 0.0, 0.0, 1.0],
    };
    
    let result = renderer.add_instance(99, 99, instance);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Mesh or Material not found");
}

#[test]
fn test_clear_instances() {
    if skip_if_no_adapter() { return; }
    
    let (device, _) = pollster::block_on(create_test_device());
    let mut renderer = MapObjectInstancedRenderer::new();
    
    let vertices = [MapObjectVertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        tex_coord: [0.0, 0.0],
    }];
    let indices = [0, 1, 2];
    
    let material_id = 10;
    let mesh_id = 5;
    
    renderer.add_object_type(&device, mesh_id, material_id, &vertices, &indices);
    
    let instance = InstanceData {
        model_matrix_0: [1.0, 0.0, 0.0, 0.0],
        model_matrix_1: [0.0, 1.0, 0.0, 0.0],
        model_matrix_2: [0.0, 0.0, 1.0, 0.0],
        model_matrix_3: [0.0, 0.0, 0.0, 1.0],
    };
    
    renderer.add_instance(mesh_id, material_id, instance).unwrap();
    renderer.build_instance_buffers(&device);
    
    let material_group = renderer.material_groups.get(&material_id).unwrap();
    let mesh_group = material_group.meshes.get(&mesh_id).unwrap();
    assert_eq!(mesh_group.instances.len(), 1);
    assert!(mesh_group.instance_buffer.is_some());
    
    renderer.clear_instances();
    
    let material_group_after = renderer.material_groups.get(&material_id).unwrap();
    let mesh_group_after = material_group_after.meshes.get(&mesh_id).unwrap();
    assert_eq!(mesh_group_after.instances.len(), 0);
    assert!(mesh_group_after.instance_buffer.is_none());
}
