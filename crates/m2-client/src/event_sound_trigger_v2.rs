use std::sync::{Mutex, OnceLock};
use std::collections::VecDeque;
use std::ptr;
use std::ffi::CString;

#[derive(Debug, Clone, PartialEq)]
pub struct TMotionSoundEventData {
    pub frame: u32,
    pub starting_time: f32,
    pub duration_time: f32,
    pub sound_file_name: String,
}

impl TMotionSoundEventData {
    pub fn new(frame: u32, starting_time: f32, duration_time: f32, sound_file_name: String) -> Self {
        Self {
            frame,
            starting_time,
            duration_time,
            sound_file_name,
        }
    }
}

static SOUND_EVENT_QUEUE: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();

fn get_sound_queue() -> &'static Mutex<VecDeque<String>> {
    SOUND_EVENT_QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

/// Evaluates a motion sound event and queues the sound if the current frame matches the event's frame.
pub fn process_motion_sound_event(current_frame: u32, event: &TMotionSoundEventData) {
    if current_frame == event.frame {
        if let Ok(mut queue) = get_sound_queue().lock() {
            queue.push_back(event.sound_file_name.clone());
        }
    }
}

/// Clears the global sound queue.
pub fn clear_motion_sound_events() {
    if let Ok(mut queue) = get_sound_queue().lock() {
        queue.clear();
    }
}

/// FFI exported function to pop a sound event from the queue.
/// Returns true if a sound was popped, and copies its name (up to 255 chars) into `out_sound_name`.
#[no_mangle]
pub extern "C" fn m2_motion_pop_sound_event(out_sound_name: *mut u8) -> bool {
    if out_sound_name.is_null() {
        return false;
    }

    let mut popped_sound = None;
    if let Ok(mut queue) = get_sound_queue().lock() {
        if !queue.is_empty() {
            popped_sound = queue.pop_front();
        }
    }

    if let Some(sound_name) = popped_sound {
        if let Ok(c_str) = CString::new(sound_name) {
            let bytes = c_str.as_bytes_with_nul();
            // We assume out_sound_name is large enough (e.g., 256 bytes buffer).
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), out_sound_name, bytes.len());
            }
            return true;
        }
    }

    false
}
