use m2_client::d3d9_vertex_buffer::*;

#[test]
fn test_vertex_pnt_size_and_alignment() {
    let expected_size = 36;
    assert_eq!(
        std::mem::size_of::<VertexPNT>(),
        expected_size,
        "VertexPNT must be exactly 36 bytes for C++ FFI compatibility"
    );

    // Verify offsets
    let pnt = VertexPNT {
        pos: [1.0, 2.0, 3.0],
        normal: [0.0, 1.0, 0.0],
        tex: [0.5, 0.5],
        _pad: [0; 4],
    };
    let base_ptr = &pnt as *const _ as usize;
    let pos_ptr = &pnt.pos as *const _ as usize;
    let normal_ptr = &pnt.normal as *const _ as usize;
    let tex_ptr = &pnt.tex as *const _ as usize;
    let pad_ptr = &pnt._pad as *const _ as usize;

    assert_eq!(pos_ptr - base_ptr, 0, "pos offset must be 0");
    assert_eq!(normal_ptr - base_ptr, 12, "normal offset must be 12");
    assert_eq!(tex_ptr - base_ptr, 24, "tex offset must be 24");
    assert_eq!(pad_ptr - base_ptr, 32, "_pad offset must be 32");
}

#[test]
fn test_vertex_pdnt_size_and_alignment() {
    let expected_size = 28;
    assert_eq!(
        std::mem::size_of::<VertexPDNT>(),
        expected_size,
        "VertexPDNT must be exactly 28 bytes"
    );

    let pdnt = VertexPDNT {
        pos: [1.0, 2.0, 3.0, 1.0],
        diffuse: 0xFFFFFFFF,
        tex: [0.5, 0.5],
    };

    let base_ptr = &pdnt as *const _ as usize;
    let pos_ptr = &pdnt.pos as *const _ as usize;
    let diffuse_ptr = &pdnt.diffuse as *const _ as usize;
    let tex_ptr = &pdnt.tex as *const _ as usize;

    assert_eq!(pos_ptr - base_ptr, 0, "pos offset must be 0");
    assert_eq!(diffuse_ptr - base_ptr, 16, "diffuse offset must be 16");
    assert_eq!(tex_ptr - base_ptr, 20, "tex offset must be 20");
}

#[test]
fn test_fvf_codes() {
    assert_eq!(get_fvf_code_pnt(), 0x0112);
    assert_eq!(get_fvf_code_pdnt(), 0x0144);
}

#[test]
fn test_allocate_vertex_buffer() {
    let stride = std::mem::size_of::<VertexPNT>();
    let count = 100;
    let result = allocate_vertex_buffer(count, stride, true);
    assert!(result.is_ok());
    let id = result.unwrap();
    assert!(id > 0);

    let fail_result = allocate_vertex_buffer(0, stride, true);
    assert!(fail_result.is_err());
}

#[test]
fn test_update_vertices_dynamic() {
    let stride = std::mem::size_of::<VertexPNT>();
    let count = 2;
    let buffer_id = allocate_vertex_buffer(count, stride, true).unwrap();

    let vertices = vec![
        VertexPNT { pos: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0], tex: [0.0, 0.0], _pad: [0; 4] },
        VertexPNT { pos: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex: [1.0, 1.0], _pad: [0; 4] },
    ];

    let result = update_vertices(buffer_id, 0, &vertices);
    assert!(result.is_ok());
}

#[test]
fn test_update_vertices_static_fails() {
    let stride = std::mem::size_of::<VertexPNT>();
    let count = 2;
    // is_dynamic = false
    let buffer_id = allocate_vertex_buffer(count, stride, false).unwrap();

    let vertices = vec![
        VertexPNT { pos: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0], tex: [0.0, 0.0], _pad: [0; 4] },
    ];

    let result = update_vertices(buffer_id, 0, &vertices);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Cannot update static vertex buffer");
}

#[test]
fn test_update_vertices_bounds_check() {
    let stride = std::mem::size_of::<VertexPNT>();
    let count = 1;
    let buffer_id = allocate_vertex_buffer(count, stride, true).unwrap();

    let vertices = vec![
        VertexPNT { pos: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0], tex: [0.0, 0.0], _pad: [0; 4] },
        VertexPNT { pos: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex: [1.0, 1.0], _pad: [0; 4] },
    ];

    let result = update_vertices(buffer_id, 0, &vertices);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Update exceeds buffer bounds");
}

#[test]
fn test_update_vertices_stride_mismatch() {
    let stride = std::mem::size_of::<VertexPDNT>();
    let count = 1;
    let buffer_id = allocate_vertex_buffer(count, stride, true).unwrap();

    let vertices = vec![
        VertexPNT { pos: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0], tex: [0.0, 0.0], _pad: [0; 4] },
    ];

    let result = update_vertices(buffer_id, 0, &vertices);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Data type size"));
}
