use m2_client::d3d9_index_buffer::{
    create_index_buffer, create_index_buffer_32, generate_quad_indices, get_triangle_count,
    lock_and_update, IndexBufferError, IndexFormat,
};

#[test]
fn test_generate_quad_indices() {
    // 1 quad
    let indices = generate_quad_indices(1);
    assert_eq!(indices.len(), 6);
    assert_eq!(indices, vec![0, 1, 2, 0, 2, 3]);

    // 2 quads
    let indices2 = generate_quad_indices(2);
    assert_eq!(indices2.len(), 12);
    assert_eq!(indices2, vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]);
}

#[test]
fn test_create_index_buffer_16() {
    let indices = generate_quad_indices(1);
    let buffer = create_index_buffer(&indices).expect("Failed to create index buffer");

    assert_eq!(buffer.format, IndexFormat::Index16);
    assert_eq!(buffer.count, 6);
}

#[test]
fn test_create_index_buffer_32() {
    let indices = vec![0u32, 1, 2, 0, 2, 3];
    let buffer = create_index_buffer_32(&indices).expect("Failed to create 32-bit index buffer");

    assert_eq!(buffer.format, IndexFormat::Index32);
    assert_eq!(buffer.count, 6);
}

#[test]
fn test_lock_and_update_success() {
    let indices = generate_quad_indices(1);
    let mut buffer = create_index_buffer(&indices).expect("Failed to create index buffer");

    let new_indices = vec![4, 5, 6, 4, 6, 7];
    let result = lock_and_update(&mut buffer, &new_indices);
    assert!(result.is_ok(), "Failed to update index buffer");
}

#[test]
fn test_lock_and_update_size_mismatch() {
    let indices = generate_quad_indices(1);
    let mut buffer = create_index_buffer(&indices).expect("Failed to create index buffer");

    // Try updating with 2 quads (12 indices) instead of 1 quad (6 indices)
    let new_indices = generate_quad_indices(2);
    let result = lock_and_update(&mut buffer, &new_indices);

    assert_eq!(result, Err(IndexBufferError::SizeMismatch));
}

#[test]
fn test_lock_and_update_invalid_format() {
    let indices = vec![0u32, 1, 2, 0, 2, 3];
    let mut buffer = create_index_buffer_32(&indices).expect("Failed to create 32-bit index buffer");

    let new_indices = vec![4u16, 5, 6, 4, 6, 7];
    let result = lock_and_update(&mut buffer, &new_indices);

    assert_eq!(result, Err(IndexBufferError::InvalidFormat));
}

#[test]
fn test_get_triangle_count() {
    assert_eq!(get_triangle_count(3), 1);
    assert_eq!(get_triangle_count(6), 2);
    assert_eq!(get_triangle_count(30), 10);
    // Integer division correctly handles invalid vertex counts, returning whole triangles
    assert_eq!(get_triangle_count(4), 1); 
}

#[test]
fn test_buffer_not_found() {
    // Attempting to update a mock buffer ID that hasn't been allocated
    let mut dummy_buffer = m2_client::d3d9_index_buffer::IndexBufferContainer {
        format: IndexFormat::Index16,
        count: 6,
        buffer_id: 999999,
    };
    
    let new_indices = vec![0u16, 1, 2, 0, 2, 3];
    let result = lock_and_update(&mut dummy_buffer, &new_indices);
    
    assert_eq!(result, Err(IndexBufferError::BufferNotFound));
}
