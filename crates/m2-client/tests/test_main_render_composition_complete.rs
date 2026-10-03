use std::sync::atomic::{AtomicU32, Ordering};

#[path = "../src/main_render_composition_complete.rs"]
mod main_render_composition_complete;

use main_render_composition_complete::{
    m2_render_composition_execute_frame, m2_render_composition_get_metrics,
    m2_render_composition_register_callbacks, reset_state_for_test, RenderCompositionCallbacks,
    RenderCompositionMetrics, RenderPass,
};

static EXECUTION_ORDER: AtomicU32 = AtomicU32::new(0);
static SHADOW_MAP_CALLED: AtomicU32 = AtomicU32::new(0);
static SKYBOX_CALLED: AtomicU32 = AtomicU32::new(0);
static TERRAIN_SPLATTING_CALLED: AtomicU32 = AtomicU32::new(0);
static WATER_CALLED: AtomicU32 = AtomicU32::new(0);
static CHARACTERS_CALLED: AtomicU32 = AtomicU32::new(0);
static MAGIC_EFFECTS_CALLED: AtomicU32 = AtomicU32::new(0);
static UI_2D_CALLED: AtomicU32 = AtomicU32::new(0);

extern "C" fn pass_shadow_map() {
    SHADOW_MAP_CALLED.store(EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
}
extern "C" fn pass_sky_box() {
    SKYBOX_CALLED.store(EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
}
extern "C" fn pass_terrain_splatting() {
    TERRAIN_SPLATTING_CALLED.store(EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
}
extern "C" fn pass_water() {
    WATER_CALLED.store(EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
}
extern "C" fn pass_characters() {
    CHARACTERS_CALLED.store(EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
}
extern "C" fn pass_magic_effects() {
    MAGIC_EFFECTS_CALLED.store(EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
}
extern "C" fn pass_ui_2d() {
    UI_2D_CALLED.store(EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
}

fn reset_counters() {
    EXECUTION_ORDER.store(0, Ordering::SeqCst);
    SHADOW_MAP_CALLED.store(0, Ordering::SeqCst);
    SKYBOX_CALLED.store(0, Ordering::SeqCst);
    TERRAIN_SPLATTING_CALLED.store(0, Ordering::SeqCst);
    WATER_CALLED.store(0, Ordering::SeqCst);
    CHARACTERS_CALLED.store(0, Ordering::SeqCst);
    MAGIC_EFFECTS_CALLED.store(0, Ordering::SeqCst);
    UI_2D_CALLED.store(0, Ordering::SeqCst);
}

#[test]
fn test_initial_state() {
    reset_state_for_test();
    let mut metrics = RenderCompositionMetrics::default();
    let success = m2_render_composition_get_metrics(&mut metrics as *mut _);
    assert!(success);
    assert_eq!(metrics.frames_rendered, 0);
    assert_eq!(metrics.last_pass_completed, u32::MAX);
    assert!(!metrics.is_rendering);
}

#[test]
fn test_null_pointers() {
    reset_state_for_test();
    assert!(!m2_render_composition_register_callbacks(std::ptr::null()));
    assert!(!m2_render_composition_get_metrics(std::ptr::null_mut()));
}

#[test]
fn test_execute_frame_no_callbacks() {
    reset_state_for_test();
    // Execute frame with no callbacks registered
    assert!(m2_render_composition_execute_frame());
    
    let mut metrics = RenderCompositionMetrics::default();
    assert!(m2_render_composition_get_metrics(&mut metrics as *mut _));
    assert_eq!(metrics.frames_rendered, 1);
    assert_eq!(metrics.last_pass_completed, RenderPass::Ui2D as u32);
    assert!(!metrics.is_rendering);
}

#[test]
fn test_execute_frame_with_callbacks() {
    reset_state_for_test();
    reset_counters();

    let callbacks = RenderCompositionCallbacks {
        pass_shadow_map: Some(pass_shadow_map),
        pass_sky_box: Some(pass_sky_box),
        pass_terrain_splatting: Some(pass_terrain_splatting),
        pass_water: Some(pass_water),
        pass_characters: Some(pass_characters),
        pass_magic_effects: Some(pass_magic_effects),
        pass_ui_2d: Some(pass_ui_2d),
    };

    assert!(m2_render_composition_register_callbacks(&callbacks as *const _));
    assert!(m2_render_composition_execute_frame());

    let mut metrics = RenderCompositionMetrics::default();
    assert!(m2_render_composition_get_metrics(&mut metrics as *mut _));
    assert_eq!(metrics.frames_rendered, 1);
    assert_eq!(metrics.last_pass_completed, RenderPass::Ui2D as u32);
    assert!(!metrics.is_rendering);

    // Verify all callbacks were called exactly once
    assert_eq!(SHADOW_MAP_CALLED.load(Ordering::SeqCst), 1);
    assert_eq!(SKYBOX_CALLED.load(Ordering::SeqCst), 2);
    assert_eq!(TERRAIN_SPLATTING_CALLED.load(Ordering::SeqCst), 3);
    assert_eq!(WATER_CALLED.load(Ordering::SeqCst), 4);
    assert_eq!(CHARACTERS_CALLED.load(Ordering::SeqCst), 5);
    assert_eq!(MAGIC_EFFECTS_CALLED.load(Ordering::SeqCst), 6);
    assert_eq!(UI_2D_CALLED.load(Ordering::SeqCst), 7);
}

#[test]
fn test_callback_execution_order() {
    reset_state_for_test();
    reset_counters();

    let callbacks = RenderCompositionCallbacks {
        pass_shadow_map: Some(pass_shadow_map),
        pass_sky_box: Some(pass_sky_box),
        pass_terrain_splatting: Some(pass_terrain_splatting),
        pass_water: Some(pass_water),
        pass_characters: Some(pass_characters),
        pass_magic_effects: Some(pass_magic_effects),
        pass_ui_2d: Some(pass_ui_2d),
    };

    m2_render_composition_register_callbacks(&callbacks as *const _);
    m2_render_composition_execute_frame();

    // Verification of execution order happens inside test_execute_frame_with_callbacks as well,
    // but we can assert the final value of EXECUTION_ORDER to be 7.
    assert_eq!(EXECUTION_ORDER.load(Ordering::SeqCst), 7);
}

#[test]
fn test_multiple_frames() {
    reset_state_for_test();
    reset_counters();

    let callbacks = RenderCompositionCallbacks {
        pass_shadow_map: Some(pass_shadow_map),
        pass_sky_box: Some(pass_sky_box),
        pass_terrain_splatting: Some(pass_terrain_splatting),
        pass_water: Some(pass_water),
        pass_characters: Some(pass_characters),
        pass_magic_effects: Some(pass_magic_effects),
        pass_ui_2d: Some(pass_ui_2d),
    };

    m2_render_composition_register_callbacks(&callbacks as *const _);

    // Frame 1
    m2_render_composition_execute_frame();
    // Frame 2
    m2_render_composition_execute_frame();
    // Frame 3
    m2_render_composition_execute_frame();

    let mut metrics = RenderCompositionMetrics::default();
    m2_render_composition_get_metrics(&mut metrics as *mut _);
    
    assert_eq!(metrics.frames_rendered, 3);
    assert_eq!(EXECUTION_ORDER.load(Ordering::SeqCst), 21); // 7 callbacks per frame * 3 frames
}
