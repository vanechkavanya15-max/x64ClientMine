use m2_client_image::pixel_format::*;

#[test]
fn test_pixel_format_properties() {
    assert_eq!(PixelFormat::RGBA8.bytes_per_pixel(), Some(4));
    assert_eq!(PixelFormat::BGRA8.bytes_per_pixel(), Some(4));
    assert_eq!(PixelFormat::RGB565.bytes_per_pixel(), Some(2));
    assert_eq!(PixelFormat::ARGB1555.bytes_per_pixel(), Some(2));
    assert_eq!(PixelFormat::DXT1.bytes_per_pixel(), None);

    assert_eq!(PixelFormat::RGBA8.is_compressed(), false);
    assert_eq!(PixelFormat::DXT1.is_compressed(), true);
    assert_eq!(PixelFormat::DXT3.is_compressed(), true);
    assert_eq!(PixelFormat::DXT5.is_compressed(), true);

    assert_eq!(PixelFormat::DXT1.block_size_bytes(), Some(8));
    assert_eq!(PixelFormat::DXT3.block_size_bytes(), Some(16));
    assert_eq!(PixelFormat::DXT5.block_size_bytes(), Some(16));
    assert_eq!(PixelFormat::RGBA8.block_size_bytes(), None);
}

#[test]
fn test_rgba8_to_bgra8_pixel() {
    let (b, g, r, a) = rgba8_to_bgra8(255, 128, 64, 255);
    assert_eq!(r, 255);
    assert_eq!(g, 128);
    assert_eq!(b, 64);
    assert_eq!(a, 255);
}

#[test]
fn test_rgba8_to_rgb565_pixel() {
    let rgb565 = rgba8_to_rgb565(255, 255, 255);
    assert_eq!(rgb565, 0xFFFF);
    
    let rgb565_black = rgba8_to_rgb565(0, 0, 0);
    assert_eq!(rgb565_black, 0x0000);
}

#[test]
fn test_rgba8_to_argb1555_pixel() {
    let argb1555 = rgba8_to_argb1555(255, 255, 255, 255);
    assert_eq!(argb1555, 0xFFFF);

    let argb1555_transp = rgba8_to_argb1555(255, 255, 255, 0);
    assert_eq!(argb1555_transp, 0x7FFF);
    
    let argb1555_black = rgba8_to_argb1555(0, 0, 0, 255);
    assert_eq!(argb1555_black, 0x8000);
}

#[test]
fn test_rgb565_to_rgba8_pixel() {
    let (r, g, b, a) = rgb565_to_rgba8(0xFFFF);
    assert_eq!(r, 255);
    assert_eq!(g, 255);
    assert_eq!(b, 255);
    assert_eq!(a, 255);
    
    let (r2, g2, b2, a2) = rgb565_to_rgba8(0x0000);
    assert_eq!(r2, 0);
    assert_eq!(g2, 0);
    assert_eq!(b2, 0);
    assert_eq!(a2, 255);
}

#[test]
fn test_argb1555_to_rgba8_pixel() {
    let (r, g, b, a) = argb1555_to_rgba8(0xFFFF);
    assert_eq!(r, 255);
    assert_eq!(g, 255);
    assert_eq!(b, 255);
    assert_eq!(a, 255);
    
    let (r2, g2, b2, a2) = argb1555_to_rgba8(0x7FFF);
    assert_eq!(r2, 255);
    assert_eq!(g2, 255);
    assert_eq!(b2, 255);
    assert_eq!(a2, 0);
}

#[test]
fn test_convert_buffer_rgba8_to_bgra8() {
    let src = [255, 0, 0, 255, 0, 255, 0, 128];
    let mut dst = [0; 8];
    
    let result = convert_buffer_rgba8_to_bgra8(&src, &mut dst);
    assert!(result.is_ok());
    assert_eq!(dst, [0, 0, 255, 255, 0, 255, 0, 128]);
}

#[test]
fn test_convert_buffer_rgba8_to_bgra8_errors() {
    let src = [255, 0, 0, 255, 0, 255, 0]; // 7 bajtow
    let mut dst = [0; 7];
    
    let result = convert_buffer_rgba8_to_bgra8(&src, &mut dst);
    assert!(result.is_err());
    
    let src2 = [255, 0, 0, 255];
    let mut dst2 = [0; 8];
    let result2 = convert_buffer_rgba8_to_bgra8(&src2, &mut dst2);
    assert!(result2.is_err());
}

#[test]
fn test_convert_buffer_rgba8_to_rgb565() {
    let src = [255, 255, 255, 255, 0, 0, 0, 255];
    let mut dst = [0; 4];
    
    let result = convert_buffer_rgba8_to_rgb565(&src, &mut dst);
    assert!(result.is_ok());
    // biale 0xFFFF (w pamieci 0xFF, 0xFF) oraz czarne 0x0000 (0x00, 0x00)
    assert_eq!(dst, [255, 255, 0, 0]);
}

#[test]
fn test_convert_buffer_rgba8_to_argb1555() {
    let src = [255, 255, 255, 255, 0, 0, 0, 0];
    let mut dst = [0; 4];
    
    let result = convert_buffer_rgba8_to_argb1555(&src, &mut dst);
    assert!(result.is_ok());
    // 0xFFFF oraz 0x0000
    assert_eq!(dst, [255, 255, 0, 0]);
}

#[test]
fn test_convert_buffer_rgb565_to_rgba8() {
    let src = [255, 255, 0, 0]; // 0xFFFF i 0x0000
    let mut dst = [0; 8];
    
    let result = convert_buffer_rgb565_to_rgba8(&src, &mut dst);
    assert!(result.is_ok());
    assert_eq!(dst, [255, 255, 255, 255, 0, 0, 0, 255]);
}

#[test]
fn test_convert_buffer_argb1555_to_rgba8() {
    let src = [255, 255, 255, 127]; // 0xFFFF (biale) i 0x7FFF (biale transparentne)
    let mut dst = [0; 8];
    
    let result = convert_buffer_argb1555_to_rgba8(&src, &mut dst);
    assert!(result.is_ok());
    assert_eq!(dst, [255, 255, 255, 255, 255, 255, 255, 0]);
}
