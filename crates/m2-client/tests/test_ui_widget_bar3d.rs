#[path = "../src/ui_widget_bar3d.rs"]
mod ui_widget_bar3d;

use ui_widget_bar3d::{Bar3D, D3dxColor, RenderCommand, project_world_to_screen, wndMgr_RegisterBar3D_Mock, wndMgr_SetColorBar3D_Mock};

#[test]
fn test_bar3d_initialization() {
    let bar = Bar3D::new();
    
    assert_eq!(bar.left_color, D3dxColor::new(0.2, 0.2, 0.2, 1.0));
    assert_eq!(bar.right_color, D3dxColor::new(0.7, 0.7, 0.7, 1.0));
    assert_eq!(bar.center_color, D3dxColor::new(0.0, 0.0, 0.0, 1.0));
    assert_eq!(bar.rect.left, 0.0);
    assert_eq!(bar.rect.top, 0.0);
    assert_eq!(bar.rect.right, 0.0);
    assert_eq!(bar.rect.bottom, 0.0);
}

#[test]
fn test_bar3d_set_color_and_rect() {
    let mut bar = Bar3D::new();
    
    let left = D3dxColor::new(1.0, 0.0, 0.0, 1.0);
    let right = D3dxColor::new(0.0, 1.0, 0.0, 1.0);
    let center = D3dxColor::new(0.0, 0.0, 1.0, 1.0);
    
    bar.set_color(left, right, center);
    bar.set_rect(10.0, 20.0, 100.0, 30.0);
    
    assert_eq!(bar.left_color, left);
    assert_eq!(bar.right_color, right);
    assert_eq!(bar.center_color, center);
    
    assert_eq!(bar.rect.left, 10.0);
    assert_eq!(bar.rect.top, 20.0);
    assert_eq!(bar.rect.right, 100.0);
    assert_eq!(bar.rect.bottom, 30.0);
}

#[test]
fn test_bar3d_on_render_commands() {
    let mut bar = Bar3D::new();
    let left = D3dxColor::new(1.0, 0.0, 0.0, 1.0);
    let right = D3dxColor::new(0.0, 1.0, 0.0, 1.0);
    let center = D3dxColor::new(0.0, 0.0, 1.0, 1.0);
    bar.set_color(left, right, center);
    bar.set_rect(10.0, 20.0, 110.0, 30.0);
    
    let commands = bar.on_render();
    assert_eq!(commands.len(), 7);
    
    // Check Center Fill
    assert_eq!(commands[0], RenderCommand::SetColor(center));
    assert_eq!(commands[1], RenderCommand::RenderBar2d { left: 10.0, top: 20.0, right: 110.0, bottom: 30.0 });
    
    // Check Left Border
    assert_eq!(commands[2], RenderCommand::SetColor(left));
    assert_eq!(commands[3], RenderCommand::RenderLine2d { x1: 10.0, y1: 20.0, x2: 110.0, y2: 20.0 });
    assert_eq!(commands[4], RenderCommand::RenderLine2d { x1: 10.0, y1: 20.0, x2: 10.0, y2: 30.0 });
    
    // Check Right Border
    assert_eq!(commands[5], RenderCommand::SetColor(right));
    assert_eq!(commands[6], RenderCommand::RenderLine2d { x1: 10.0, y1: 30.0, x2: 110.0, y2: 30.0 });
    // Note: C++ has RenderLine2d(right, top, right, bottom)
    // Commands should match exactly 1:1
}

#[test]
fn test_bar3d_world_to_screen_projection() {
    // Identity matrix should map (0,0,0) to center of screen (with Z=0 inside frustum if w=1)
    let identity_vp: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    
    let screen_w = 800.0;
    let screen_h = 600.0;
    
    let result = project_world_to_screen(0.0, 0.0, 0.0, &identity_vp, screen_w, screen_h);
    assert!(result.is_some());
    
    if let Some((sx, sy)) = result {
        assert_eq!(sx, 400.0);
        assert_eq!(sy, 300.0);
    }
}

#[test]
fn test_bar3d_world_to_screen_behind_camera() {
    // Modify "W" coordinate to represent point behind camera
    let behind_vp: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, -1.0, // W = -1.0
    ];
    
    let result = project_world_to_screen(0.0, 0.0, 0.0, &behind_vp, 800.0, 600.0);
    assert!(result.is_none());
}

#[test]
fn test_bar3d_ffi_mocks() {
    let handle = wndMgr_RegisterBar3D_Mock(12345);
    assert_eq!(handle, 999);
    
    let success = wndMgr_SetColorBar3D_Mock(
        handle,
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 1.0,
    );
    assert!(success);
}
