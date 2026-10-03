use glam::{Mat4, Vec3};

pub const CAMERA_MIN_DISTANCE: f32 = 200.0;
pub const CAMERA_MAX_DISTANCE: f32 = 2500.0;
pub const CAMERA_TARGET_STANDARD: f32 = 100.0;
pub const CAMERA_TARGET_FACE: f32 = 150.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CameraState {
    Normal,
    CantGoDown,
    CantGoRight,
    CantGoLeft,
    ScreenByBuilding,
    ScreenByBuildingAndTooClose,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct CameraPos {
    pub up_dir: f32,
    pub view_dir: f32,
    pub cross_dir: f32,
}

impl Default for CameraPos {
    fn default() -> Self {
        Self {
            up_dir: 0.0,
            view_dir: 0.0,
            cross_dir: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct CameraSpeed {
    pub up_dir: f32,
    pub view_dir: f32,
    pub cross_dir: f32,
}

impl Default for CameraSpeed {
    fn default() -> Self {
        Self {
            up_dir: 0.0,
            view_dir: 0.0,
            cross_dir: 0.0,
        }
    }
}

pub struct CameraControllerComplete {
    pub distance: f32,
    pub pitch: f32, // degrees
    pub roll: f32,  // degrees

    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub view: Vec3,
    pub cross: Vec3,

    pub view_matrix: Mat4,
    pub proj_matrix: Mat4,

    pub camera_state: CameraState,
    
    pub terrain_collision_radius: f32,
    pub object_collision_radius: f32,
    pub process_terrain_collision: bool,

    pub angular_acceleration: Vec3,
    pub angular_velocity: Vec3,
    
    pub fov: f32,
    pub aspect: f32,
    pub znear: f32,
    pub zfar: f32,
    
    pub target_height: f32,
}

impl Default for CameraControllerComplete {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraControllerComplete {
    pub fn new() -> Self {
        let pitch: f32 = 30.0;
        let distance: f32 = 1500.0;
        let pitch_rad = pitch.to_radians();
        let eye = Vec3::new(0.0, -distance * pitch_rad.cos(), distance * pitch_rad.sin());

        let mut camera = Self {
            distance,
            pitch,
            roll: 0.0,

            eye,
            target: Vec3::ZERO,
            up: Vec3::new(0.0, 0.0, 1.0),
            view: Vec3::new(0.0, 1.0, 0.0),
            cross: Vec3::new(1.0, 0.0, 0.0),

            view_matrix: Mat4::IDENTITY,
            proj_matrix: Mat4::IDENTITY,

            camera_state: CameraState::Normal,
            
            terrain_collision_radius: 10.0,
            object_collision_radius: 10.0,
            process_terrain_collision: true,
            
            angular_acceleration: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            
            fov: 45.0_f32.to_radians(),
            aspect: 16.0 / 9.0,
            znear: 10.0,
            zfar: 25000.0,
            
            target_height: CAMERA_TARGET_STANDARD,
        };
        camera.set_view_matrix();
        camera.update_projection_matrix();
        camera
    }

    pub fn set_terrain_collision(&mut self, enable: bool) {
        self.process_terrain_collision = enable;
    }

    pub fn set_distance(&mut self, dist: f32) {
        if self.distance > 0.0 {
            self.zoom(dist / self.distance);
        } else {
            self.distance = dist;
        }
    }

    pub fn zoom(&mut self, ratio: f32) {
        if ratio == 1.0 {
            return;
        }

        let mut temp = self.eye - self.target;
        temp *= ratio;
        self.eye = temp + self.target;
        self.set_view_matrix();
    }

    pub fn move_along_view(&mut self, distance: f32) {
        let temp = self.view.normalize();
        self.eye += temp * distance;
        self.target += temp * distance;
        self.set_view_matrix();
    }

    pub fn move_along_cross(&mut self, distance: f32) {
        let temp = self.cross.normalize();
        self.eye += temp * distance;
        self.target += temp * distance;
        self.set_view_matrix();
    }

    pub fn move_along_up(&mut self, distance: f32) {
        let temp = self.up.normalize();
        self.target += temp * distance;
        self.eye += temp * distance;
        self.set_view_matrix();
    }

    pub fn move_front(&mut self, distance: f32) {
        let mut temp = Vec3::new(self.view.x, self.view.y, 0.0);
        if temp.length_squared() > f32::EPSILON {
            temp = temp.normalize();
        }
        
        self.eye += temp * distance;
        self.target += temp * distance;
        self.set_view_matrix();
    }

    pub fn move_vertical(&mut self, distance: f32) {
        self.eye.z += distance;
        self.target.z += distance;
        self.set_view_matrix();
    }

    pub fn rotate_eye_around_target(&mut self, pitch_degree: f32, roll_degree: f32) {
        let mut actual_pitch = pitch_degree;
        
        // 머리위로 넘어가기 막기... (Prevent going over head)
        if self.pitch + actual_pitch > 80.0 {
            actual_pitch = 80.0 - self.pitch;
        } else if self.pitch + actual_pitch < -80.0 {
            actual_pitch = -80.0 - self.pitch;
        }

        let pitch_rad = actual_pitch.to_radians();
        let roll_rad = -roll_degree.to_radians(); // -D3DXToRadian(fRollDegree)

        // Rotation around cross (pitch)
        let cos_p = pitch_rad.cos();
        let sin_p = pitch_rad.sin();
        let cross = self.cross;
        
        let pitch_rot = |v: Vec3| -> Vec3 {
            v * cos_p + cross.cross(v) * sin_p + cross * cross.dot(v) * (1.0 - cos_p)
        };

        // Rotation around Z (roll)
        let cos_r = roll_rad.cos();
        let sin_r = roll_rad.sin();
        let roll_rot = |v: Vec3| -> Vec3 {
            Vec3::new(
                v.x * cos_r - v.y * sin_r,
                v.x * sin_r + v.y * cos_r,
                v.z
            )
        };

        let temp = self.eye - self.target;
        let mut rotated = pitch_rot(temp);
        rotated = roll_rot(rotated);
        
        self.eye = self.target + rotated;
        self.set_up(Vec3::new(0.0, 0.0, 1.0));

        self.roll += roll_degree;

        if self.roll > 360.0 {
            self.roll -= 360.0;
        } else if self.roll < -360.0 {
            self.roll += 360.0;
        }
        
        self.pitch += actual_pitch;
    }

    pub fn pitch(&mut self, pitch_delta: f32) {
        self.rotate_eye_around_target(pitch_delta, 0.0);
    }

    pub fn roll(&mut self, roll_delta: f32) {
        self.rotate_eye_around_target(0.0, roll_delta);
    }

    pub fn set_up(&mut self, up: Vec3) {
        self.up = up;
        self.set_view_matrix();
    }

    pub fn set_view_matrix(&mut self) {
        self.view = self.target - self.eye;
        if self.view.length_squared() > f32::EPSILON {
            self.view = self.view.normalize();
        } else {
            self.view = Vec3::new(0.0, 0.0, 1.0);
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
            self.up = Vec3::new(0.0, 1.0, 0.0);
        }
        
        self.distance = (self.eye - self.target).length();

        self.view_matrix = Mat4::look_at_rh(self.eye, self.target, self.up);
    }
    
    pub fn update_projection_matrix(&mut self) {
        self.proj_matrix = Mat4::perspective_rh(self.fov, self.aspect, self.znear, self.zfar);
    }

    fn process_terrain_collision<F: Fn(f32, f32) -> f32>(&mut self, get_height: &F) {
        self.camera_state = CameraState::Normal;

        // m_kTargetToCameraBottomRay
        let dir = self.eye - self.target;
        let num_steps = 10;
        let step = dir / num_steps as f32;
        let mut cur_pos = self.target;
        
        let mut hit = false;
        let mut collision_pt = cur_pos;
        
        for _ in 0..num_steps {
            cur_pos += step;
            let h = get_height(cur_pos.x, cur_pos.y);
            if cur_pos.z < h {
                hit = true;
                collision_pt = Vec3::new(cur_pos.x, cur_pos.y, h);
                break;
            }
        }
        
        if hit {
            self.camera_state = CameraState::CantGoDown;
            let check_vector = self.eye - 2.0 * self.terrain_collision_radius * self.up;
            let h = get_height(check_vector.x, check_vector.y);
            let check_vector_z = h;
            
            let new_eye = Vec3::new(check_vector.x, check_vector.y, check_vector_z) + 2.0 * self.terrain_collision_radius * self.up;
            if new_eye.z > self.eye.z {
                self.eye = new_eye;
                self.set_view_matrix();
            }
        } else {
            // Check bottom ray directly below camera
            let h = get_height(self.eye.x, self.eye.y);
            let diff = self.eye.z - h;
            if diff < 2.0 * self.terrain_collision_radius {
                self.camera_state = CameraState::CantGoDown;
                let new_eye = Vec3::new(self.eye.x, self.eye.y, h) + 2.0 * self.terrain_collision_radius * self.up;
                if new_eye.z > self.eye.z {
                    self.eye = new_eye;
                    self.set_view_matrix();
                }
            }
        }
    }

    pub fn update<F: Fn(f32, f32) -> f32>(&mut self, get_height: F) {
        self.rotate_eye_around_target(self.angular_velocity.z, self.angular_velocity.x);

        let cur_dist = self.distance - self.angular_velocity.y;
        let new_distance = CAMERA_MIN_DISTANCE.max(CAMERA_MAX_DISTANCE.min(cur_dist));
        self.zoom(new_distance / self.distance.max(1.0));

        if self.process_terrain_collision {
            self.process_terrain_collision(&get_height);
        }

        self.angular_velocity *= 0.5;
        if self.angular_velocity.x.abs() < 1.0 { self.angular_velocity.x = 0.0; }
        if self.angular_velocity.y.abs() < 1.0 { self.angular_velocity.y = 0.0; }
        if self.angular_velocity.z.abs() < 1.0 { self.angular_velocity.z = 0.0; }

        let camera_movable_distance = CAMERA_MAX_DISTANCE - CAMERA_MIN_DISTANCE;
        let camera_target_delta = CAMERA_TARGET_FACE - CAMERA_TARGET_STANDARD;
        let camera_cur_movable_distance = CAMERA_MAX_DISTANCE - self.distance;
        
        let new_target_height = CAMERA_TARGET_STANDARD + camera_target_delta * camera_cur_movable_distance / camera_movable_distance;
        
        self.target_height = new_target_height;
    }
}
