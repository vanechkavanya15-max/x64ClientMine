use m2_client::d3d9_texture_factory::{
    create_texture_from_memory, lock_texture_rect, release_texture, TextureData, TextureFormat, TextureHandle,
};

#[test]
fn test_create_and_release_texture() {
    let width = 4;
    let height = 4;
    let format = TextureFormat::ARGB8888;
    let bpp = 4;
    let raw_bytes = vec![255u8; width * height * bpp];

    let tex_data = TextureData {
        width: width as u32,
        height: height as u32,
        format,
        mip_levels: 1,
        raw_bytes,
    };

    let handle = create_texture_from_memory(&tex_data).expect("Failed to create texture");
    
    // Verify it's locked and readable
    {
        let locked_rect = lock_texture_rect(handle, 0).expect("Failed to lock texture rect");
        assert_eq!(locked_rect.len(), width * height * bpp);
        assert_eq!(locked_rect[0], 255);
    }
    
    release_texture(handle);

    // Verify it's released
    assert!(lock_texture_rect(handle, 0).is_none());
}

#[test]
fn test_dxt1_texture_creation() {
    let width = 4;
    let height = 4;
    let format = TextureFormat::DXT1;
    let block_size = 8;
    // 1 block of 4x4
    let raw_bytes = vec![0u8; block_size];

    let tex_data = TextureData {
        width: width as u32,
        height: height as u32,
        format,
        mip_levels: 1,
        raw_bytes,
    };

    let handle = create_texture_from_memory(&tex_data).expect("Failed to create DXT1 texture");

    {
        let locked_rect = lock_texture_rect(handle, 0).expect("Failed to lock DXT1 texture rect");
        // DXT1 decoder outputs RGBA buffer (width * height * 4)
        assert_eq!(locked_rect.len(), width * height * 4);
    }

    release_texture(handle);
}

#[test]
fn test_dxt3_texture_creation() {
    let width = 4;
    let height = 4;
    let format = TextureFormat::DXT3;
    let block_size = 16;
    // 1 block of 4x4
    let raw_bytes = vec![0u8; block_size];

    let tex_data = TextureData {
        width: width as u32,
        height: height as u32,
        format,
        mip_levels: 1,
        raw_bytes,
    };

    let handle = create_texture_from_memory(&tex_data).expect("Failed to create DXT3 texture");

    {
        let locked_rect = lock_texture_rect(handle, 0).expect("Failed to lock DXT3 texture rect");
        // Decoder outputs RGBA buffer
        assert_eq!(locked_rect.len(), width * height * 4);
    }

    release_texture(handle);
}

#[test]
fn test_dxt5_texture_creation() {
    let width = 4;
    let height = 4;
    let format = TextureFormat::DXT5;
    let block_size = 16;
    // 1 block of 4x4
    let raw_bytes = vec![0u8; block_size];

    let tex_data = TextureData {
        width: width as u32,
        height: height as u32,
        format,
        mip_levels: 1,
        raw_bytes,
    };

    let handle = create_texture_from_memory(&tex_data).expect("Failed to create DXT5 texture");

    {
        let locked_rect = lock_texture_rect(handle, 0).expect("Failed to lock DXT5 texture rect");
        // Decoder outputs RGBA buffer
        assert_eq!(locked_rect.len(), width * height * 4);
    }

    release_texture(handle);
}

#[test]
fn test_texture_mipmaps() {
    let width = 8; // Mip 0: 8x8, Mip 1: 4x4, Mip 2: 2x2, Mip 3: 1x1
    let height = 8;
    let format = TextureFormat::ARGB8888;
    
    // Calculate required bytes for all 4 mips
    let mip0 = 8 * 8 * 4;
    let mip1 = 4 * 4 * 4;
    let mip2 = 2 * 2 * 4;
    let mip3 = 1 * 1 * 4;
    
    let total_bytes = mip0 + mip1 + mip2 + mip3;
    let mut raw_bytes = vec![0u8; total_bytes];

    // Give mip 1 a specific color to verify
    for i in 0..mip1 {
        raw_bytes[mip0 + i] = 128;
    }

    let tex_data = TextureData {
        width: width as u32,
        height: height as u32,
        format,
        mip_levels: 4,
        raw_bytes,
    };

    let handle = create_texture_from_memory(&tex_data).expect("Failed to create mipmapped texture");

    {
        // Check mip 0 size
        let mip0_rect = lock_texture_rect(handle, 0).unwrap();
        assert_eq!(mip0_rect.len(), mip0);

        // Check mip 1 size and content
        let mip1_rect = lock_texture_rect(handle, 1).unwrap();
        assert_eq!(mip1_rect.len(), mip1);
        assert_eq!(mip1_rect[0], 128);

        // Check mip 3 size
        let mip3_rect = lock_texture_rect(handle, 3).unwrap();
        assert_eq!(mip3_rect.len(), mip3);
    }

    release_texture(handle);
}

#[test]
fn test_invalid_dimensions() {
    let tex_data = TextureData {
        width: 0,
        height: 10,
        format: TextureFormat::ARGB8888,
        mip_levels: 1,
        raw_bytes: vec![],
    };

    let result = create_texture_from_memory(&tex_data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid texture dimensions");
}

#[test]
fn test_not_enough_data() {
    let width = 4;
    let height = 4;
    let format = TextureFormat::ARGB8888;
    // Require 4 * 4 * 4 = 64 bytes, but provide only 32
    let raw_bytes = vec![0u8; 32];

    let tex_data = TextureData {
        width: width as u32,
        height: height as u32,
        format,
        mip_levels: 1,
        raw_bytes,
    };

    let result = create_texture_from_memory(&tex_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Not enough raw data"));
}

#[test]
fn test_tga_texture_loading() {
    // Construct a minimal uncompressed TGA header and some pixels
    // 18 bytes header + 2x2 pixels (32bpp) = 18 + 16 = 34 bytes
    let mut raw_bytes = vec![0u8; 34];
    raw_bytes[2] = 2; // uncompressed RGB
    raw_bytes[12] = 2; // width low
    raw_bytes[13] = 0; // width high
    raw_bytes[14] = 2; // height low
    raw_bytes[15] = 0; // height high
    raw_bytes[16] = 32; // bpp
    raw_bytes[17] = 1 << 5; // top-to-bottom

    let tex_data = TextureData {
        width: 2,
        height: 2,
        format: TextureFormat::ARGB8888,
        mip_levels: 1,
        raw_bytes,
    };

    let handle = create_texture_from_memory(&tex_data).expect("Failed to create TGA texture");

    {
        let locked_rect = lock_texture_rect(handle, 0).expect("Failed to lock TGA texture rect");
        assert_eq!(locked_rect.len(), 2 * 2 * 4);
    }

    release_texture(handle);
}
