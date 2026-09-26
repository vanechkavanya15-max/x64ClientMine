use std::f32::consts::PI;

pub struct CameraOrbit {
    pub target_pos: [f32; 3],
    pub pitch_deg: f32,
    pub yaw_deg: f32,
    pub distance: f32,
}

impl CameraOrbit {
    pub fn new(target_pos: [f32; 3], pitch_deg: f32, yaw_deg: f32, distance: f32) -> Self {
        let mut cam = Self {
            target_pos,
            pitch_deg: 0.0,
            yaw_deg: 0.0,
            distance: 0.0,
        };
        cam.set_pitch(pitch_deg);
        cam.set_yaw(yaw_deg);
        cam.set_distance(distance);
        cam
    }

    pub fn set_pitch(&mut self, pitch_deg: f32) {
        self.pitch_deg = pitch_deg.clamp(10.0, 80.0);
    }

    pub fn set_yaw(&mut self, yaw_deg: f32) {
        self.yaw_deg = yaw_deg;
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance.clamp(500.0, 3500.0);
    }

    pub fn set_target(&mut self, target_pos: [f32; 3]) {
        self.target_pos = target_pos;
    }

    pub fn get_eye_position(&self) -> [f32; 3] {
        let rad_pitch = self.pitch_deg * PI / 180.0;
        let rad_yaw = self.yaw_deg * PI / 180.0;
        
        let eye_x = self.target_pos[0] - self.distance * rad_pitch.cos() * rad_yaw.sin();
        let eye_y = self.target_pos[1] - self.distance * rad_pitch.cos() * rad_yaw.cos();
        let eye_z = self.target_pos[2] + self.distance * rad_pitch.sin();
        
        [eye_x, eye_y, eye_z]
    }

    pub fn get_view_matrix(&self) -> [f32; 16] {
        let eye = self.get_eye_position();
        let target = self.target_pos;
        let up = [0.0, 0.0, 1.0];

        // zaxis = normalize(eye - target)
        let mut zaxis = [
            eye[0] - target[0],
            eye[1] - target[1],
            eye[2] - target[2],
        ];
        let z_len = (zaxis[0] * zaxis[0] + zaxis[1] * zaxis[1] + zaxis[2] * zaxis[2]).sqrt();
        if z_len > 1e-6 {
            zaxis[0] /= z_len;
            zaxis[1] /= z_len;
            zaxis[2] /= z_len;
        }

        // xaxis = normalize(cross(up, zaxis))
        let mut xaxis = [
            up[1] * zaxis[2] - up[2] * zaxis[1],
            up[2] * zaxis[0] - up[0] * zaxis[2],
            up[0] * zaxis[1] - up[1] * zaxis[0],
        ];
        let x_len = (xaxis[0] * xaxis[0] + xaxis[1] * xaxis[1] + xaxis[2] * xaxis[2]).sqrt();
        if x_len > 1e-6 {
            xaxis[0] /= x_len;
            xaxis[1] /= x_len;
            xaxis[2] /= x_len;
        }

        // yaxis = cross(zaxis, xaxis)
        let yaxis = [
            zaxis[1] * xaxis[2] - zaxis[2] * xaxis[1],
            zaxis[2] * xaxis[0] - zaxis[0] * xaxis[2],
            zaxis[0] * xaxis[1] - zaxis[1] * xaxis[0],
        ];

        let dot_x = -(xaxis[0] * eye[0] + xaxis[1] * eye[1] + xaxis[2] * eye[2]);
        let dot_y = -(yaxis[0] * eye[0] + yaxis[1] * eye[1] + yaxis[2] * eye[2]);
        let dot_z = -(zaxis[0] * eye[0] + zaxis[1] * eye[1] + zaxis[2] * eye[2]);

        [
            xaxis[0], yaxis[0], zaxis[0], 0.0,
            xaxis[1], yaxis[1], zaxis[1], 0.0,
            xaxis[2], yaxis[2], zaxis[2], 0.0,
            dot_x,    dot_y,    dot_z,    1.0,
        ]
    }
}
