use m2_client::audio_volume_controller::{
    calculate_attenuation, calculate_stereo_panning, AudioListener3D, AudioSource3D,
};

#[test]
fn test_attenuation_within_min_distance() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0],
        up: [0.0, 0.0, 1.0],
    };

    let source = AudioSource3D {
        pos: [5.0, 0.0, 0.0],
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let volume = calculate_attenuation(&listener, &source);
    assert!((volume - 1.0).abs() < 1e-6);
}

#[test]
fn test_attenuation_beyond_max_distance() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0],
        up: [0.0, 0.0, 1.0],
    };

    let source = AudioSource3D {
        pos: [150.0, 0.0, 0.0],
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let volume = calculate_attenuation(&listener, &source);
    assert!((volume - 0.0).abs() < 1e-6);
}

#[test]
fn test_attenuation_middle_distance() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0],
        up: [0.0, 0.0, 1.0],
    };

    let source = AudioSource3D {
        pos: [55.0, 0.0, 0.0],
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let volume = calculate_attenuation(&listener, &source);
    assert!((volume - 0.5).abs() < 1e-6);
}

#[test]
fn test_stereo_panning_source_on_right() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0], // Looking at Y
        up: [0.0, 0.0, 1.0],      // Up is Z
    };
    // Right is X

    let source = AudioSource3D {
        pos: [10.0, 0.0, 0.0], // Source is on positive X (right)
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let (left, right) = calculate_stereo_panning(&listener, &source);
    // Pan should be 1.0 (fully right) -> angle = PI/2
    // left = cos(PI/2) = 0, right = sin(PI/2) = 1
    assert!(left.abs() < 1e-6);
    assert!((right - 1.0).abs() < 1e-6);
}

#[test]
fn test_stereo_panning_source_on_left() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0], // Looking at Y
        up: [0.0, 0.0, 1.0],      // Up is Z
    };
    // Right is X

    let source = AudioSource3D {
        pos: [-10.0, 0.0, 0.0], // Source is on negative X (left)
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let (left, right) = calculate_stereo_panning(&listener, &source);
    // Pan should be -1.0 (fully left) -> angle = 0
    // left = cos(0) = 1, right = sin(0) = 0
    assert!((left - 1.0).abs() < 1e-6);
    assert!(right.abs() < 1e-6);
}

#[test]
fn test_stereo_panning_source_in_front() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0], // Looking at Y
        up: [0.0, 0.0, 1.0],      // Up is Z
    };

    let source = AudioSource3D {
        pos: [0.0, 10.0, 0.0], // Source is exactly in front
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let (left, right) = calculate_stereo_panning(&listener, &source);
    // Pan should be 0.0 (center) -> angle = PI/4
    // left = cos(PI/4) = 0.707, right = sin(PI/4) = 0.707
    let expected = std::f32::consts::FRAC_PI_4.cos();
    assert!((left - expected).abs() < 1e-6);
    assert!((right - expected).abs() < 1e-6);
}

#[test]
fn test_stereo_panning_source_behind() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0], // Looking at Y
        up: [0.0, 0.0, 1.0],      // Up is Z
    };

    let source = AudioSource3D {
        pos: [0.0, -10.0, 0.0], // Source is exactly behind
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let (left, right) = calculate_stereo_panning(&listener, &source);
    // Pan should be 0.0 (center)
    let expected = std::f32::consts::FRAC_PI_4.cos();
    assert!((left - expected).abs() < 1e-6);
    assert!((right - expected).abs() < 1e-6);
}

#[test]
fn test_stereo_panning_source_at_listener() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0],
        up: [0.0, 0.0, 1.0],
    };

    let source = AudioSource3D {
        pos: [0.0, 0.0, 0.0], // Same position
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let (left, right) = calculate_stereo_panning(&listener, &source);
    // Fallback returns dot product 0, pan 0 (center)
    let expected = std::f32::consts::FRAC_PI_4.cos();
    assert!((left - expected).abs() < 1e-6);
    assert!((right - expected).abs() < 1e-6);
}

#[test]
fn test_stereo_panning_invalid_listener_vectors() {
    let listener = AudioListener3D {
        pos: [0.0, 0.0, 0.0],
        forward: [0.0, 1.0, 0.0],
        up: [0.0, 2.0, 0.0], // Collinear up vector
    };

    let source = AudioSource3D {
        pos: [10.0, 0.0, 0.0],
        min_distance: 10.0,
        max_distance: 100.0,
        base_volume: 1.0,
    };

    let (left, right) = calculate_stereo_panning(&listener, &source);
    // Fallback right vector is (1,0,0)
    // Dir is (1,0,0)
    // dot is 1.0, pan is 1.0
    // angle = PI/2, left = 0, right = 1
    assert!(left.abs() < 1e-6);
    assert!((right - 1.0).abs() < 1e-6);
}
