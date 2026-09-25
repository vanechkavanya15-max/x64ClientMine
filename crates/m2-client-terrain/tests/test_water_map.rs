use m2_client_terrain::water_map::*;

#[test]
fn test_default_water_map() {
    let instance = WaterMap::default();
    assert!(!instance.is_ready());
    assert_eq!(instance.get_num_water(), 0);
    assert_eq!(instance.get_water_height(0), -1);
    assert_eq!(instance.get_water_map_value(0, 0), 0xFF);
}

#[test]
fn test_new_water_map() {
    let instance = WaterMap::new();
    assert!(instance.is_ready());
}

#[test]
fn test_invalid_header_size() {
    let buf = vec![0u8; 6];
    let res = WaterMap::load_from_bytes(&buf);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "FILE SIZE ERROR");
}

#[test]
fn test_invalid_magic() {
    let mut buf = vec![0u8; 7];
    // Zly magic
    buf[0..2].copy_from_slice(&9999u16.to_le_bytes());
    let res = WaterMap::load_from_bytes(&buf);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "MAGIC NUMBER ERROR");
}

#[test]
fn test_invalid_width() {
    let mut buf = vec![0u8; 7];
    buf[0..2].copy_from_slice(&WATER_MAGIC.to_le_bytes());
    // Zla szerokosc
    buf[2..4].copy_from_slice(&100u16.to_le_bytes());
    let res = WaterMap::load_from_bytes(&buf);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "WIDTH ERROR");
}

#[test]
fn test_invalid_height() {
    let mut buf = vec![0u8; 7];
    buf[0..2].copy_from_slice(&WATER_MAGIC.to_le_bytes());
    buf[2..4].copy_from_slice(&WATERMAP_XSIZE.to_le_bytes());
    // Zla wysokosc
    buf[4..6].copy_from_slice(&100u16.to_le_bytes());
    let res = WaterMap::load_from_bytes(&buf);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "HEIGHT ERROR");
}

#[test]
fn test_invalid_file_rest_size() {
    let mut buf = vec![0u8; 7];
    buf[0..2].copy_from_slice(&WATER_MAGIC.to_le_bytes());
    buf[2..4].copy_from_slice(&WATERMAP_XSIZE.to_le_bytes());
    buf[4..6].copy_from_slice(&WATERMAP_YSIZE.to_le_bytes());
    buf[6] = 1; // 1 layer

    // 100 bajtow mapy wody + 7 bajtow naglowka (zbyt malo w porownaniu do 128x128)
    let full_buf = [buf, vec![0u8; 100]].concat();

    let res = WaterMap::load_from_bytes(&full_buf);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "FILE DATA SIZE ERROR");
}

#[test]
fn test_load_u16_format() {
    let mut buf = vec![0u8; 7];
    buf[0..2].copy_from_slice(&WATER_MAGIC.to_le_bytes());
    buf[2..4].copy_from_slice(&WATERMAP_XSIZE.to_le_bytes());
    buf[4..6].copy_from_slice(&WATERMAP_YSIZE.to_le_bytes());
    buf[6] = 2; // 2 warstwy wody

    let mut map_data = vec![0x1; (WATERMAP_XSIZE as usize) * (WATERMAP_YSIZE as usize)];
    map_data[0] = 0; // x: 0, y: 0 -> warstwa 0
    map_data[1] = 1; // x: 1, y: 0 -> warstwa 1

    let mut heights_data = vec![0u8; 4];
    heights_data[0..2].copy_from_slice(&150u16.to_le_bytes()); // Warstwa 0 = 150
    heights_data[2..4].copy_from_slice(&300u16.to_le_bytes()); // Warstwa 1 = 300

    let full_buf = [buf, map_data, heights_data].concat();

    let res = WaterMap::load_from_bytes(&full_buf);
    assert!(res.is_ok());

    let instance = res.unwrap();
    assert!(instance.is_ready());
    assert_eq!(instance.get_num_water(), 2);
    
    assert_eq!(instance.get_water_height(0), 150);
    assert_eq!(instance.get_water_height(1), 300);
    
    assert_eq!(instance.get_water_map_value(0, 0), 0);
    assert_eq!(instance.get_water_map_value(1, 0), 1);
}

#[test]
fn test_load_i32_format() {
    let mut buf = vec![0u8; 7];
    buf[0..2].copy_from_slice(&WATER_MAGIC.to_le_bytes());
    buf[2..4].copy_from_slice(&WATERMAP_XSIZE.to_le_bytes());
    buf[4..6].copy_from_slice(&WATERMAP_YSIZE.to_le_bytes());
    buf[6] = 3; // 3 warstwy wody

    let mut map_data = vec![0xFF; (WATERMAP_XSIZE as usize) * (WATERMAP_YSIZE as usize)];
    map_data[127] = 2; // gdzies w mapie ustawiamy indeks warstwy 2

    let mut heights_data = vec![0u8; 12];
    heights_data[0..4].copy_from_slice(&1500i32.to_le_bytes()); // Warstwa 0
    heights_data[4..8].copy_from_slice(&(-50i32).to_le_bytes()); // Warstwa 1
    heights_data[8..12].copy_from_slice(&80000i32.to_le_bytes()); // Warstwa 2

    let full_buf = [buf, map_data, heights_data].concat();

    let res = WaterMap::load_from_bytes(&full_buf);
    assert!(res.is_ok());

    let instance = res.unwrap();
    assert!(instance.is_ready());
    assert_eq!(instance.get_num_water(), 3);
    
    assert_eq!(instance.get_water_height(0), 1500);
    assert_eq!(instance.get_water_height(1), -50);
    assert_eq!(instance.get_water_height(2), 80000);
    
    assert_eq!(instance.get_water_map_value(127, 0), 2);
}

#[test]
fn test_get_out_of_bounds() {
    let instance = WaterMap::new();
    assert_eq!(instance.get_water_map_value(200, 200), 0xFF); // poza zakresem mapy
    assert_eq!(instance.get_water_height(255), -1); // u8 nie moze byc > 255 bez overflow, ale testuje maksymalny limit u8
}
