use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WgpuFlyVertex {
    pub position: [f32; 3],
    pub color: u32,
    pub uv: [f32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimePosition {
    pub time: f32,
    pub position: [f32; 3],
}

pub struct FlyTraceRenderer {
    pub rect_shape: bool,
    pub color: u32,
    pub size: f32,
    pub tail_length: f32,
    pub positions: VecDeque<TimePosition>,
}

impl FlyTraceRenderer {
    pub fn new(rect_shape: bool, color: u32, size: f32, tail_length: f32) -> Self {
        Self {
            rect_shape,
            color,
            size,
            tail_length,
            positions: VecDeque::new(),
        }
    }
}

pub struct FlyingProjectileRenderer {
    pub data_id: u32,
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion (x, y, z, w)
}

impl FlyingProjectileRenderer {
    pub fn new(data_id: u32) -> Self {
        Self {
            data_id,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

pub fn cross_product(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-5 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        [0.0, 0.0, 0.0]
    }
}

pub fn look_at_quaternion(velocity: [f32; 3]) -> [f32; 4] {
    // Basic implementation of aligning Z-axis to the velocity vector.
    // In Metin2, Y is usually down/forward or similar, we'll align Z to direction here
    // based on `D3DXQuaternionRotationYawPitchRoll`.
    let (vx, vy, vz) = (velocity[0], velocity[1], velocity[2]);
    let len_xy = (vx * vx + vy * vy).sqrt();
    
    let mut yaw = 0.0;
    if len_xy > 0.0 {
        yaw = vy.atan2(vx);
    }
    
    let len_xyz = (vx * vx + vy * vy + vz * vz).sqrt();
    let mut pitch = 0.0;
    if len_xyz > 0.0 {
        pitch = vz.atan2(len_xy);
    }
    
    let roll = 0.0; // Assume no roll for simple look-at

    let cy = (yaw * 0.5).cos();
    let sy = (yaw * 0.5).sin();
    let cp = (pitch * 0.5).cos();
    let sp = (pitch * 0.5).sin();
    let cr = (roll * 0.5_f32).cos();
    let sr = (roll * 0.5_f32).sin();

    [
        sr * cp * cy - cr * sp * sy, // x
        cr * sp * cy + sr * cp * sy, // y
        cr * cp * sy - sr * sp * cy, // z
        cr * cp * cy + sr * sp * sy, // w
    ]
}

impl FlyTraceRenderer {
    pub fn update_new_position(&mut self, pos: [f32; 3], current_time: f32) {
        self.positions.push_front(TimePosition {
            time: current_time,
            position: pos,
        });

        while let Some(back) = self.positions.back() {
            if back.time + self.tail_length < current_time {
                self.positions.pop_back();
            } else {
                break;
            }
        }
    }

    /// Replicates `CFlyTrace::Render` logic to generate the triangle strip vertices for the ribbon trail.
    pub fn generate_vertices(
        &self,
        current_time: f32,
        camera_eye: [f32; 3],
        camera_forward: [f32; 3],
    ) -> Vec<WgpuFlyVertex> {
        if self.positions.len() <= 1 {
            return Vec::new();
        }

        let mut vertices = Vec::new();

        let mut it1_idx = 0;
        let mut it2_idx = 1;

        while it2_idx < self.positions.len() {
            let p1 = &self.positions[it1_idx];
            let p2 = &self.positions[it2_idx];

            let rk_old = p1.position;
            let rk_new = p2.position;

            let b = [
                rk_new[0] - rk_old[0],
                rk_new[1] - rk_old[1],
                rk_new[2] - rk_old[2],
            ];

            // In C++ it does a frustum check here. We'll skip frustum check for just generating the mesh geometry, 
            // as this is usually handled by the renderer before dispatching.

            let rate1 = 1.0 - (current_time - p1.time) / self.tail_length;
            let rate2 = 1.0 - (current_time - p2.time) / self.tail_length;
            
            let rate1 = rate1.clamp(0.0, 1.0);
            let rate2 = rate2.clamp(0.0, 1.0);

            let mut size1 = self.size;
            let mut size2 = self.size;

            if !self.rect_shape {
                size1 *= rate1;
                size2 *= rate2;
            }

            // Local coordinates of the 6 vertices (2 triangles) forming a quad segment, similar to C++ TFlyVertex
            let mut local_v = [
                ([0.0, size1, 0.0], [0.0, 0.0]),
                ([-size1, 0.0, 0.0], [0.0, 0.5]),
                ([size1, 0.0, 0.0], [0.5, 0.0]),
                ([-size2, 0.0, 0.0], [0.5, 1.0]),
                ([size2, 0.0, 0.0], [1.0, 0.5]),
                ([0.0, -size2, 0.0], [1.0, 1.0]),
            ];

            let mut e = [
                camera_eye[0] - rk_old[0],
                camera_eye[1] - rk_old[1],
                camera_eye[2] - rk_old[2],
            ];

            let p = cross_product(b, e);
            let mut u = cross_product(camera_forward, p);
            u = normalize(u);
            let r = cross_product(camera_forward, u);

            // C++ constructs a matrix m where:
            // m._21 = u.x; m._22 = u.y; m._23 = u.z;
            // m._11 = r.x; m._12 = r.y; m._13 = r.z;
            // Transforming local points by this matrix.
            
            for i in 0..6 {
                let px = local_v[i].0[0];
                let py = local_v[i].0[1];
                let pz = local_v[i].0[2];

                let transformed_x = px * r[0] + py * u[0] + pz * camera_forward[0];
                let transformed_y = px * r[1] + py * u[1] + pz * camera_forward[1];
                let transformed_z = px * r[2] + py * u[2] + pz * camera_forward[2];

                local_v[i].0 = [transformed_x, transformed_y, transformed_z];
            }

            for i in 0..3 {
                local_v[i].0[0] += rk_old[0];
                local_v[i].0[1] += rk_old[1];
                local_v[i].0[2] += rk_old[2];
            }
            
            for i in 3..6 {
                local_v[i].0[0] += rk_new[0];
                local_v[i].0[1] += rk_new[1];
                local_v[i].0[2] += rk_new[2];
            }

            // In C++, DrawPrimitiveUP(D3DPT_TRIANGLESTRIP, 4, v) renders:
            // v[0], v[1], v[2], v[3], v[4], v[5] -> forming a strip.
            // A triangle strip of 6 vertices produces 4 triangles.
            
            // Extract ARGB channels
            let a = (self.color >> 24) & 0xFF;
            let r_col = (self.color >> 16) & 0xFF;
            let g = (self.color >> 8) & 0xFF;
            let b_col = self.color & 0xFF;
            
            let alpha1 = (a as f32 * rate1) as u32;
            let alpha2 = (a as f32 * rate2) as u32;
            
            let color1 = (alpha1 << 24) | (r_col << 16) | (g << 8) | b_col;
            let color2 = (alpha2 << 24) | (r_col << 16) | (g << 8) | b_col;

            for i in 0..6 {
                let vertex_color = if i < 3 { color1 } else { color2 };
                vertices.push(WgpuFlyVertex {
                    position: local_v[i].0,
                    color: vertex_color,
                    uv: local_v[i].1,
                });
            }

            it1_idx += 1;
            it2_idx += 1;
        }

        vertices
    }
}