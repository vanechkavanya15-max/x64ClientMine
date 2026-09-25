use m2_client::software_framebuffer::SoftwareFramebuffer;

#[test]
fn test_integration_software_framebuffer() {
    let instance = SoftwareFramebuffer::new();
    assert!(instance.is_ready());
}
