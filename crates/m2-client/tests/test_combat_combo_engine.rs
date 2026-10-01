use m2_client::combat_combo_engine::{CombatComboEngine, WeaponType, ComboTimeData, EnemyTarget, HEADER_CG_ATTACK, TPacketCGAttack};
use std::mem::size_of;

#[test]
fn test_combo_ground_progression() {
    let mut engine = CombatComboEngine::new(false, WeaponType::Sword);
    
    // Zwykły pierwszy atak
    assert!(engine.input_attack(0.0));
    assert_eq!(engine.cur_combo_index, 1);
    
    // Ustawienie okna czasowego (np: start 0.3s, combo_time 0.6s, end 0.9s, total 1.0s)
    engine.time_data = Some(ComboTimeData {
        combo_window_start: 0.3,
        next_combo_time: 0.6,
        combo_window_end: 0.9,
        motion_duration: 1.0,
        has_time_data: true,
    });
    
    // Proba ataku na poczatku - falsz
    assert!(!engine.input_attack(0.0));
    assert!(!engine.is_pre_input);
    
    // Przesuwamy sie na 0.4s (wpadamy w okno pre_input)
    engine.update(0.4);
    assert!(!engine.input_attack(0.0)); // w pre input oddaje false
    assert!(engine.is_pre_input);
    
    // Przesuwamy sie na 0.61s (wchodzi nastepny cios)
    engine.update(0.21);
    assert_eq!(engine.cur_combo_index, 2);
    assert!(!engine.is_pre_input);
    
    // Analogicznie dla kolejnych ciosów, bez pre_input od razu uderzając w oknie od next_combo_time
    engine.update(0.61);
    assert!(engine.input_attack(0.0));
    assert_eq!(engine.cur_combo_index, 3);
    
    engine.update(0.61);
    assert!(engine.input_attack(0.0));
    assert_eq!(engine.cur_combo_index, 4); // Finisher
    
    // Następny cios zablokowany - czekamy na powrót Wait
    engine.update(0.61);
    assert!(!engine.input_attack(0.0));
    assert_eq!(engine.cur_combo_index, 4);
    
    // Zbyt długi czas -> wygasa
    engine.update(0.4); // Sumarycznie przekroczone end_time (0.9s)
    assert_eq!(engine.cur_combo_index, 0); // Wyczyszczone
}

#[test]
fn test_combo_horse_progression() {
    let mut engine = CombatComboEngine::new(true, WeaponType::TwoHand);
    
    // Atak 1
    assert!(engine.input_attack(0.0));
    assert_eq!(engine.cur_combo_index, 1);
    
    engine.time_data = Some(ComboTimeData {
        combo_window_start: 0.3,
        next_combo_time: 0.6,
        combo_window_end: 0.9,
        motion_duration: 1.0,
        has_time_data: true,
    });
    
    // Atak 2
    engine.update(0.61);
    assert!(engine.input_attack(0.0));
    assert_eq!(engine.cur_combo_index, 2);
    
    // Atak 3 (Max horse)
    engine.update(0.61);
    assert!(engine.input_attack(0.0));
    assert_eq!(engine.cur_combo_index, 3);
    
    // Na koniu combo zapętla się do 1, nie do blokady!
    engine.update(0.61);
    assert!(engine.input_attack(0.0));
    assert_eq!(engine.cur_combo_index, 1);
}

#[test]
fn test_attack_cone_geometry() {
    let mut engine = CombatComboEngine::new(false, WeaponType::Sword);
    assert!(engine.input_attack(0.0)); // rotation 0.0 -> Y up
    
    // Wróg na Y z przodu, Wróg na Y z tyłu, Wróg na krawędzi
    let enemies = vec![
        EnemyTarget { vid: 1, x: 0.0, y: 100.0, radius: 10.0 }, // Front, hit
        EnemyTarget { vid: 2, x: 0.0, y: -100.0, radius: 10.0 }, // Back, miss
        EnemyTarget { vid: 3, x: 100.0, y: 100.0, radius: 10.0 }, // 45 deg, cos(45)=0.7 > 0.5 (hit)
        EnemyTarget { vid: 4, x: 100.0, y: 0.0, radius: 10.0 }, // 90 deg, cos(90)=0.0 < 0.5 (miss)
    ];
    
    let result = engine.process_attack_geometry(0.0, 0.0, &enemies);
    
    assert_eq!(result.packets.len(), 2);
    
    // Zdekoduj wroga 1
    let pkt1: *const TPacketCGAttack = result.packets[0].as_ptr() as *const TPacketCGAttack;
    let pkt2: *const TPacketCGAttack = result.packets[1].as_ptr() as *const TPacketCGAttack;
    
    unsafe {
        let v1 = { (*pkt1).dw_victim_vid };
        let b1 = { (*pkt1).b_type };
        let v2 = { (*pkt2).dw_victim_vid };
        assert_eq!(v1, 1);
        assert_eq!(b1, 1);
        assert_eq!(v2, 3);
    }
}

