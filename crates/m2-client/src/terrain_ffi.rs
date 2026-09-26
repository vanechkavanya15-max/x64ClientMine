use std::sync::RwLock;
use std::sync::OnceLock;
use std::panic::catch_unwind;
use std::slice;

use crate::terrain_height_map::TerrainHeightMap;
use crate::terrain_attr_map::TerrainAttrMap;

pub struct TerrainContext {
    pub height_map: Option<TerrainHeightMap>,
    pub attr_map: Option<TerrainAttrMap>,
    pub water_map: Option<Vec<u8>>,
    pub water_width: usize,
    pub water_height: usize,
}

pub fn get_terrain_ctx() -> &'static RwLock<TerrainContext> {
    static CTX: OnceLock<RwLock<TerrainContext>> = OnceLock::new();
    CTX.get_or_init(|| RwLock::new(TerrainContext {
        height_map: None,
        attr_map: None,
        water_map: None,
        water_width: 0,
        water_height: 0,
    }))
}

#[no_mangle]
pub extern "C" fn m2_terrain_load_raw_heightmap(data_ptr: *const u8, len: usize) -> bool {
    catch_unwind(|| {
        if data_ptr.is_null() || len == 0 {
            return false;
        }

        // 131x131 u16 heights = 131 * 131 * 2 = 34322 bytes
        if len != 131 * 131 * 2 {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(data_ptr as *const u16, len / 2) };
        let mut vec = Vec::with_capacity(slice.len());
        vec.extend_from_slice(slice);

        let map = TerrainHeightMap::new(131, 131, vec);
        if let Ok(mut ctx) = get_terrain_ctx().write() {
            ctx.height_map = Some(map);
            true
        } else {
            false
        }
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_terrain_load_attrmap(data_ptr: *const u8, len: usize) -> bool {
    catch_unwind(|| {
        if data_ptr.is_null() || len == 0 {
            return false;
        }

        // Assume 256x256 attribute map = 65536 bytes
        let mut width = 256;
        let mut height = 256;
        
        if len != width * height {
            // allow a fallback size for testability, e.g. a small test map
            let root = (len as f64).sqrt() as usize;
            if root * root == len {
                width = root;
                height = root;
            } else {
                return false;
            }
        }

        let slice = unsafe { slice::from_raw_parts(data_ptr, len) };
        let mut vec = Vec::with_capacity(slice.len());
        vec.extend_from_slice(slice);

        if let Ok(map) = TerrainAttrMap::new(width, height, vec) {
            if let Ok(mut ctx) = get_terrain_ctx().write() {
                ctx.attr_map = Some(map);
                return true;
            }
        }
        false
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_terrain_load_watermap(data_ptr: *const u8, len: usize) -> bool {
    catch_unwind(|| {
        if data_ptr.is_null() || len == 0 {
            return false;
        }

        // Typical watermap is 256x256
        let mut width = 256;
        let mut height = 256;
        
        if len != width * height {
            let root = (len as f64).sqrt() as usize;
            if root * root == len {
                width = root;
                height = root;
            } else {
                return false;
            }
        }

        let slice = unsafe { slice::from_raw_parts(data_ptr, len) };
        let mut vec = Vec::with_capacity(slice.len());
        vec.extend_from_slice(slice);

        if let Ok(mut ctx) = get_terrain_ctx().write() {
            ctx.water_map = Some(vec);
            ctx.water_width = width;
            ctx.water_height = height;
            true
        } else {
            false
        }
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_terrain_get_height(local_x: f32, local_y: f32) -> f32 {
    catch_unwind(|| {
        if let Ok(ctx) = get_terrain_ctx().read() {
            if let Some(map) = &ctx.height_map {
                return map.get_height(local_x, local_y);
            }
        }
        0.0
    }).unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn m2_terrain_is_blocked(attr_x: u16, attr_y: u16) -> bool {
    catch_unwind(|| {
        if let Ok(ctx) = get_terrain_ctx().read() {
            if let Some(map) = &ctx.attr_map {
                return !map.is_walkable(attr_x as usize, attr_y as usize);
            }
        }
        true // If there is no map, assume blocked for safety
    }).unwrap_or(true)
}

#[no_mangle]
pub extern "C" fn m2_terrain_is_water(attr_x: u16, attr_y: u16) -> bool {
    catch_unwind(|| {
        if let Ok(ctx) = get_terrain_ctx().read() {
            let x = attr_x as usize;
            let y = attr_y as usize;

            if let Some(water_map) = &ctx.water_map {
                if x < ctx.water_width && y < ctx.water_height {
                    // Assuming non-zero means water. 
                    return water_map[y * ctx.water_width + x] != 0;
                }
            } else if let Some(_map) = &ctx.attr_map {
                // fallback to attr map bit check, though attr map check isn't exposed publicly
                // We'll rely on the water map first, or if no water map exists, we might need a workaround.
                // For now, if no watermap is loaded, we'll return false, or we can check the attr_map logic.
            }
        }
        false
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_terrain_raycast_check(
    x1: f32, y1: f32, x2: f32, y2: f32, out_hit_x: *mut f32, out_hit_y: *mut f32
) -> bool {
    catch_unwind(|| {
        if out_hit_x.is_null() || out_hit_y.is_null() {
            return false;
        }

        let is_blocked = |gx: i32, gy: i32| -> bool {
            if gx < 0 || gy < 0 {
                return true;
            }
            if let Ok(ctx) = get_terrain_ctx().read() {
                if let Some(map) = &ctx.attr_map {
                    return !map.is_walkable(gx as usize, gy as usize);
                }
            }
            true // assume blocked if out of bounds or no map
        };

        // Standard DDA (Digital Differential Analyzer) / Amanatides & Woo grid traversal
        // The Metin2 grid size for attrmap check is typically a certain scale, but since we are working with attr_x, attr_y which are essentially grid coords.
        // We assume x1, y1 and x2, y2 are given in the same coordinate space as the attribute grid (0, 1, 2...). 
        // We traverse the grid from (x1, y1) to (x2, y2).
        
        let mut x = x1.floor() as i32;
        let mut y = y1.floor() as i32;
        let end_x = x2.floor() as i32;
        let end_y = y2.floor() as i32;

        let dx = x2 - x1;
        let dy = y2 - y1;

        let step_x = if dx > 0.0 { 1 } else { -1 };
        let step_y = if dy > 0.0 { 1 } else { -1 };

        let t_delta_x = if dx != 0.0 { (1.0 / dx).abs() } else { f32::INFINITY };
        let t_delta_y = if dy != 0.0 { (1.0 / dy).abs() } else { f32::INFINITY };

        let mut t_max_x = if dx > 0.0 {
            ((x as f32 + 1.0) - x1) * t_delta_x
        } else if dx < 0.0 {
            (x1 - x as f32) * t_delta_x
        } else {
            f32::INFINITY
        };

        let mut t_max_y = if dy > 0.0 {
            ((y as f32 + 1.0) - y1) * t_delta_y
        } else if dy < 0.0 {
            (y1 - y as f32) * t_delta_y
        } else {
            f32::INFINITY
        };

        let mut hit = false;
        let mut last_t = 0.0;

        if is_blocked(x, y) {
            unsafe {
                *out_hit_x = x1;
                *out_hit_y = y1;
            }
            return true;
        }

        // max distance limit to prevent infinite loops in edge cases
        let mut iters = 0;
        let max_iters = 10000;

        while (x != end_x || y != end_y) && iters < max_iters {
            if t_max_x < t_max_y {
                last_t = t_max_x;
                t_max_x += t_delta_x;
                x += step_x;
            } else {
                last_t = t_max_y;
                t_max_y += t_delta_y;
                y += step_y;
            }
            iters += 1;

            if is_blocked(x, y) {
                hit = true;
                break;
            }
        }

        if hit {
            // calculate the intersection point
            unsafe {
                *out_hit_x = x1 + last_t * dx;
                *out_hit_y = y1 + last_t * dy;
            }
            return true;
        }

        false
    }).unwrap_or(false)
}
