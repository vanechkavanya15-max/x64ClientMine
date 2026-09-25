use m2_client_render::render_target::{RenderTarget, RenderTargetDescriptor, TextureFormat};

#[test]
fn test_default_render_target() {
    let instance = RenderTarget::default();
    assert!(instance.is_ready());
    assert_eq!(instance.descriptor.width, 800);
    assert_eq!(instance.descriptor.height, 600);
    assert_eq!(instance.descriptor.color_format, TextureFormat::Rgba8Unorm);
    assert_eq!(
        instance.descriptor.depth_stencil_format,
        TextureFormat::Depth24PlusStencil8
    );
    assert_eq!(instance.descriptor.sample_count, 1);
    assert!(instance.has_depth_stencil());
    assert!(!instance.is_msaa_enabled());
}

#[test]
fn test_custom_render_target() {
    let descriptor = RenderTargetDescriptor {
        width: 1920,
        height: 1080,
        color_format: TextureFormat::Rgba8UnormSrgb,
        depth_stencil_format: TextureFormat::None,
        sample_count: 4,
    };
    let instance = RenderTarget::new(descriptor);
    assert!(instance.is_ready());
    assert_eq!(instance.descriptor.width, 1920);
    assert_eq!(instance.descriptor.height, 1080);
    assert_eq!(
        instance.descriptor.color_format,
        TextureFormat::Rgba8UnormSrgb
    );
    assert_eq!(
        instance.descriptor.depth_stencil_format,
        TextureFormat::None
    );
    assert_eq!(instance.descriptor.sample_count, 4);
    assert!(!instance.has_depth_stencil());
    assert!(instance.is_msaa_enabled());
}

#[test]
fn test_render_target_resize() {
    let mut instance = RenderTarget::default();
    instance.resize(1024, 768);
    assert_eq!(instance.descriptor.width, 1024);
    assert_eq!(instance.descriptor.height, 768);
    
    // Test ignoring zero dimensions
    instance.resize(0, 0);
    assert_eq!(instance.descriptor.width, 1024);
    assert_eq!(instance.descriptor.height, 768);
    
    instance.resize(0, 500);
    assert_eq!(instance.descriptor.width, 1024);
    assert_eq!(instance.descriptor.height, 768);

    instance.resize(800, 0);
    assert_eq!(instance.descriptor.width, 1024);
    assert_eq!(instance.descriptor.height, 768);
}

#[test]
fn test_render_target_sample_count() {
    let mut instance = RenderTarget::default();
    assert!(!instance.is_msaa_enabled());
    
    instance.set_sample_count(4);
    assert_eq!(instance.descriptor.sample_count, 4);
    assert!(instance.is_msaa_enabled());
    
    // Test ignoring zero sample count
    instance.set_sample_count(0);
    assert_eq!(instance.descriptor.sample_count, 4);
    assert!(instance.is_msaa_enabled());
}
