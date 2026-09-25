use m2_client::software_framebuffer::{SoftwareFramebuffer, DeviceState, create_return_values};

#[test]
fn test_integration_software_framebuffer() {
    let instance = SoftwareFramebuffer::new();
    assert!(instance.is_ready());
    assert_eq!(instance.device_state, DeviceState::Null);
    assert_eq!(instance.back_buffer_count, 0);
    assert_eq!(instance.diffuse_color, 0xffffffff);
    assert_eq!(instance.clear_color, 0);
    assert_eq!(instance.clear_stencil, 0);
    assert_eq!(instance.clear_depth, 1.0);
}

#[test]
fn test_device_state_transitions() {
    let mut instance = SoftwareFramebuffer::default();
    assert_eq!(instance.get_device_state(), DeviceState::Null);
    assert!(!instance.is_ready());

    let ret = instance.create(0, 800, 600, true, 32, 60);
    assert_eq!(ret, create_return_values::OK);
    assert_eq!(instance.get_device_state(), DeviceState::Ok);
    assert!(instance.is_ready());
    assert_eq!(instance.width, 800);
    assert_eq!(instance.height, 600);
    assert_eq!(instance.back_buffer.len(), 800 * 600);

    instance.destroy();
    assert_eq!(instance.get_device_state(), DeviceState::Null);
    assert!(!instance.is_ready());
    assert!(instance.back_buffer.is_empty());
}

#[test]
fn test_clear_colors_configuration() {
    let mut instance = SoftwareFramebuffer::new();
    
    instance.set_diffuse_color(1.0, 0.0, 0.0, 1.0);
    assert_eq!(instance.diffuse_color, u32::from_le_bytes([0, 0, 255, 255]));

    instance.set_clear_color(0.0, 1.0, 0.0, 1.0);
    assert_eq!(instance.clear_color, u32::from_le_bytes([0, 255, 0, 255]));

    instance.set_clear_depth(0.5);
    assert_eq!(instance.clear_depth, 0.5);

    instance.set_clear_stencil(42);
    assert_eq!(instance.clear_stencil, 42);
}

#[test]
fn test_restore_device() {
    let mut instance = SoftwareFramebuffer::new();
    assert!(instance.is_lost_device());
    
    assert!(instance.restore_device());
    assert_eq!(instance.get_device_state(), DeviceState::Ok);
    assert!(!instance.is_lost_device());
}

#[test]
fn test_cursor_position() {
    let instance = SoftwareFramebuffer::new();
    let mut px = 10.0;
    let mut py = 10.0;
    let mut pz = 10.0;
    
    let res = instance.get_cursor_position(&mut px, &mut py, &mut pz);
    assert!(res);
    assert_eq!(px, 0.0);
    assert_eq!(py, 0.0);
    assert_eq!(pz, 0.0);
}

#[test]
fn test_software_rasterizer_draw_line() {
    let mut instance = SoftwareFramebuffer::new();
    instance.create(0, 100, 100, true, 32, 60); // Mala rozdzielczosc dla szybkiego testu
    instance.clear();

    instance.set_diffuse_color_u32(0xff0000ff); // Czerwony AARRGGBB? Zobaczymy co wyjdzie
    let color = instance.diffuse_color;

    // Pozioma linia
    instance.render_line_2d(10.0, 10.0, 20.0, 10.0, 0.5);

    let idx1 = (10 * 100 + 10) as usize;
    let idx2 = (10 * 100 + 15) as usize;
    let idx3 = (10 * 100 + 20) as usize;
    let idx_empty = (11 * 100 + 15) as usize;

    assert_eq!(instance.back_buffer[idx1], color);
    assert_eq!(instance.back_buffer[idx2], color);
    assert_eq!(instance.back_buffer[idx3], color);
    assert_eq!(instance.back_buffer[idx_empty], 0); // Sprawdzamy czy poza linia jest czysto
}

#[test]
fn test_software_rasterizer_draw_bar() {
    let mut instance = SoftwareFramebuffer::new();
    instance.create(0, 10, 10, true, 32, 60);
    instance.clear();

    instance.set_diffuse_color_u32(0xff00ff00);
    let color = instance.diffuse_color;

    // Rysowanie wypelnionego prostokata na calym ekranie
    instance.render_bar_2d(0.0, 0.0, 9.0, 9.0, 0.5);

    assert_eq!(instance.back_buffer[0], color); // lewy gorny
    assert_eq!(instance.back_buffer[99], color); // prawy dolny
    assert_eq!(instance.back_buffer[55], color); // srodek
}
