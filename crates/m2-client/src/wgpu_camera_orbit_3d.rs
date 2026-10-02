use glam::{Mat4, Vec3, Vec4};

pub const CAMERA_MIN_DISTANCE: f32 = 400.0; // Updated from 200.0 based on new defaults? The user request said: (od 400 do 2500 jednostek).
pub const CAMERA_MAX_DISTANCE: f32 = 2500.0;
pub const CAMERA_TARGET_STANDARD: f32 = 100.0;
pub const CAMERA_TARGET_FACE: f32 = 150.0;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum CameraState {
    Normal,
    CantGoDown,
    CantGoRight,
    CantGoLeft,
    ScreenByBuilding,
    ScreenByBuildingAndTooClose,
}

#[derive(Debug, Clone)]
pub struct Ray3D {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub struct WgpuCameraOrbit3D {
    pub distance: f32,
    pub pitch: f32, // degrees (10 to 80)
    pub yaw: f32, // degrees (0 to 360)
    pub target_pos: Vec3,
    pub eye_pos: Vec3,
    pub up: Vec3,
    pub view: Vec3,
    pub cross: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub znear: f32,
    pub zfar: f32,
    pub view_matrix: Mat4,
    pub proj_matrix: Mat4,
    pub camera_state: CameraState,
    pub terrain_collision_radius: f32,
    pub process_terrain_collision: bool,
}

impl Default for WgpuCameraOrbit3D {
    fn default() -> Self {
        Self::new()
    }
}

impl WgpuCameraOrbit3D {
    pub fn new() -> Self {
        let mut camera = Self {
            distance: 1500.0,
            pitch: 30.0,
            yaw: 0.0,
            target_pos: Vec3::ZERO,
            eye_pos: Vec3::new(0.0, -1500.0, 150.0),
            up: Vec3::new(0.0, 0.0, 1.0),
            view: Vec3::new(0.0, 1.0, 0.0),
            cross: Vec3::new(1.0, 0.0, 0.0),
            fov: 45.0_f32.to_radians(),
            aspect: 16.0 / 9.0,
            znear: 10.0,
            zfar: 25000.0,
            view_matrix: Mat4::IDENTITY,
            proj_matrix: Mat4::IDENTITY,
            camera_state: CameraState::Normal,
            terrain_collision_radius: 10.0,
            process_terrain_collision: true,
        };
        camera.update_matrices();
        camera
    }

    pub fn set_distance(&mut self, dist: f32) {
        self.distance = dist.clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.set_distance(self.distance + delta);
    }

    pub fn rotate(&mut self, pitch_delta: f32, yaw_delta: f32) {
        let mut new_pitch = self.pitch + pitch_delta;
        new_pitch = new_pitch.clamp(10.0, 80.0);
        self.pitch = new_pitch;

        self.yaw += yaw_delta;
        while self.yaw >= 360.0 {
            self.yaw -= 360.0;
        }
        while self.yaw < 0.0 {
            self.yaw += 360.0;
        }
    }

    pub fn get_dynamic_target_height(&self) -> f32 {
        let movable_dist = (CAMERA_MAX_DISTANCE - CAMERA_MIN_DISTANCE).max(1.0);
        let cur_movable_dist = (CAMERA_MAX_DISTANCE - self.distance).max(0.0);
        
        CAMERA_TARGET_STANDARD + (CAMERA_TARGET_FACE - CAMERA_TARGET_STANDARD) * (cur_movable_dist / movable_dist).clamp(0.0, 1.0)
    }

