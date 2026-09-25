use m2_client::fly_manager::{FlyManager, FlyData, FlyTarget, FlyEventHandler, Vector3};
use std::collections::HashMap;

struct MockEventHandler {
    hit_target_vid: u32,
    hit_another_target_vid: u32,
    out_of_range: bool,
    hit_background: bool,
}

impl MockEventHandler {
    fn new() -> Self {
        Self {
            hit_target_vid: 0,
            hit_another_target_vid: 0,
            out_of_range: false,
            hit_background: false,
        }
    }
}

impl FlyEventHandler for MockEventHandler {
    fn on_exploding_at_target(&mut self, _skill_index: u32, target_vid: u32) {
        self.hit_target_vid = target_vid;
    }

    fn on_exploding_at_another_target(&mut self, _skill_index: u32, target_vid: u32) {
        self.hit_another_target_vid = target_vid;
    }

    fn on_exploding_out_of_range(&mut self) {
        self.out_of_range = true;
    }

    fn on_exploding_at_background(&mut self) {
        self.hit_background = true;
    }
}
 
 #[test]
fn test_straight_flight_to_position() {
    let mut manager = FlyManager::new();
    
    let mut data = FlyData::default();
    data.init_vel = 100.0;
    data.bomb_range = 10.0;
    data.range = 500.0;
    manager.register_fly_data(1, data);
    
    let start_pos = Vector3::new(0.0, 0.0, 0.0);
    let target_pos = Vector3::new(100.0, 0.0, 0.0);
    
    manager.create_fly_instance(1, start_pos, FlyTarget::Position(target_pos), 10, true, 0).unwrap();
    
    let mut handler = MockEventHandler::new();
    let empty_targets = HashMap::new();
    
    // Po pol sekundy nie doleci
    manager.update_time(0.5);
    manager.update(0.5, &empty_targets, Some(&mut handler));
    assert_eq!(manager.get_instance_count(), 1);
    
    // Po kolejnej pol sekundy doleci na x=100
    manager.update_time(0.6); // 1.1s razem
    manager.update(0.6, &empty_targets, Some(&mut handler));
    
    assert_eq!(manager.get_instance_count(), 0);
    assert_eq!(handler.hit_target_vid, 0); // VID to 0 poniewaz target to pozycja
}

#[test]
fn test_flight_with_gravity_out_of_range() {
    let mut manager = FlyManager::new();
    
    let mut data = FlyData::default();
    data.init_vel = 100.0;
    data.gravity = -9.81;
    data.range = 150.0; // krotki zasieg
    manager.register_fly_data(2, data);
    
    let start_pos = Vector3::new(0.0, 0.0, 0.0);
    let target_pos = Vector3::new(200.0, 0.0, 0.0);
    
    manager.create_fly_instance(2, start_pos, FlyTarget::Position(target_pos), 10, true, 0).unwrap();
    
    let mut handler = MockEventHandler::new();
    let empty_targets = HashMap::new();
    
    manager.update_time(2.0);
    manager.update(2.0, &empty_targets, Some(&mut handler));
    
    assert_eq!(manager.get_instance_count(), 0);
    assert!(handler.out_of_range); // wybuchnie out of range a nie uderzy cel
}

#[test]
fn test_flight_to_object_with_pierce() {
    let mut manager = FlyManager::new();
    
    let mut data = FlyData::default();
    data.init_vel = 100.0;
    data.bomb_range = 10.0;
    data.pierce_count = 1;
    manager.register_fly_data(3, data);
    
    let start_pos = Vector3::new(0.0, 0.0, 0.0);
    
    let target_vid = 100;
    let mut targets = HashMap::new();
    targets.insert(target_vid, Vector3::new(100.0, 0.0, 0.0));

    let instance_id = manager.create_fly_instance(3, start_pos, FlyTarget::Object(target_vid), 10, true, 0).unwrap();
    // We must manually set initial direction for object target in test since update won't do it before moving
    manager.get_instances_mut().iter_mut().find(|i| i.id == instance_id).unwrap().set_target_direction(Some(Vector3::new(100.0, 0.0, 0.0)));
    
    let mut handler = MockEventHandler::new();
    
    manager.update_time(1.1);
    manager.update(1.1, &targets, Some(&mut handler));
    
    // Przebija wiec zyje
    assert_eq!(manager.get_instance_count(), 1);
    assert_eq!(handler.hit_target_vid, 100);
    
    // Zresetuj mock
    handler.hit_target_vid = 0;
    
    // Zmien pozycje celu, doleci po znowu pol sekundy
    targets.insert(target_vid, Vector3::new(200.0, 0.0, 0.0));
    manager.get_instances_mut().iter_mut().find(|i| i.id == instance_id).unwrap().set_target_direction(Some(Vector3::new(200.0, 0.0, 0.0)));

    manager.update_time(1.1);
    manager.update(1.1, &targets, Some(&mut handler));
    
    assert_eq!(manager.get_instance_count(), 0); // Zginie bo pierce_count wykorzystany
    assert_eq!(handler.hit_target_vid, 100);
}

#[test]
fn test_homing_logic() {
    let mut manager = FlyManager::new();
    
    let mut data = FlyData::default();
    data.init_vel = 100.0;
    data.bomb_range = 10.0;
    data.is_homing = true;
    data.homing_max_angle = 90.0; 
    data.homing_start_time = 0.0;
    manager.register_fly_data(4, data);
    
    let start_pos = Vector3::new(0.0, 0.0, 0.0);
    let target_vid = 200;
    
    let instance_id = manager.create_fly_instance(4, start_pos, FlyTarget::Object(target_vid), 10, true, 0).unwrap();
    
    let mut targets = HashMap::new();
    targets.insert(target_vid, Vector3::new(50.0, 50.0, 0.0)); // ukosem
    
    manager.get_instances_mut().iter_mut().find(|i| i.id == instance_id).unwrap().set_target_direction(Some(Vector3::new(50.0, 50.0, 0.0)));

    let mut handler = MockEventHandler::new();
    
    // Poczatkowy kierunek jest odswiezany podczas homingu, wiec skreci do 50,50
    manager.update_time(1.0);
    manager.update(1.0, &targets, Some(&mut handler));
    
    assert_eq!(manager.get_instance_count(), 0); // powinnien dosc szybko uderzyc
    assert_eq!(handler.hit_target_vid, 200);
 }
