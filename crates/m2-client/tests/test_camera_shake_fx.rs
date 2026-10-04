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
    // Kiedy delta = 0 (czas sie nie posuwa) to z racji mnoznika delta_time, nie powinnismy miec zadnego offsetu
    // Poniewaz implementacja bazuje na delta_time dla framerate independence, dla 0 zwraca ZERO offset
    assert_eq!(offset, Vec3::ZERO, "Przy delt=0 wstrzas powinien zwrocic Vec3::ZERO poniewaz ruch jest zalezny od ramki");
    assert!(shake.is_active());
}

#[test]
fn test_camera_shake_randomness_distribution() {
    // Sprawdzamy, czy wartosci zachowuja logike z kodu referencyjnego (amplituda = max shake)
    let amplitude = 20.0;
    let mut shake = CameraShakeFx::new(5.0, amplitude, 1.0, 0.0);
    
    // Sprawdzamy kilkukrotnie granice randomizacji
    for _ in 0..10 {
        let offset = shake.update(0.1);
        // Maksymalne wychylenie offsetu to amplitude. Sprawdzamy wiec, ze jestesmy w limitach.
        // Po uwzglednieniu f32 moze to byc rowne (albo bliskie).
        assert!(offset.x.abs() <= amplitude, "Wychylenie kamery na osi X powinno byc mniejsze badz rowne amplitudzie.");
        assert!(offset.y.abs() <= amplitude, "Wychylenie kamery na osi Y powinno byc mniejsze badz rowne amplitudzie.");
        assert!(offset.z.abs() <= amplitude, "Wychylenie kamery na osi Z powinno byc mniejsze badz rowne amplitudzie.");
    }
}
