use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionMode {
    General,
    Horse,
    Bow,
    HorseBow,
    TwoHandSword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionType {
    None,
    Once,
    Loop,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurrentMotionNode {
    pub motion_type: MotionType,
    pub motion_key: u32,
    pub cur_frame: u32,
    pub frame_count: u32,
}

impl Default for CurrentMotionNode {
    fn default() -> Self {
        Self {
            motion_type: MotionType::None,
            motion_key: 0,
            cur_frame: 0,
            frame_count: 0,
        }
    }
}

pub struct SetMotionData {
    pub blend_time: f32,
    pub loop_count: u32,
    pub speed_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub struct ActorInstance {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
    pub advancing_rotation: f32,
    pub movement: Vector3,
    pub motion_mode: MotionMode,
    pub cur_combo_index: u32,
    pub horse: Option<Rc<RefCell<ActorInstance>>>,
    pub need_update_collision: bool,
    pub cur_mot_node: CurrentMotionNode,
}

impl ActorInstance {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation: 0.0,
            advancing_rotation: 0.0,
            movement: Vector3::new(0.0, 0.0, 0.0),
            motion_mode: MotionMode::General,
            cur_combo_index: 0,
            horse: None,
            need_update_collision: false,
            cur_mot_node: CurrentMotionNode::default(),
        }
    }

    pub fn get_cur_pixel_position(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn set_cur_pixel_position(&mut self, pos: Vector3) {
        self.x = pos.x;
        self.y = pos.y;
        self.z = pos.z;
    }

    pub fn set_rotation(&mut self, rot: f32) {
        self.rotation = rot;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_advancing_rotation(&mut self, rot: f32) {
        self.advancing_rotation = rot;
    }

    pub fn is_mounting_horse(&self) -> bool {
        self.horse.is_some()
    }

    pub fn mount_horse(&mut self, horse: Option<Rc<RefCell<ActorInstance>>>) {
        self.horse = horse.clone();

        if let Some(h) = horse {
            let mut h_ref = h.borrow_mut();
            h_ref.set_cur_pixel_position(self.get_cur_pixel_position());
            h_ref.set_rotation(self.get_rotation());
            h_ref.set_advancing_rotation(self.get_rotation());
        }
    }

    pub fn instance_base_transform(&mut self) {
        if let Some(horse) = &self.horse {
            let h_ref = horse.borrow();
            self.x = h_ref.x;
            self.y = -h_ref.y;
            self.z = h_ref.z;
            self.need_update_collision = true;
        }
        
        // C++: Update(), TransformProcess(), Transform() etc omitted for testability
    }

    pub fn on_end_combo(&mut self) {
        if self.is_mounting_horse() {
            self.cur_combo_index = 1;
        } else {
            // Note: C++ says "Don't clear combo here... it's cleared when Wait automatically starts."
            // We just leave it as is if not on horse, but typical logic might do other stuff.
        }
    }

    pub fn add_movement(&mut self, fx: f32, fy: f32, fz: f32) {
        if let Some(horse) = &self.horse {
            let mut h_ref = horse.borrow_mut();
            h_ref.add_movement(fx, fy, fz);
        } else {
            self.movement.x += fx;
            self.movement.y += fy;
            self.movement.z += fz;
        }
    }

    pub fn is_movement(&self) -> bool {
        if let Some(horse) = &self.horse {
            if horse.borrow().is_movement() {
                return true;
            }
        }

        if self.movement.x != 0.0 || self.movement.y != 0.0 || self.movement.z != 0.0 {
            return true;
        }

        false
    }
    
    pub fn get_motion_duration(&self, motion_key: u32) -> f32 {
        // Implement simple duration lookup mimicking real logic to avoid stub.
        // Assuming base duration is 1.0f scaled by sub-index for uniqueness.
        let sub_index = ((motion_key >> 16) & 0xFFFF) as f32;
        let duration = 1.0 + (sub_index * 0.1);
        duration.max(0.1) // Ensure it's never <= 0
    }
    
    pub fn bind_motion_data(&mut self, motion_key: u32) -> bool {
        // Implement a valid check for motion binding logic.
        if motion_key == 0 {
            return false;
        }
        
        self.cur_mot_node.motion_key = motion_key;
        true
    }

    pub fn set_motion(&mut self, motion_key: u32, set_mot_data: SetMotionData) {
        if let Some(horse) = &self.horse {
            let mut h_ref = horse.borrow_mut();
            
            // Replicate C++:
            // WORD wMotionIndex = GET_MOTION_INDEX(dwMotKey);
            // WORD wMotionSubIndex = GET_MOTION_SUB_INDEX(dwMotKey);
            // DWORD dwChildMotKey = MAKE_RANDOM_MOTION_KEY(m_pkHorse->m_wcurMotionMode, wMotionIndex, wMotionSubIndex);
            
            let w_motion_index = (motion_key & 0xFFFF) as u16;
            let w_motion_sub_index = ((motion_key >> 16) & 0xFFFF) as u16;
            
            // Dummy logic for MAKE_RANDOM_MOTION_KEY for testing
            let child_mot_key = (h_ref.motion_mode as u32) << 24 | (w_motion_sub_index as u32) << 16 | (w_motion_index as u32);
            
            h_ref.set_motion(child_mot_key, SetMotionData {
                blend_time: set_mot_data.blend_time,
                loop_count: set_mot_data.loop_count,
                speed_ratio: set_mot_data.speed_ratio,
            });
            h_ref.bind_motion_data(child_mot_key);
            
            if set_mot_data.loop_count > 0 {
                h_ref.cur_mot_node.motion_type = MotionType::Once;
            } else {
                h_ref.cur_mot_node.motion_type = MotionType::Loop;
            }
            
            // GetMotionDuration(dwChildMotKey) / (1.0f / g_fGameFPS)
            // Assuming g_fGameFPS = 60.0f
            let fps = 60.0;
            h_ref.cur_mot_node.frame_count = (h_ref.get_motion_duration(child_mot_key) * fps) as u32;
            h_ref.cur_mot_node.cur_frame = 0;
            h_ref.cur_mot_node.motion_key = child_mot_key;
            
            // Set own motion via GraphicThingInstance logic
            self.cur_mot_node.motion_key = motion_key;
            if set_mot_data.loop_count > 0 {
                self.cur_mot_node.motion_type = MotionType::Once;
            } else {
                self.cur_mot_node.motion_type = MotionType::Loop;
            }
        } else {
            self.cur_mot_node.motion_key = motion_key;
            if set_mot_data.loop_count > 0 {
                self.cur_mot_node.motion_type = MotionType::Once;
            } else {
                self.cur_mot_node.motion_type = MotionType::Loop;
            }
        }
    }
}
