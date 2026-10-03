use m2_client::guild_mark_atlas_loader::{GuildMarkAtlas, GuildMarkAtlasLoader, GuildMarkError};
use m2_client::vfs_pck_memory_mapped::write_pck_archive;

// Creates a simple 16x12 TGA image entirely in red
fn create_dummy_16x12_tga() -> Vec<u8> {
    let mut data = vec![0; 18];
    // TGA Header for uncompressed true-color
    data[2] = 2; // Image type: 2 (Truecolor)
    data[12] = 16; // Width LSB
    data[13] = 0;  // Width MSB
    data[14] = 12; // Height LSB
    data[15] = 0;  // Height MSB
    data[16] = 32; // 32 bpp
    data[17] = 8;  // Descriptor

    for _ in 0..(16 * 12) {
        data.push(0);   // B
        data.push(0);   // G
        data.push(255); // R
        data.push(255); // A
    }
    data
}

// Creates a simple 10x10 TGA image (invalid dimensions)
fn create_dummy_10x10_tga() -> Vec<u8> {
    let mut data = vec![0; 18];
    data[2] = 2; // Image type: 2 (Truecolor)
    data[12] = 10;
    data[13] = 0;
    data[14] = 10;
    data[15] = 0;
    data[16] = 32;
    data[17] = 8;

    for _ in 0..(10 * 10) {
        data.push(0);
        data.push(0);
        data.push(255);
        data.push(255);
    }
    data
}

// Helper to create a PCK archive on disk
fn create_dummy_pck(path: &str, files: &[(&str, &[u8])]) {
    let mut pck_files = Vec::new();
    for &(name, data) in files {
        // false for no encryption flag
        pck_files.push((name, data, false));
    }
    write_pck_archive(&pck_files, path).unwrap();
}

#[test]
fn test_atlas_creation_and_add_mark() {
    let mut atlas = GuildMarkAtlas::new(32, 12);
    
    // Create a 16x12 RGBA mark manually (red)
    let mark_rgba = vec![255; 16 * 12 * 4];

    let result = atlas.add_mark(&mark_rgba);
    assert_eq!(result, Ok((0, 0)));
    assert_eq!(atlas.next_x, 16);
    assert_eq!(atlas.next_y, 0);

    // Verify some pixels are set correctly
    // The very first pixel
    assert_eq!(atlas.rgba_data[0], 255);
    assert_eq!(atlas.rgba_data[1], 255);
    assert_eq!(atlas.rgba_data[2], 255);
    assert_eq!(atlas.rgba_data[3], 255);
}

#[test]
fn test_atlas_full() {
    // Atlas can only hold two marks
    let mut atlas = GuildMarkAtlas::new(32, 12);
    let mark_rgba = vec![255; 16 * 12 * 4];

    let result1 = atlas.add_mark(&mark_rgba);
    assert_eq!(result1, Ok((0, 0)));

    let result2 = atlas.add_mark(&mark_rgba);
    assert_eq!(result2, Ok((16, 0)));

    // Next one should wrap to next line, but height is only 12, so it's full
    let result3 = atlas.add_mark(&mark_rgba);
    assert_eq!(result3, Err(GuildMarkError::AtlasFull));
}

#[test]
fn test_pck_archive_mount_error() {
    let atlas = GuildMarkAtlas::new(256, 256);
    let mut loader = GuildMarkAtlasLoader::new(atlas);

    let result = loader.load_from_pck("non_existent_archive.pck", "some_file.tga");
    match result {
        Err(GuildMarkError::ArchiveMountError(_)) => (),
        _ => panic!("Expected ArchiveMountError"),
    }
}

#[test]
fn test_pck_file_not_found() {
    let pck_path = "test_file_not_found.pck";
    create_dummy_pck(pck_path, &[("dummy.txt", b"hello")]);

    let atlas = GuildMarkAtlas::new(256, 256);
    let mut loader = GuildMarkAtlasLoader::new(atlas);

    let result = loader.load_from_pck(pck_path, "missing_mark.tga");
    assert_eq!(result, Err(GuildMarkError::FileNotFound));

    // Cleanup
    let _ = std::fs::remove_file(pck_path);
}

#[test]
fn test_invalid_image_dimensions() {
    let pck_path = "test_invalid_dim.pck";
    let tga_data = create_dummy_10x10_tga();
    create_dummy_pck(pck_path, &[("bad_mark.tga", &tga_data)]);

    let atlas = GuildMarkAtlas::new(256, 256);
    let mut loader = GuildMarkAtlasLoader::new(atlas);

    let result = loader.load_from_pck(pck_path, "bad_mark.tga");
    assert_eq!(result, Err(GuildMarkError::InvalidDimensions(10, 10)));

    // Cleanup
    let _ = std::fs::remove_file(pck_path);
}

#[test]
fn test_multiple_rows_wrapping() {
    let mut atlas = GuildMarkAtlas::new(32, 24); // holds 2 columns, 2 rows = 4 marks
    let mark_rgba = vec![128; 16 * 12 * 4];

    // Mark 1
    assert_eq!(atlas.add_mark(&mark_rgba), Ok((0, 0)));
    
    // Mark 2
    assert_eq!(atlas.add_mark(&mark_rgba), Ok((16, 0)));
    
    // Mark 3 (wraps to next row)
    assert_eq!(atlas.add_mark(&mark_rgba), Ok((0, 12)));
    
    // Mark 4
    assert_eq!(atlas.add_mark(&mark_rgba), Ok((16, 12)));
    
    // Mark 5 (fails, atlas full)
    assert_eq!(atlas.add_mark(&mark_rgba), Err(GuildMarkError::AtlasFull));
}

#[test]
fn test_successful_load_from_pck() {
    let pck_path = "test_success.pck";
    let tga_data = create_dummy_16x12_tga();
    create_dummy_pck(pck_path, &[("mark_01.tga", &tga_data)]);

    let atlas = GuildMarkAtlas::new(256, 256);
    let mut loader = GuildMarkAtlasLoader::new(atlas);

    let result = loader.load_from_pck(pck_path, "mark_01.tga");
    assert_eq!(result, Ok((0, 0)));

    // Cleanup
    let _ = std::fs::remove_file(pck_path);
}
