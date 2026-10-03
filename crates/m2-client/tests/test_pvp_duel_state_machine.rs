#[path = "../src/pvp_duel_state_machine.rs"]
mod pvp_duel_state_machine;

use pvp_duel_state_machine::{
    PvpDuelStateMachine,
    PvpMode,
    DuelMode,
    PixelPosition,
    TerrainAttributeProvider,
    ATTRIBUTE_BANPK,
    get_pvp_key
};

struct MockTerrain {
    safe_zones: Vec<(f32, f32)>,
}

impl TerrainAttributeProvider for MockTerrain {
    fn is_attr_on(&self, x: f32, y: f32, attr: u32) -> bool {
        if attr == ATTRIBUTE_BANPK {
            for &(zx, zy) in &self.safe_zones {
                // simple check for mock
                if (x - zx).abs() < 1.0 && (y - zy).abs() < 1.0 {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test_pvp_key_symmetry() {
    let key1 = get_pvp_key(10, 20);
    let key2 = get_pvp_key(20, 10);
    assert_eq!(key1, key2);
}

#[test]
fn test_pvp_mode_transitions() {
    let mut state = PvpDuelStateMachine::new();
    
    // Test Agree
    state.insert_pvp_key(10, 20);
    assert!(state.find_pvp_key(10, 20));
    state.handle_pvp_packet_mode(10, 20, PvpMode::Agree);
    assert!(!state.find_pvp_key(10, 20));
    
    // Test Fight
    state.handle_pvp_packet_mode(10, 20, PvpMode::Fight);
    assert!(state.find_pvp_key(10, 20));
    
    // Test Revenge
    state.handle_pvp_packet_mode(10, 20, PvpMode::Revenge);
    assert!(!state.find_pvp_key(10, 20));
    
    // Test None
    state.insert_pvp_key(10, 20);
    state.handle_pvp_packet_mode(10, 20, PvpMode::None);
    assert!(!state.find_pvp_key(10, 20));
}

#[test]
fn test_duel_mode_and_pvp_instance() {
    let mut state = PvpDuelStateMachine::new();
    
    assert!(!state.is_pvp_instance(10, 0, 20, 0));
    
    // Duel mode forces is_pvp_instance to return true
    state.set_duel_mode(DuelMode::Start);
    assert!(state.is_pvp_instance(10, 0, 20, 0));
    
    state.set_duel_mode(DuelMode::None);
    assert!(!state.is_pvp_instance(10, 0, 20, 0));
    
    // Insert PVP key directly
    state.insert_pvp_key(10, 20);
    assert!(state.is_pvp_instance(10, 0, 20, 0));
    
    // Insert GVG key directly
    state.remove_pvp_key(10, 20);
    assert!(!state.is_pvp_instance(10, 0, 20, 0));
    state.insert_gvg_key(1, 2);
    assert!(state.is_pvp_instance(10, 1, 20, 2));
}

#[test]
fn test_safe_zone_is_in_safe() {
    let state = PvpDuelStateMachine::new();
    let terrain = MockTerrain {
        safe_zones: vec![(100.0, 100.0)],
    };
    
    let safe_pos = PixelPosition::new(100.0, 100.0, 0.0);
    let unsafe_pos = PixelPosition::new(200.0, 200.0, 0.0);
    
    assert!(state.is_in_safe(safe_pos, &terrain));
    assert!(!state.is_in_safe(unsafe_pos, &terrain));
}

#[test]
fn test_safe_zone_check_attacking() {
    let state = PvpDuelStateMachine::new();
    let terrain = MockTerrain {
        safe_zones: vec![(100.0, 100.0)],
    };
    
    let safe_pos = PixelPosition::new(100.0, 100.0, 0.0);
    let unsafe_pos1 = PixelPosition::new(200.0, 200.0, 0.0);
    let unsafe_pos2 = PixelPosition::new(300.0, 300.0, 0.0);
    
    // Both unsafe -> attack allowed
    assert!(state.check_attacking(unsafe_pos1, unsafe_pos2, &terrain));
    
    // Attacker in safe zone -> attack blocked
    assert!(!state.check_attacking(safe_pos, unsafe_pos1, &terrain));
    
    // Target in safe zone -> attack blocked
    assert!(!state.check_attacking(unsafe_pos1, safe_pos, &terrain));
    
    // Both in safe zone -> attack blocked
    assert!(!state.check_attacking(safe_pos, safe_pos, &terrain));
}
