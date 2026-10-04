use m2_client::vfs_epk_eix_archive_reader::{
    load_eix_index, read_epk_file_data, TEterPackIndex, COMPRESSED_TYPE_NONE, COMPRESSED_TYPE_COMPRESS,
    EPKD_FOURCC, MCOZ_FOURCC,
};
use m2_client::vfs_security_decrypt::SecurityKey;

#[test]
fn test_eix_header_invalid() {
    let bad_data = vec![0x12, 0x34, 0x56, 0x78, 0, 0, 0, 0, 0, 0, 0, 0];
    let key: SecurityKey = [0, 0, 0, 0];
    let res = load_eix_index(&bad_data, &key);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Invalid FourCC"));
}

#[test]
fn test_epk_read_out_of_bounds() {
    let epk_data = b"Short";
    let mut index: TEterPackIndex = unsafe { std::mem::zeroed() };
    index.data_position = 2;
    index.data_size = 10;
    index.real_data_size = 10;
    index.compressed_type = COMPRESSED_TYPE_NONE;

    let res = read_epk_file_data(epk_data, &index);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Data out of bounds in EPK"));
}

#[test]
fn test_epk_uncompressed_read() {
    let epk_data = b"Hello, World! Here is some extra uncompressed data to test block reads.";
    let mut index: TEterPackIndex = unsafe { std::mem::zeroed() };
    index.data_position = 7;
    index.real_data_size = 5; // decompressed size
    index.data_size = 5;      // compressed size
    index.compressed_type = COMPRESSED_TYPE_NONE;

    let res = read_epk_file_data(epk_data, &index).unwrap();
    assert_eq!(res, b"World");
}

#[test]
fn test_epk_lzo_compressed_read() {
    // A small valid LZO block
    // LZO1X format: 
    // 17 + length of raw bytes
    // raw bytes
    let msg = b"World";
    let len_byte = 17 + msg.len() as u8;
    let mut epk_data = vec![0, 0, 0, 0, 0, 0, 0];
    epk_data.push(len_byte);
    epk_data.extend_from_slice(msg);
    // Add some trailing padding just to mimic a larger file
    epk_data.extend_from_slice(b"TRAILI");

    let mut index: TEterPackIndex = unsafe { std::mem::zeroed() };
    index.data_position = 7;
    index.data_size = 1 + msg.len() as i32;      // length of the LZO compressed slice
    index.real_data_size = msg.len() as i32;         // final decompressed size
    index.compressed_type = COMPRESSED_TYPE_COMPRESS;

    let res = read_epk_file_data(&epk_data, &index).unwrap();
    assert_eq!(res, b"World");
}

#[test]
fn test_eix_parsing_success() {
    let key: SecurityKey = [173217, 72619434, 408587239, 27973291];
    
    // Create an unencrypted MCOZ block
    let mut mcoz_block = Vec::new();
    mcoz_block.extend_from_slice(&MCOZ_FOURCC.to_le_bytes()); // FourCC
    mcoz_block.extend_from_slice(&0u32.to_le_bytes()); // encrypt_size = 0 (unencrypted)
    
    // One TEterPackIndex is 192 bytes
    let stride = std::mem::size_of::<TEterPackIndex>();
    
    let mut mock_index: TEterPackIndex = unsafe { std::mem::zeroed() };
    let filename = b"test/file.txt";
    mock_index.filename[..filename.len()].copy_from_slice(filename);
    mock_index.id = 42;
    mock_index.real_data_size = 100;
    mock_index.data_size = 200;
    mock_index.data_position = 1024;
    mock_index.compressed_type = COMPRESSED_TYPE_COMPRESS;
    
    let mut index_bytes = vec![0u8; stride];
    unsafe {
        std::ptr::copy_nonoverlapping(
            &mock_index as *const TEterPackIndex as *const u8,
            index_bytes.as_mut_ptr(),
            stride,
        );
    }
    
    // LZO compress the index_bytes (mocked as uncompressed LZO payload: 17+len, bytes)
    // Actually LZO limits raw match to 238 bytes with `17 + len`. Max len = 255-17 = 238.
    // So 17 + 192 = 209, which is valid.
    let mut lzo_compressed = vec![17 + stride as u8];
    lzo_compressed.extend_from_slice(&index_bytes);
    
    mcoz_block.extend_from_slice(&(lzo_compressed.len() as u32).to_le_bytes()); // compressed_size
    mcoz_block.extend_from_slice(&(stride as u32).to_le_bytes()); // real_size
    
    // Padding to 16 bytes for header
    mcoz_block.extend_from_slice(&lzo_compressed);
    
    // Wrap in EPKD header
    let mut eix_data = Vec::new();
    eix_data.extend_from_slice(&EPKD_FOURCC.to_le_bytes()); // EPKD
    eix_data.extend_from_slice(&2u32.to_le_bytes()); // version
    eix_data.extend_from_slice(&1u32.to_le_bytes()); // index_count
    eix_data.extend_from_slice(&mcoz_block);
    
    let res = load_eix_index(&eix_data, &key).unwrap();
    assert_eq!(res.len(), 1);
    
    let parsed_index = res.get("test/file.txt").unwrap();
    assert_eq!(parsed_index.id, 42);
    assert_eq!(parsed_index.real_data_size, 100);
    assert_eq!(parsed_index.data_size, 200);
    assert_eq!(parsed_index.data_position, 1024);
    assert_eq!(parsed_index.compressed_type, COMPRESSED_TYPE_COMPRESS);
}

