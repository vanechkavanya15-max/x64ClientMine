use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::terrain_attr_collision_v2::{m2_terrain_get_attr, FLAG_SAFE_ZONE};

pub fn is_in_safe_zone(x: f32, y: f32) -> bool {
    let attr = m2_terrain_get_attr(x, y);
    (attr & FLAG_SAFE_ZONE) != 0
}

#[no_mangle]
pub extern "C" fn m2_map_is_safe_zone(x: f32, y: f32) -> bool {
    safe_ffi_call(false, || {
        is_in_safe_zone(x, y)
    })
}
