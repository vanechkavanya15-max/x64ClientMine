use m2_client::texture_set_parser::TextureSet;

#[test]
fn test_parse_multi_entry_texture_set() {
    let input = r#"
TextureSet

TextureCount 2

Start Texture001
    "d:/ymir work/terrainmaps/ground/grass_01.dds"
    4.0
    4.0
    0.0
    0.0
    1
    0
    65535
End Texture001

Start Texture002
    "d:/ymir work/terrainmaps/ground/dirt_01.dds"
    2.5
    3.5
    10.0
    5.0
    0
    100
    200
End Texture002
"#;

    let ts = TextureSet::parse(input).expect("Failed to parse texture set");

    // Default texture at 0
    let t0 = ts.get_texture(0).expect("Missing default texture");
    assert_eq!(t0.file_name, "");

    let t1 = ts.get_texture(1).expect("Missing texture 1");
    assert_eq!(t1.file_name, "d:/ymir work/terrainmaps/ground/grass_01.dds");
    assert_eq!(t1.u_scale, 4.0);
    assert_eq!(t1.v_scale, 4.0);
    assert_eq!(t1.u_offset, 0.0);
    assert_eq!(t1.v_offset, 0.0);
    assert_eq!(t1.b_splat, true);
    assert_eq!(t1.begin, 0);
    assert_eq!(t1.end, 65535);

    let t2 = ts.get_texture(2).expect("Missing texture 2");
    assert_eq!(t2.file_name, "d:/ymir work/terrainmaps/ground/dirt_01.dds");
    assert_eq!(t2.u_scale, 2.5);
    assert_eq!(t2.v_scale, 3.5);
    assert_eq!(t2.u_offset, 10.0);
    assert_eq!(t2.v_offset, 5.0);
    assert_eq!(t2.b_splat, false);
    assert_eq!(t2.begin, 100);
    assert_eq!(t2.end, 200);
}

#[test]
fn test_transform_uv() {
    let input = r#"
TextureSet
TextureCount 1
Start Texture001
    "test.dds"
    2.0
    3.0
    10.0
    20.0
    1
    0
    100
End Texture001
"#;
    let ts = TextureSet::parse(input).expect("Failed to parse");

    // Calculate UV for texture 1: u_out = u * 2.0 + 10.0, v_out = v * 3.0 + 20.0
    let (u_out, v_out) = ts.transform_uv(1, 5.0, 10.0);
    assert_eq!(u_out, 5.0 * 2.0 + 10.0);
    assert_eq!(v_out, 10.0 * 3.0 + 20.0);
}

#[test]
fn test_missing_texture_index() {
    let ts = TextureSet::new();

    // Default UV transform should return (u*4.0, v*4.0)
    let (u_out, v_out) = ts.transform_uv(99, 10.0, 20.0);
    assert_eq!(u_out, 40.0);
    assert_eq!(v_out, 80.0);

    // Get texture should return None
    assert!(ts.get_texture(99).is_none());
}

#[test]
fn test_height_interval() {
    let input = r#"
TextureSet
TextureCount 1
Start Texture001
    "test.dds"
    1.0
    1.0
    0.0
    0.0
    1
    500
    1000
End Texture001
"#;
    let ts = TextureSet::parse(input).unwrap();
    let t1 = ts.get_texture(1).unwrap();

    assert_eq!(t1.begin, 500);
    assert_eq!(t1.end, 1000);
    
    // Interval validation logic test
    let height: u16 = 750;
    assert!(height >= t1.begin && height <= t1.end);
    let height_low: u16 = 400;
    assert!(!(height_low >= t1.begin && height_low <= t1.end));
}

#[test]
fn test_parse_invalid_header() {
    let input = "WrongHeader\nTextureCount 1\n";
    let res = TextureSet::parse(input);
    assert!(res.is_err());
}

#[test]
fn test_parse_missing_texture_count() {
    let input = "TextureSet\nStart Texture001\n\"t.dds\"\n1 1 0 0 1 0 0\nEnd Texture001";
    let res = TextureSet::parse(input);
    assert!(res.is_err());
}

#[test]
fn test_parse_invalid_floats() {
    let input = r#"
TextureSet
TextureCount 1
Start Texture001
    "test.dds"
    abc
    3.0
    10.0
    20.0
    1
    0
    100
End Texture001
"#;
    let res = TextureSet::parse(input);
    assert!(res.is_err());
}