#[test]
fn test_epk_eix_e2e_text_file_unpack() {
    let key: SecurityKey = [0x11223344, 0x55667788, 0x99AABBCC, 0xDDEEFF00];
    
    // 1. Create a mocked EPK file with a text payload
    let text_payload = b"This is a mocked configuration file for testing.";
    
    // Using uncompressed LZO payload simulation (17 + length, data)
    let len_byte = 17 + text_payload.len() as u8;
    let mut epk_data = vec![len_byte];
    epk_data.extend_from_slice(text_payload);
    
    // 2. Create a mocked EIX index corresponding to the EPK
    let stride = std::mem::size_of::<TEterPackIndex>();
    let mut mock_index: TEterPackIndex = unsafe { std::mem::zeroed() };
    let filename = b"config/settings.txt";
    mock_index.filename[..filename.len()].copy_from_slice(filename);
    mock_index.id = 1;
    mock_index.data_position = 0; // Starts at 0 in our mocked EPK
    mock_index.data_size = epk_data.len() as i32;
    mock_index.real_data_size = text_payload.len() as i32;
    mock_index.compressed_type = COMPRESSED_TYPE_COMPRESS;

    let mut index_bytes = vec![0u8; stride];
    unsafe {
        std::ptr::copy_nonoverlapping(
            &mock_index as *const TEterPackIndex as *const u8,
            index_bytes.as_mut_ptr(),
            stride,
        );
    }
    
    // Compress the index bytes (mock LZO uncompressed)
    let mut lzo_index = vec![17 + stride as u8];
    lzo_index.extend_from_slice(&index_bytes);
    
    // Build MCOZ block
    let mut mcoz_block = Vec::new();
    mcoz_block.extend_from_slice(&MCOZ_FOURCC.to_le_bytes()); // FourCC
    mcoz_block.extend_from_slice(&0u32.to_le_bytes()); // encrypt_size = 0 (unencrypted)
    mcoz_block.extend_from_slice(&(lzo_index.len() as u32).to_le_bytes()); // compressed_size
    mcoz_block.extend_from_slice(&(stride as u32).to_le_bytes()); // real_size
    mcoz_block.extend_from_slice(&lzo_index);
    
    // Build final EPKD header
    let mut eix_data = Vec::new();
    eix_data.extend_from_slice(&EPKD_FOURCC.to_le_bytes()); // EPKD
    eix_data.extend_from_slice(&2u32.to_le_bytes()); // version
    eix_data.extend_from_slice(&1u32.to_le_bytes()); // index_count
    eix_data.extend_from_slice(&mcoz_block);
    
    // 3. E2E Process
    let index_map = load_eix_index(&eix_data, &key).expect("Failed to load EIX index");
    let file_index = index_map.get("config/settings.txt").expect("File not found in index");
    
    let extracted_data = read_epk_file_data(&epk_data, file_index).expect("Failed to read EPK data");
    
    assert_eq!(extracted_data, text_payload);
    assert_eq!(String::from_utf8(extracted_data).unwrap(), "This is a mocked configuration file for testing.");
}
