use m2_client::guild_mark_symbol_cache_manager::*;

#[test]
fn test_guild_mark_rgb565_parsing() {
    let mut data = vec![0u8; MARK_SIZE * 2];
    // Red color in RGB565: 11111 000000 00000 -> 0xF800
    // Little endian: 0x00, 0xF8
    data[0] = 0x00;
    data[1] = 0xF8;

    let mark = GuildMark::from_rgb565(&data).expect("Failed to parse RGB565");
    // val = 0xF800
    // r = (0xF800 >> 11) & 0x1F = 31 -> 31 * 255 / 31 = 255
    // g = (0xF800 >> 5) & 0x3F = 0 -> 0 * 255 / 63 = 0
    // b = 0xF800 & 0x1F = 0 -> 0 * 255 / 31 = 0
    // a = 255
    // Our logic is: mark.pixels[i] = (a << 24) | (b << 16) | (g << 8) | r;
    // So 255 << 24 | 255
    let expected = (255u32 << 24) | 255u32;
    assert_eq!(mark.pixels[0], expected);
    assert_eq!(mark.pixels[1], 0); // Other pixels should be black
}

#[test]
fn test_guild_mark_rgba_parsing() {
    let mut data = vec![0u8; MARK_SIZE * 4];
    // Red pixel in RGBA
    data[0] = 255; // R
    data[1] = 0;   // G
    data[2] = 0;   // B
    data[3] = 255; // A

    let mark = GuildMark::from_rgba(&data).expect("Failed to parse RGBA");
    let expected = (255u32 << 24) | 255u32;
    assert_eq!(mark.pixels[0], expected);
    assert_eq!(mark.pixels[1], 0);
}

#[test]
fn test_guild_mark_image_pack_and_retrieve() {
    let mut image = GuildMarkImage::new();
    let mut mark = GuildMark::new();
    let expected = (255u32 << 24) | 255u32;
    // Fill mark with red pixels
    for i in 0..MARK_SIZE {
        mark.pixels[i] = expected; // Red color
    }

    let pos_mark = 5; // Put mark at position 5
    assert!(image.put_mark(pos_mark, &mark));

    let retrieved_mark = image.get_mark(pos_mark).expect("Failed to retrieve mark");
    assert_eq!(retrieved_mark.pixels[0], expected);
    assert_eq!(retrieved_mark.pixels[MARK_SIZE - 1], expected);

    // Get an empty mark
    let empty_mark = image.get_mark(6).expect("Failed to retrieve empty mark");
    assert!(empty_mark.is_empty());

    // Delete the mark
    assert!(image.delete_mark(pos_mark));
    let retrieved_after_delete = image.get_mark(pos_mark).expect("Failed to retrieve mark after delete");
    assert!(retrieved_after_delete.is_empty());
}

#[test]
fn test_guild_mark_symbol_cache_manager() {
    let mut manager = GuildMarkSymbolCacheManager::new();

    let guild_id = 1234;
    let mut mark = GuildMark::new();
    let expected = (255u32 << 24) | 255u32;
    mark.pixels[0] = expected;

    // Save the mark
    let mark_id = manager.save_mark(guild_id, &mark);
    assert_ne!(mark_id, INVALID_MARK_ID);

    // Retrieve the mark
    let retrieved_mark = manager.get_mark(guild_id).expect("Failed to retrieve mark");
    assert_eq!(retrieved_mark.pixels[0], expected);

    assert_eq!(manager.get_mark_count(), 1);
    assert_eq!(manager.get_mark_image_count(), 1);

    // Delete the mark
    manager.delete_mark(guild_id);
    assert!(manager.get_mark(guild_id).is_none());
    assert_eq!(manager.get_mark_count(), 0);

    // Test symbol upload
    let symbol_data = vec![1, 2, 3, 4, 5];
    manager.upload_symbol(guild_id, &symbol_data);
    let symbol = manager.get_guild_symbol(guild_id).expect("Failed to retrieve symbol");
    assert_eq!(symbol.raw, symbol_data);
    // basic crc check, shouldn't be 0
    assert_ne!(symbol.crc, 0); 
}
