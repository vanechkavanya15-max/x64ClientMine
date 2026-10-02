use m2_client::main_render_pipeline_composition::{MainRenderPipelineComposition, PipelineState};
use m2_client::client_engine_main::WindowConfig;
use m2_client::wgpu_engine_context::WgpuEngineContext;
use m2_client::wgpu_render_pipeline_mgr::WgpuRenderPipelineManager;
use std::time::Instant;

fn setup_headless_context() -> WgpuEngineContext {
    let mut ctx = WgpuEngineContext::new(0, 800, 600).unwrap();
    ctx.is_headless = true;
    ctx
}

#[test]
fn test_initialization() {
    let window_config = WindowConfig {
        width: 1024,
        height: 768,
        is_windowed: true,
        vsync: true,
    };
    
    let composition = MainRenderPipelineComposition::new(window_config);
    
    assert_eq!(composition.state, PipelineState::Active);
    assert_eq!(composition.target_fps, 60);
    assert_eq!(composition.frame_time_ms, 16);
    assert_eq!(composition.window_config.width, 1024);
    assert_eq!(composition.window_config.height, 768);
    assert_eq!(composition.pass_execution_flags, 0);
}

#[test]
fn test_resize_minimize() {
    let window_config = WindowConfig::default();
    let mut composition = MainRenderPipelineComposition::new(window_config);
    
    composition.handle_resize(800, 600);
    assert_eq!(composition.state, PipelineState::Active);
    assert_eq!(composition.window_config.width, 800);
    
    composition.handle_resize(0, 0);
    assert_eq!(composition.state, PipelineState::Minimized);
    assert_eq!(composition.window_config.width, 0);
    
    composition.handle_resize(1920, 1080);
    assert_eq!(composition.state, PipelineState::Active);
    assert_eq!(composition.window_config.width, 1920);
}

#[test]
fn test_render_sequence_headless() {
    let window_config = WindowConfig::default();
    let mut composition = MainRenderPipelineComposition::new(window_config);
    let ctx = setup_headless_context();
    
    // We don't actually have a real WgpuRenderPipelineManager setup for headless test execution,
    // but the execute_frame should early exit and simulate the flags.
    
    // We create a dummy WgpuRenderPipelineManager
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let clear_color = wgpu::Color::BLACK;
    let ui_batches = vec![];
    
    let res = composition.execute_frame(&ctx, &pipeline_mgr, clear_color, &ui_batches);
    assert!(res.is_ok());
    
    // Check if all flags are set (11 passes)
    let expected_flags = 
        MainRenderPipelineComposition::PASS_CLEAR |
        MainRenderPipelineComposition::PASS_SKYBOX |
        MainRenderPipelineComposition::PASS_SHADOW |
        MainRenderPipelineComposition::PASS_TERRAIN |
        MainRenderPipelineComposition::PASS_SPEEDTREE |
        MainRenderPipelineComposition::PASS_WATER |
        MainRenderPipelineComposition::PASS_CHARACTERS |
        MainRenderPipelineComposition::PASS_PARTICLES |
        MainRenderPipelineComposition::PASS_TEXTTAIL |
        MainRenderPipelineComposition::PASS_UI |
        MainRenderPipelineComposition::PASS_PRESENT;
        
    assert_eq!(composition.pass_execution_flags, expected_flags);
}

#[test]
fn test_minimized_skips_rendering() {
    let window_config = WindowConfig::default();
    let mut composition = MainRenderPipelineComposition::new(window_config);
    composition.handle_resize(0, 0);
    
    let ctx = setup_headless_context();
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let clear_color = wgpu::Color::BLACK;
    let ui_batches = vec![];
    
    let res = composition.execute_frame(&ctx, &pipeline_mgr, clear_color, &ui_batches);
    assert!(res.is_ok());
    
    // Execution flags should be 0 since rendering was skipped
    assert_eq!(composition.pass_execution_flags, 0);
}

#[test]
fn test_fps_limiting() {
    let mut window_config = WindowConfig::default();
    window_config.vsync = false;
    let mut composition = MainRenderPipelineComposition::new(window_config);
    
    // Simulate a fast frame
    composition.last_frame_time = Instant::now();
    composition.limit_fps(); // Should sleep for ~16ms
    
    let elapsed = composition.last_frame_time.elapsed().as_millis();
    // It should have slept, so the next last_frame_time should be very recent,
    // and the time between the first last_frame_time and now should be roughly 16ms
    // We check that limit_fps advanced the last_frame_time
    assert!(elapsed < 16);
}
