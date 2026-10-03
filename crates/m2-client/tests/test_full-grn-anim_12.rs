#[path = "../src/wgpu_granny_actor_renderer.rs"]
pub mod wgpu_granny_actor_renderer;
use wgpu_granny_actor_renderer::GrannyActorVertex;

#[path = "../src/granny_mesh_decompressor.rs"]
pub mod granny_mesh_decompressor;
use granny_mesh_decompressor::GrannyPwnt3432Vertex;

#[path = "../src/full_grn_anim_12.rs"]
pub mod full_grn_anim_12;
use full_grn_anim_12::extract_vertex_weights;

#[test]
fn test_empty_vertices() {
    let vertices = vec![];
    let result = extract_vertex_weights(&vertices).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_single_vertex() {
    let vertices = vec![GrannyPwnt3432Vertex {
        position: [1.0, 2.0, 3.0],
        bone_weights: [255, 0, 0, 0],
        bone_indices: [1, 0, 0, 0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.5, 0.5],
    }];
    
    let result = extract_vertex_weights(&vertices).unwrap();
    assert_eq!(result.len(), 1);
    
    let v = &result[0];
    let pos = { v.position };
    let norm = { v.normal };
    let tex = { v.tex_coord };
    let b_ind = { v.bone_indices };
    let b_wt = { v.bone_weights };
    
    assert_eq!(pos, [1.0, 2.0, 3.0]);
    assert_eq!(norm, [0.0, 1.0, 0.0]);
    assert_eq!(tex, [0.5, 0.5]);
    assert_eq!(b_ind, [1, 0, 0, 0]);
    assert_eq!(b_wt, [1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_multiple_vertices() {
    let vertices = vec![
        GrannyPwnt3432Vertex {
            position: [1.0, 2.0, 3.0],
            bone_weights: [127, 128, 0, 0],
            bone_indices: [1, 2, 0, 0],
            normal: [0.0, 1.0, 0.0],
            uv: [0.5, 0.5],
        },
        GrannyPwnt3432Vertex {
            position: [-1.0, -2.0, -3.0],
            bone_weights: [64, 64, 64, 63],
            bone_indices: [0, 1, 2, 3],
            normal: [1.0, 0.0, 0.0],
            uv: [0.0, 1.0],
        },
    ];
    
    let result = extract_vertex_weights(&vertices).unwrap();
    assert_eq!(result.len(), 2);
    
    let b_ind_0 = { result[0].bone_indices };
    let b_ind_1 = { result[1].bone_indices };
    assert_eq!(b_ind_0, [1, 2, 0, 0]);
    assert_eq!(b_ind_1, [0, 1, 2, 3]);
}

#[test]
fn test_bone_weights_normalization() {
    let vertices = vec![GrannyPwnt3432Vertex {
        position: [0.0, 0.0, 0.0],
        bone_weights: [255, 127, 63, 0],
        bone_indices: [0, 0, 0, 0],
        normal: [0.0, 0.0, 0.0],
        uv: [0.0, 0.0],
    }];
    
    let result = extract_vertex_weights(&vertices).unwrap();
    let weights = { result[0].bone_weights };
    
    assert!((weights[0] - 1.0).abs() < 1e-6);
    assert!((weights[1] - (127.0 / 255.0)).abs() < 1e-6);
    assert!((weights[2] - (63.0 / 255.0)).abs() < 1e-6);
    assert_eq!(weights[3], 0.0);
}

#[test]
fn test_bone_indices_conversion() {
    let vertices = vec![GrannyPwnt3432Vertex {
        position: [0.0, 0.0, 0.0],
        bone_weights: [0, 0, 0, 0],
        bone_indices: [255, 128, 10, 0],
        normal: [0.0, 0.0, 0.0],
        uv: [0.0, 0.0],
    }];
    
    let result = extract_vertex_weights(&vertices).unwrap();
    let b_ind = { result[0].bone_indices };
    assert_eq!(b_ind, [255, 128, 10, 0]);
}
