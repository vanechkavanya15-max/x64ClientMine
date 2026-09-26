use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct TracePoint {
    pub position: (f32, f32, f32),
    pub created_time: f32,
    pub width: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrailVertex {
    pub position: (f32, f32, f32),
    pub uv: (f32, f32),
}

pub struct FlyTrace {
    pub duration: f32,
    pub base_width: f32,
    pub points: VecDeque<TracePoint>,
    pub is_rect_shape: bool,
}

impl FlyTrace {
    pub fn new(duration: f32, base_width: f32) -> Self {
        Self {
            duration,
            base_width,
            points: VecDeque::new(),
            is_rect_shape: false,
        }
    }

    pub fn add_point(&mut self, pos: (f32, f32, f32), cur_time: f32) {
        self.points.push_front(TracePoint {
            position: pos,
            created_time: cur_time,
            width: self.base_width,
        });

        while let Some(back) = self.points.back() {
            if cur_time - back.created_time > self.duration {
                self.points.pop_back();
            } else {
                break;
            }
        }
    }

    pub fn generate_mesh_vertices(&self, cur_time: f32) -> Vec<TrailVertex> {
        let mut vertices = Vec::new();
        if self.points.len() < 2 {
            return vertices;
        }

        let total_points = self.points.len();
        
        let mut total_length = 0.0;
        let mut lengths = Vec::with_capacity(total_points);
        lengths.push(0.0);
        for i in 0..total_points - 1 {
            let p1 = self.points[i].position;
            let p2 = self.points[i + 1].position;
            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;
            let dz = p2.2 - p1.2;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            total_length += dist;
            lengths.push(total_length);
        }

        for i in 0..total_points {
            let p = &self.points[i];
            
            let rate = 1.0 - (cur_time - p.created_time) / self.duration;
            let rate = rate.clamp(0.0, 1.0);
            let mut current_width = p.width;
            if !self.is_rect_shape {
                current_width *= rate;
            }
            
            let (mut dx, mut dy, mut dz) = if i == 0 {
                let next = &self.points[1];
                (next.position.0 - p.position.0, next.position.1 - p.position.1, next.position.2 - p.position.2)
            } else if i == total_points - 1 {
                let prev = &self.points[i - 1];
                (p.position.0 - prev.position.0, p.position.1 - prev.position.1, p.position.2 - prev.position.2)
            } else {
                let prev = &self.points[i - 1];
                let next = &self.points[i + 1];
                (next.position.0 - prev.position.0, next.position.1 - prev.position.1, next.position.2 - prev.position.2)
            };
            
            let len = (dx * dx + dy * dy + dz * dz).sqrt();
            if len > 1e-5 {
                dx /= len;
                dy /= len;
                dz /= len;
            } else {
                dx = 1.0;
                dy = 0.0;
                dz = 0.0;
            }

            let up_x = 0.0;
            let mut up_y = 0.0;
            let mut up_z = 1.0;

            if dx.abs() < 1e-3 && dy.abs() < 1e-3 {
                up_y = 1.0;
                up_z = 0.0;
            }

            let mut nx = dy * up_z - dz * up_y;
            let mut ny = dz * up_x - dx * up_z;
            let mut nz = dx * up_y - dy * up_x;

            let n_len = (nx * nx + ny * ny + nz * nz).sqrt();
            if n_len > 1e-5 {
                nx /= n_len;
                ny /= n_len;
                nz /= n_len;
            } else {
                nx = 1.0;
                ny = 0.0;
                nz = 0.0;
            }

            let u = if total_length > 1e-5 {
                lengths[i] / total_length
            } else {
                i as f32 / (total_points - 1) as f32
            };

            vertices.push(TrailVertex {
                position: (
                    p.position.0 + nx * current_width * 0.5,
                    p.position.1 + ny * current_width * 0.5,
                    p.position.2 + nz * current_width * 0.5,
                ),
                uv: (u, 0.0),
            });

            vertices.push(TrailVertex {
                position: (
                    p.position.0 - nx * current_width * 0.5,
                    p.position.1 - ny * current_width * 0.5,
                    p.position.2 - nz * current_width * 0.5,
                ),
                uv: (u, 1.0),
            });
        }

        vertices
    }
}
