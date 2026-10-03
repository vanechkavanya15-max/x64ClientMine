use rodio::{OutputStream, OutputStreamHandle};

/// Represents an active Rodio audio device stream.
/// The `OutputStream` must be kept alive for audio playback to function.
pub struct RodioAudioDevice {
    pub stream: Option<OutputStream>,
    pub handle: Option<OutputStreamHandle>,
    pub is_initialized: bool,
}

impl RodioAudioDevice {
    /// Creates a new, uninitialized RodioAudioDevice.
    pub fn new() -> Self {
        Self {
            stream: None,
            handle: None,
            is_initialized: false,
        }
    }

    /// Initializes the physical audio device.
    /// This is supported on Windows.
    #[cfg(windows)]
    pub fn init(&mut self) -> Result<(), String> {
        match OutputStream::try_default() {
            Ok((stream, handle)) => {
                self.stream = Some(stream);
                self.handle = Some(handle);
                self.is_initialized = true;
                Ok(())
            }
            Err(e) => {
                self.is_initialized = false;
                Err(format!("Failed to initialize audio device: {}", e))
            }
        }
    }

    /// On non-Windows platforms, returns an error as per the implementation requirement.
    #[cfg(not(windows))]
    pub fn init(&mut self) -> Result<(), String> {
        self.is_initialized = false;
        Err("Audio device initialization is only supported on Windows in this implementation".to_string())
    }
}
