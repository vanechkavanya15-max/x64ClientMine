#[path = "../src/actor_effect_system_wire.rs"]
mod actor_effect_system_wire;

use actor_effect_system_wire::{ActorEffectSystem, ActorEffectType, ArmorSubType, DamageFlag, ItemData, ItemType, RefinedEffect, WeaponSubType};

#[test]
fn test_actor_effect_system_init() {
    let system = ActorEffectSystem::new();
    assert_eq!(system.is_stunned, false);
    assert_eq!(system.is_poisoned, false);
    assert_eq!(system.active_effects.len(), 0);
    assert_eq!(system.damage_queue.len(), 0);
}

#[test]
fn test_actor_effect_stun_poison() {
    let mut system = ActorEffectSystem::new();
    
    system.set_stunned(true);
    assert!(system.is_stunned);
    assert!(system.active_effects.contains(&(ActorEffectType::Stun as u32)));
    
    system.set_poisoned(true);
    assert!(system.is_poisoned);
    assert!(system.active_effects.contains(&(ActorEffectType::DamagePoison as u32)));
    
    system.set_stunned(false);
    assert!(!system.active_effects.contains(&(ActorEffectType::Stun as u32)));
}

#[test]
fn test_refine_logic_dagger_plus9() {
    let mut system = ActorEffectSystem::new();
    
    // Simulating Dagger +9. 9 + 3 socket = 12. max(12, 3) - 3 = 9 refine points.
    let item = ItemData {
        vnum: 1109,
        item_type: ItemType::Weapon,
        sub_type: WeaponSubType::Dagger as u8,
        refine: 9,
        socket_count: 3,
    };
    
    system.get_refined_effect(&item);
    
    let base = ActorEffectType::Refined as u32;
    let expected_right = base + RefinedEffect::SmallSwordRefined7 as u32 + 9 - 7;
    let expected_left = base + RefinedEffect::SmallSwordRefined7Left as u32 + 9 - 7;
    
    assert_eq!(system.sword_refine_effect_right, expected_right);
    assert_eq!(system.sword_refine_effect_left, expected_left);
    assert!(system.active_effects.contains(&expected_right));
    assert!(system.active_effects.contains(&expected_left));
}

#[test]
fn test_refine_logic_body_armor_plus9() {
    let mut system = ActorEffectSystem::new();
    
    let item = ItemData {
        vnum: 11299,
        item_type: ItemType::Armor,
        sub_type: ArmorSubType::Body as u8,
        refine: 9,
        socket_count: 3,
    };
    
    system.get_refined_effect(&item);
    
    let base = ActorEffectType::Refined as u32;
    let expected = base + RefinedEffect::BodyArmorRefined7 as u32 + 9 - 7;
    
    assert_eq!(system.armor_refine_effect, expected);
    assert!(system.active_effects.contains(&expected));
}

#[test]
fn test_refine_logic_special_armor_12010() {
    let mut system = ActorEffectSystem::new();
    
    let item = ItemData {
        vnum: 12015,
        item_type: ItemType::Armor,
        sub_type: ArmorSubType::Body as u8,
        refine: 4, // refine < 7 so no base refine effect
        socket_count: 0,
    };
    
    system.get_refined_effect(&item);
    
    let base = ActorEffectType::Refined as u32;
    let special1 = base + RefinedEffect::BodyArmorSpecial as u32;
    let special2 = base + RefinedEffect::BodyArmorSpecial2 as u32;
    
    // Refine < 7, but special armor vnum 12010-12049 applies these two effects unconditionally.
    assert!(system.active_effects.contains(&special1));
    assert!(system.active_effects.contains(&special2));
    assert_eq!(system.armor_refine_effect, 0); // Not +7 or higher
}

#[test]
fn test_refine_logic_no_effect_under_7() {
    let mut system = ActorEffectSystem::new();
    
    let item = ItemData {
        vnum: 19,
        item_type: ItemType::Weapon,
        sub_type: WeaponSubType::Sword as u8,
        refine: 6,
        socket_count: 0,
    }; // max(6+0, 3) - 3 = 3 -> < 7 so no effect
    
    system.get_refined_effect(&item);
    
    assert_eq!(system.sword_refine_effect_right, 0);
    assert_eq!(system.active_effects.len(), 0);
}

#[test]
fn test_damage_queue_and_process() {
    let mut system = ActorEffectSystem::new();
    
    // Add Miss/Dodge self
    system.add_damage_effect(0, DamageFlag::Dodge as u8, true, true);
    let res = system.process_damage();
    assert_eq!(res, Some(ActorEffectType::DamageMiss as u32));
    
    // Add Damage Self toggle
    system.add_damage_effect(100, DamageFlag::Normal as u8, true, true);
    let res1 = system.process_damage();
    assert_eq!(res1, Some(ActorEffectType::DamageSelf as u32));
    
    system.add_damage_effect(200, DamageFlag::Normal as u8, true, true);
    let res2 = system.process_damage();
    assert_eq!(res2, Some(ActorEffectType::DamageSelf2 as u32));
    
    // Add Damage Target
    system.add_damage_effect(300, DamageFlag::Normal as u8, false, true);
    let res3 = system.process_damage();
    assert_eq!(res3, Some(ActorEffectType::DamageTarget as u32));
}
