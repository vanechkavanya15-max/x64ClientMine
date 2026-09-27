use std::collections::HashMap;
use std::sync::RwLock;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexPNT {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub tex: [f32; 2],
    pub _pad: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexPDNT {
    pub pos: [f32; 4],
    pub diffuse: u32,
    pub tex: [f32; 2],
}

#[derive(Debug, Clone)]
pub struct VertexBufferData {
    pub data: Vec<u8>,
    pub stride: usize,
    pub count: usize,
    pub is_dynamic: bool,
}

use std::sync::OnceLock;

static VERTEX_BUFFERS: OnceLock<RwLock<HashMap<u32, VertexBufferData>>> = OnceLock::new();
static NEXT_BUFFER_ID: OnceLock<RwLock<u32>> = OnceLock::new();

fn get_buffers() -> &'static RwLock<HashMap<u32, VertexBufferData>> {
    VERTEX_BUFFERS.get_or_init(|| RwLock::new(HashMap::new()))
}

fn get_next_id() -> &'static RwLock<u32> {
    NEXT_BUFFER_ID.get_or_init(|| RwLock::new(1))
}

pub fn allocate_vertex_buffer(count: usize, stride: usize, is_dynamic: bool) -> Result<u32, String> {
    if count == 0 || stride == 0 {
        return Err("Invalid vertex buffer allocation parameters".to_string());
    }

    let mut id_lock = get_next_id().write().unwrap();
    let id = *id_lock;
    *id_lock += 1;

    let buffer = VertexBufferData {
        data: vec![0; count * stride],
        stride,
        count,
        is_dynamic,
    };

    let mut buffers = get_buffers().write().unwrap();
    buffers.insert(id, buffer);

    Ok(id)
}

pub fn update_vertices<T: Copy>(buffer_id: u32, offset: usize, data: &[T]) -> Result<(), String> {
    let mut buffers = get_buffers().write().unwrap();
    if let Some(buffer) = buffers.get_mut(&buffer_id) {
        if !buffer.is_dynamic {
            return Err("Cannot update static vertex buffer".to_string());
        }

        let type_size = std::mem::size_of::<T>();
        if type_size != buffer.stride {
            return Err(format!("Data type size ({}) does not match buffer stride ({})", type_size, buffer.stride));
        }

        let required_bytes = data.len() * type_size;
        let start_byte = offset * type_size;
        
        if start_byte + required_bytes > buffer.data.len() {
            return Err("Update exceeds buffer bounds".to_string());
        }

        unsafe {
            let src_ptr = data.as_ptr() as *const u8;
            let dst_ptr = buffer.data.as_mut_ptr().add(start_byte);
            std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, required_bytes);
        }

        Ok(())
    } else {
        Err(format!("Vertex buffer {} not found", buffer_id))
    }
}

pub fn get_fvf_code_pnt() -> u32 {
    0x0112 // D3DFVF_XYZ (0x002) | D3DFVF_NORMAL (0x010) | D3DFVF_TEX1 (0x100)
}

pub fn get_fvf_code_pdnt() -> u32 {
    0x0144 // D3DFVF_XYZRHW (0x004) | D3DFVF_DIFFUSE (0x040) | D3DFVF_TEX1 (0x100)
}
