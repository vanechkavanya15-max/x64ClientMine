 //! Wzorzec referencyjny C++: `source/UserInterface/PythonApplicationCamera.cpp oraz source/EterLib/Camera.cpp`
 //! Opis: Odwzorowanie klasy CCamera i metod kamery w CPythonApplication: Pitch, Roll, Distance, interpolacja pozycji oka i celu, zapobieganie przenikaniu kamery pod ziemie.
 
use glam::{Mat4, Vec2, Vec3};
use std::f32::consts::PI;

pub const CAMERA_TARGET_STANDARD: f32 = 100.0;
pub const CAMERA_TARGET_FACE: f32 = 150.0;
pub const CAMERA_MIN_DISTANCE: f32 = 200.0;
pub const CAMERA_MAX_DISTANCE: f32 = 2500.0;
pub const DEFAULT_RESISTANCE: f32 = 0.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraState {
    Normal,
    CantGoDown,
    CantGoRight,
    CantGoLeft,
    ScreenByBuilding,
    ScreenByBuildingAndTooClose,
}

impl Default for CameraState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    Normal = 0,
    Stand = 1,
    Blend = 2,
}

impl Default for CameraMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CameraPos {
    pub up_dir: f32,
    pub view_dir: f32,
    pub cross_dir: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CameraSpeed {
    pub up_dir: f32,
    pub view_dir: f32,
    pub cross_dir: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CameraSetting {
    pub center_position: Vec3,
    pub cmr_pos: CameraPos,
    pub rotation: f32,
    pub pitch: f32,
    pub zoom: f32,
}

#[derive(Debug, Clone)]
 pub struct AppCamera {
     pub initialized: bool,

    // Core CCamera properties
    pub camera_state: CameraState,
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub view: Vec3,
    pub cross: Vec3,

    pub pitch: f32,
    pub roll: f32,
    pub distance: f32,

    pub view_matrix: Mat4,
    pub inverse_view_matrix: Mat4,
    pub billboard_matrix: Mat4,

    pub is_lock: bool,
    pub drag: bool,
    pub mouse_pos_x: i32,
    pub mouse_pos_y: i32,
    pub pitch_sum: f32,
    pub roll_sum: f32,

    pub angular_velocity: Vec3,
    pub resistance: f32,
    pub target_height: f32,

    pub process_terrain_collision: bool,

    // CPythonApplication properties
    pub camera_mode: CameraMode,
    pub blend_camera_start_time: f32,
    pub blend_camera_blend_time: f32,
    pub event_camera_setting: CameraSetting,
    pub end_blend_camera_setting: CameraSetting,
    pub default_camera_setting: CameraSetting,

    pub rotation_speed: f32,
    pub pitch_speed: f32,
    pub zoom_speed: f32,
    pub cmr_pos: CameraPos,
    pub cmr_spd: CameraSpeed,
    pub center_position: Vec3,
}

impl Default for AppCamera {
    fn default() -> Self {
        let cam = Self {
            initialized: true,
            camera_state: CameraState::Normal,
            eye: Vec3::new(0.0, 0.0, 1.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            view: Vec3::ZERO,
            cross: Vec3::ZERO,
            pitch: 0.0,
            roll: 0.0,
            distance: 1.0,
            view_matrix: Mat4::IDENTITY,
            inverse_view_matrix: Mat4::IDENTITY,
            billboard_matrix: Mat4::IDENTITY,
            is_lock: false,
            drag: false,
            mouse_pos_x: -1,
            mouse_pos_y: -1,
            pitch_sum: 0.0,
            roll_sum: 0.0,
            angular_velocity: Vec3::ZERO,
            resistance: DEFAULT_RESISTANCE,
            target_height: CAMERA_TARGET_STANDARD,
            process_terrain_collision: true,
            camera_mode: CameraMode::Normal,
            blend_camera_start_time: 0.0,
            blend_camera_blend_time: 0.0,
            event_camera_setting: CameraSetting::default(),
            end_blend_camera_setting: CameraSetting::default(),
            default_camera_setting: CameraSetting::default(),
            rotation_speed: 0.0,
            pitch_speed: 0.0,
            zoom_speed: 0.0,
            cmr_pos: CameraPos::default(),
            cmr_spd: CameraSpeed::default(),
            center_position: Vec3::ZERO,
        };
        cam
    }
}

impl AppCamera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}
 
impl AppCamera {
    pub fn set_view_params(&mut self, eye: Vec3, target: Vec3, up: Vec3) {
        if self.is_lock {
            return;
        }
        self.eye = eye;
        self.target = target;
        self.up = up;
        self.set_view_matrix();
    }

    pub fn set_eye(&mut self, eye: Vec3) {
        if self.is_lock {
            return;
        }
        self.eye = eye;
        self.set_view_matrix();
    }

    pub fn set_target(&mut self, target: Vec3) {
        if self.is_lock {
            return;
        }
        self.target = target;
        self.set_view_matrix();
    }

    pub fn set_up(&mut self, up: Vec3) {
        if self.is_lock {
            return;
        }
        self.up = up;
        self.set_view_matrix();
    }

    pub fn calculate_roll(&mut self) {
        let view_xy = Vec2::new(self.view.x, self.view.y).normalize_or_zero();
        let vv = Vec2::new(0.0, 1.0);
        let mut dot = view_xy.dot(vv);
        if dot >= 1.0 {
            dot = 1.0;
        } else if dot <= -1.0 {
            dot = -1.0;
        }
        dot = dot.acos();
        dot *= 180.0 / PI;

        // D3DXVec2CCW equivalent: v1.x * v2.y - v1.y * v2.x
        let cross = view_xy.x * vv.y - view_xy.y * vv.x;
        if cross < 0.0 {
            dot = -dot;
        }
        self.roll = dot;
    }

    pub fn set_view_matrix(&mut self) {
        self.view = self.target - self.eye;
        
        self.calculate_roll();
        self.distance = self.view.length();
        self.view = self.view.normalize_or_zero();

        self.cross = self.up.cross(self.view).normalize_or_zero();
        self.up = self.view.cross(self.cross).normalize_or_zero();

        let vv = Vec3::new(0.0, 0.0, 1.0);
        let mut dot = self.up.dot(vv);
        if dot >= 1.0 {
            dot = 1.0;
        } else if dot <= -1.0 {
            dot = -1.0;
        }
        self.pitch = dot.acos();
        self.pitch *= 180.0 / PI;
        if self.view.z > 0.0 {
            self.pitch = -self.pitch;
        }

        self.view_matrix = Mat4::look_at_rh(self.eye, self.target, self.up);
        self.inverse_view_matrix = self.view_matrix.inverse();

        self.billboard_matrix = self.inverse_view_matrix;
        // self.billboard_matrix._41, _42, _43 = 0.0, which means translation component is 0
        self.billboard_matrix.w_axis.x = 0.0;
        self.billboard_matrix.w_axis.y = 0.0;
        self.billboard_matrix.w_axis.z = 0.0;

        // Rays and collisions are omitted as per CPythonApplicationCamera / EterLib Camera translation
        // but we handle basic state here.
    }

    pub fn r#move(&mut self, displacement: Vec3) {
        if self.is_lock {
            return;
        }
        self.eye += displacement;
        self.target += displacement;
        self.set_view_matrix();
    }

    pub fn zoom(&mut self, ratio: f32) {
        if self.is_lock || ratio == 1.0 {
            return;
        }
        let temp = (self.eye - self.target) * ratio;
        self.eye = temp + self.target;
        self.set_view_matrix();
    }

    pub fn move_along_view(&mut self, distance: f32) {
        if self.is_lock {
            return;
        }
        let temp = self.view.normalize_or_zero();
        self.eye += temp * distance;
        self.target += temp * distance;
        self.set_view_matrix();
    }

    pub fn move_along_cross(&mut self, distance: f32) {
        if self.is_lock {
            return;
        }
        let temp = self.cross.normalize_or_zero();
        self.eye += temp * distance;
        self.target += temp * distance;
        self.set_view_matrix();
    }

    pub fn move_along_up(&mut self, distance: f32) {
        if self.is_lock {
            return;
        }
        let temp = self.up.normalize_or_zero();
        self.target += temp * distance;
        self.eye += temp * distance;
        self.set_view_matrix();
    }

    pub fn move_lateral(&mut self, distance: f32) {
        self.move_along_cross(distance);
    }

    pub fn move_front(&mut self, distance: f32) {
        if self.is_lock {
            return;
        }
        let temp = Vec3::new(self.view.x, self.view.y, 0.0).normalize_or_zero();
        self.eye += temp * distance;
        self.target += temp * distance;
        self.set_view_matrix();
    }

    pub fn move_vertical(&mut self, distance: f32) {
        if self.is_lock {
            return;
        }
        self.eye.z += distance;
        self.target.z += distance;
        self.set_view_matrix();
    }

    pub fn rotate_eye_around_target(&mut self, mut pitch_degree: f32, roll_degree: f32) {
        if self.is_lock {
            return;
        }

        if self.pitch + pitch_degree > 80.0 {
            pitch_degree = 80.0 - self.pitch;
        } else if self.pitch + pitch_degree < -80.0 {
            pitch_degree = -80.0 - self.pitch;
        }

        let mat_rot_pitch = Mat4::from_axis_angle(self.cross, pitch_degree.to_radians());
        let mat_rot_roll = Mat4::from_rotation_z(-roll_degree.to_radians());
        let mat_rot = mat_rot_pitch * mat_rot_roll;

        let temp = self.eye - self.target;
        self.eye = mat_rot.transform_vector3(temp) + self.target;

        self.set_up(Vec3::new(0.0, 0.0, 1.0));

        self.roll += roll_degree;
        if self.roll > 360.0 {
            self.roll -= 360.0;
        } else if self.roll < -360.0 {
            self.roll += 360.0;
        }
    }

    pub fn do_pitch(&mut self, pitch_delta: f32) {
        self.rotate_eye_around_target(pitch_delta, 0.0);
    }

    pub fn do_roll(&mut self, roll_delta: f32) {
        self.rotate_eye_around_target(0.0, roll_delta);
    }

    pub fn set_distance(&mut self, distance: f32) {
        if self.distance != 0.0 {
            self.zoom(distance / self.distance);
        }
    }

    pub fn lock(&mut self) {
        self.is_lock = true;
    }

    pub fn unlock(&mut self) {
        self.is_lock = false;
    }

    pub fn wheel(&mut self, len: i32) {
        if self.is_lock {
            return;
        }
        self.angular_velocity.y = (len as f32) * self.resistance;
    }

    pub fn begin_drag(&mut self, mouse_x: i32, mouse_y: i32) {
        if self.is_lock {
            return;
        }
        self.drag = true;
        self.mouse_pos_x = mouse_x;
        self.mouse_pos_y = mouse_y;
        self.pitch_sum = 0.0;
        self.roll_sum = 0.0;
    }

    pub fn end_drag(&mut self) -> bool {
        if self.is_lock {
            return false;
        }
        self.drag = false;
        let sum = (self.pitch_sum * self.pitch_sum + self.roll_sum * self.roll_sum).sqrt();
        self.pitch_sum = 0.0;
        self.roll_sum = 0.0;
        sum >= 1.0
    }

    pub fn drag_update(&mut self, mouse_x: i32, mouse_y: i32, return_x: &mut i32, return_y: &mut i32) -> bool {
        if self.is_lock {
            return false;
        }
        if !self.drag {
            self.mouse_pos_x = mouse_x;
            self.mouse_pos_y = mouse_y;
            *return_x = self.mouse_pos_x;
            *return_y = self.mouse_pos_y;
            return false;
        }

        let mut new_pitch_velocity = (mouse_y - self.mouse_pos_y) as f32 * self.resistance;
        let mut new_rotation_velocity = (mouse_x - self.mouse_pos_x) as f32 * self.resistance;

        self.pitch_sum += new_pitch_velocity;
        self.roll_sum += new_rotation_velocity;

        if self.camera_state == CameraState::CantGoLeft {
            new_rotation_velocity = new_rotation_velocity.max(0.0);
        }
        if self.camera_state == CameraState::CantGoRight {
            new_rotation_velocity = new_rotation_velocity.min(0.0);
        }
        if self.camera_state == CameraState::CantGoDown {
            new_pitch_velocity = new_pitch_velocity.max(0.0);
        }

        self.angular_velocity.x = new_rotation_velocity;
        self.angular_velocity.z = new_pitch_velocity;

        *return_x = self.mouse_pos_x;
        *return_y = self.mouse_pos_y;
        true
    }
}

fn blend_value_by_linear(elapsed: f32, duration: f32, start: f32, end: f32) -> f32 {
    if duration <= 0.0 {
        return end;
    }
    if elapsed >= duration {
        return end;
    }
    start + (end - start) * (elapsed / duration)
}

impl AppCamera {
    pub fn update_camera(&mut self, current_time: f32) {
        if self.camera_mode == CameraMode::Blend {
            let elapsed_time = current_time - self.blend_camera_start_time;

            let fx_center = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.center_position.x,
                self.end_blend_camera_setting.center_position.x,
            );
            let fy_center = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.center_position.y,
                self.end_blend_camera_setting.center_position.y,
            );
            let fz_center = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.center_position.z,
                self.end_blend_camera_setting.center_position.z,
            );