#[test]
fn test_weapon_distance() {
    let mut engine = CombatComboEngine::new(false, WeaponType::Dagger); // max 100 range
    engine.input_attack(90.0); // rot 90, vector X = 1.0, Y = 0.0
    
    let enemies = vec![
        EnemyTarget { vid: 1, x: 90.0, y: 0.0, radius: 10.0 }, // Range 90 < 100+10, hit
        EnemyTarget { vid: 2, x: 120.0, y: 0.0, radius: 5.0 }, // Range 120 > 100+5, miss
    ];
    
    let result = engine.process_attack_geometry(0.0, 0.0, &enemies);
    assert_eq!(result.packets.len(), 1);
    
    unsafe {
        let pkt: *const TPacketCGAttack = result.packets[0].as_ptr() as *const TPacketCGAttack;
        let v = { (*pkt).dw_victim_vid };
        assert_eq!(v, 1);
    }
}

#[test]
fn test_finisher_pushback() {
    let mut engine = CombatComboEngine::new(false, WeaponType::Sword);
    
    // Doprowadzamy do finishera
    engine.input_attack(0.0);
    engine.time_data = Some(ComboTimeData {
        combo_window_start: 0.0, next_combo_time: 0.0, combo_window_end: 10.0, motion_duration: 10.0, has_time_data: true,
    });
    engine.update(1.0); engine.input_attack(0.0); // Hit 2
    engine.update(1.0); engine.input_attack(0.0); // Hit 3
    engine.update(1.0); engine.input_attack(0.0); // Hit 4
    
    assert_eq!(engine.cur_combo_index, 4);
    
    let enemies = vec![
        EnemyTarget { vid: 1, x: 0.0, y: 50.0, radius: 10.0 },
        EnemyTarget { vid: 2, x: 0.0, y: 80.0, radius: 10.0 },
    ];
    
    let result = engine.process_attack_geometry(0.0, 0.0, &enemies);
    
    assert_eq!(result.packets.len(), 2);
    assert_eq!(result.pushback_targets.len(), 2); // Odrzucenie następuje na wszystkich trafionych celach podczas finishera
    assert!(result.pushback_targets.contains(&1));
    assert!(result.pushback_targets.contains(&2));
}

#[test]
fn test_no_pushback_on_early_hits() {
    let mut engine = CombatComboEngine::new(false, WeaponType::Sword);
    engine.input_attack(0.0); // Hit 1
    
    let enemies = vec![
        EnemyTarget { vid: 1, x: 0.0, y: 50.0, radius: 10.0 },
    ];
    
    let result = engine.process_attack_geometry(0.0, 0.0, &enemies);
    
    assert_eq!(result.packets.len(), 1);
    assert!(result.pushback_targets.is_empty()); // Brak pushback dla hit 1
}

#[test]
fn test_packet_structure_integrity() {
    let mut engine = CombatComboEngine::new(false, WeaponType::Sword);
    engine.input_attack(0.0);
    
    let enemies = vec![
        EnemyTarget { vid: 1337, x: 0.0, y: 50.0, radius: 10.0 },
    ];
    
    let result = engine.process_attack_geometry(0.0, 0.0, &enemies);
    
    assert_eq!(result.packets.len(), 1);
    let buf = &result.packets[0];
    
    assert_eq!(buf.len(), size_of::<TPacketCGAttack>());
    
    unsafe {
        let pkt: *const TPacketCGAttack = buf.as_ptr() as *const TPacketCGAttack;
        let h = { (*pkt).header };
        let l = { (*pkt).length };
        let b = { (*pkt).b_type };
        let v = { (*pkt).dw_victim_vid };
        
        assert_eq!(h, HEADER_CG_ATTACK);
        assert_eq!(l, size_of::<TPacketCGAttack>() as u16);
        assert_eq!(b, 1); // Combo index 1
        assert_eq!(v, 1337);
    }
}