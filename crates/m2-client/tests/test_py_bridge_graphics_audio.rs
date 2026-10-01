use m2_client::py_bridge_graphics_audio::*;
use m2_client::sound_ffi::{get_audio_state, m2_audio_init};
use std::ffi::CString;

#[test]
fn test_text_tail_registration() {
    let name = CString::new("Jules").unwrap();
    let title = CString::new("Hero").unwrap();
    
    assert!(m2_text_tail_register_character(101, name.as_ptr(), 1.0, 1.0, 1.0));
    assert!(m2_text_tail_show_character(101));
    assert!(m2_text_tail_attach_title(101, title.as_ptr(), 1.0, 0.0, 0.0));
    assert!(m2_text_tail_hide_character(101));
    
    // Test null ptrs
    assert!(!m2_text_tail_register_character(102, std::ptr::null(), 1.0, 1.0, 1.0));
    assert!(!m2_text_tail_attach_title(102, std::ptr::null(), 1.0, 0.0, 0.0));
}

#[test]
fn test_grp_rendering_functions() {
    m2_grp_set_color(1.0, 0.5, 0.25, 1.0);
    
    {
        let state = get_grp_state();
        assert_eq!(state.color_r, 1.0);
        assert_eq!(state.color_g, 0.5);
        assert_eq!(state.color_b, 0.25);
        assert_eq!(state.color_a, 1.0);
    }
    
    // Clear counts
    {
        let mut state = get_grp_state();
        state.batch.vertices.clear();
        state.batch.indices.clear();
    }
    
    assert!(m2_grp_render_box(10.0, 10.0, 100.0, 50.0));
    assert!(!m2_grp_render_box(10.0, 10.0, -10.0, 50.0)); // invalid width
    assert!(m2_grp_render_line(0.0, 0.0, 100.0, 100.0));
    
    {
        let state = get_grp_state();
        // A box is a quad (4 vertices, 6 indices), a line is simulated as a quad too (4 vertices, 6 indices).
        assert_eq!(state.batch.vertices.len(), 8);
        assert_eq!(state.batch.indices.len(), 12);
    }
}

#[test]
fn test_snd_audio_bridge() {
    // Init audio subsystem
    m2_audio_init();
    
    let snd_file = CString::new("test.wav").unwrap();
    let bgm_file = CString::new("music.mp3").unwrap();
    
    assert!(m2_snd_play_sound(snd_file.as_ptr()));
    
    {
        let state = get_audio_state().lock().unwrap();
        assert_eq!(state.last_played_2d.as_ref().unwrap().0, "test.wav");
    }
    
    assert!(m2_snd_play_bgm(bgm_file.as_ptr()));
    
    {
        let state = get_audio_state().lock().unwrap();
        assert_eq!(state.current_bgm.as_ref().unwrap(), "music.mp3");
    }
    
    m2_snd_set_sound_volume(0.5);
    m2_snd_set_bgm_volume(0.8);
    
    {
        let state = get_audio_state().lock().unwrap();
        assert_eq!(state.category_volumes[0], 0.5);
        assert_eq!(state.category_volumes[1], 0.8);
    }
    
    assert!(m2_snd_stop_bgm());
    
    {
        let state = get_audio_state().lock().unwrap();
        assert!(state.current_bgm.is_none());
    }
}

#[test]
fn test_system_settings_resolution() {
    let mut w = 0;
    let mut h = 0;
    
    assert!(m2_sys_set_resolution(1024, 768));
    assert!(m2_sys_get_resolution(&mut w, &mut h));
    
    assert_eq!(w, 1024);
    assert_eq!(h, 768);
    
    assert!(!m2_sys_set_resolution(0, 0));
}

#[test]
fn test_system_settings_bgm_volume() {
    m2_sys_set_bgm_volume(0.35);
    let vol = m2_sys_get_bgm_volume();
    // Compare floating points carefully, but direct equality is fine for explicit values
    assert_eq!(vol, 0.35);
}

#[test]
fn test_dbg_logging() {
    let msg1 = CString::new("test trace").unwrap();
    let msg2 = CString::new("test error").unwrap();
    let msg3 = CString::new("test box").unwrap();
    
    // Clear state
    {
        let mut state = get_dbg_state();
        state.traces.clear();
        state.errors.clear();
        state.log_boxes.clear();
    }
    
    assert!(m2_dbg_trace(msg1.as_ptr()));
    assert!(m2_dbg_trace_error(msg2.as_ptr()));
    assert!(m2_dbg_log_box(msg3.as_ptr()));
    
    {
        let state = get_dbg_state();
        assert_eq!(state.traces.len(), 1);
        assert_eq!(state.traces[0], "test trace");
        
        assert_eq!(state.errors.len(), 1);
        assert_eq!(state.errors[0], "test error");
        
        assert_eq!(state.log_boxes.len(), 1);
        assert_eq!(state.log_boxes[0], "test box");
    }
}
