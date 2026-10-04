pub const FISHING_DISTANCE: f32 = 600.0;
pub const ATTRIBUTE_WATER: u8 = 1 << 1;

/// Represents a 2D position
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Rotates a point around a center by a given degree, similar to ELPlainCoord_GetRotatedPixelPosition
pub fn get_rotated_position(center: Position, distance: f32, rot_degree: f32) -> Position {
    let rot_radian = rot_degree.to_radians();
    Position {
        x: center.x + distance * rot_radian.sin(),
        y: center.y + distance * rot_radian.cos(),
    }
}

/// Tries to find a valid fishing rotation based on current position and rotation.
/// Translates to CInstanceBase::GetFishingRot logic.
pub fn get_fishing_rotation(
    pos_x: f32,
    pos_y: f32,
    char_rot_degree: f32,
    check_water_fn: &impl Fn(f32, f32) -> bool,
) -> Option<f32> {
    let center = Position { x: pos_x, y: pos_y };

    let mut f_rot = 0.0;
    while f_rot <= 180.0 {
        let p_right = get_rotated_position(center, FISHING_DISTANCE, char_rot_degree + f_rot);
        if check_water_fn(p_right.x, p_right.y) {
            return Some(char_rot_degree + f_rot);
        }

        let p_left = get_rotated_position(center, FISHING_DISTANCE, char_rot_degree - f_rot);
        if check_water_fn(p_left.x, p_left.y) {
            return Some(char_rot_degree - f_rot);
        }

        f_rot += 10.0;
    }
    None
}

/// A line segment representation for raycast testing
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct LineSegment {
    pub start: Position,
    pub end: Position,
}

impl LineSegment {
    /// Helper to test if a raycast from start to end hits anything according to a block check function.
    /// Used for validating if fishing cast hits a wall/obstacle before reaching the water.
    pub fn raycast_hit(self, block_check_fn: &impl Fn(f32, f32) -> bool, step_size: f32) -> bool {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let distance = dx.hypot(dy);

        if distance <= 0.0 {
            return block_check_fn(self.start.x, self.start.y);
        }

        let steps = (distance / step_size).ceil() as u32;
        let step_x = dx / steps as f32;
        let step_y = dy / steps as f32;

        let mut cur_x = self.start.x;
        let mut cur_y = self.start.y;

        for _ in 0..=steps {
            if block_check_fn(cur_x, cur_y) {
                return true;
            }
            cur_x += step_x;
            cur_y += step_y;
        }

        false
    }
}

/// Replicating UI fishing minigame scoring mechanism.
/// (Hits required to catch fish)
#[derive(Debug, Clone, PartialEq)]
pub struct FishingMinigame {
    pub required_hits: u32,
    pub current_hits: u32,
    pub time_limit_ms: u32,
    pub elapsed_ms: u32,
    pub is_active: bool,
    pub is_success: bool,
}

impl FishingMinigame {
    pub fn new(required_hits: u32, time_limit_ms: u32) -> Self {
        Self {
            required_hits,
            current_hits: 0,
            time_limit_ms,
            elapsed_ms: 0,
            is_active: true,
            is_success: false,
        }
    }

    pub fn update(&mut self, delta_ms: u32) {
        if !self.is_active {
            return;
        }

        self.elapsed_ms += delta_ms;
        if self.elapsed_ms >= self.time_limit_ms {
            self.is_active = false;
            self.is_success = false; // timed out
        }
    }

    pub fn register_hit(&mut self) {
        if !self.is_active {
            return;
        }

        self.current_hits += 1;
        if self.current_hits >= self.required_hits {
            self.is_active = false;
            self.is_success = true;
        }
    }
}

// FFI Wrappers

pub type FfiCheckFn = extern "C" fn(f32, f32) -> bool;

#[no_mangle]
pub extern "C" fn m2_fishing_get_rotation_ffi(
    pos_x: f32,
    pos_y: f32,
    char_rot_degree: f32,
    check_water_cb: FfiCheckFn,
) -> f32 {
    let check_water_wrapper = |x: f32, y: f32| -> bool { check_water_cb(x, y) };

    if let Some(rot) = get_fishing_rotation(pos_x, pos_y, char_rot_degree, &check_water_wrapper) {
        rot
    } else {
        -1.0
    }
}

#[no_mangle]
pub extern "C" fn m2_fishing_raycast_hit_ffi(
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    step_size: f32,
    block_check_cb: FfiCheckFn,
) -> bool {
    let segment = LineSegment {
        start: Position {
            x: start_x,
            y: start_y,
        },
        end: Position { x: end_x, y: end_y },
    };

    let block_check_wrapper = |x: f32, y: f32| -> bool { block_check_cb(x, y) };

    segment.raycast_hit(&block_check_wrapper, step_size)
}

#[no_mangle]
pub extern "C" fn m2_fishing_minigame_new_ffi(
    required_hits: u32,
    time_limit_ms: u32,
) -> *mut FishingMinigame {
    let minigame = Box::new(FishingMinigame::new(required_hits, time_limit_ms));
    Box::into_raw(minigame)
}

/// # Safety
/// This function frees a `FishingMinigame` pointer allocated by `m2_fishing_minigame_new_ffi`.
#[no_mangle]
pub unsafe extern "C" fn m2_fishing_minigame_free_ffi(ptr: *mut FishingMinigame) {
    if !ptr.is_null() {
        let _ = Box::from_raw(ptr);
    }
}

/// # Safety
/// This function updates a `FishingMinigame` pointer allocated by `m2_fishing_minigame_new_ffi`.
#[no_mangle]
pub unsafe extern "C" fn m2_fishing_minigame_update_ffi(ptr: *mut FishingMinigame, delta_ms: u32) {
    if !ptr.is_null() {
        (*ptr).update(delta_ms);
    }
}

/// # Safety
/// This function registers a hit on a `FishingMinigame` pointer allocated by `m2_fishing_minigame_new_ffi`.
#[no_mangle]
pub unsafe extern "C" fn m2_fishing_minigame_register_hit_ffi(ptr: *mut FishingMinigame) {
    if !ptr.is_null() {
        (*ptr).register_hit();
    }
}

/// # Safety
/// This function checks if a `FishingMinigame` pointer allocated by `m2_fishing_minigame_new_ffi` is active.
#[no_mangle]
pub unsafe extern "C" fn m2_fishing_minigame_is_active_ffi(ptr: *const FishingMinigame) -> bool {
    if !ptr.is_null() {
        (*ptr).is_active
    } else {
        false
    }
}

/// # Safety
/// This function checks if a `FishingMinigame` pointer allocated by `m2_fishing_minigame_new_ffi` is successful.
#[no_mangle]
pub unsafe extern "C" fn m2_fishing_minigame_is_success_ffi(ptr: *const FishingMinigame) -> bool {
    if !ptr.is_null() {
        (*ptr).is_success
    } else {
        false
    }
}
