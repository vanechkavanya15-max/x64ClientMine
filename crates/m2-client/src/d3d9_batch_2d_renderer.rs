#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3D9Vertex2D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rhw: f32,
    pub color: u32,
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad2D {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
    pub color_argb: u32,
    pub texture_id: u32,
}

pub struct Batch2DRenderer {
    batches: Vec<(u32, Vec<D3D9Vertex2D>)>,
    is_batching: bool,
}

impl Batch2DRenderer {
    pub fn new() -> Self {
        Self {
            batches: Vec::new(),
            is_batching: false,
        }
    }

    pub fn begin_2d_batch(&mut self) {
        self.batches.clear();
        self.is_batching = true;
    }

    pub fn push_quad(&mut self, quad: Quad2D) {
        if !self.is_batching {
            return;
        }

        let z = 0.0;
        let rhw = 1.0;

        let v1 = D3D9Vertex2D {
            x: quad.x,
            y: quad.y,
            z,
            rhw,
            color: quad.color_argb,
            u: quad.u_min,
            v: quad.v_min,
        };
        let v2 = D3D9Vertex2D {
            x: quad.x + quad.width,
            y: quad.y,
            z,
            rhw,
            color: quad.color_argb,
            u: quad.u_max,
            v: quad.v_min,
        };
        let v3 = D3D9Vertex2D {
            x: quad.x,
            y: quad.y + quad.height,
            z,
            rhw,
            color: quad.color_argb,
            u: quad.u_min,
            v: quad.v_max,
        };
        let v4 = D3D9Vertex2D {
            x: quad.x + quad.width,
            y: quad.y + quad.height,
            z,
            rhw,
            color: quad.color_argb,
            u: quad.u_max,
            v: quad.v_max,
        };

        let push_vertices = [v1, v2, v3, v3, v2, v4];

        if let Some(last_batch) = self.batches.last_mut() {
            if last_batch.0 == quad.texture_id {
                last_batch.1.extend_from_slice(&push_vertices);
                return;
            }
        }

        self.batches.push((quad.texture_id, push_vertices.to_vec()));
    }

    pub fn push_colored_bar(&mut self, x: f32, y: f32, w: f32, h: f32, color: u32) {
        self.push_quad(Quad2D {
            x,
            y,
            width: w,
            height: h,
            u_min: 0.0,
            v_min: 0.0,
            u_max: 0.0,
            v_max: 0.0,
            color_argb: color,
            texture_id: 0,
        });
    }

    pub fn flush_batch(&mut self, mut render_fn: impl FnMut(&[u8], u32, usize)) {
        if !self.is_batching {
            return;
        }

        for (texture_id, vertices) in &self.batches {
            let vertex_count = vertices.len();
            if vertex_count == 0 {
                continue;
            }

            let byte_slice = unsafe {
                std::slice::from_raw_parts(
                    vertices.as_ptr() as *const u8,
                    vertex_count * std::mem::size_of::<D3D9Vertex2D>(),
                )
            };

            render_fn(byte_slice, *texture_id, vertex_count);
        }

        self.batches.clear();
    }

    pub fn end_2d_batch(&mut self) {
        self.batches.clear();
        self.is_batching = false;
    }
}

impl Default for Batch2DRenderer {
    fn default() -> Self {
        Self::new()
    }
}
