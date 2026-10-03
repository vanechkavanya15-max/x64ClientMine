#[path = "../src/wgpu_granny_actor_renderer.rs"]
pub mod wgpu_granny_actor_renderer;
use wgpu_granny_actor_renderer::GrannyActorVertex;

#[path = "../src/granny_mesh_decompressor.rs"]
pub mod granny_mesh_decompressor;
use granny_mesh_decompressor::GrannyPwnt3432Vertex;

#[path = "../src/full_grn_anim_12.rs"]
pub mod full_grn_anim_12;

#[path = "../src/granny_vertex_weights_extractor.rs"]
pub mod granny_vertex_weights_extractor;

use granny_vertex_weights_extractor::extract_vertex_weights;

#[test]
fn test_extractor_works() {
    let vertices = vec![];
    let result = extract_vertex_weights(&vertices).unwrap();
    assert!(result.is_empty());
}
