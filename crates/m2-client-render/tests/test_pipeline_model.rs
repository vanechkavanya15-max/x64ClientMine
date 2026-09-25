use m2_client_render::pipeline_model::{PipelineModel, VertexPNTW, Matrix4x4, MatrixPalette};

#[test]
fn test_integration_pipeline_model() {
    let instance = PipelineModel::new();
    assert!(instance.is_ready());
}

#[test]
fn test_pipeline_model_flow() {
    let mut model = PipelineModel::new();
    
    // Dodawanie wierzcholkow
    let v1 = VertexPNTW::default();
    let v2 = VertexPNTW {
        position: [1.0, 1.0, 1.0],
        ..Default::default()
    };
    model.add_vertex(v1);
    model.add_vertex(v2);
    
    assert_eq!(model.vertices.len(), 2);
    
    // Aktualizacja macierzy kosci
    let mut custom_mat = Matrix4x4::default();
    custom_mat.data[0] = 5.0;
    
    // Zmiana dzialajaca
    assert!(model.set_bone_matrix(3, custom_mat));
    assert_eq!(model.matrix_palette.matrices[3].data[0], 5.0);
    
    // Zmiana nie dzialajaca - poza zasiegiem
    assert!(!model.set_bone_matrix(999, custom_mat));
    
    // Czyszczenie
    model.clear();
    assert_eq!(model.vertices.len(), 0);
    assert_eq!(model.matrix_palette.matrices[3].data[0], 1.0); // Wrocilo do domyslnego 1.0 (identycznosc)
}

#[test]
fn test_vertex_pntw_creation() {
    let vertex = VertexPNTW {
        position: [1.0, 2.0, 3.0],
        normal: [0.0, 1.0, 0.0],
        texcoord: [0.5, 0.5],
        bone_indices: [0, 1, 2, 3],
        bone_weights: [0.4, 0.3, 0.2, 0.1],
    };
    
    assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
    assert_eq!(vertex.bone_indices, [0, 1, 2, 3]);
    assert_eq!(vertex.bone_weights, [0.4, 0.3, 0.2, 0.1]);
}

#[test]
fn test_matrix_palette() {
    let palette = MatrixPalette::default();
    assert_eq!(palette.matrices.len(), 256);
    
    // Sprawdzamy, czy defaultowa macierz to macierz identycznosciowa
    let default_mat = Matrix4x4::default();
    assert_eq!(palette.matrices[0].data, default_mat.data);
}
