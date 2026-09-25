use m2_client_granny::gr2_mesh::{Gr2Mesh, Submesh, Vertex};

#[test]
fn test_integration_gr2_mesh_vertices_and_indices() {
    let mut instance = Gr2Mesh::new();
    assert!(!instance.is_ready());

    let v1 = Vertex {
        position: [1.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    };
    let v2 = Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    };
    let v3 = Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [1.0, 0.0, 0.0],
        uv: [0.0, 1.0],
    };

    instance.add_vertex(v1);
    instance.add_vertex(v2);
    instance.add_vertex(v3);

    assert!(instance.is_ready());
    assert_eq!(instance.vertices.len(), 3);

    instance.add_index(0);
    instance.add_index(1);
    instance.add_index(2);

    assert_eq!(instance.indices.len(), 3);
}

#[test]
fn test_integration_gr2_mesh_submesh_valid() {
    let mut instance = Gr2Mesh::new();

    // Dodajemy 6 indeksow (dwa trojkaty)
    for i in 0..6 {
        instance.add_index(i);
    }

    let submesh = Submesh {
        start_index: 0,
        index_count: 6,
        material_index: 0,
    };

    let result = instance.add_submesh(submesh.clone());
    assert!(result.is_ok());
    assert_eq!(instance.submeshes.len(), 1);
    assert_eq!(instance.submeshes[0], submesh);
}

#[test]
fn test_integration_gr2_mesh_submesh_invalid() {
    let mut instance = Gr2Mesh::new();

    // Dodajemy tylko 3 indeksy
    for i in 0..3 {
        instance.add_index(i);
    }

    // Probujemy dodac submesh odwolujacy sie do 6 indeksow
    let submesh = Submesh {
        start_index: 0,
        index_count: 6,
        material_index: 0,
    };

    let result = instance.add_submesh(submesh);
    assert!(result.is_err());
    assert_eq!(instance.submeshes.len(), 0);
}

#[test]
fn test_integration_gr2_mesh_submesh_out_of_bounds_start() {
    let mut instance = Gr2Mesh::new();

    for i in 0..3 {
        instance.add_index(i);
    }

    let submesh = Submesh {
        start_index: 3,
        index_count: 3,
        material_index: 0,
    };

    let result = instance.add_submesh(submesh);
    assert!(result.is_err());
}
