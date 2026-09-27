use m2_client::d3d9_render_target_mgr::*;

#[test]
fn test_default_viewport() {
    let vp = D3DViewport::default();
    assert_eq!(vp.x, 0);
    assert_eq!(vp.y, 0);
    assert_eq!(vp.width, 0);
    assert_eq!(vp.height, 0);
    assert_eq!(vp.min_z, 0.0);
    assert_eq!(vp.max_z, 1.0);
}

#[test]
fn test_set_and_get_viewport() {
    let mut mgr = RenderTargetManager::new();
    let vp = D3DViewport {
        x: 10,
        y: 20,
        width: 800,
        height: 600,
        min_z: 0.1,
        max_z: 0.9,
    };
    
    assert!(mgr.set_viewport(&vp).is_ok());
    
    let retrieved_vp = mgr.get_viewport();
    assert_eq!(retrieved_vp, vp);
}

#[test]
fn test_set_viewport_invalid() {
    let mut mgr = RenderTargetManager::new();
    let vp = D3DViewport {
        x: 0,
        y: 0,
        width: 0,
        height: 600,
        min_z: 0.0,
        max_z: 1.0,
    };
    
    assert_eq!(mgr.set_viewport(&vp), Err(RenderTargetError::InvalidViewportDimensions));
}

#[test]
fn test_clear_render_target_valid_flags() {
    let mgr = RenderTargetManager::new();
    let flags = D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER;
    
    assert!(mgr.clear_render_target(flags, 0xFFFFFFFF, 1.0, 0).is_ok());
}

#[test]
fn test_clear_render_target_invalid_flags() {
    let mgr = RenderTargetManager::new();
    let invalid_flag = 0x00000008; // Arbitrary unused flag
    let flags = D3DCLEAR_TARGET | invalid_flag;
    
    assert_eq!(mgr.clear_render_target(flags, 0xFFFFFFFF, 1.0, 0), Err(RenderTargetError::InvalidClearFlags));
}

#[test]
fn test_calculate_letterbox_exact_ratio() {
    let mgr = RenderTargetManager::new();
    // 1920x1080 -> 16:9 ratio
    let target_ratio = 1920.0 / 1080.0;
    
    let vp = mgr.calculate_letterbox_viewport(1920, 1080, target_ratio).unwrap();
    assert_eq!(vp.x, 0);
    assert_eq!(vp.y, 0);
    assert_eq!(vp.width, 1920);
    assert_eq!(vp.height, 1080);
}

#[test]
fn test_calculate_letterbox_pillarbox() {
    let mgr = RenderTargetManager::new();
    // Ultra-wide screen: 2560x1080, but target is 16:9 (1.777...)
    let target_ratio = 1920.0 / 1080.0;
    
    let vp = mgr.calculate_letterbox_viewport(2560, 1080, target_ratio).unwrap();
    // Expected width = 1080 * (16/9) = 1920
    assert_eq!(vp.width, 1920);
    assert_eq!(vp.height, 1080);
    // x should be centered: (2560 - 1920) / 2 = 320
    assert_eq!(vp.x, 320);
    assert_eq!(vp.y, 0);
}

#[test]
fn test_calculate_letterbox_true_letterbox() {
    let mgr = RenderTargetManager::new();
    // Square-ish screen: 1024x1024, target is 16:9
    let target_ratio = 16.0 / 9.0;
    
    let vp = mgr.calculate_letterbox_viewport(1024, 1024, target_ratio).unwrap();
    // Expected height = 1024 / (16/9) = 576
    assert_eq!(vp.width, 1024);
    assert_eq!(vp.height, 576);
    // y should be centered: (1024 - 576) / 2 = 224
    assert_eq!(vp.x, 0);
    assert_eq!(vp.y, 224);
}

#[test]
fn test_calculate_letterbox_invalid() {
    let mgr = RenderTargetManager::new();
    let target_ratio = 16.0 / 9.0;
    
    assert_eq!(mgr.calculate_letterbox_viewport(0, 1080, target_ratio), Err(RenderTargetError::InvalidViewportDimensions));
    assert_eq!(mgr.calculate_letterbox_viewport(1920, 0, target_ratio), Err(RenderTargetError::InvalidViewportDimensions));
    assert_eq!(mgr.calculate_letterbox_viewport(1920, 1080, 0.0), Err(RenderTargetError::InvalidViewportDimensions));
}
