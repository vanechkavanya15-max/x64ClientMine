use std::panic::catch_unwind;
use crate::terrain_ffi::get_terrain_ctx;
use crate::terrain_camera_collision::resolve_terrain_camera_collision;

#[no_mangle]
pub extern "C" fn m2_terrain_get_height_v2(x: f32, y: f32) -> f32 {
    crate::terrain_ffi::m2_terrain_get_height(x, y)
}

#[no_mangle]
pub extern "C" fn m2_terrain_is_attr(x: f32, y: f32, attr_flag: u8) -> bool {
    catch_unwind(|| {
        if let Ok(ctx) = get_terrain_ctx().read() {
            if let Some(map) = &ctx.attr_map {
                let attr_x = x.floor() as usize;
                let attr_y = y.floor() as usize;
                
                if attr_flag == crate::terrain_attr_map::ATTR_BLOCK || attr_flag == crate::terrain_attr_map::ATTR_WATER {
                    return !map.is_walkable(attr_x, attr_y);
                }
                if attr_flag == crate::terrain_attr_map::ATTR_BANPK || attr_flag == crate::terrain_attr_map::ATTR_SAFE_ZONE {
                    return map.is_safe_zone(attr_x, attr_y);
                }
            }
        }
        false
    }).unwrap_or(false)
}

#[repr(C)]
pub struct Tuple3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[no_mangle]
pub extern "C" fn m2_terrain_clamp_camera(
    eye_x: f32, eye_y: f32, eye_z: f32,
    target_x: f32, target_y: f32, target_z: f32
) -> Tuple3f32 {
    catch_unwind(|| {
        let (nx, ny, nz) = resolve_terrain_camera_collision(
            (target_x, target_y, target_z),
            (eye_x, eye_y, eye_z),
            crate::terrain_camera_collision::MIN_CAMERA_GROUND_CLEARANCE,
            |x, y| {
                if let Ok(ctx) = get_terrain_ctx().read() {
                    if let Some(map) = &ctx.height_map {
                        return map.get_height(x, y);
                    }
                }
                0.0
            }
        );
        Tuple3f32 { x: nx, y: ny, z: nz }
    }).unwrap_or(Tuple3f32 { x: eye_x, y: eye_y, z: eye_z })
}

#[no_mangle]
pub extern "C" fn m2_terrain_get_water_height(x: f32, y: f32) -> f32 {
    catch_unwind(|| {
        if let Ok(ctx) = get_terrain_ctx().read() {
            if let Some(water_map_data) = &ctx.water_map {
                if let Ok(water_map) = crate::terrain_water_map::WaterMap::from_bytes(water_map_data) {
                    let wx = (x.floor() as usize % 128) as u8;
                    let wy = (y.floor() as usize % 128) as u8;
                    if let Some(height) = water_map.get_water_height(wx, wy) {
                        return height;
                    }
                }
            }
        }
        0.0
    }).unwrap_or(0.0)
}
