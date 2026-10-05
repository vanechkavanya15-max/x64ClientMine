use m2_client::game_horse_mount_controller::{InstanceBase, MotionMode, PixelPosition};

#[test]
fn test_is_mounting_horse() {
    let mut instance = InstanceBase::new(12345);

    // Z początku postać nie siedzi na koniu
    assert!(!instance.is_mounting_horse());
    assert!(!instance.is_new_mount());

    // Dosiada zwykłego konia (np. 20101)
    instance.mount_horse(20101);

    assert!(instance.is_mounting_horse());
    assert_eq!(instance.motion_mode, MotionMode::Horse);
    assert_eq!(
        instance.rotation_speed,
        instance.default_horse_rotation_speed
    );

    // Sprawdź czy to nowy mount - stary koń nie jest
    assert!(!instance.is_new_mount());

    // Zejście z konia
    instance.dismount_horse();

    assert!(!instance.is_mounting_horse());
}

#[test]
fn test_horse_level_logic() {
    let mut instance = InstanceBase::new(12345);

    // Poziom konia gdy go nie ma
    assert_eq!(instance.horse.get_level(), 0);

    // Kucyk (1 lvl)
    instance.mount_horse(20101);
    assert_eq!(instance.horse.get_level(), 1);

    // Koń bojowy (2 lvl)
    instance.mount_horse(20104);
    assert_eq!(instance.horse.get_level(), 2);

    // Koń wojskowy (3 lvl)
    instance.mount_horse(20107);
    assert_eq!(instance.horse.get_level(), 3);
}

#[test]
fn test_new_mount_logic() {
    let mut instance = InstanceBase::new(12345);

    // Brak mounta
    assert!(!instance.is_new_mount());

    // Nowy mount z puli 20205-20208
    instance.mount_horse(20206);
    assert!(instance.horse.is_new_mount());
    assert_eq!(instance.horse.get_level(), 2);

    // Nowy mount z puli 20209-20212 (3 lvl)
    instance.mount_horse(20209);
    assert!(instance.horse.is_new_mount());
    assert_eq!(instance.horse.get_level(), 3);
}

#[test]
fn test_position_sync_on_mount() {
    let mut instance = InstanceBase::new(12345);
    let pos = PixelPosition::new(100.0, 200.0, 300.0);

    // Ustaw pozycje main aktora
    instance.graphic_thing_instance.pos = pos;

    // Dosiadanie konia powinno przekazac pozycje
    instance.mount_horse(20101);

    if let Some(actor) = instance.horse.get_actor_ptr() {
        assert_eq!(actor.borrow().pos, pos);
    } else {
        panic!("Horse actor not found!");
    }
}

#[test]
fn test_movement_speed_and_attack_logic() {
    let mut instance = InstanceBase::new(12345);

    // Test that speed cascades correctly
    instance.mount_horse(20101);
    instance.set_move_speed(200);

    assert_eq!(instance.base_move_speed, 2.0);

    if let Some(actor) = instance.horse.get_actor_ptr() {
        assert_eq!(actor.borrow().move_speed, 2.0);
    } else {
        panic!("Horse actor not found!");
    }

    // Test attack block level check
    // kucyk (lvl 1) nie moze atakowac
    assert!(!instance.can_attack_horse_level());

    // kon bojowy (lvl 2) moze atakowac
    instance.mount_horse(20104);
    assert!(instance.can_attack_horse_level());
}

#[test]
fn test_dismount_horse_resets_motion_mode() {
    let mut instance = InstanceBase::new(12345);
    
    // Ustaw sztucznie złą pozycję, by sprawdzić resetowanie
    instance.rotation_speed = 5.0;
    instance.motion_mode = MotionMode::Horse;

    // Wejscie i zejscie na konia powinno zresetować motion i rotacje do stanu defaultowego
    instance.mount_horse(20101);
    
    assert_eq!(instance.motion_mode, MotionMode::Horse);
    assert_eq!(instance.rotation_speed, instance.default_horse_rotation_speed);
    
    instance.dismount_horse();
    
    assert_eq!(instance.motion_mode, MotionMode::General);
    assert_eq!(instance.rotation_speed, 1.0);
}

#[test]
fn test_saddle_attachment() {
    let mut instance = InstanceBase::new(12345);

    // Dosiadamy konia
    instance.mount_horse(20101);

    // Sprawdzamy, czy saddle został dodany do attach_models
    if let Some(actor) = instance.horse.get_actor_ptr() {
        assert!(actor.borrow().attached_models.contains(&"saddle".to_string()));
    } else {
        panic!("Horse actor nie istnieje po mount_horse");
    }

    // Zsiadamy z konia, powinno usunąć siodło
    instance.detach_horse_saddle();
    
    // Siodlo jest detachowane jeszcze kiedy obiekt horse.actor istnieje
    if let Some(actor) = instance.horse.get_actor_ptr() {
        assert!(actor.borrow().attached_models.is_empty());
    } else {
        panic!("Horse actor zniknął za wcześnie");
    }
}
