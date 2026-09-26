#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexPNT {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
}

#[derive(Debug, Clone, Default)]
pub struct MeshBuffer {
    pub vertices: Vec<VertexPNT>,
    pub indices: Vec<u32>,
}

impl MeshBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, pos: [f32; 3], norm: [f32; 3], uv: [f32; 2]) -> u32 {
        let index = self.vertices.len() as u32;
        self.vertices.push(VertexPNT {
            position: pos,
            normal: norm,
            tex_coord: uv,
        });
        index
    }

    pub fn add_triangle(&mut self, i0: u32, i1: u32, i2: u32) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    pub fn as_byte_slice(&self) -> &[u8] {
        if self.vertices.is_empty() {
            return &[];
        }
        let ptr = self.vertices.as_ptr() as *const u8;
        let len = self.vertices.len() * std::mem::size_of::<VertexPNT>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}
