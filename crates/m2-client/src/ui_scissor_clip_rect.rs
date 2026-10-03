#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScissorVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rhw: f32,
    pub color: u32,
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScissorRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl ScissorVertex {
    pub fn new(x: f32, y: f32, z: f32, rhw: f32, color: u32, u: f32, v: f32) -> Self {
        Self { x, y, z, rhw, color, u, v }
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let inv_t = 1.0 - t;
        
        let a0 = ((self.color >> 24) & 0xFF) as f32;
        let r0 = ((self.color >> 16) & 0xFF) as f32;
        let g0 = ((self.color >> 8) & 0xFF) as f32;
        let b0 = (self.color & 0xFF) as f32;

        let a1 = ((other.color >> 24) & 0xFF) as f32;
        let r1 = ((other.color >> 16) & 0xFF) as f32;
        let g1 = ((other.color >> 8) & 0xFF) as f32;
        let b1 = (other.color & 0xFF) as f32;

        let a = (a0 * inv_t + a1 * t).round() as u32;
        let r = (r0 * inv_t + r1 * t).round() as u32;
        let g = (g0 * inv_t + g1 * t).round() as u32;
        let b = (b0 * inv_t + b1 * t).round() as u32;

        let new_color = (a << 24) | (r << 16) | (g << 8) | b;

        Self {
            x: self.x * inv_t + other.x * t,
            y: self.y * inv_t + other.y * t,
            z: self.z * inv_t + other.z * t,
            rhw: self.rhw * inv_t + other.rhw * t,
            color: new_color,
            u: self.u * inv_t + other.u * t,
            v: self.v * inv_t + other.v * t,
        }
    }
}

pub fn clip_polygon(mut polygon: Vec<ScissorVertex>, rect: &ScissorRect) -> Vec<ScissorVertex> {
    if polygon.is_empty() {
        return polygon;
    }

    polygon = clip_edge(&polygon, rect.left, true, true);
    polygon = clip_edge(&polygon, rect.right, true, false);
    polygon = clip_edge(&polygon, rect.top, false, true);
    polygon = clip_edge(&polygon, rect.bottom, false, false);

    polygon
}

fn clip_edge(
    polygon: &[ScissorVertex],
    edge_value: f32,
    is_vertical: bool,
    is_greater_than: bool,
) -> Vec<ScissorVertex> {
    if polygon.is_empty() {
        return Vec::new();
    }

    let mut output = Vec::with_capacity(polygon.len() + 2);

    let is_inside = |v: &ScissorVertex| -> bool {
        let val = if is_vertical { v.x } else { v.y };
        if is_greater_than {
            val >= edge_value
        } else {
            val <= edge_value
        }
    };

    let intersect = |v1: &ScissorVertex, v2: &ScissorVertex| -> ScissorVertex {
        let val1 = if is_vertical { v1.x } else { v1.y };
        let val2 = if is_vertical { v2.x } else { v2.y };
        
        let t = if (val2 - val1).abs() < 1e-5 {
            0.5
        } else {
            (edge_value - val1) / (val2 - val1)
        };
        
        v1.lerp(v2, t)
    };

    let mut prev = polygon.last().unwrap();
    let mut prev_inside = is_inside(prev);

    for curr in polygon {
        let curr_inside = is_inside(curr);

        if curr_inside != prev_inside {
            output.push(intersect(prev, curr));
        }

        if curr_inside {
            output.push(curr.clone());
        }

        prev = curr;
        prev_inside = curr_inside;
    }

    output
}

pub fn clip_triangles(vertices: &[ScissorVertex], rect: &ScissorRect) -> Vec<ScissorVertex> {
    let mut result = Vec::new();

    for i in (0..vertices.len()).step_by(3) {
        if i + 2 < vertices.len() {
            let polygon = vec![vertices[i].clone(), vertices[i + 1].clone(), vertices[i + 2].clone()];
            let clipped = clip_polygon(polygon, rect);

            if clipped.len() >= 3 {
                let v0 = clipped[0].clone();
                for j in 1..clipped.len() - 1 {
                    result.push(v0.clone());
                    result.push(clipped[j].clone());
                    result.push(clipped[j + 1].clone());
                }
            }
        }
    }

    result
}
