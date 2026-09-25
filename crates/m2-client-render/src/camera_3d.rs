//! Modul camera_3d dla skrzynki m2-client-render
//! Kamera orbitalna 3D: macierz widoku i rzutowania perspektywicznego
//! Kat obrotu Yaw, Pitch, odleglosc zoom, wektory Eye/Target, macierze View i Projection

use glam::{Mat4, Vec3};

/// Glowna struktura implementacyjna
#[derive(Debug, Clone)]
pub struct Camera3d {
    pub initialized: bool,
    
    // Orbital camera parameters
    pub yaw: f32,       // W radianach
    pub pitch: f32,     // W radianach
    pub distance: f32,  // Odleglosc od celu (zoom)
    
    // Vectors
    pub target: Vec3,   // Punkt na ktory patrzy kamera
    pub eye: Vec3,      // Aktualna pozycja kamery
    pub up: Vec3,       // Wektor "w gore"
    
    // Matrices
    pub view_matrix: Mat4,
    pub proj_matrix: Mat4,
    
    // Projection parameters
    pub fov_y: f32,     // Kat widzenia w pionie (w radianach)
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for Camera3d {
    fn default() -> Self {
        Self {
            initialized: false,
            yaw: 0.0,
            pitch: 0.0,
            distance: 10.0,
            target: Vec3::ZERO,
            eye: Vec3::new(0.0, 0.0, 10.0),
            up: Vec3::Y,
            view_matrix: Mat4::IDENTITY,
            proj_matrix: Mat4::IDENTITY,
            fov_y: std::f32::consts::FRAC_PI_4, // 45 stopni
            aspect_ratio: 16.0 / 9.0,
            z_near: 0.1,
            z_far: 1000.0,
        }
    }
}

impl Camera3d {
    pub fn new() -> Self {
        let mut camera = Self {
            initialized: true,
            ..Default::default()
        };
        camera.update_matrices();
        camera
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
        self.update_matrices();
    }
    
    pub fn set_distance(&mut self, distance: f32) {
        // Zabezpieczenie przed ujemnym zoomem
        self.distance = distance.max(0.1);
        self.update_matrices();
    }
    
    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        // Ograniczenie pitch by nie przekroczylo biegunow (unikanie Gimbal locka i problemu z wektorem UP)
        let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.01;
        self.pitch = pitch.clamp(-pitch_limit, pitch_limit);
        self.update_matrices();
    }
    
    pub fn set_projection(&mut self, fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) {
        self.fov_y = fov_y;
        self.aspect_ratio = aspect_ratio;
        self.z_near = z_near;
        self.z_far = z_far;
        self.update_proj_matrix();
    }
    
    pub fn update_matrices(&mut self) {
        self.update_view_matrix();
        self.update_proj_matrix();
    }
    
    #[allow(deprecated)]
    pub fn update_view_matrix(&mut self) {
        // Obliczenie pozycji kamery (eye) na sferze wokol targetu
        // Rownania sferyczne (w konwencji Y w gore)
        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();
        
        let offset = Vec3::new(
            self.distance * cos_pitch * sin_yaw,
            self.distance * sin_pitch,
            self.distance * cos_pitch * cos_yaw
        );
        
        self.eye = self.target + offset;
        self.view_matrix = Mat4::look_at_rh(self.eye, self.target, self.up);
    }
    
    #[allow(deprecated)]
    pub fn update_proj_matrix(&mut self) {
        self.proj_matrix = Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.z_near, self.z_far);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_skeleton_camera_3d() {
        let obj = Camera3d::new();
        assert!(obj.is_ready());
    }
    
    #[test]
    fn test_camera_initialization() {
        let cam = Camera3d::new();
        assert_eq!(cam.distance, 10.0);
        assert_eq!(cam.target, Vec3::ZERO);
        assert_eq!(cam.yaw, 0.0);
        assert_eq!(cam.pitch, 0.0);
    }
    
    #[test]
    fn test_camera_eye_calculation() {
        let mut cam = Camera3d::new();
        
        // Z prawidlowym domyslnym ustawieniem (yaw=0, pitch=0) offset to (0, 0, distance)
        assert!((cam.eye.z - 10.0).abs() < 1e-5);
        assert!(cam.eye.x.abs() < 1e-5);
        assert!(cam.eye.y.abs() < 1e-5);
        
        // Obrot o 90 stopni wzgledem Y
        cam.set_rotation(PI / 2.0, 0.0);
        assert!((cam.eye.x - 10.0).abs() < 1e-5);
        assert!(cam.eye.z.abs() < 1e-5);
        
        // Obrot w pionie (pitch)
        cam.set_rotation(0.0, PI / 4.0);
        let expected_y = 10.0 * (PI / 4.0).sin();
        let expected_z = 10.0 * (PI / 4.0).cos();
        assert!((cam.eye.y - expected_y).abs() < 1e-5);
        assert!((cam.eye.z - expected_z).abs() < 1e-5);
    }
}
