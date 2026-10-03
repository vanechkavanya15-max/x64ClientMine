use serial_test::serial;

#[path = "../src/wgpu_rendertarget_integration.rs"]
mod wgpu_rendertarget_integration;

// Import niezbędnych elementów bezpośrednio z załadowanego modułu w teście
use wgpu_rendertarget_integration::{RenderTargetManager, RenderTargetCamera};
use m2_client::wgpu_engine_context::{WgpuEngineContext, GLOBAL_WGPU_CONTEXT, set_global_wgpu_context};

/// Tworzy headless context WGPU do testów
async fn setup_headless_context() -> WgpuEngineContext {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::new_without_display_handle());
    
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: None, // headless
            force_fallback_adapter: false,
        })
        .await
        .expect("Nie znaleziono adaptera WGPU");
        
    let device_and_queue = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .expect("Nie udalo sie utworzyc urzadzenia WGPU");

    let device = device_and_queue.0;
    let queue = device_and_queue.1;

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        ..Default::default()
    });

    WgpuEngineContext {
        instance,
        surface: None,
        adapter,
        device,
        queue,
        config: None,
        width: 800,
        height: 600,
        is_headless: true,
        pipeline_2d: None,
        uniform_buffer: None,
        uniform_bind_group: None,
        texture_bind_group_layout: None,
        sampler: Some(sampler),
    }
}

/// Zapewnia zainicjowanie globalnego kontekstu przed użyciem (z serializacją testów).
fn ensure_global_wgpu_context() {
    if GLOBAL_WGPU_CONTEXT.get().is_none() {
        let ctx = pollster::block_on(setup_headless_context());
        set_global_wgpu_context(ctx);
    }
}

#[test]
#[serial]
fn test_render_target_manager_create_and_delete() {
    ensure_global_wgpu_context();
    let guard = GLOBAL_WGPU_CONTEXT.get().unwrap().lock().unwrap();
    let ctx = guard.as_ref().unwrap();

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
    let guard = GLOBAL_WGPU_CONTEXT.get().unwrap().lock().unwrap();
    let ctx = guard.as_ref().unwrap();

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
    let guard = GLOBAL_WGPU_CONTEXT.get().unwrap().lock().unwrap();
    let ctx = guard.as_ref().unwrap();

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
    let guard = GLOBAL_WGPU_CONTEXT.get().unwrap().lock().unwrap();
    let ctx = guard.as_ref().unwrap();

    let mut manager = RenderTargetManager::new();
    manager.create_target(ctx, 99, 128, 128).unwrap();
    let target = manager.get_target(99).unwrap();

    // Symulacja wykonania clear na teksturze offscreen.
    // Metoda wrzuca encodery do kolejki urządzenia - brak crashu oznacza
    // poprawne wkomponowanie rander passa do command buffora WGPU.
    target.clear(ctx);

    ctx.device.poll(wgpu::MaintainBase::Wait);
}

#[test]
#[serial]
fn test_render_target_visibility_flag() {
    ensure_global_wgpu_context();
    let guard = GLOBAL_WGPU_CONTEXT.get().unwrap().lock().unwrap();
    let ctx = guard.as_ref().unwrap();

    let mut manager = RenderTargetManager::new();
    manager.create_target(ctx, 22, 64, 64).unwrap();
    
    let target = manager.get_target_mut(22).unwrap();
    // Domyślnie zgaszone
    assert_eq!(target.visibility, false);

    // Przełącz flagę
    target.visibility = true;
    
    let verified = manager.get_target(22).unwrap();
    assert_eq!(verified.visibility, true);
}
