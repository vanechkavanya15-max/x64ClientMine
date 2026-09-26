use m2_client::sub_image_parser::SubImage;

#[test]
fn test_valid_parsing() {
    let script = "\
        title subimage\n\
        version 2.0\n\
        image d:/ymir work/ui/pattern/board.dds\n\
        left 10\n\
        top 20\n\
        right 42\n\
        bottom 52\n\
    ";

    let result = SubImage::parse(script).expect("Failed to parse valid script");

    assert_eq!(result.version.as_deref(), Some("2.0"));
    assert_eq!(result.image, "d:/ymir work/ui/pattern/board.dds");
    assert_eq!(result.left, 10);
    assert_eq!(result.top, 20);
    assert_eq!(result.right, 42);
    assert_eq!(result.bottom, 52);

    assert_eq!(result.width(), 32);
    assert_eq!(result.height(), 32);
}

#[test]
fn test_missing_fields() {
    let script = "\
        version 2.0\n\
        image d:/ymir work/ui/pattern/board.dds\n\
        left 10\n\
        top 20\n\
        right 42\n\
        bottom 52\n\
    ";
    assert_eq!(SubImage::parse(script), Err("missing title"));

    let script2 = "\
        title subimage\n\
        version 2.0\n\
        left 10\n\
        top 20\n\
        right 42\n\
        bottom 52\n\
    ";
    assert_eq!(SubImage::parse(script2), Err("missing image"));
    
    let script3 = "\
        title subimage\n\
        version 2.0\n\
        image asdf\n\
        top 20\n\
        right 42\n\
        bottom 52\n\
    ";
    assert_eq!(SubImage::parse(script3), Err("missing left"));
}

#[test]
fn test_invalid_title() {
    let script = "\
        title not_subimage\n\
        version 2.0\n\
        image d:/ymir work/ui/pattern/board.dds\n\
        left 10\n\
        top 20\n\
        right 42\n\
        bottom 52\n\
    ";
    assert_eq!(SubImage::parse(script), Err("invalid title"));
}

#[test]
fn test_case_insensitivity() {
    let script = "\
        TiTlE SuBiMaGe\n\
        VERSION 1.0\n\
        IMAGE some_path\n\
        LEFT 0\n\
        ToP 0\n\
        RiGhT 100\n\
        bOtToM 100\n\
    ";

    let result = SubImage::parse(script).expect("Failed case insensitive parsing");

    assert_eq!(result.version.as_deref(), Some("1.0"));
    assert_eq!(result.image, "some_path");
    assert_eq!(result.left, 0);
    assert_eq!(result.top, 0);
    assert_eq!(result.right, 100);
    assert_eq!(result.bottom, 100);
}

#[test]
fn test_uv_calculations() {
    let script = "\
        title subimage\n\
        image test\n\
        left 64\n\
        top 32\n\
        right 128\n\
        bottom 128\n\
    ";

    let result = SubImage::parse(script).unwrap();

    let (u0, v0, u1, v1) = result.get_uv(256, 256);
    assert_eq!(u0, 64.0 / 256.0);
    assert_eq!(v0, 32.0 / 256.0);
    assert_eq!(u1, 128.0 / 256.0);
    assert_eq!(v1, 128.0 / 256.0);
}

#[test]
fn test_uv_calculations_zero_size() {
    let sub = SubImage {
        version: None,
        image: "test".to_string(),
        left: 10,
        top: 20,
        right: 30,
        bottom: 40,
    };

    let (u0, v0, u1, v1) = sub.get_uv(0, 100);
    assert_eq!((u0, v0, u1, v1), (0.0, 0.0, 0.0, 0.0));

    let (u0, v0, u1, v1) = sub.get_uv(100, 0);
    assert_eq!((u0, v0, u1, v1), (0.0, 0.0, 0.0, 0.0));
}

#[test]
fn test_whitespace_handling() {
    let script = "\
        \n\
        \ttitle\t\tsubimage\t\n\
        image   spaced out string   \n\
        left \t 10 \n\
        top  20 \n\
        right\t42\n\
        bottom 52\n\
    ";

    let result = SubImage::parse(script).expect("Failed handling whitespace");
    assert_eq!(result.image, "spaced out string");
    assert_eq!(result.left, 10);
    assert_eq!(result.top, 20);
    assert_eq!(result.right, 42);
    assert_eq!(result.bottom, 52);
}

#[test]
fn test_invalid_number_format() {
    let script = "\
        title subimage\n\
        image test\n\
        left abc\n\
        top 20\n\
        right 42\n\
        bottom 52\n\
    ";
    assert_eq!(SubImage::parse(script), Err("invalid left value"));
}
