use m2_client::wgpu_camera_shake_fx::CameraShakeFx;
use glam::Vec3;

#[test]
fn test_camera_shake_start_and_apply() {
    let mut shake = CameraShakeFx::new(2.0, 10.0, 1.0, 1.0);
    assert!(shake.is_active(), "Wstrzas powinien byc aktywny bezposrednio po inicjalizacji");

    let mut eye = Vec3::new(100.0, 200.0, 300.0);
    let original_eye = eye;

    // Aplikujemy wstrzas przy malym delta time, zeby upewnic sie, ze cos sie przesunelo
    shake.apply_to(&mut eye, 0.1);
    
    // Z uwagi na wstrzas pozycja powinna sie zmienic 
    assert!(eye != original_eye, "Oko kamery powinno zmienic pozycje ze wzgledu na dzialajacy wstrzas");
}

#[test]
fn test_camera_shake_decay() {
    let mut shake = CameraShakeFx::new(2.0, 10.0, 1.0, 1.0);
    
    // Na poczatku wstrzas jest silny
    let offset_early = shake.update(0.1);
    
    // Pozwalamy wstrzasowi opadnac do samego konca
    shake.update(1.8); 
    
    // Wstrzas na koncu swojego cyklu (zostalo mu 0.1s z 2.0s) powinien byc bliski zeru ze wzgledu na decay
    let offset_late = shake.update(0.09);

    assert!(offset_late.length() < offset_early.length(), "Wstrzas pod koniec trwania powinien byc mniejszy (zastosowano tlumienie)");
}

#[test]
fn test_camera_shake_duration_expiration() {
    let mut shake = CameraShakeFx::new(1.0, 10.0, 1.0, 1.0);
    assert!(shake.is_active());

    // Przekraczamy czas zycia
    let offset_expired = shake.update(1.1);

    assert!(!shake.is_active(), "Wstrzas nie powinien byc aktywny po uplywie czasu");
    assert_eq!(offset_expired, Vec3::ZERO, "Po wygasnieciu wstrzas powinien zwracac zerowe przesuniecie");
}

#[test]
fn test_camera_shake_zero_delta() {
    let mut shake = CameraShakeFx::new(1.0, 10.0, 1.0, 1.0);
    let offset = shake.update(0.0);
    assert_eq!(offset, Vec3::ZERO, "Przy czasie = 0 nie powinno byc zadnego wstrzasu ze wzgledu na brak uplywu czasu oraz rownanie (sin(0) = 0)");
}