    pub fn update<F: Fn(f32, f32) -> f32>(&mut self, player_pos: Vec3, terrain_height_fn: F) {
        let target_height = self.get_dynamic_target_height();
        self.target_pos = player_pos + Vec3::new(0.0, 0.0, target_height);

        let pitch_rad = self.pitch.to_radians();
        let yaw_rad = self.yaw.to_radians();

        let cos_p = pitch_rad.cos();
        let sin_p = pitch_rad.sin();

        // Eye relative to target. In Metin2, view is roughly towards +Y (North), so eye is at -Y.
        let offset_y = -self.distance;
        
        // Pitch rotates around X (up/down).
        let pitched_y = offset_y * cos_p;
        let pitched_z = -offset_y * sin_p; // z goes up when pitch is positive
        
        // Yaw rotates around Z.
        let cos_y = (-yaw_rad).cos();
        let sin_y = (-yaw_rad).sin();
        
        let rotated_x = pitched_y * -sin_y;
        let rotated_y = pitched_y * cos_y;
        let rotated_z = pitched_z;

        self.eye_pos = self.target_pos + Vec3::new(rotated_x, rotated_y, rotated_z);

        self.view = self.target_pos - self.eye_pos;
        if self.view.length_squared() > f32::EPSILON {
            self.view = self.view.normalize();
        } else {
            self.view = Vec3::new(0.0, 1.0, 0.0);
        }
        
        // Standard D3D-like up vector initially
        let temp_up = Vec3::new(0.0, 0.0, 1.0);
        
        self.cross = temp_up.cross(self.view);
        if self.cross.length_squared() > f32::EPSILON {
            self.cross = self.cross.normalize();
        } else {
            self.cross = Vec3::new(1.0, 0.0, 0.0);
        }
        
        self.up = self.view.cross(self.cross);
        if self.up.length_squared() > f32::EPSILON {
            self.up = self.up.normalize();
        } else {
            self.up = Vec3::new(0.0, 0.0, 1.0);
        }

        if self.process_terrain_collision {
            self.process_terrain_collision(&terrain_height_fn);
        }

        self.update_matrices();
    }

    fn process_terrain_collision<F: Fn(f32, f32) -> f32>(&mut self, terrain_height_fn: &F) {
        self.camera_state = CameraState::Normal;

        // Simplification of CCamera::ProcessTerrainCollision
        // Check terrain height at eye position.
        let terrain_z = terrain_height_fn(self.eye_pos.x, self.eye_pos.y);
        
        // Collision threshold logic from Metin2
        let check_z = self.eye_pos.z - 2.0 * self.terrain_collision_radius * self.up.z;
        
        if check_z < terrain_z {
            self.camera_state = CameraState::CantGoDown;
            let new_eye_z = terrain_z + 2.0 * self.terrain_collision_radius * self.up.z;
            if new_eye_z > self.eye_pos.z {
                self.eye_pos.z = new_eye_z;
                
                // Recalculate view vector due to lifted eye
                self.view = self.target_pos - self.eye_pos;
                if self.view.length_squared() > f32::EPSILON {
                    self.view = self.view.normalize();
                }
            }
        }
    }

    pub fn update_matrices(&mut self) {
        self.view_matrix = Mat4::look_at_rh(self.eye_pos, self.target_pos, self.up);
        self.proj_matrix = Mat4::perspective_rh(self.fov, self.aspect, self.znear, self.zfar);
    }

    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        self.view_matrix.to_cols_array_2d()
    }

    pub fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
        self.proj_matrix.to_cols_array_2d()
    }

    pub fn unproject_ray(&self, screen_x: f32, screen_y: f32, w: f32, h: f32) -> Ray3D {
        if w <= 0.0 || h <= 0.0 {
            return Ray3D {
                origin: Vec3::ZERO,
                direction: Vec3::new(0.0, 0.0, 1.0),
            };
        }

        let ndc_x = (2.0 * screen_x / w) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_y / h);

        let view_proj = self.proj_matrix * self.view_matrix;
        let inv_vp = view_proj.inverse();

        let near_vec = Vec4::new(ndc_x, ndc_y, 0.0, 1.0);
        let near_unprojected = inv_vp * near_vec;
        let p_near = if near_unprojected.w != 0.0 {
            near_unprojected.truncate() / near_unprojected.w
        } else {
            near_unprojected.truncate()
        };

        let far_vec = Vec4::new(ndc_x, ndc_y, 1.0, 1.0);
        let far_unprojected = inv_vp * far_vec;
        let p_far = if far_unprojected.w != 0.0 {
            far_unprojected.truncate() / far_unprojected.w
        } else {
            far_unprojected.truncate()
        };

        let direction = p_far - p_near;
        let direction = if direction.length_squared() > f32::EPSILON {
            direction.normalize()
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };

        Ray3D {
            origin: p_near,
            direction,
        }
    }
}
