use std::collections::HashMap;
use crate::equipment_slots;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatDelta {
    pub apply_type: u8,
    pub delta: i32,
}

pub fn calculate_stat_deltas(
    equipped_stats: &[(u8, i32)],
    hovered_stats: &[(u8, i32)],
) -> Vec<StatDelta> {
    let mut stats_map: HashMap<u8, i32> = HashMap::new();

    // Subtract equipped stats
    for &(apply_type, value) in equipped_stats {
        if apply_type != 0 {
            *stats_map.entry(apply_type).or_insert(0) -= value;
        }
    }

    // Add hovered stats
    for &(apply_type, value) in hovered_stats {
        if apply_type != 0 {
            *stats_map.entry(apply_type).or_insert(0) += value;
        }
    }

    let mut deltas: Vec<StatDelta> = stats_map
        .into_iter()
        .filter(|&(_, delta)| delta != 0)
        .map(|(apply_type, delta)| StatDelta { apply_type, delta })
        .collect();

    // Sort by apply_type to maintain consistent order
    deltas.sort_by_key(|stat| stat.apply_type);

    deltas
}

pub fn find_equipped_slot(wearable_flag: u32) -> Option<u8> {
    if (wearable_flag & equipment_slots::WEARABLE_BODY) != 0 {
        return Some(equipment_slots::SLOT_ARMOR);
    }
    if (wearable_flag & equipment_slots::WEARABLE_HEAD) != 0 {
        return Some(equipment_slots::SLOT_HEAD);
    }
    if (wearable_flag & equipment_slots::WEARABLE_FOOTS) != 0 {
        return Some(equipment_slots::SLOT_FOOTS);
    }
    if (wearable_flag & equipment_slots::WEARABLE_WRIST) != 0 {
        return Some(equipment_slots::SLOT_WRIST);
    }
    if (wearable_flag & equipment_slots::WEARABLE_WEAPON) != 0 {
        return Some(equipment_slots::SLOT_WEAPON);
    }
    if (wearable_flag & equipment_slots::WEARABLE_NECK) != 0 {
        return Some(equipment_slots::SLOT_NECK);
    }
    if (wearable_flag & equipment_slots::WEARABLE_EAR) != 0 {
        return Some(equipment_slots::SLOT_EAR);
    }
    if (wearable_flag & equipment_slots::WEARABLE_UNIQUE) != 0 {
        // Just return the first unique slot, though multiple exist.
        return Some(equipment_slots::SLOT_UNIQUE1);
    }
    if (wearable_flag & equipment_slots::WEARABLE_ARROW) != 0 {
        return Some(equipment_slots::SLOT_ARROW);
    }
    if (wearable_flag & equipment_slots::WEARABLE_SHIELD) != 0 {
        return Some(equipment_slots::SLOT_SHIELD);
    }
    if (wearable_flag & equipment_slots::WEARABLE_HAIR) != 0 {
        return Some(equipment_slots::SLOT_HEAD); // Usually mapped to head in simple logic, or costume hair
    }
    if (wearable_flag & equipment_slots::WEARABLE_ABILITY) != 0 {
        return Some(equipment_slots::SLOT_ABILITY1);
    }
    if (wearable_flag & equipment_slots::WEARABLE_COSTUME_BODY) != 0 {
        return Some(equipment_slots::SLOT_COSTUME_BODY);
    }
    if (wearable_flag & equipment_slots::WEARABLE_COSTUME_HAIR) != 0 {
        return Some(equipment_slots::SLOT_COSTUME_HAIR);
    }
    if (wearable_flag & equipment_slots::WEARABLE_BELT) != 0 {
        return Some(equipment_slots::SLOT_BELT);
    }

    None
}
