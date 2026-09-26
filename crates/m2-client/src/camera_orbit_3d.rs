use glam::{Mat4, Vec3};

pub struct OrbitCamera {
    pub target: (f32, f32, f32),
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub fov_degrees: f32,
    pub aspect_ratio: f32,
    pub near_z: f32,
    pub far_z: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

impl OrbitCamera {
    pub fn new(
        target: (f32, f32, f32),
        yaw: f32,
        pitch: f32,
        distance: f32,
        fov_degrees: f32,
        aspect_ratio: f32,
        near_z: f32,
        far_z: f32,
        min_distance: f32,
        max_distance: f32,
    ) -> Self {
        let mut camera = Self {
            target,
            yaw: 0.0,
            pitch: 0.0,
            distance: 0.0,
            fov_degrees,
            aspect_ratio,
            near_z,
            far_z,
            min_distance,
            max_distance,
        };
        
        // initialize properly with clamped values
        camera.yaw = yaw % 360.0;
        if camera.yaw < 0.0 {
            camera.yaw += 360.0;
        }
        
        camera.pitch = pitch.clamp(-80.0, 80.0);
        camera.distance = distance.clamp(min_distance, max_distance);
        
        camera
    }

    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw = (self.yaw + delta_yaw) % 360.0;
        if self.yaw < 0.0 {
            self.yaw += 360.0;
        }

        self.pitch = (self.pitch + delta_pitch).clamp(-80.0, 80.0);
    }

    pub fn zoom(&mut self, delta_dist: f32) {
        self.distance = (self.distance + delta_dist).clamp(self.min_distance, self.max_distance);
    }

    pub fn get_eye(&self) -> (f32, f32, f32) {
        let rad_yaw = self.yaw.to_radians();
        let rad_pitch = self.pitch.to_radians();
        
        let eye_x = self.target.0 - self.distance * rad_pitch.cos() * rad_yaw.sin();
        let eye_y = self.target.1 - self.distance * rad_pitch.cos() * rad_yaw.cos();
        let eye_z = self.target.2 + self.distance * rad_pitch.sin();
        
        (eye_x, eye_y, eye_z)
    }

    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        let eye_pos = self.get_eye();
        let eye = Vec3::new(eye_pos.0, eye_pos.1, eye_pos.2);
        let target = Vec3::new(self.target.0, self.target.1, self.target.2);
        let up = Vec3::Z;

        // Using look_at_rh
        let view_matrix = Mat4::look_at_rh(eye, target, up);
        
        view_matrix.to_cols_array_2d()
    }

    pub fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
        let proj_matrix = Mat4::perspective_rh(
            self.fov_degrees.to_radians(),
            self.aspect_ratio,
            self.near_z,
            self.far_z,
        );
        
        proj_matrix.to_cols_array_2d()
    }
}
