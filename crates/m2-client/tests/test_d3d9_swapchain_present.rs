use m2_client::d3d9_swapchain_present::{DeviceState, SwapChainManager};

// Lock state ensures exclusive execution when running FFI tests (if applicable)
static TEST_MUTEX: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();

#[test]
fn test_swapchain_initial_state() {
    let _lock = TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let manager = SwapChainManager::new();
    
    assert_eq!(manager.state, DeviceState::Ready);
    assert_eq!(manager.frame_counter, 0);
    assert_eq!(manager.dropped_frames, 0);
    assert_eq!(manager.test_cooperative_level(), DeviceState::Ready);
}

#[test]
fn test_swapchain_present_frame_success() {
    let _lock = TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let mut manager = SwapChainManager::new();
    
    // Presenting in Ready state should succeed and increment frame_counter
    assert_eq!(manager.present_frame(), Ok(()));
    assert_eq!(manager.frame_counter, 1);
    assert_eq!(manager.dropped_frames, 0);

    // Presenting again
    assert_eq!(manager.present_frame(), Ok(()));
    assert_eq!(manager.frame_counter, 2);
}

#[test]
fn test_swapchain_device_lost_presentation_failure() {
    let _lock = TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let mut manager = SwapChainManager::new();
    
    // Simulate device loss
    manager.set_device_state(DeviceState::DeviceLost);
    assert_eq!(manager.test_cooperative_level(), DeviceState::DeviceLost);

    // Presenting should fail and increment dropped_frames
    assert_eq!(manager.present_frame(), Err(DeviceState::DeviceLost));
    assert_eq!(manager.frame_counter, 0);
    assert_eq!(manager.dropped_frames, 1);

    assert_eq!(manager.present_frame(), Err(DeviceState::DeviceLost));
    assert_eq!(manager.dropped_frames, 2);
}

#[test]
fn test_swapchain_handle_device_not_reset_success() {
    let _lock = TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let mut manager = SwapChainManager::new();
    
    // Simulate device ready for reset
    manager.set_device_state(DeviceState::DeviceNotReset);
    
    // Simulate a successful reset callback
    let result = manager.handle_device_lost(|| true);
    
    assert!(result, "handle_device_lost should return true on successful reset");
    assert_eq!(manager.state, DeviceState::Ready);
    
    // We should be able to present now
    assert_eq!(manager.present_frame(), Ok(()));
    assert_eq!(manager.frame_counter, 1);
}

#[test]
fn test_swapchain_handle_device_not_reset_failure() {
    let _lock = TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let mut manager = SwapChainManager::new();
    
    // Simulate device ready for reset
    manager.set_device_state(DeviceState::DeviceNotReset);
    
    // Simulate a failed reset callback
    let result = manager.handle_device_lost(|| false);
    
    assert!(!result, "handle_device_lost should return false on failed reset");
    assert_eq!(manager.state, DeviceState::InternalError);
    
    // Presenting should still fail
    assert_eq!(manager.present_frame(), Err(DeviceState::InternalError));
    assert_eq!(manager.dropped_frames, 1);
}

#[test]
fn test_swapchain_handle_device_lost_state() {
    let _lock = TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let mut manager = SwapChainManager::new();
    
    manager.set_device_state(DeviceState::DeviceLost);
    
    // Callback should not be called in DeviceLost state
    let result = manager.handle_device_lost(|| {
        panic!("Callback should not be invoked when state is DeviceLost");
    });
    
    assert!(!result, "handle_device_lost should return false when device is just lost (not ready to reset)");
    assert_eq!(manager.state, DeviceState::DeviceLost);
}

#[test]
fn test_swapchain_handle_ready_state() {
    let _lock = TEST_MUTEX.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let mut manager = SwapChainManager::new();
    
    // If handle_device_lost is called while in Ready state, it should just return true
    let result = manager.handle_device_lost(|| {
        panic!("Callback should not be invoked when state is Ready");
    });
    
    assert!(result, "handle_device_lost should return true when device is already ready");
    assert_eq!(manager.state, DeviceState::Ready);
}
