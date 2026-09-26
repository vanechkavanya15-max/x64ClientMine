use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone)]
pub struct FlyEventData {
    pub is_attaching: bool,
    pub attaching_bone_name: String,
    pub fly_position: [f32; 3],
    pub fly_index: u32,
    pub fly_file_name: String,
}

impl FlyEventData {
    pub fn new() -> Self {
        Self {
            is_attaching: false,
            attaching_bone_name: String::new(),
            fly_position: [0.0, 0.0, 0.0],
            fly_index: 0,
            fly_file_name: String::new(),
        }
    }
}

pub struct CActorInstance {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub has_target: bool,
    bone_matrices: HashMap<String, [f32; 16]>,
}

impl CActorInstance {
    pub fn new(vid: u32) -> Self {
        Self {
            vid,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            has_target: false,
            bone_matrices: HashMap::new(),
        }
    }

    pub fn set_bone_matrix(&mut self, bone_name: &str, matrix: [f32; 16]) {
        self.bone_matrices.insert(bone_name.to_string(), matrix);
    }

    pub fn get_bone_matrix(&self, bone_name: &str) -> Option<[f32; 16]> {
        self.bone_matrices.get(bone_name).copied()
    }

    pub fn process_motion_event_fly(&self, data: &FlyEventData) -> Option<[f32; 3]> {
        if !self.has_target {
            return None;
        }

        let mut start_pos = [
            data.fly_position[0] + self.x,
            data.fly_position[1] + self.y,
            data.fly_position[2] + self.z,
        ];

        if data.is_attaching {
            if let Some(matrix) = self.get_bone_matrix(&data.attaching_bone_name) {
                start_pos[0] += matrix[12];
                start_pos[1] += matrix[13];
                start_pos[2] += matrix[14];
            }
        }

        Some(start_pos)
    }
}

static ACTOR_INSTANCES: OnceLock<RwLock<HashMap<u32, CActorInstance>>> = OnceLock::new();

fn get_actor_instances() -> &'static RwLock<HashMap<u32, CActorInstance>> {
    ACTOR_INSTANCES.get_or_init(|| RwLock::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn m2_motion_set_actor_position(vid: u32, x: f32, y: f32, z: f32) -> bool {
    let mut instances = get_actor_instances().write().unwrap();
    let actor = instances.entry(vid).or_insert_with(|| CActorInstance::new(vid));
    actor.x = x;
    actor.y = y;
    actor.z = z;
    true
}

#[no_mangle]
pub extern "C" fn m2_motion_set_actor_target(vid: u32, has_target: bool) -> bool {
    let mut instances = get_actor_instances().write().unwrap();
    let actor = instances.entry(vid).or_insert_with(|| CActorInstance::new(vid));
    actor.has_target = has_target;
    true
}

#[no_mangle]
pub extern "C" fn m2_motion_set_bone_matrix(vid: u32, bone_name: *const std::os::raw::c_char, matrix: *const f32) -> bool {
    if bone_name.is_null() || matrix.is_null() {
        return false;
    }

    let c_str = unsafe { std::ffi::CStr::from_ptr(bone_name) };
    let bone_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    let mut mat_array = [0.0; 16];
    unsafe {
        std::ptr::copy_nonoverlapping(matrix, mat_array.as_mut_ptr(), 16);
    }

    let mut instances = get_actor_instances().write().unwrap();
    let actor = instances.entry(vid).or_insert_with(|| CActorInstance::new(vid));
    actor.set_bone_matrix(bone_str, mat_array);

    true
}

#[no_mangle]
pub extern "C" fn m2_motion_get_fly_start_pos(
    vid: u32,
    is_attaching: bool,
    attaching_bone_name: *const std::os::raw::c_char,
    fly_pos_x: f32,
    fly_pos_y: f32,
    fly_pos_z: f32,
    out_pos: *mut f32
) -> bool {
    if out_pos.is_null() {
        return false;
    }

    let instances = get_actor_instances().read().unwrap();
    if let Some(actor) = instances.get(&vid) {
        let mut data = FlyEventData::new();
        data.fly_position = [fly_pos_x, fly_pos_y, fly_pos_z];
        data.is_attaching = is_attaching;

        if is_attaching && !attaching_bone_name.is_null() {
            let c_str = unsafe { std::ffi::CStr::from_ptr(attaching_bone_name) };
            if let Ok(s) = c_str.to_str() {
                data.attaching_bone_name = s.to_string();
            }
        }

        if let Some(pos) = actor.process_motion_event_fly(&data) {
            unsafe {
                *out_pos = pos[0];
                *out_pos.add(1) = pos[1];
                *out_pos.add(2) = pos[2];
            }
            return true;
        }
    }

    false
}
