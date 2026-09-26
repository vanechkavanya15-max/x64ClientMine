/// Phase B: Metin2 64-bit Game Engine Core
/// Render states for ActorInstance parts.

pub const PART_MAIN: u8 = 0;
pub const PART_WEAPON: u8 = 1;
pub const PART_HEAD: u8 = 2;
pub const PART_HAIR: u8 = 3;

pub const D3DCULL_NONE: u32 = 1;
pub const D3DCULL_CW: u32 = 2;
pub const D3DCULL_CCW: u32 = 3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ActorRenderStateResult {
    pub cull_mode: u32,
    pub alpha_ref: u32,
    pub specular_enable: bool,
}

#[no_mangle]
pub extern "C" fn m2_actor_get_render_state(part_type: u8) -> ActorRenderStateResult {
    let mut cull_mode = D3DCULL_CW;
    let alpha_ref = 128; // Default threshold for alpha testing

    // In Metin2, hair uses two-sided rendering (culling disabled)
    if part_type == PART_HAIR {
        cull_mode = D3DCULL_NONE;
    }

    // Specular sheen for armor is usually enabled by default if it's the main part
    let specular_enable = part_type == PART_MAIN;

    ActorRenderStateResult {
        cull_mode,
        alpha_ref,
        specular_enable,
    }
}

#[no_mangle]
pub extern "C" fn m2_actor_get_specular_power(armor_vnum: u32) -> f32 {
    let refine_level = armor_vnum % 10;
    match refine_level {
        7 => 0.5,
        8 => 0.7,
        9 => 1.0,
        _ => 0.0,
    }
}
