use serial_test::serial;
use m2_client::wgpu_rendertarget_integration::{RenderTargetManager, RenderTargetCamera};
use m2_client::wgpu_engine_context::{WgpuEngineContext, GLOBAL_WGPU_CONTEXT, set_global_wgpu_context};

/// Zapewnia zainicjowanie globalnego kontekstu przed użyciem (z serializacją testów).
fn ensure_global_wgpu_context() {
    if GLOBAL_WGPU_CONTEXT.get().is_none() {
        if let Ok(ctx) = WgpuEngineContext::new(0, 800, 600) {
            set_global_wgpu_context(ctx);
        }
    }
}

#[test]
#[serial]
fn test_render_target_manager_create_and_delete() {
    ensure_global_wgpu_context();
    let global_opt = GLOBAL_WGPU_CONTEXT.get();
    if global_opt.is_none() {
        eprintln!("Brak GPU w srodowisku CI/testowym, pomijanie testu");
        return;
    }
    let guard = global_opt.unwrap().lock().unwrap();
    let ctx = match guard.as_ref() {
        Some(c) => c,
        None => return,
    };

    let mut manager = RenderTargetManager::new();

    // Utworzenie targetu o ID 1
    assert!(manager.create_target(ctx, 1, 256, 256).is_ok());
    assert!(manager.get_target(1).is_some());

    // Próba utworzenia targetu o tym samym ID powinna zwrócić błąd
    let dup_err = manager.create_target(ctx, 1, 128, 128);
    assert!(dup_err.is_err());
    assert!(dup_err.unwrap_err().contains("już istnieje"));

    // Utworzenie drugiego targetu
    assert!(manager.create_target(ctx, 2, 512, 512).is_ok());

    // Usunięcie pierwszego targetu
    manager.delete_target(1);
    assert!(manager.get_target(1).is_none());
    assert!(manager.get_target(2).is_some());
}

#[test]
#[serial]
fn test_render_target_ui_resource_generation() {
    ensure_global_wgpu_context();
    let global_opt = GLOBAL_WGPU_CONTEXT.get();
    if global_opt.is_none() {
        return;
    }
    let guard = global_opt.unwrap().lock().unwrap();
    let ctx = match guard.as_ref() {
        Some(c) => c,
        None => return,
    };

    let mut manager = RenderTargetManager::new();
    manager.create_target(ctx, 5, 400, 300).unwrap();

    let target = manager.get_target(5).unwrap();
    
    // Konwersja na UiImageResource
    let ui_res = target.as_ui_resource();
    
    // Weryfikacja wymiarów
    assert_eq!(ui_res.width, 400);
    assert_eq!(ui_res.height, 300);

    // Weryfikacja domyślnych UV mappingu (od [0, 0] do [1, 1])
    assert_eq!(ui_res.uv, [0.0, 0.0, 1.0, 1.0]);

    // Sprawdzenie poprzez menedżera
    let ui_res_mgr_opt = manager.get_target_ui_resource(5);
    assert!(ui_res_mgr_opt.is_some());
    
    let ui_res_mgr = ui_res_mgr_opt.unwrap();
    assert_eq!(ui_res_mgr.width, 400);
}

#[test]
#[serial]
fn test_render_target_camera_defaults_and_mutation() {
    ensure_global_wgpu_context();
    let global_opt = GLOBAL_WGPU_CONTEXT.get();
    if global_opt.is_none() {
        return;
    }
    let guard = global_opt.unwrap().lock().unwrap();
    let ctx = match guard.as_ref() {
        Some(c) => c,
        None => return,
    };

    let mut manager = RenderTargetManager::new();
    manager.create_target(ctx, 10, 100, 100).unwrap();

    let target = manager.get_target_mut(10).unwrap();
    
    // Sprawdzenie domyślnych wartości
    let default_cam = RenderTargetCamera::default();
    assert_eq!(target.camera.eye, default_cam.eye);
    assert_eq!(target.camera.target, default_cam.target);
    assert_eq!(target.camera.up, default_cam.up);
    assert_eq!(target.camera.fov_y, default_cam.fov_y);

    // Modyfikacja kamery
    target.camera.eye = glam::Vec3::new(10.0, 20.0, 30.0);
    target.camera.target = glam::Vec3::new(0.0, 0.0, 0.0);
    target.camera.fov_y = 60.0;

    let target_ref = manager.get_target(10).unwrap();
    assert_eq!(target_ref.camera.eye, glam::Vec3::new(10.0, 20.0, 30.0));
    assert_eq!(target_ref.camera.target, glam::Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(target_ref.camera.fov_y, 60.0);
}

#[test]
#[serial]
fn test_render_target_clear_pass() {
    ensure_global_wgpu_context();
    let global_opt = GLOBAL_WGPU_CONTEXT.get();
    if global_opt.is_none() {
        return;
    }
    let guard = global_opt.unwrap().lock().unwrap();
    let ctx = match guard.as_ref() {
        Some(c) => c,
        None => return,
    };

    let mut manager = RenderTargetManager::new();
    manager.create_target(ctx, 99, 128, 128).unwrap();
    let target = manager.get_target(99).unwrap();

    target.clear(ctx);

    let _ = ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
}

#[test]
#[serial]
fn test_render_target_visibility_flag() {
    ensure_global_wgpu_context();
    let global_opt = GLOBAL_WGPU_CONTEXT.get();
    if global_opt.is_none() {
        return;
    }
    let guard = global_opt.unwrap().lock().unwrap();
    let ctx = match guard.as_ref() {
        Some(c) => c,
        None => return,
    };

    let mut manager = RenderTargetManager::new();
    manager.create_target(ctx, 22, 64, 64).unwrap();
    
    let target = manager.get_target_mut(22).unwrap();
    assert_eq!(target.visibility, false);

    target.visibility = true;
    
    let verified = manager.get_target(22).unwrap();
    assert_eq!(verified.visibility, true);
}
