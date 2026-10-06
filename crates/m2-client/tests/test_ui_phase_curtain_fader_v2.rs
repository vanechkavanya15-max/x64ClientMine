#[path = "../src/ui_phase_curtain_fader_v2.rs"]
mod ui_phase_curtain_fader_v2;

use std::sync::atomic::{AtomicBool, Ordering};
use ui_phase_curtain_fader_v2::{
    m2_ui_phase_curtain_clear, m2_ui_phase_curtain_get_alpha, m2_ui_phase_curtain_get_state,
    m2_ui_phase_curtain_set_fade_in, m2_ui_phase_curtain_set_fade_out, m2_ui_phase_curtain_update,
    PhaseCurtainFader, PhaseCurtainState,
};

static CALLBACK_CALLED_1: AtomicBool = AtomicBool::new(false);
static CALLBACK_CALLED_2: AtomicBool = AtomicBool::new(false);

extern "C" fn test_callback_1() {
    CALLBACK_CALLED_1.store(true, Ordering::SeqCst);
}

extern "C" fn test_callback_2() {
    CALLBACK_CALLED_2.store(true, Ordering::SeqCst);
}

#[test]
fn test_curtain_initial_state() {
    let fader = PhaseCurtainFader::new();
    assert_eq!(fader.get_state(), PhaseCurtainState::Idle);
    assert_eq!(fader.get_alpha(), 0.0);
}

#[test]
fn test_curtain_fade_out() {
    let mut fader = PhaseCurtainFader::new();
    fader.set_fade_out(1000, None);
    assert_eq!(fader.get_state(), PhaseCurtainState::FadeOut);
    assert_eq!(fader.get_alpha(), 0.0);

    fader.update(500.0);
    assert_eq!(fader.get_state(), PhaseCurtainState::FadeOut);
    assert_eq!(fader.get_alpha(), 0.5);

    fader.update(600.0); // 1100 total > 1000
    assert_eq!(fader.get_state(), PhaseCurtainState::Idle);
    assert_eq!(fader.get_alpha(), 1.0);
}

#[test]
fn test_curtain_fade_in() {
    let mut fader = PhaseCurtainFader::new();
    fader.set_fade_in(2000, None);
    assert_eq!(fader.get_state(), PhaseCurtainState::FadeIn);
    assert_eq!(fader.get_alpha(), 1.0);

    fader.update(1000.0);
    assert_eq!(fader.get_state(), PhaseCurtainState::FadeIn);
    assert_eq!(fader.get_alpha(), 0.5);

    fader.update(1000.0);
    assert_eq!(fader.get_state(), PhaseCurtainState::Idle);
    assert_eq!(fader.get_alpha(), 0.0);
}

#[test]
fn test_curtain_callback() {
    let mut fader = PhaseCurtainFader::new();
    CALLBACK_CALLED_1.store(false, Ordering::SeqCst);

    fader.set_fade_out(100, Some(test_callback_1));
    fader.update(50.0);
    assert_eq!(CALLBACK_CALLED_1.load(Ordering::SeqCst), false);

    fader.update(50.0); // complete
    assert_eq!(CALLBACK_CALLED_1.load(Ordering::SeqCst), true);
    assert_eq!(fader.get_state(), PhaseCurtainState::Idle);
}

// These tests use FFI functions interacting with the global state,
// so we use simple serial testing techniques or ensure they are isolated
#[test]
fn test_ffi_global_fader() {
    // Clear global state
    m2_ui_phase_curtain_clear();
    assert_eq!(m2_ui_phase_curtain_get_state(), PhaseCurtainState::Idle);
    assert_eq!(m2_ui_phase_curtain_get_alpha(), 0.0);

    // Fade Out
    CALLBACK_CALLED_2.store(false, Ordering::SeqCst);
    m2_ui_phase_curtain_set_fade_out(1000, Some(test_callback_2));
    assert_eq!(m2_ui_phase_curtain_get_state(), PhaseCurtainState::FadeOut);
    assert_eq!(m2_ui_phase_curtain_get_alpha(), 0.0);

    m2_ui_phase_curtain_update(500.0);
    assert_eq!(m2_ui_phase_curtain_get_alpha(), 0.5);
    assert_eq!(CALLBACK_CALLED_2.load(Ordering::SeqCst), false);

    m2_ui_phase_curtain_update(500.0);
    assert_eq!(m2_ui_phase_curtain_get_state(), PhaseCurtainState::Idle);
    assert_eq!(m2_ui_phase_curtain_get_alpha(), 1.0);
    assert_eq!(CALLBACK_CALLED_2.load(Ordering::SeqCst), true);

    // Fade In
    CALLBACK_CALLED_2.store(false, Ordering::SeqCst);
    m2_ui_phase_curtain_set_fade_in(500, Some(test_callback_2));
    assert_eq!(m2_ui_phase_curtain_get_state(), PhaseCurtainState::FadeIn);
    assert_eq!(m2_ui_phase_curtain_get_alpha(), 1.0);

    m2_ui_phase_curtain_update(250.0);
    assert_eq!(m2_ui_phase_curtain_get_alpha(), 0.5);

    m2_ui_phase_curtain_update(300.0);
    assert_eq!(m2_ui_phase_curtain_get_state(), PhaseCurtainState::Idle);
    assert_eq!(m2_ui_phase_curtain_get_alpha(), 0.0);
    assert_eq!(CALLBACK_CALLED_2.load(Ordering::SeqCst), true);

    m2_ui_phase_curtain_clear();
}
