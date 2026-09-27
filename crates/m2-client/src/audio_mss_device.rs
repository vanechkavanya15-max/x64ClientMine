use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[cfg(windows)]
use std::ffi::CString;
#[cfg(windows)]
use std::os::raw::{c_char, c_int, c_void};

// Manual Win32 FFI bindings to avoid external dependencies like winapi
#[cfg(windows)]
extern "system" {
    fn LoadLibraryA(lpFileName: *const c_char) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
}

// Function pointer types for MSS32.dll
#[cfg(windows)]
type AilInitSampleType = unsafe extern "C" fn() -> usize;
#[cfg(windows)]
type AilSetSampleVolumeType = unsafe extern "C" fn(sample: usize, volume: c_int);
#[cfg(windows)]
type AilStartSampleType = unsafe extern "C" fn(sample: usize);
#[cfg(windows)]
type AilEndSampleType = unsafe extern "C" fn(sample: usize);
#[cfg(windows)]
type AilReleaseSampleType = unsafe extern "C" fn(sample: usize);

#[derive(Clone)]
pub struct MssAudioDevice {
    pub sample_handle: usize, // Base handle for the library (HMODULE)
    pub is_initialized: bool,
    
    // Function pointers loaded from the DLL (stored as usize to allow Send/Sync and avoid complex lifetime issues, though they are unsafe to call)
    #[cfg(windows)]
    ail_init_sample_ptr: usize,
    #[cfg(windows)]
    ail_set_sample_volume_ptr: usize,
    #[cfg(windows)]
    ail_start_sample_ptr: usize,
    #[cfg(windows)]
    ail_end_sample_ptr: usize,
    #[cfg(windows)]
    ail_release_sample_ptr: usize,
}

// Implement Debug manually because function pointers (usize here) are opaque
impl std::fmt::Debug for MssAudioDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MssAudioDevice")
            .field("sample_handle", &self.sample_handle)
            .field("is_initialized", &self.is_initialized)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct SoundEffectResource {
    pub vnum: u32,
    pub file_path: String,
    pub sound_id: u32,
}

struct AudioState {
    device: MssAudioDevice,
    resources: HashMap<u32, SoundEffectResource>,
    
    // In a real scenario this would map sound_id to an MSS sample handle (usize).
    // We also track volume for test verifiability.
    playing_sounds: HashMap<u32, (usize, f32)>, 
}

impl AudioState {
    fn new() -> Self {
        Self {
            device: MssAudioDevice {
                sample_handle: 0,
                is_initialized: false,
                #[cfg(windows)] ail_init_sample_ptr: 0,
                #[cfg(windows)] ail_set_sample_volume_ptr: 0,
                #[cfg(windows)] ail_start_sample_ptr: 0,
                #[cfg(windows)] ail_end_sample_ptr: 0,
                #[cfg(windows)] ail_release_sample_ptr: 0,
            },
            resources: HashMap::new(),
            playing_sounds: HashMap::new(),
        }
    }
}

static AUDIO_STATE: OnceLock<Mutex<AudioState>> = OnceLock::new();

fn get_audio_state() -> &'static Mutex<AudioState> {
    AUDIO_STATE.get_or_init(|| Mutex::new(AudioState::new()))
}

#[cfg(windows)]
pub fn init_mss_sound_device() -> Result<MssAudioDevice, String> {
    let mut state = get_audio_state().lock().map_err(|_| "Failed to lock audio state".to_string())?;
    
    let dll_name = CString::new("mss32.dll").unwrap();
    let handle = unsafe { LoadLibraryA(dll_name.as_ptr()) };

    if handle.is_null() {
        state.device.is_initialized = false;
        state.device.sample_handle = 0;
        return Err("Failed to load mss32.dll or sound card not present".to_string());
    }

    state.device.sample_handle = handle as usize;

    // Helper macro to load function pointers
    macro_rules! load_func {
        ($name:expr) => {
            unsafe {
                let func_name = CString::new($name).unwrap();
                let ptr = GetProcAddress(handle, func_name.as_ptr());
                ptr as usize
            }
        };
    }

    state.device.ail_init_sample_ptr = load_func!("AIL_init_sample");
    state.device.ail_set_sample_volume_ptr = load_func!("AIL_set_sample_volume");
    state.device.ail_start_sample_ptr = load_func!("AIL_start_sample");
    state.device.ail_end_sample_ptr = load_func!("AIL_end_sample");
    state.device.ail_release_sample_ptr = load_func!("AIL_release_sample");

    // Consider initialized even if some functions are missing (partial support or fallback versions)
    // But realistically, if AIL_init_sample is 0, it's a bad DLL.
    if state.device.ail_init_sample_ptr == 0 {
        state.device.is_initialized = false;
        return Err("mss32.dll loaded, but missing critical functions (e.g. AIL_init_sample)".to_string());
    }

    state.device.is_initialized = true;
    Ok(state.device.clone())
}

