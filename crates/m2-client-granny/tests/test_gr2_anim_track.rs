use m2_client_granny::gr2_anim_track::{Gr2AnimTrack, Quaternion, Vector3};

#[test]
fn test_integration_gr2_anim_track_creation() {
    let mut instance = Gr2AnimTrack::new("Bip01");
    assert!(!instance.is_ready());
    assert_eq!(instance.bone_name, "Bip01");

    instance.add_translation(0.0, Vector3::new(1.0, 2.0, 3.0));
    assert!(instance.is_ready());
}

#[test]
fn test_translation_sampling() {
    let mut track = Gr2AnimTrack::new("Spine");

    // Brak klatek - powinno zwrocic None
    assert!(track.sample_translation(0.5).is_none());

    track.add_translation(0.0, Vector3::new(0.0, 0.0, 0.0));
    track.add_translation(1.0, Vector3::new(10.0, 20.0, 30.0));

    // Przed pierwsza klatka - zwraca pierwsza klatke
    let before = track.sample_translation(-1.0).unwrap();
    assert_eq!(before.x, 0.0);
    assert_eq!(before.y, 0.0);
    assert_eq!(before.z, 0.0);

    // Po ostatniej klatce - zwraca ostatnia klatke
    let after = track.sample_translation(2.0).unwrap();
    assert_eq!(after.x, 10.0);
    assert_eq!(after.y, 20.0);
    assert_eq!(after.z, 30.0);

    // W srodku (interpolacja) - t = 0.5
    let mid = track.sample_translation(0.5).unwrap();
    assert_eq!(mid.x, 5.0);
    assert_eq!(mid.y, 10.0);
    assert_eq!(mid.z, 15.0);
}

#[test]
fn test_rotation_sampling() {
    let mut track = Gr2AnimTrack::new("Head");

    assert!(track.sample_rotation(0.5).is_none());

    // Rotacja na poczatku (0.0) -> bez rotacji
    track.add_rotation(0.0, Quaternion::new(0.0, 0.0, 0.0, 1.0));
    
    // Rotacja na koncu (1.0)
    track.add_rotation(1.0, Quaternion::new(0.0, 1.0, 0.0, 0.0));

    // W srodku (interpolacja) - t = 0.5
    let mid = track.sample_rotation(0.5).unwrap();
    
    // Sprawdzamy przyblizone wartosci dla nlerp
    assert!(mid.y > 0.0 && mid.y < 1.0);
    assert!(mid.w > 0.0 && mid.w < 1.0);
    
    // Dlugosc powinna byc w przyblizeniu 1.0
    let len = (mid.x * mid.x + mid.y * mid.y + mid.z * mid.z + mid.w * mid.w).sqrt();
    assert!((len - 1.0).abs() < 0.001);
}

#[test]
fn test_scale_sampling() {
    let mut track = Gr2AnimTrack::new("Arm");

    assert!(track.sample_scale(0.5).is_none());

    track.add_scale(0.0, Vector3::new(1.0, 1.0, 1.0));
    track.add_scale(2.0, Vector3::new(3.0, 3.0, 3.0));

    // W srodku (interpolacja) - t = 0.5 w skali klatek 0.0 do 2.0 to 25% drogi
    let sample = track.sample_scale(0.5).unwrap();
    assert_eq!(sample.x, 1.5);
    assert_eq!(sample.y, 1.5);
    assert_eq!(sample.z, 1.5);
    
    // t = 1.0 to 50% drogi
    let sample_mid = track.sample_scale(1.0).unwrap();
    assert_eq!(sample_mid.x, 2.0);
    assert_eq!(sample_mid.y, 2.0);
    assert_eq!(sample_mid.z, 2.0);
}
