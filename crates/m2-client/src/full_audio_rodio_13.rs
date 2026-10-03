use crate::audio_engine_shutdown::AudioShutdownManager;
use std::sync::{Mutex, OnceLock};

pub static GLOBAL_AUDIO_MANAGER: OnceLock<Mutex<AudioShutdownManager>> = OnceLock::new();

pub fn get_audio_manager() -> &'static Mutex<AudioShutdownManager> {
    GLOBAL_AUDIO_MANAGER.get_or_init(|| Mutex::new(AudioShutdownManager::new()))
}

#[no_mangle]
pub extern "C" fn m2_audio_engine_start() -> bool {
    let mut manager = get_audio_manager().lock().unwrap();
    manager.start().is_ok()
}

#[no_mangle]
pub extern "C" fn m2_audio_engine_shutdown() -> bool {
    let mut manager = get_audio_manager().lock().unwrap();
    manager.shutdown().is_ok()
}
