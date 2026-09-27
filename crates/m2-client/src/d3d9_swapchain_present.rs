//! Phase C - Pure Rust replacement for C++ D3D9 Swapchain & Present logic.
//! Reference: source/EterLib/GrpDevice.cpp

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum DeviceState {
    Ready,
    DeviceLost,
    DeviceNotReset,
    InternalError,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self::Ready
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct SwapChainManager {
    pub state: DeviceState,
    pub frame_counter: u64,
    pub dropped_frames: u64,
}

impl SwapChainManager {
    /// Creates a new SwapChainManager in the Ready state.
    pub fn new() -> Self {
        Self {
            state: DeviceState::Ready,
            frame_counter: 0,
            dropped_frames: 0,
        }
    }

    /// Tests the cooperative level of the device.
    /// In a real D3D9 application, this corresponds to `IDirect3DDevice9::TestCooperativeLevel`.
    pub fn test_cooperative_level(&self) -> DeviceState {
        self.state
    }

    /// Manually sets the device state. Used for simulating device loss from external sources.
    pub fn set_device_state(&mut self, state: DeviceState) {
        self.state = state;
    }

    /// Handles a lost device scenario.
    /// 
    /// If the device state is `DeviceNotReset`, it attempts to reset using `reset_callback`.
    /// Returns `true` if the device is ready or successfully reset, `false` otherwise.
    pub fn handle_device_lost<F>(&mut self, reset_callback: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        match self.state {
            DeviceState::Ready => true,
            DeviceState::DeviceLost => false,
            DeviceState::DeviceNotReset => {
                if reset_callback() {
                    self.state = DeviceState::Ready;
                    true
                } else {
                    self.state = DeviceState::InternalError;
                    false
                }
            }
            DeviceState::InternalError => false,
        }
    }

    /// Presents a frame.
    /// Increments `frame_counter` on success, or `dropped_frames` if the device is not ready.
    pub fn present_frame(&mut self) -> Result<(), DeviceState> {
        if self.state == DeviceState::Ready {
            self.frame_counter = self.frame_counter.saturating_add(1);
            Ok(())
        } else {
            self.dropped_frames = self.dropped_frames.saturating_add(1);
            Err(self.state)
        }
    }
}