#[cfg(not(windows))]
pub fn init_mss_sound_device() -> Result<MssAudioDevice, String> {
    let mut state = get_audio_state().lock().map_err(|_| "Failed to lock audio state".to_string())?;
    state.device.is_initialized = false;
    state.device.sample_handle = 0;
    Err("mss32.dll is only available on Windows".to_string())
}

pub fn play_sound_effect(sound_id: u32, volume: f32) {
    let Ok(mut state) = get_audio_state().lock() else {
        return;
    };

    if !state.device.is_initialized {
        return; // Silent fallback
    }

    let clamped_volume = volume.clamp(0.0, 1.0);

    #[cfg(windows)]
    {
        let init_ptr = state.device.ail_init_sample_ptr;
        let set_vol_ptr = state.device.ail_set_sample_volume_ptr;
        let start_ptr = state.device.ail_start_sample_ptr;
        
        let mut sample_handle = 0;
        
        // Only attempt to call actual DLL functions if they are properly loaded and we are not in a test mock
        // During tests, init_ptr will be 0 (or a mock value) unless we are actually running with a real mss32.dll
        if init_ptr != 0 && init_ptr != 1 {
            unsafe {
                let init_fn: AilInitSampleType = std::mem::transmute(init_ptr);
                sample_handle = init_fn();
                
                if sample_handle != 0 {
                    if set_vol_ptr != 0 {
                        let set_vol_fn: AilSetSampleVolumeType = std::mem::transmute(set_vol_ptr);
                        let mss_vol = (clamped_volume * 127.0) as c_int; // MSS typically uses 0-127
                        set_vol_fn(sample_handle, mss_vol);
                    }
                    
                    if start_ptr != 0 {
                        let start_fn: AilStartSampleType = std::mem::transmute(start_ptr);
                        start_fn(sample_handle);
                    }
                }
            }
        } else {
             // For testing purposes, assign a dummy sample handle if mocked
             if init_ptr == 1 {
                sample_handle = 999;
             }
        }
        state.playing_sounds.insert(sound_id, (sample_handle, clamped_volume));
    }

    #[cfg(not(windows))]
    {
        // On non-windows, this code path shouldn't realistically be hit if is_initialized is checked, 
        // but included for completeness in cross-platform builds.
        state.playing_sounds.insert(sound_id, (0, clamped_volume));
    }
}

pub fn stop_sound_effect(sound_id: u32) {
    let Ok(mut state) = get_audio_state().lock() else {
        return;
    };

    if !state.device.is_initialized {
        return; // Silent fallback
    }

    if let Some((sample_handle, _vol)) = state.playing_sounds.remove(&sound_id) {
        #[cfg(windows)]
        {
            let end_ptr = state.device.ail_end_sample_ptr;
            let rel_ptr = state.device.ail_release_sample_ptr;
            
            if sample_handle != 0 && end_ptr != 0 && end_ptr != 1 {
                unsafe {
                    let end_fn: AilEndSampleType = std::mem::transmute(end_ptr);
                    end_fn(sample_handle);
                    
                    if rel_ptr != 0 {
                        let rel_fn: AilReleaseSampleType = std::mem::transmute(rel_ptr);
                        rel_fn(sample_handle);
                    }
                }
            }
        }
    }
}

// Helper function for tests to reset state
pub fn reset_audio_state_for_test() {
    if let Ok(mut state) = get_audio_state().lock() {
        *state = AudioState::new();
    }
}

// Helper function for tests to inject a resource
pub fn add_resource_for_test(resource: SoundEffectResource) {
    if let Ok(mut state) = get_audio_state().lock() {
        state.resources.insert(resource.sound_id, resource);
    }
}

// Helper function for tests to check if a sound is playing (returns volume)
pub fn is_sound_playing_for_test(sound_id: u32) -> Option<f32> {
    if let Ok(state) = get_audio_state().lock() {
        state.playing_sounds.get(&sound_id).map(|&(_, v)| v)
    } else {
        None
    }
}

// Helper function for tests to mock successful initialization
pub fn mock_init_success_for_test() {
    if let Ok(mut state) = get_audio_state().lock() {
        state.device.is_initialized = true;
        state.device.sample_handle = 12345;
        
        #[cfg(windows)]
        {
            // Use 1 as a magic mock pointer so we don't try to call real functions in tests
            state.device.ail_init_sample_ptr = 1;
            state.device.ail_set_sample_volume_ptr = 1;
            state.device.ail_start_sample_ptr = 1;
            state.device.ail_end_sample_ptr = 1;
            state.device.ail_release_sample_ptr = 1;
        }
    }
}
