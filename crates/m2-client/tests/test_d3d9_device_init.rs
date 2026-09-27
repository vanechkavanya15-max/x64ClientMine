use m2_client::d3d9_device_init::{
    create_present_parameters, free_d3d9_dll, get_direct3d_create9, get_direct3d_create9_ex,
    initialize_d3d9_context, is_d3d9_available, load_d3d9_dll, select_best_depth_stencil_format,
    try_create_device_with_fallback, D3D9Config, D3DCREATE_HARDWARE_VERTEXPROCESSING,
    D3DCREATE_MIXED_VERTEXPROCESSING, D3DCREATE_SOFTWARE_VERTEXPROCESSING, D3DFMT_D24S8,
    D3DFMT_X8R8G8B8, D3DPRESENT_INTERVAL_IMMEDIATE, D3DPRESENT_INTERVAL_ONE,
    D3DSWAPEFFECT_DISCARD,
};

#[test]
fn test_create_present_parameters_windowed() {
    let config = D3D9Config {
        width: 1024,
        height: 768,
        is_windowed: true,
        vsync: true,
        adapter_index: 0,
    };

    let params = create_present_parameters(&config);

    assert_eq!(params.back_buffer_width, 1024);
    assert_eq!(params.back_buffer_height, 768);
    assert_eq!(params.back_buffer_format, D3DFMT_X8R8G8B8);
    assert_eq!(params.swap_effect, D3DSWAPEFFECT_DISCARD);
    assert_eq!(params.windowed, 1);
    assert_eq!(params.enable_auto_depth_stencil, 1);
    assert_eq!(params.auto_depth_stencil_format, D3DFMT_D24S8);
    assert_eq!(params.presentation_interval, D3DPRESENT_INTERVAL_ONE);
    assert_eq!(params.full_screen_refresh_rate_in_hz, 0);
}

#[test]
fn test_create_present_parameters_fullscreen() {
    let config = D3D9Config {
        width: 1920,
        height: 1080,
        is_windowed: false,
        vsync: false,
        adapter_index: 0,
    };

    let params = create_present_parameters(&config);

    assert_eq!(params.back_buffer_width, 1920);
    assert_eq!(params.back_buffer_height, 1080);
    assert_eq!(params.back_buffer_format, D3DFMT_X8R8G8B8);
    assert_eq!(params.swap_effect, D3DSWAPEFFECT_DISCARD);
    assert_eq!(params.windowed, 0);
    assert_eq!(params.enable_auto_depth_stencil, 1);
    assert_eq!(params.auto_depth_stencil_format, D3DFMT_D24S8);
    assert_eq!(params.presentation_interval, D3DPRESENT_INTERVAL_IMMEDIATE);
    assert_eq!(params.full_screen_refresh_rate_in_hz, 60);
}

#[test]
fn test_select_best_depth_stencil_format() {
    let format = select_best_depth_stencil_format();
    assert_eq!(format, D3DFMT_D24S8);
}

#[test]
fn test_try_create_device_with_fallback_modes() {
    // Adapter 0 supports all modes
    assert!(try_create_device_with_fallback(0, D3DCREATE_HARDWARE_VERTEXPROCESSING).is_ok());
    assert!(try_create_device_with_fallback(0, D3DCREATE_MIXED_VERTEXPROCESSING).is_ok());
    assert!(try_create_device_with_fallback(0, D3DCREATE_SOFTWARE_VERTEXPROCESSING).is_ok());

    // Adapter 1 rejects hardware, supports mixed & software
    assert!(try_create_device_with_fallback(1, D3DCREATE_HARDWARE_VERTEXPROCESSING).is_err());
    assert!(try_create_device_with_fallback(1, D3DCREATE_MIXED_VERTEXPROCESSING).is_ok());
    assert!(try_create_device_with_fallback(1, D3DCREATE_SOFTWARE_VERTEXPROCESSING).is_ok());

    // Adapter 2 rejects hardware & mixed, supports software only
    assert!(try_create_device_with_fallback(2, D3DCREATE_HARDWARE_VERTEXPROCESSING).is_err());
    assert!(try_create_device_with_fallback(2, D3DCREATE_MIXED_VERTEXPROCESSING).is_err());
    assert!(try_create_device_with_fallback(2, D3DCREATE_SOFTWARE_VERTEXPROCESSING).is_ok());

    // Adapter 3+ rejects all
    assert!(try_create_device_with_fallback(3, D3DCREATE_SOFTWARE_VERTEXPROCESSING).is_err());
}

