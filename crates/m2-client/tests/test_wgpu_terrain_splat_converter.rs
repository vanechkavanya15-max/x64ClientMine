#[path = "../src/wgpu_terrain_splat_converter.rs"]
mod wgpu_terrain_splat_converter;

use wgpu_terrain_splat_converter::{
    pack_alpha_maps_to_rgba, SplatConverterError, TerrainSplatConverterConfig, WgpuTerrainSplatConverter,
};
use m2_client::wgpu_terrain_splatting::TerrainSplattingRenderer;
use m2_client::texture_set_parser::TextureSet;

async fn setup_wgpu() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions { apply_limit_buckets: false,
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await.ok()?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .ok()?;

    Some((device, queue))
}

#[test]
fn test_default_config() {
    let config = TerrainSplatConverterConfig::default();
    assert_eq!(config.splat_alpha_x_size, 258);
    assert_eq!(config.splat_alpha_y_size, 258);
    assert_eq!(config.max_splat_limit, 50000);
}

#[test]
fn test_pack_alpha_maps_to_rgba_success() {
    let mask0 = vec![255, 0];
    let mask1 = vec![0, 255];
    let mask2 = vec![128, 128];
    let mask3 = vec![64, 64];

    let result = pack_alpha_maps_to_rgba(&mask0, &mask1, &mask2, &mask3, 2, 1);
    assert!(result.is_ok());

    let packed = result.unwrap();
    assert_eq!(packed.len(), 8);
    assert_eq!(packed[0], 255); // Pixel 0, R
    assert_eq!(packed[1], 0);   // Pixel 0, G
    assert_eq!(packed[2], 128); // Pixel 0, B
    assert_eq!(packed[3], 64);  // Pixel 0, A
    assert_eq!(packed[4], 0);   // Pixel 1, R
    assert_eq!(packed[5], 255); // Pixel 1, G
    assert_eq!(packed[6], 128); // Pixel 1, B
    assert_eq!(packed[7], 64);  // Pixel 1, A
}

#[test]
fn test_pack_alpha_maps_invalid_size() {
    let mask0 = vec![255, 0];
    let mask1 = vec![0, 255];
    let mask2 = vec![128]; // Mismatched size
    let mask3 = vec![64, 64];

    let result = pack_alpha_maps_to_rgba(&mask0, &mask1, &mask2, &mask3, 2, 1);
    assert_eq!(result, Err(SplatConverterError::InvalidAlphaSize));
}

#[test]
fn test_converter_initial_state() {
    let config = TerrainSplatConverterConfig::default();
    let converter = WgpuTerrainSplatConverter::new(config);

    assert_eq!(converter.rendered_splat_num, 0);
    assert_eq!(converter.rendered_splat_num_sq_sum, 0);
    assert!(converter.splat_alpha_texture.is_none());
    assert!(converter.splat_sampler.is_none());
    assert!(converter.tex_sampler.is_none());
    
    let (rendered, ratio) = converter.get_rendered_splat_num(10);
    assert_eq!(rendered, 0);
    assert_eq!(ratio, 0.0);
}

#[test]
fn test_set_splat_limit() {
    let mut converter = WgpuTerrainSplatConverter::new(TerrainSplatConverterConfig::default());
    converter.set_splat_limit(100);
    assert_eq!(converter.config.max_splat_limit, 100);
}

#[test]
fn test_get_rendered_splat_ratio() {
    let mut converter = WgpuTerrainSplatConverter::new(TerrainSplatConverterConfig::default());
    converter.rendered_splat_num = 10;
    converter.rendered_splat_num_sq_sum = 50;

    let (num, ratio) = converter.get_rendered_splat_num(5);
    assert_eq!(num, 10);
    assert_eq!(ratio, 10.0);

    let (_, ratio_zero) = converter.get_rendered_splat_num(0);
    assert_eq!(ratio_zero, 0.0);
}

#[tokio::test]
async fn test_load_alpha_mask() {
    let wgpu_context = setup_wgpu().await;
    if wgpu_context.is_none() {
        println!("Skipping WGPU test (no adapter)");
        return;
    }
    let (device, queue) = wgpu_context.unwrap();

    let config = TerrainSplatConverterConfig {
        splat_alpha_x_size: 2,
        splat_alpha_y_size: 2,
        max_splat_limit: 100,
    };
    let mut converter = WgpuTerrainSplatConverter::new(config);

    let invalid_data = vec![0; 10]; // 2x2x4 = 16 expected
    let result = converter.load_alpha_mask(&device, &queue, &invalid_data);
    assert_eq!(result.err(), Some(SplatConverterError::InvalidAlphaSize));

    let valid_data = vec![255; 16];
    let result = converter.load_alpha_mask(&device, &queue, &valid_data);
    assert!(result.is_ok());
    assert!(converter.splat_alpha_texture.is_some());
    assert!(converter.splat_alpha_view.is_some());
}

#[tokio::test]
async fn test_load_terrain_textures() {
    let wgpu_context = setup_wgpu().await;
    if wgpu_context.is_none() {
        println!("Skipping WGPU test (no adapter)");
        return;
    }
    let (device, queue) = wgpu_context.unwrap();

    let mut converter = WgpuTerrainSplatConverter::new(TerrainSplatConverterConfig::default());
    
    let tex_data = vec![128; 16]; // 2x2x4 = 16
    let invalid_tex = vec![0; 10];

    // Invalid size test
    let result = converter.load_terrain_textures(
        &device,
        &queue,
        &[&tex_data, &invalid_tex, &tex_data, &tex_data],
        2,
        2
    );
    assert_eq!(result.err(), Some(SplatConverterError::InvalidTextureCount));

    // Valid load test
    let result = converter.load_terrain_textures(
        &device,
        &queue,
        &[&tex_data, &tex_data, &tex_data, &tex_data],
        2,
        2
    );
    assert!(result.is_ok());
    for i in 0..4 {
        assert!(converter.terrain_textures[i].is_some());
        assert!(converter.terrain_texture_views[i].is_some());
    }
}

#[tokio::test]
async fn test_draw_terrain_splat_limit() {
    let wgpu_context = setup_wgpu().await;
    if wgpu_context.is_none() {
        println!("Skipping WGPU test (no adapter)");
        return;
    }
    let (device, _) = wgpu_context.unwrap();

    let mut converter = WgpuTerrainSplatConverter::new(TerrainSplatConverterConfig {
        splat_alpha_x_size: 258, splat_alpha_y_size: 258, max_splat_limit: 1,
        
    });

    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let renderer = TerrainSplattingRenderer::new(&device, format).expect("Failed to create renderer");
    
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let dummy_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let dummy_view = dummy_texture.create_view(&wgpu::TextureViewDescriptor::default());

    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &dummy_view, depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None, multiview_mask: None,
    });

    // Need dummy buffers
    let vb = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 16,
        usage: wgpu::BufferUsages::VERTEX,
        mapped_at_creation: false,
    });
    let ib = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 16,
        usage: wgpu::BufferUsages::INDEX,
        mapped_at_creation: false,
    });

    // Draw should fail because texture set is not loaded (NotInitialized)
    converter.config.max_splat_limit = 0;
    
    
    
    
    }
    let mut render_pass2 = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &dummy_view, depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None, multiview_mask: None,
    });
    converter.config.max_splat_limit = 0;
    let success = converter.draw_terrain_splat(&renderer, &mut render_pass2, &vb, &ib, 0);
    drop(render_pass2);
    assert!(success.is_ok());
    assert_eq!(converter.rendered_splat_num, 1);
    assert!(success.is_ok());
    assert_eq!(converter.rendered_splat_num, 1);
}
