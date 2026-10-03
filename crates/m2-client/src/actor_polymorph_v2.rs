use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::character_manager_core::get_character_manager;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolymorphState {
    pub mob_vnum: u32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub equipment_hidden: bool,
}

#[no_mangle]
pub extern "C" fn m2_poly_apply(vid: u32, mob_vnum: u32, scale_x: f32, scale_y: f32, scale_z: f32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = get_character_manager().write().unwrap();
        if let Some(inst) = mgr.get_instance_mut(vid) {
            if mob_vnum > 0 {
                inst.polymorph = Some(PolymorphState {
                    mob_vnum, scale_x, scale_y, scale_z, equipment_hidden: true,
                });
            } else {
                inst.polymorph = None;
            }
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_poly_is_active(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = get_character_manager().read().unwrap();
        mgr.get_instance(vid).map_or(false, |inst| inst.polymorph.is_some())
    })
}

#[no_mangle]
pub extern "C" fn m2_poly_get_vnum(vid: u32) -> u32 {
    safe_ffi_call(0, || {
        let mgr = get_character_manager().read().unwrap();
        if let Some(inst) = mgr.get_instance(vid) {
            if let Some(poly) = inst.polymorph.as_ref() {
                return poly.mob_vnum;
            }
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_poly_get_scale(vid: u32, out_x: *mut f32, out_y: *mut f32, out_z: *mut f32) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() { return false; }
        let mgr = get_character_manager().read().unwrap();
        if let Some(inst) = mgr.get_instance(vid) {
            if let Some(poly) = inst.polymorph.as_ref() {
                unsafe {
                    *out_x = poly.scale_x;
                    *out_y = poly.scale_y;
                    *out_z = poly.scale_z;
                }
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_poly_is_equipment_hidden(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mgr = get_character_manager().read().unwrap();
        if let Some(inst) = mgr.get_instance(vid) {
            if let Some(poly) = inst.polymorph.as_ref() {
                return poly.equipment_hidden;
            }
        }
        false
    })
}
