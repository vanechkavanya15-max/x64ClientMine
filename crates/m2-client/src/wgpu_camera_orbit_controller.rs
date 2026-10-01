use glam::{Mat4, Vec3};

const CAMERA_MIN_DISTANCE: f32 = 400.0;
const CAMERA_MAX_DISTANCE: f32 = 2500.0;

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

pub struct CameraOrbitController {
    pub distance: f32,
    pub pitch: f32, // degrees
    pub yaw: f32, // degrees
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

impl Default for CameraOrbitController {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraOrbitController {
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
            terrain_collision_radius: 10.0, // Default terrain collision radius
            process_terrain_collision: true,
        };
        camera.update_matrices();
        camera
    }

    pub fn set_distance(&mut self, dist: f32) {
        self.distance = dist.clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
    }

    pub fn zoom(&mut self, ratio: f32) {
        self.set_distance(self.distance * ratio);
    }

    pub fn rotate_eye_around_target(&mut self, pitch_degree: f32, roll_degree: f32) {
        let mut pitch_delta = pitch_degree;
        if self.pitch + pitch_delta > 80.0 {
            pitch_delta = 80.0 - self.pitch;
        } else if self.pitch + pitch_delta < 10.0 {
            pitch_delta = 10.0 - self.pitch;
        }

        self.pitch += pitch_delta;
        
        self.yaw += roll_degree;
        while self.yaw > 360.0 {
            self.yaw -= 360.0;
        }
        while self.yaw < 0.0 {
            self.yaw += 360.0;
        }
    }
    
    pub fn pitch(&mut self, pitch_delta: f32) {
        self.rotate_eye_around_target(pitch_delta, 0.0);
    }
    
    pub fn roll(&mut self, roll_delta: f32) {
        self.rotate_eye_around_target(0.0, roll_delta);
    }

    pub fn update<F: Fn(f32, f32) -> f32>(&mut self, player_pos: Vec3, terrain_height_fn: F) {
        self.target_pos = player_pos + Vec3::new(0.0, 0.0, 150.0);

        // Spherical coords
        // In M2:
        // Yaw (Roll in M2 terminology): rotation around Z axis. 0 = looking North (Y+).
        // Pitch: rotation around X axis. 0 = looking horizontal. 90 = looking down.
        // Eterlib Camera uses Pitch/Roll, and D3DXMatrixRotationAxis.
        
        // Eterlib CCamera RotateEyeAroundTarget equivalent logic:
        // m_v3Eye calculation based on target, distance, pitch and yaw.
        let pitch_rad = self.pitch.to_radians();
        let yaw_rad = self.yaw.to_radians();

        // Calculate offset vector based on pitch and yaw
        // In M2, standard Z is up. Y is North. X is East.
        // Assuming target is origin:
        // eye is at (0, -distance, 0) before rotation.
        // Rotate by pitch around X axis (cross vector)
        // Rotate by roll around Z axis
        
        // CCamera uses rotate pitch then roll (around Z).
        let cos_p = pitch_rad.cos();
        let sin_p = pitch_rad.sin();
        let cos_y = yaw_rad.cos();
        let sin_y = yaw_rad.sin();

        // Standard position looking from South to North
        let mut offset = Vec3::new(0.0, -self.distance, 0.0);
        
        // Pitch rotation
        // In order to look down from above, a positive pitch should elevate the Z coordinate.
        // We negate the pitch rotation here so it rotates 'up' over the target.
        let mut pitched = Vec3::new(
            offset.x,
            offset.y * cos_p + offset.z * sin_p,
            -offset.y * sin_p + offset.z * cos_p
        );
        
        // Yaw/Roll rotation (around Z axis)
        // D3DXMatrixRotationZ(&matRotRoll, -D3DXToRadian(fRollDegree));
        let yaw_rot = -yaw_rad; // -D3DXToRadian(fRollDegree)
        let cos_y = yaw_rot.cos();
        let sin_y = yaw_rot.sin();
        let rotated = Vec3::new(
            pitched.x * cos_y - pitched.y * sin_y,
            pitched.x * sin_y + pitched.y * cos_y,
            pitched.z
        );

        self.eye_pos = self.target_pos + rotated;

        self.up = Vec3::new(0.0, 0.0, 1.0);
        self.view = self.target_pos - self.eye_pos;
        if self.view.length_squared() > f32::EPSILON {
            self.view = self.view.normalize();
        } else {
            self.view = Vec3::new(0.0, 1.0, 0.0); // default
        }
        
        self.cross = self.up.cross(self.view);
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
        // CCamera::ProcessTerrainCollision
        self.camera_state = CameraState::Normal;

        // m_kTargetToCameraBottomRay
        // Start: Target
        // Dir: CenterRay - TerrainCollisionRadius * Up
        
        // This is a simplified raycast checking points along the ray.
        // We will do a line-of-sight check from target_pos to eye_pos.
        let num_steps = 10;
        let step_vec = (self.eye_pos - self.target_pos) / num_steps as f32;
        
        let mut collision_point: Option<Vec3> = None;
        let mut cur_pos = self.target_pos;
        
        for _ in 0..num_steps {
            cur_pos += step_vec;
            let terrain_z = terrain_height_fn(cur_pos.x, cur_pos.y);
            // Add margin for collision radius (2.0 * m_fTerrainCollisionRadius)
            let check_z = cur_pos.z - 2.0 * self.terrain_collision_radius * self.up.z;
            
            if check_z < terrain_z {
                // Ray hit terrain
                collision_point = Some(Vec3::new(cur_pos.x, cur_pos.y, terrain_z));
                break;
            }
        }

        if let Some(col_pt) = collision_point {
            self.camera_state = CameraState::CantGoDown;
            let new_eye = col_pt + 2.0 * self.terrain_collision_radius * self.up;
            if new_eye.z > self.eye_pos.z {
                self.eye_pos = new_eye;
                // Recompute distance
                self.distance = (self.eye_pos - self.target_pos).length();
            }
        }
    }

    pub fn update_matrices(&mut self) {
        // Equivalent to D3DXMatrixLookAtRH
        // look_at_rh requires eye, center, up
        self.view_matrix = Mat4::look_at_rh(self.eye_pos, self.target_pos, self.up);
        // Equivalent to D3DXMatrixPerspectiveFovRH for WGPU (0 to 1 depth range)
        self.proj_matrix = Mat4::perspective_rh(self.fov, self.aspect, self.znear, self.zfar);
    }
}