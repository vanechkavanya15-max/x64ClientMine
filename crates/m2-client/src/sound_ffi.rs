use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock, mpsc};
use std::thread;

use crate::py_bridge_pack::pack_get;
use crate::sound_wav_parser::WavAudio;
use rodio::{OutputStream, Sink, buffer::SamplesBuffer};

pub enum AudioCommand {
    Play2D(String, f32),
    Play3D(String, f32, f32, f32, f32),
}

pub struct AudioState {
    pub is_initialized: bool,
    pub master_volume: f32,
    pub category_volumes: [f32; 256],
    pub is_muted: bool,
    pub current_bgm: Option<String>,
    
    // Uzywane w testach do weryfikacji stanow 2D i 3D z odpowiednimi obliczeniami
    pub last_played_2d: Option<(String, f32)>,
    pub last_played_3d: Option<(String, f32, f32, f32, f32)>,
    
    pub sender: Option<mpsc::Sender<AudioCommand>>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            master_volume: 1.0,
            category_volumes: [1.0; 256],
            is_muted: false,
            current_bgm: None,
            last_played_2d: None,
            last_played_3d: None,
            sender: None,
        }
    }
}

static AUDIO_STATE: OnceLock<Mutex<AudioState>> = OnceLock::new();

pub fn get_audio_state() -> &'static Mutex<AudioState> {
    AUDIO_STATE.get_or_init(|| Mutex::new(AudioState::new()))
}

#[no_mangle]
pub extern "C" fn m2_audio_init() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_audio_state().lock().unwrap();
        if !state.is_initialized {
            state.is_initialized = true;
            
            let (tx, rx) = mpsc::channel::<AudioCommand>();
            state.sender = Some(tx);
            
            thread::spawn(move || {
                let (_stream, stream_handle) = match OutputStream::try_default() {
                    Ok(s) => s,
                    Err(_) => return, // Brak urzadzenia audio
                };
                
                while let Ok(cmd) = rx.recv() {
                    match cmd {
                        AudioCommand::Play2D(filename, vol) | AudioCommand::Play3D(filename, _, _, _, vol) => {
                            if let Some(data) = pack_get(&filename) {
                                if let Ok(wav) = WavAudio::from_bytes(&data) {
                                    if let Ok(sink) = Sink::try_new(&stream_handle) {
                                        sink.set_volume(vol);
                                        let source = SamplesBuffer::new(wav.channels, wav.sample_rate, wav.samples);
                                        sink.append(source);
                                        sink.detach();
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_audio_play_sound_2d(file_name_ptr: *const c_char, volume: f32) -> bool {
    safe_ffi_call(false, || {
        if file_name_ptr.is_null() {
            return false;
        }
        
        let mut state = get_audio_state().lock().unwrap();
        if !state.is_initialized || state.is_muted {
            return false;
        }
        
        let c_str = unsafe { CStr::from_ptr(file_name_ptr) };
        if let Ok(file_name) = c_str.to_str() {
            let actual_volume = (volume * state.master_volume).clamp(0.0, 1.0);
            state.last_played_2d = Some((file_name.to_string(), actual_volume));
            
            if let Some(sender) = &state.sender {
                let _ = sender.send(AudioCommand::Play2D(file_name.to_string(), actual_volume));
            }
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_audio_play_sound_3d(
    file_name_ptr: *const c_char,
    x: f32,
    y: f32,
    z: f32,
    volume: f32,
) -> bool {
    safe_ffi_call(false, || {
        if file_name_ptr.is_null() {
            return false;
        }

        let mut state = get_audio_state().lock().unwrap();
        if !state.is_initialized || state.is_muted {
            return false;
        }

        let c_str = unsafe { CStr::from_ptr(file_name_ptr) };
        if let Ok(file_name) = c_str.to_str() {
            let actual_volume = (volume * state.master_volume).clamp(0.0, 1.0);
            state.last_played_3d = Some((file_name.to_string(), x, y, z, actual_volume));
            
            if let Some(sender) = &state.sender {
                let _ = sender.send(AudioCommand::Play3D(file_name.to_string(), x, y, z, actual_volume));
            }
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_audio_play_bgm(file_name_ptr: *const c_char, fade_in: bool) -> bool {
    safe_ffi_call(false, || {
        if file_name_ptr.is_null() {
            return false;
        }

        let mut state = get_audio_state().lock().unwrap();
        if !state.is_initialized {
            return false;
        }

        let c_str = unsafe { CStr::from_ptr(file_name_ptr) };
        if let Ok(file_name) = c_str.to_str() {
            if file_name.is_empty() {
                return false;
            }
            state.current_bgm = Some(file_name.to_string());
            let _ = fade_in; 
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_audio_stop_bgm(fade_out: bool) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_audio_state().lock().unwrap();
        if !state.is_initialized {
            return false;
        }
        state.current_bgm = None;
        let _ = fade_out;
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_audio_set_master_volume(vol: f32) {
    safe_ffi_call((), || {
        let mut state = get_audio_state().lock().unwrap();
        state.master_volume = vol.clamp(0.0, 1.0);
    })
}

#[no_mangle]
pub extern "C" fn m2_audio_set_category_volume(category_id: u8, vol: f32) {
    safe_ffi_call((), || {
        let mut state = get_audio_state().lock().unwrap();
        state.category_volumes[category_id as usize] = vol.clamp(0.0, 1.0);
    })
}

#[no_mangle]
pub extern "C" fn m2_audio_mute_all(mute: bool) {
    safe_ffi_call((), || {
        let mut state = get_audio_state().lock().unwrap();
        state.is_muted = mute;
    })
}

#[no_mangle]
pub extern "C" fn m2_audio_stop_all_sound() -> bool {
    safe_ffi_call(false, || {
        let mut state = get_audio_state().lock().unwrap();
        if !state.is_initialized {
            return false;
        }
        state.current_bgm = None;
        state.last_played_2d = None;
        state.last_played_3d = None;
        true
    })
}



