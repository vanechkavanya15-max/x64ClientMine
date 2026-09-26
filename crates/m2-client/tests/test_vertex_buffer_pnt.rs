use m2_client::vertex_buffer_pnt::{MeshBuffer, VertexPNT};
use std::mem::size_of;

#[test]
fn test_vertex_pnt_memory_size() {
    // Requirements state exact memory structure
    assert_eq!(
        size_of::<VertexPNT>(),
        32,
        "VertexPNT size must be exactly 32 bytes to match GPU layout and EterLib/GrpVertexBuffer.h"
    );
    assert_eq!(
        size_of::<[f32; 3]>(),
        12,
        "Position array must be 12 bytes"
    );
    assert_eq!(size_of::<[f32; 2]>(), 8, "TexCoord array must be 8 bytes");
}

#[test]
fn test_add_vertex_and_indices() {
    let mut buffer = MeshBuffer::new();
    let idx0 = buffer.add_vertex([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], [0.5, 0.5]);
    let idx1 = buffer.add_vertex([4.0, 5.0, 6.0], [1.0, 0.0, 0.0], [1.0, 1.0]);

    assert_eq!(idx0, 0, "First vertex index should be 0");
    assert_eq!(idx1, 1, "Second vertex index should be 1");
    assert_eq!(buffer.vertices.len(), 2, "Buffer should contain 2 vertices");

    let v0 = &buffer.vertices[0];
    assert_eq!(v0.position, [1.0, 2.0, 3.0]);
    assert_eq!(v0.normal, [0.0, 1.0, 0.0]);
    assert_eq!(v0.tex_coord, [0.5, 0.5]);
}

#[test]
fn test_add_triangle() {
    let mut buffer = MeshBuffer::new();
    let idx0 = buffer.add_vertex([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]);
    let idx1 = buffer.add_vertex([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]);
    let idx2 = buffer.add_vertex([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0]);

    buffer.add_triangle(idx0, idx1, idx2);

    assert_eq!(buffer.indices.len(), 3, "Buffer should contain 3 indices");
    assert_eq!(buffer.indices[0], 0);
    assert_eq!(buffer.indices[1], 1);
    assert_eq!(buffer.indices[2], 2);
}

#[test]
fn test_as_byte_slice() {
    let mut buffer = MeshBuffer::new();
    let _idx0 = buffer.add_vertex([1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.5, 0.5]);

    let bytes = buffer.as_byte_slice();
    assert_eq!(
        bytes.len(),
        32,
        "Byte slice length should be 32 for one vertex"
    );

    // Verify first 4 bytes (f32 x = 1.0)
    let float_bytes = 1.0f32.to_ne_bytes();
    assert_eq!(&bytes[0..4], &float_bytes);
}

#[test]
fn test_as_byte_slice_empty() {
    let buffer = MeshBuffer::new();
    let bytes = buffer.as_byte_slice();
    assert_eq!(bytes.len(), 0, "Empty buffer should return 0 byte slice");
}

#[test]
fn test_add_many_vertices_and_triangles() {
    let mut buffer = MeshBuffer::new();
    
    // Create a 10x10 grid of vertices
    for y in 0..10 {
        for x in 0..10 {
            buffer.add_vertex(
                [x as f32, y as f32, 0.0],
                [0.0, 0.0, 1.0],
                [x as f32 / 9.0, y as f32 / 9.0]
            );
        }
    }

    assert_eq!(buffer.vertices.len(), 100);

    for y in 0..9 {
        for x in 0..9 {
            let i0 = y * 10 + x;
            let i1 = y * 10 + x + 1;
            let i2 = (y + 1) * 10 + x;
            let i3 = (y + 1) * 10 + x + 1;

            buffer.add_triangle(i0, i1, i2);
            buffer.add_triangle(i1, i3, i2);
        }
    }

    assert_eq!(buffer.indices.len(), 9 * 9 * 2 * 3); // 81 quads * 2 tris * 3 indices
}
