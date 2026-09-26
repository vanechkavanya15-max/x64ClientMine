use m2_client::flying_data_parser::{FlyingData, FLY_LINE, FLY_HOMING};

#[test]
fn test_parse_full_config() {
    let script = "\
        fly_type 1
        speed 1500.0
        acceleration 2.5
        max_range 1000.0
        has_tail 1
        pierce_count 3
        hit_radius 25.0
    ";
    let data = FlyingData::parse(script).unwrap();
    assert_eq!(data.fly_type, FLY_HOMING);
    assert_eq!(data.speed, 1500.0);
    assert_eq!(data.acceleration, 2.5);
    assert_eq!(data.max_range, 1000.0);
    assert_eq!(data.has_tail, true);
    assert_eq!(data.pierce_count, 3);
    assert_eq!(data.hit_radius, 25.0);
}

#[test]
fn test_parse_invalid_speed() {
    let script1 = "speed 0.0\nhit_radius 10.0";
    let err1 = FlyingData::parse(script1);
    assert_eq!(err1, Err("Invalid speed"));

    let script2 = "speed -50.0\nhit_radius 10.0";
    let err2 = FlyingData::parse(script2);
    assert_eq!(err2, Err("Invalid speed"));
}

#[test]
fn test_parse_invalid_hit_radius() {
    let script1 = "speed 1000.0\nhit_radius 0.0";
    let err1 = FlyingData::parse(script1);
    assert_eq!(err1, Err("Invalid hit radius"));

    let script2 = "speed 1000.0\nhit_radius -5.0";
    let err2 = FlyingData::parse(script2);
    assert_eq!(err2, Err("Invalid hit radius"));
}

#[test]
fn test_parse_has_tail_and_pierce_count() {
    let script = "\
        has_tail true
        pierce_count 5
        speed 500.0
        hit_radius 5.0
    ";
    let data = FlyingData::parse(script).unwrap();
    assert_eq!(data.has_tail, true);
    assert_eq!(data.pierce_count, 5);

    let script2 = "\
        has_tail 0
        pierce_count 0
        speed 500.0
        hit_radius 5.0
    ";
    let data2 = FlyingData::parse(script2).unwrap();
    assert_eq!(data2.has_tail, false);
    assert_eq!(data2.pierce_count, 0);
}

#[test]
fn test_parse_defaults_for_shortened_format() {
    // A script with only required positive fields.
    let script = "\
        speed 2000.0
        hit_radius 10.0
    ";
    let data = FlyingData::parse(script).unwrap();
    
    // Check if the defaults match the expected struct default values.
    assert_eq!(data.fly_type, FLY_LINE);
    assert_eq!(data.speed, 2000.0);
    assert_eq!(data.acceleration, 0.0);
    assert_eq!(data.max_range, 500.0);
    assert_eq!(data.has_tail, false);
    assert_eq!(data.pierce_count, 0);
    assert_eq!(data.hit_radius, 10.0);
}

#[test]
fn test_parse_ignore_comments_and_empty_lines() {
    let script = "
        // This is a comment
        # Another comment
        
        speed 1200.0
        hit_radius 15.0
    ";
    let data = FlyingData::parse(script).unwrap();
    assert_eq!(data.speed, 1200.0);
    assert_eq!(data.hit_radius, 15.0);
}