            let distance = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.zoom,
                self.end_blend_camera_setting.zoom,
            );
            let pitch = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.pitch,
                self.end_blend_camera_setting.pitch,
            );
            let rotation = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.rotation,
                self.end_blend_camera_setting.rotation,
            );

            let up_dir = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.cmr_pos.up_dir,
                self.end_blend_camera_setting.cmr_pos.up_dir,
            );
            let view_dir = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.cmr_pos.view_dir,
                self.end_blend_camera_setting.cmr_pos.view_dir,
            );
            let cross_dir = blend_value_by_linear(
                elapsed_time,
                self.blend_camera_blend_time,
                self.event_camera_setting.cmr_pos.cross_dir,
                self.end_blend_camera_setting.cmr_pos.cross_dir,
            );

            self.unlock();
            self.set_position_camera(
                fx_center, fy_center, fz_center, distance, pitch, rotation,
            );
            self.move_vertical(up_dir);
            self.move_front(view_dir);
            self.move_along_cross(cross_dir);
            self.lock();
        } else if self.camera_mode == CameraMode::Stand {
            let distance = self.distance;
            let pitch = self.pitch;
            let rotation = self.roll;
            let target_z = self.target_height;

            self.set_position_camera(
                self.event_camera_setting.center_position.x,
                self.event_camera_setting.center_position.y,
                self.event_camera_setting.center_position.z + target_z,
                distance,
                pitch,
                rotation,
            );
        } else if self.camera_mode == CameraMode::Normal {
            let distance = self.distance;
            let pitch = self.pitch;
            let rotation = self.roll;
            let target_z = self.target_height;

            self.set_position_camera(
                self.center_position.x,
                self.center_position.y,
                self.center_position.z + target_z,
                distance,
                pitch,
                rotation,
            );
        }

        if self.rotation_speed != 0.0 {
            let rs = self.rotation_speed;
            self.do_roll(rs);
        }
        if self.pitch_speed != 0.0 {
            let ps = self.pitch_speed;
            self.do_pitch(ps);
        }
        if self.zoom_speed != 0.0 {
            let zs = self.zoom_speed;
            self.zoom(zs);
        }

        if self.cmr_spd.view_dir != 0.0 {
            self.cmr_pos.view_dir += self.cmr_spd.view_dir;
        }
        if self.cmr_spd.cross_dir != 0.0 {
            self.cmr_pos.cross_dir += self.cmr_spd.cross_dir;
        }
        if self.cmr_spd.up_dir != 0.0 {
            self.cmr_pos.up_dir += self.cmr_spd.up_dir;
        }

        if self.cmr_pos.view_dir != 0.0 {
            let vd = self.cmr_pos.view_dir;
            self.move_front(vd);
        }
        if self.cmr_pos.cross_dir != 0.0 {
            let cd = self.cmr_pos.cross_dir;
            self.move_along_cross(cd);
        }
        if self.cmr_pos.up_dir != 0.0 {
            let ud = self.cmr_pos.up_dir;
            self.move_vertical(ud);
        }
    }

    pub fn set_position_camera(
        &mut self,
        fx: f32,
        fy: f32,
        fz: f32,
        distance: f32,
        mut pitch: f32,
        roll: f32,
    ) {
        self.set_view_params(
            Vec3::new(fx, fy - distance, fz),
            Vec3::new(fx, fy, fz),
            Vec3::new(0.0, 0.0, 1.0),
        );
        pitch = pitch.clamp(-80.0, 80.0);
        self.rotate_eye_around_target(pitch, roll);
    }

    pub fn set_event_camera(&mut self, setting: CameraSetting) {
        // Omitting camera switching as we only have 1 active AppCamera here.
        self.set_camera_setting(setting);
        self.event_camera_setting = setting;
        self.camera_mode = CameraMode::Stand;
    }

    pub fn blend_event_camera(&mut self, setting: CameraSetting, blend_time: f32, current_time: f32) {
        self.camera_mode = CameraMode::Blend;
        self.blend_camera_start_time = current_time;
        self.blend_camera_blend_time = blend_time;
        self.end_blend_camera_setting = setting;
        self.lock();
    }

    pub fn set_default_camera(&mut self) {
        self.camera_mode = CameraMode::Normal;
        self.blend_camera_start_time = 0.0;
        self.blend_camera_blend_time = 0.0;
        self.unlock();
        // Assuming we reset to default
        let default = self.default_camera_setting;
        self.set_camera_setting(default);
    }

    pub fn set_camera_setting(&mut self, setting: CameraSetting) {
        self.set_position_camera(
            setting.center_position.x,
            setting.center_position.y,
            setting.center_position.z,
            setting.zoom,
            setting.pitch,
            setting.rotation,
        );

        if setting.cmr_pos.view_dir != 0.0 {
            self.move_front(setting.cmr_pos.view_dir);
        }
        if setting.cmr_pos.cross_dir != 0.0 {
            self.move_along_cross(setting.cmr_pos.cross_dir);
        }
        if setting.cmr_pos.up_dir != 0.0 {
            self.move_vertical(setting.cmr_pos.up_dir);
        }

        self.cmr_pos.up_dir = 0.0;
        self.cmr_pos.view_dir = 0.0;
        self.cmr_pos.cross_dir = 0.0;
        self.cmr_spd.up_dir = 0.0;
        self.cmr_spd.view_dir = 0.0;
        self.cmr_spd.cross_dir = 0.0;
        self.zoom_speed = 0.0;
        self.pitch_speed = 0.0;
        self.rotation_speed = 0.0;
    }

    pub fn update<F>(&mut self, terrain_height_func: Option<F>)
    where F: Fn(f32, f32) -> f32
    {
        self.rotate_eye_around_target(self.angular_velocity.z, self.angular_velocity.x);

        let new_distance = (self.distance - self.angular_velocity.y).clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
        self.set_distance(new_distance);

        // Terrain collision processing dummy call, logic would be hooked to terrain system
        if self.process_terrain_collision && terrain_height_func.is_some() {
            let get_height = terrain_height_func.as_ref().unwrap();
            let check_pos = self.eye - 2.0 * 50.0 /* m_fTerrainCollisionRadius */ * self.up;
            let height = get_height(check_pos.x, check_pos.y);
            if height > self.eye.z {
                self.eye.z = height + 10.0;
                self.set_view_matrix();
            }
        }

        self.angular_velocity *= 0.5;
        if self.angular_velocity.x.abs() < 1.0 {
            self.angular_velocity.x = 0.0;
        }
        if self.angular_velocity.y.abs() < 1.0 {
            self.angular_velocity.y = 0.0;
        }
        if self.angular_velocity.z.abs() < 1.0 {
            self.angular_velocity.z = 0.0;
        }
 
        let camera_movable_distance = CAMERA_MAX_DISTANCE - CAMERA_MIN_DISTANCE;
        let camera_target_delta = CAMERA_TARGET_FACE - CAMERA_TARGET_STANDARD;
        let camera_cur_movable_distance = CAMERA_MAX_DISTANCE - self.distance;
        
        let new_target_height = CAMERA_TARGET_STANDARD
            + camera_target_delta * camera_cur_movable_distance / camera_movable_distance;
        self.target_height = new_target_height;
     }
 }