#[test]
fn test_initialize_d3d9_context_invalid_resolution() {
    let config = D3D9Config {
        width: 0,
        height: 1080,
        is_windowed: false,
        vsync: false,
        adapter_index: 0,
    };

    let result = initialize_d3d9_context(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid resolution");

    let config2 = D3D9Config {
        width: 1920,
        height: 0,
        is_windowed: false,
        vsync: false,
        adapter_index: 0,
    };

    let result2 = initialize_d3d9_context(&config2);
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), "Invalid resolution");
}

#[test]
fn test_initialize_d3d9_context_adapter0_hardware() {
    let config = D3D9Config {
        width: 800,
        height: 600,
        is_windowed: true,
        vsync: true,
        adapter_index: 0, // Should accept hardware vertex processing
    };

    let result = initialize_d3d9_context(&config);
    assert!(result.is_ok());

    let caps = result.unwrap();
    assert_eq!(caps.hardware_tnl, true);
    assert_eq!(caps.max_texture_width, 8192);
    assert_eq!(caps.max_texture_height, 8192);
    assert_eq!(caps.max_anisotropy, 16);
}

#[test]
fn test_initialize_d3d9_context_adapter1_mixed() {
    let config = D3D9Config {
        width: 800,
        height: 600,
        is_windowed: true,
        vsync: true,
        adapter_index: 1, // Should fallback to mixed vertex processing
    };

    let result = initialize_d3d9_context(&config);
    assert!(result.is_ok());

    let caps = result.unwrap();
    assert_eq!(caps.hardware_tnl, true); // Mixed is still considered hardware T&L for our caps
    assert_eq!(caps.max_texture_width, 8192);
    assert_eq!(caps.max_texture_height, 8192);
    assert_eq!(caps.max_anisotropy, 16);
}

#[test]
fn test_initialize_d3d9_context_adapter2_software() {
    let config = D3D9Config {
        width: 800,
        height: 600,
        is_windowed: true,
        vsync: true,
        adapter_index: 2, // Should fallback to software vertex processing
    };

    let result = initialize_d3d9_context(&config);
    assert!(result.is_ok());

    let caps = result.unwrap();
    assert_eq!(caps.hardware_tnl, false);
    assert_eq!(caps.max_texture_width, 2048);
    assert_eq!(caps.max_texture_height, 2048);
    assert_eq!(caps.max_anisotropy, 4);
}

#[test]
fn test_initialize_d3d9_context_adapter3_fails() {
    let config = D3D9Config {
        width: 800,
        height: 600,
        is_windowed: true,
        vsync: true,
        adapter_index: 99, // Non-existent adapter
    };

    let result = initialize_d3d9_context(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Failed to create D3D9 device with any vertex processing mode");
}

#[test]
fn test_d3d9_library_loading_windows() {
    #[cfg(windows)]
    {
        let res = load_d3d9_dll();
        assert!(res.is_ok(), "d3d9.dll should load successfully on Windows");
        let handle = res.unwrap();
        assert!(!handle.is_null());

        let create_fn = get_direct3d_create9();
        assert!(create_fn.is_some(), "Direct3DCreate9 function pointer should be resolved");

        let create_ex_fn = get_direct3d_create9_ex();
        assert!(create_ex_fn.is_some(), "Direct3DCreate9Ex function pointer should be resolved");

        assert!(is_d3d9_available());

        let freed = free_d3d9_dll(handle);
        assert!(freed);
    }
}
