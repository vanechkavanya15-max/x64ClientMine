use m2_client_render::pipeline_terrain::{PipelineTerrain, TerrainVertex, TerrainMesh};

#[test]
fn test_integration_pipeline_terrain() {
    let instance = PipelineTerrain::new();
    assert!(instance.is_ready());
}

#[test]
fn test_terrain_vertex_creation() {
    let pos = [1.0, 2.0, 3.0];
    let norm = [0.0, 1.0, 0.0];
    let tex = [0.5, 0.5];
    let alpha = [1.0, 0.0, 0.0, 0.0];
    
    let vertex = TerrainVertex::new(pos, norm, tex, alpha);
    
    assert_eq!(vertex.position, pos);
    assert_eq!(vertex.normal, norm);
    assert_eq!(vertex.tex_coord, tex);
    assert_eq!(vertex.alpha_weights, alpha);
}

#[test]
fn test_terrain_mesh_operations() {
    let mut mesh = TerrainMesh::new();
    assert_eq!(mesh.vertices.len(), 0);
    assert_eq!(mesh.indices.len(), 0);
    
    let vertex = TerrainVertex::new(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
    );
    
    mesh.add_vertex(vertex);
    assert_eq!(mesh.vertices.len(), 1);
    
    mesh.add_index(0);
    mesh.add_index(1);
    mesh.add_index(2);
    assert_eq!(mesh.indices.len(), 3);
    
    let retrieved = mesh.get_vertex(0);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().position, [0.0, 0.0, 0.0]);
    
    let out_of_bounds = mesh.get_vertex(999);
    assert!(out_of_bounds.is_none());
    
    mesh.clear();
    assert_eq!(mesh.vertices.len(), 0);
    assert_eq!(mesh.indices.len(), 0);
}

#[test]
fn test_pipeline_terrain_operations() {
    let mut pipeline = PipelineTerrain::new();
    assert!(pipeline.is_ready());
    assert_eq!(pipeline.vertex_count(), 0);
    assert_eq!(pipeline.index_count(), 0);
    
    let vertex = TerrainVertex::new(
        [10.0, 0.0, 10.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0],
        [0.5, 0.5, 0.0, 0.0],
    );
    pipeline.add_vertex(vertex);
    assert_eq!(pipeline.vertex_count(), 1);
    
    pipeline.mesh.add_index(0);
    assert_eq!(pipeline.index_count(), 1);
    
    let tex_result = pipeline.set_texture(0, "grass.png".to_string());
    assert!(tex_result.is_ok());
    assert_eq!(pipeline.get_texture(0).unwrap(), "grass.png");
    
    let tex_result = pipeline.set_texture(3, "rock.png".to_string());
    assert!(tex_result.is_ok());
    assert_eq!(pipeline.get_texture(3).unwrap(), "rock.png");
    
    let tex_err = pipeline.set_texture(4, "invalid.png".to_string());
    assert!(tex_err.is_err());
    assert!(pipeline.get_texture(4).is_none());
    
    pipeline.clear();
    assert_eq!(pipeline.vertex_count(), 0);
    assert_eq!(pipeline.index_count(), 0);
    assert!(pipeline.get_texture(0).is_none());
    assert!(pipeline.get_texture(3).is_none());
}
