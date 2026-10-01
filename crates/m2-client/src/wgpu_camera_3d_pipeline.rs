use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

/// Uniforms required by the 3D camera pipeline in WGPU.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniforms {
    pub view_matrix: [[f32; 4]; 4],
    pub proj_matrix: [[f32; 4]; 4],
    pub camera_pos: [f32; 4],
}

impl Default for CameraUniforms {
    fn default() -> Self {
        Self {
            view_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            proj_matrix: Mat4::IDENTITY.to_cols_array_2d(),
            camera_pos: [0.0; 4],
        }
    }
}

/// A Third-Person Shooter (TPS) orbital camera.
#[derive(Debug, Clone)]
pub struct TpsCamera {
    pub target_pos: Vec3,
    pub pitch: f32, // Degrees
    pub yaw: f32,   // Degrees
    pub zoom: f32,  // Units
}

impl Default for TpsCamera {
    fn default() -> Self {
        Self {
            target_pos: Vec3::ZERO,
            pitch: 30.0,
            yaw: 0.0,
            zoom: 1000.0,
        }
    }
}

impl TpsCamera {
    pub const PITCH_MIN: f32 = -10.0;
    pub const PITCH_MAX: f32 = 80.0;
    
    pub const ZOOM_MIN: f32 = 400.0;
    pub const ZOOM_MAX: f32 = 2500.0;

    /// Creates a new TPS camera.
    pub fn new(target_pos: Vec3, pitch: f32, yaw: f32, zoom: f32) -> Self {
        let mut camera = Self {
            target_pos,
            pitch,
            yaw,
            zoom,
        };
        camera.clamp_constraints();
        camera
    }

    /// Clamps pitch and zoom to their respective constraints. Wraps yaw around 360 degrees.
    pub fn clamp_constraints(&mut self) {
        self.pitch = self.pitch.clamp(Self::PITCH_MIN, Self::PITCH_MAX);
        self.zoom = self.zoom.clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
        
        // Wrap yaw to [0.0, 360.0)
        self.yaw = self.yaw % 360.0;
        if self.yaw < 0.0 {
            self.yaw += 360.0;
        }
    }

    /// Sets the pitch and enforces constraints.
    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
        self.clamp_constraints();
    }

    /// Sets the yaw and enforces constraints.
    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.clamp_constraints();
    }

    /// Sets the zoom and enforces constraints.
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        self.clamp_constraints();
    }

    /// Calculates the camera's position in world space.
    pub fn get_camera_pos(&self) -> Vec3 {
        let pitch_rad = self.pitch.to_radians();
        let yaw_rad = self.yaw.to_radians();

        let forward = Vec3::new(
            pitch_rad.cos() * yaw_rad.sin(),
            pitch_rad.cos() * yaw_rad.cos(),
            -pitch_rad.sin()
        ).normalize();
        
        self.target_pos - forward * self.zoom
    }
    
    /// Calculates the view matrix (LookAt)
    pub fn get_view_matrix(&self) -> Mat4 {
        let up = Vec3::Z;
        let camera_pos = self.get_camera_pos();

        // glam's look_at_rh expects (eye, center, up)
        Mat4::look_at_rh(camera_pos, self.target_pos, up)
    }

    /// Calculates the perspective projection matrix.
    /// FOV: 60 degrees, near: 10.0, far: 50000.0
    pub fn get_projection_matrix(aspect_ratio: f32) -> Mat4 {
        let fov = 60.0_f32.to_radians();
        let near = 10.0;
        let far = 50000.0;
        
        Mat4::perspective_rh(fov, aspect_ratio, near, far)
    }

    /// Returns the Uniforms ready for WGPU.
    pub fn get_uniforms(&self, aspect_ratio: f32) -> CameraUniforms {
        let view = self.get_view_matrix();
        let proj = Self::get_projection_matrix(aspect_ratio);
        
        let eye = self.get_camera_pos();

        CameraUniforms {
            view_matrix: view.to_cols_array_2d(),
            proj_matrix: proj.to_cols_array_2d(),
            camera_pos: [eye.x, eye.y, eye.z, 1.0],
        }
    }
}
