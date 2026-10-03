use std::mem;
use m2_client::wgpu_granny_actor_renderer::GrannyActorVertex;
use m2_client::granny_mesh_gpu_uploader::{GrannyMeshGpuUploader, GrannyPnt332Vertex, GrannyPwnt3432Vertex};
use m2_client::wgpu_engine_context::WgpuEngineContext;

#[test]
fn test_vertex_layouts_size() {
    assert_eq!(mem::size_of::<GrannyPwnt3432Vertex>(), 40, "GrannyPwnt3432Vertex should be exactly 40 bytes");
    assert_eq!(mem::size_of::<GrannyPnt332Vertex>(), 32, "GrannyPnt332Vertex should be exactly 32 bytes");
}

#[test]
fn test_convert_pnt332_to_actor() {
    let vertices = vec![
        GrannyPnt332Vertex {
            position: [1.0, 2.0, 3.0],
            normal: [0.0, 1.0, 0.0],
            uv: [0.5, 0.5],
        },
    ];

    let actor_vertices = GrannyMeshGpuUploader::convert_pnt332_to_actor(&vertices);
    assert_eq!(actor_vertices.len(), 1);

    let v = &actor_vertices[0];
    assert_eq!(v.position, [1.0, 2.0, 3.0]);
    assert_eq!(v.normal, [0.0, 1.0, 0.0]);
    assert_eq!(v.tex_coord, [0.5, 0.5]);
    assert_eq!(v.bone_indices, [0, 0, 0, 0]);
    assert_eq!(v.bone_weights, [1.0, 0.0, 0.0, 0.0], "Rigid meshes must map 100% weight to bone 0");
}

#[test]
fn test_convert_pwnt3432_to_actor() {
    let vertices = vec![
        GrannyPwnt3432Vertex {
            position: [1.0, -1.0, 0.0],
            bone_weights: [255, 128, 64, 0],
            bone_indices: [1, 2, 3, 4],
            normal: [0.0, 0.0, 1.0],
            uv: [1.0, 0.0],
        },
    ];

    let actor_vertices = GrannyMeshGpuUploader::convert_pwnt3432_to_actor(&vertices);
    assert_eq!(actor_vertices.len(), 1);

    let v = &actor_vertices[0];
    assert_eq!(v.position, [1.0, -1.0, 0.0]);
    assert_eq!(v.normal, [0.0, 0.0, 1.0]);
    assert_eq!(v.tex_coord, [1.0, 0.0]);
    assert_eq!(v.bone_indices, [1, 2, 3, 4]);

    let inv_255 = 1.0 / 255.0;
    assert_eq!(v.bone_weights[0], 255.0 * inv_255);
    assert_eq!(v.bone_weights[1], 128.0 * inv_255);
    assert_eq!(v.bone_weights[2], 64.0 * inv_255);
    assert_eq!(v.bone_weights[3], 0.0);
}

#[test]
fn test_upload_mesh_empty_and_valid() {
    let ctx = WgpuEngineContext::new(0, 800, 600).expect("Headless ctx");
    
    let res = GrannyMeshGpuUploader::upload_mesh(&ctx.device, &[], &[], None);
    assert!(res.is_err(), "Should return error for empty slices");

    let vertices = vec![
        GrannyActorVertex {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            tex_coord: [0.0, 0.0],
            bone_indices: [0, 0, 0, 0],
            bone_weights: [1.0, 0.0, 0.0, 0.0],
        }
    ];
    let indices = vec![0, 0, 0];

    let buffers = GrannyMeshGpuUploader::upload_mesh(&ctx.device, &vertices, &indices, Some("test_mesh")).unwrap();
    assert_eq!(buffers.vertex_count, 1);
    assert_eq!(buffers.index_count, 3);
}
