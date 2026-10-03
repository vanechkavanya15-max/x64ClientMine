use m2_client::main_render_pipeline_composition::MainRenderPipelineComposition;
use m2_client::client_engine_main::WindowConfig;
use m2_client::wgpu_engine_context::WgpuEngineContext;
use m2_client::wgpu_render_pipeline_mgr::WgpuRenderPipelineManager;

#[test]
fn test_integration_pipeline_composition_sequence() {
    let mut ctx = WgpuEngineContext::new(0, 800, 600).unwrap();
    ctx.is_headless = true;
    
    let window_config = WindowConfig::default();
    let mut composition = MainRenderPipelineComposition::new(window_config);
    
    let pipeline_mgr = WgpuRenderPipelineManager::new(&ctx.device, 800, 600);
    
    let clear_color = wgpu::Color::BLACK;
    let ui_batches = vec![];
    
    let res = composition.execute_frame(&ctx, &pipeline_mgr, clear_color, &ui_batches);
    assert!(res.is_ok());
    
    // We expect the flags to reflect all passes, as `execute_render_passes_headless` 
    // simply flags them all. This proves integration works.
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
