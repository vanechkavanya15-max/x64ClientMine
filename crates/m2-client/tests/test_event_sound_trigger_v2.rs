


use m2_client::event_sound_trigger_v2::*;
use std::ffi::CStr;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_struct_initialization() {
    let event = TMotionSoundEventData::new(5, 0.5, 1.0, "sound/attack.wav".to_string());
    assert_eq!(event.frame, 5);
    assert_eq!(event.starting_time, 0.5);
    assert_eq!(event.duration_time, 1.0);
    assert_eq!(event.sound_file_name, "sound/attack.wav");
}

#[test]
fn test_process_motion_sound_event_match() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_motion_sound_events();
    
    let event = TMotionSoundEventData::new(10, 0.0, 0.0, "sound/step.wav".to_string());
    process_motion_sound_event(10, &event);
    
    let mut buffer = [0u8; 256];
    let popped = m2_motion_pop_sound_event(buffer.as_mut_ptr());
    assert!(popped);
    
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    assert_eq!(c_str.to_str().unwrap(), "sound/step.wav");
    
    // Should be empty now
    let popped_again = m2_motion_pop_sound_event(buffer.as_mut_ptr());
    assert!(!popped_again);
}

#[test]
fn test_process_motion_sound_event_no_match() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_motion_sound_events();
    
    let event = TMotionSoundEventData::new(15, 0.0, 0.0, "sound/jump.wav".to_string());
    // current_frame != event.frame
    process_motion_sound_event(14, &event);
    
    let mut buffer = [0u8; 256];
    let popped = m2_motion_pop_sound_event(buffer.as_mut_ptr());
    assert!(!popped);
}

#[test]
fn test_multiple_events_queueing() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_motion_sound_events();
    
    let event1 = TMotionSoundEventData::new(20, 0.0, 0.0, "sound/hit1.wav".to_string());
    let event2 = TMotionSoundEventData::new(20, 0.0, 0.0, "sound/hit2.wav".to_string());
    let event3 = TMotionSoundEventData::new(21, 0.0, 0.0, "sound/hit3.wav".to_string());
    
    process_motion_sound_event(20, &event1);
    process_motion_sound_event(20, &event2);
    process_motion_sound_event(20, &event3); // Should not queue
    
    let mut buffer = [0u8; 256];
    
    let popped1 = m2_motion_pop_sound_event(buffer.as_mut_ptr());
    assert!(popped1);
    let c_str1 = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    assert_eq!(c_str1.to_str().unwrap(), "sound/hit1.wav");
    
    let popped2 = m2_motion_pop_sound_event(buffer.as_mut_ptr());
    assert!(popped2);
    let c_str2 = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    assert_eq!(c_str2.to_str().unwrap(), "sound/hit2.wav");
    
    let popped3 = m2_motion_pop_sound_event(buffer.as_mut_ptr());
    assert!(!popped3);
}

#[test]
fn test_pop_sound_event_null_pointer() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_motion_sound_events();
    let event = TMotionSoundEventData::new(0, 0.0, 0.0, "sound/test.wav".to_string());
    process_motion_sound_event(0, &event);
    
    let popped = m2_motion_pop_sound_event(std::ptr::null_mut());
    assert!(!popped);
    clear_motion_sound_events();
}
