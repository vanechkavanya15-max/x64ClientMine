use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    Index16,
    Index32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexBufferContainer {
    pub format: IndexFormat,
    pub count: usize,
    pub buffer_id: u32,
}

struct IndexBufferState {
    buffers: HashMap<u32, Vec<u8>>,
    next_id: u32,
}

static INDEX_BUFFER_STATE: OnceLock<Mutex<IndexBufferState>> = OnceLock::new();

fn get_state() -> &'static Mutex<IndexBufferState> {
    INDEX_BUFFER_STATE.get_or_init(|| {
        Mutex::new(IndexBufferState {
            buffers: HashMap::new(),
            next_id: 1,
        })
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexBufferError {
    LockFailed,
    BufferNotFound,
    SizeMismatch,
    InvalidFormat,
}

impl std::fmt::Display for IndexBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockFailed => write!(f, "Failed to acquire mutex lock"),
            Self::BufferNotFound => write!(f, "Buffer with the specified ID was not found"),
            Self::SizeMismatch => write!(f, "Provided indices do not match the buffer's capacity"),
            Self::InvalidFormat => write!(f, "Operation is not supported for this buffer format"),
        }
    }
}

impl std::error::Error for IndexBufferError {}

pub fn create_index_buffer(indices: &[u16]) -> Result<IndexBufferContainer, IndexBufferError> {
    let mut state = get_state().lock().map_err(|_| IndexBufferError::LockFailed)?;
    let buffer_id = state.next_id;
    state.next_id = state.next_id.wrapping_add(1);

    let mut data = Vec::with_capacity(indices.len() * 2);
    for &idx in indices {
        data.extend_from_slice(&idx.to_ne_bytes());
    }

    state.buffers.insert(buffer_id, data);

    Ok(IndexBufferContainer {
        format: IndexFormat::Index16,
        count: indices.len(),
        buffer_id,
    })
}

pub fn create_index_buffer_32(indices: &[u32]) -> Result<IndexBufferContainer, IndexBufferError> {
    let mut state = get_state().lock().map_err(|_| IndexBufferError::LockFailed)?;
    let buffer_id = state.next_id;
    state.next_id = state.next_id.wrapping_add(1);

    let mut data = Vec::with_capacity(indices.len() * 4);
    for &idx in indices {
        data.extend_from_slice(&idx.to_ne_bytes());
    }

    state.buffers.insert(buffer_id, data);

    Ok(IndexBufferContainer {
        format: IndexFormat::Index32,
        count: indices.len(),
        buffer_id,
    })
}

pub fn lock_and_update(
    buffer: &mut IndexBufferContainer,
    new_indices: &[u16],
) -> Result<(), IndexBufferError> {
    if buffer.format != IndexFormat::Index16 {
        return Err(IndexBufferError::InvalidFormat);
    }
    if buffer.count != new_indices.len() {
        return Err(IndexBufferError::SizeMismatch);
    }

    let mut state = get_state().lock().map_err(|_| IndexBufferError::LockFailed)?;
    let data = state
        .buffers
        .get_mut(&buffer.buffer_id)
        .ok_or(IndexBufferError::BufferNotFound)?;

    data.clear();
    for &idx in new_indices {
        data.extend_from_slice(&idx.to_ne_bytes());
    }

    Ok(())
}

pub fn generate_quad_indices(quad_count: usize) -> Vec<u16> {
    let mut indices = Vec::with_capacity(quad_count * 6);
    for i in 0..quad_count {
        let base = (i * 4) as u16;
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);
        indices.push(base);
        indices.push(base + 2);
        indices.push(base + 3);
    }
    indices
}

pub fn get_triangle_count(index_count: usize) -> usize {
    index_count / 3
}
