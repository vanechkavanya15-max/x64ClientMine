#[path = "../src/actor_armor_hair_morph.rs"]
mod actor_armor_hair_morph;

use actor_armor_hair_morph::{ActorMorphState, Mesh, GrannyPwnt3432Vertex};

fn create_valid_vertex() -> GrannyPwnt3432Vertex {
    GrannyPwnt3432Vertex {
        position: [0.0, 0.0, 0.0],
        bone_weights: [255, 0, 0, 0],
        bone_indices: [0, 0, 0, 0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    }
}

fn create_valid_mesh() -> Mesh {
    Mesh {
        vertices: vec![create_valid_vertex()],
        indices: vec![0, 0, 0],
    }
}

#[test]
fn test_set_armor_mesh_successfully() {
    let mut state = ActorMorphState::new();
    let mesh = create_valid_mesh();
    
    let result = state.set_armor(mesh.clone());
    assert!(result.is_ok());
    assert!(state.armor_mesh.is_some());
    assert_eq!(state.armor_mesh.unwrap(), mesh);
}

#[test]
fn test_set_hair_mesh_successfully_and_replace() {
    let mut state = ActorMorphState::new();
    let mesh1 = create_valid_mesh();
    
    let result1 = state.set_hair(mesh1.clone());
    assert!(result1.is_ok());
    
    let mut mesh2 = create_valid_mesh();
    mesh2.indices.push(1);
    
    let result2 = state.set_hair(mesh2.clone());
    assert!(result2.is_ok());
    
    assert!(state.hair_mesh.is_some());
    assert_eq!(state.hair_mesh.unwrap(), mesh2);
}

#[test]
fn test_error_when_bone_weights_do_not_sum_up_to_one() {
    let mut state = ActorMorphState::new();
    let mut mesh = create_valid_mesh();
    
    // Set invalid weights (sum < 1.0, specifically ~0.5)
    mesh.vertices[0].bone_weights = [128, 0, 0, 0];
    
    let result = state.set_armor(mesh);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bone weights do not sum to 1.0");
    assert!(state.armor_mesh.is_none());
}

#[test]
fn test_error_when_bone_index_exceeds_max_skeleton_bones() {
    let mut state = ActorMorphState::new();
    let mut mesh = create_valid_mesh();
    
    // Set invalid bone index (128 >= 128)
    mesh.vertices[0].bone_indices = [128, 0, 0, 0];
    
    let result = state.set_hair(mesh);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bone index exceeds maximum allowed (127)");
    assert!(state.hair_mesh.is_none());
}

#[test]
fn test_successful_removal_of_armor_and_hair_meshes() {
    let mut state = ActorMorphState::new();
    let mesh = create_valid_mesh();
    
    let _ = state.set_armor(mesh.clone());
    let _ = state.set_hair(mesh);
    
    assert!(state.armor_mesh.is_some());
    assert!(state.hair_mesh.is_some());
    
    state.remove_armor();
    assert!(state.armor_mesh.is_none());
    assert!(state.hair_mesh.is_some());
    
    state.remove_hair();
    assert!(state.hair_mesh.is_none());
}
