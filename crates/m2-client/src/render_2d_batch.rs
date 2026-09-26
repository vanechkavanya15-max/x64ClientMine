#[derive(Debug, Clone, PartialEq)]
pub struct Vertex2D {
    pub pos: (f32, f32),
    pub uv: (f32, f32),
    pub color: [f32; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClipRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub struct Batch2D {
    pub vertices: Vec<Vertex2D>,
    pub indices: Vec<u16>,
    pub clip_rect: Option<ClipRect>,
}

impl Batch2D {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            clip_rect: None,
        }
    }

    pub fn set_clip_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.clip_rect = Some(ClipRect {
            left,
            top,
            right,
            bottom,
        });
    }

    pub fn clear_clip_rect(&mut self) {
        self.clip_rect = None;
    }

    pub fn add_quad(&mut self, x: f32, y: f32, w: f32, h: f32, uv_rect: (f32, f32, f32, f32), color: [f32; 4]) {
        let (mut qx, mut qy, mut qw, mut qh) = (x, y, w, h);
        let (mut su, mut sv, mut eu, mut ev) = uv_rect;

        if let Some(clip) = &self.clip_rect {
            if qx >= clip.right || qy >= clip.bottom || qx + qw <= clip.left || qy + qh <= clip.top {
                return;
            }

            let dx1 = (clip.left - qx).max(0.0);
            let dy1 = (clip.top - qy).max(0.0);
            let dx2 = ((qx + qw) - clip.right).max(0.0);
            let dy2 = ((qy + qh) - clip.bottom).max(0.0);

            if dx1 > 0.0 {
                let u_ratio = dx1 / qw;
                qx += dx1;
                qw -= dx1;
                su += (eu - su) * u_ratio;
            }
            if dy1 > 0.0 {
                let v_ratio = dy1 / qh;
                qy += dy1;
                qh -= dy1;
                sv += (ev - sv) * v_ratio;
            }
            if dx2 > 0.0 {
                let u_ratio = dx2 / qw;
                qw -= dx2;
                eu -= (eu - su) * u_ratio;
            }
            if dy2 > 0.0 {
                let v_ratio = dy2 / qh;
                qh -= dy2;
                ev -= (ev - sv) * v_ratio;
            }
        }

        let start_idx = self.vertices.len() as u16;

        self.vertices.push(Vertex2D { pos: (qx, qy), uv: (su, sv), color });
        self.vertices.push(Vertex2D { pos: (qx + qw, qy), uv: (eu, sv), color });
        self.vertices.push(Vertex2D { pos: (qx, qy + qh), uv: (su, ev), color });
        self.vertices.push(Vertex2D { pos: (qx + qw, qy + qh), uv: (eu, ev), color });

        self.indices.push(start_idx);
        self.indices.push(start_idx + 1);
        self.indices.push(start_idx + 2);
        self.indices.push(start_idx + 2);
        self.indices.push(start_idx + 1);
        self.indices.push(start_idx + 3);
    }
}
