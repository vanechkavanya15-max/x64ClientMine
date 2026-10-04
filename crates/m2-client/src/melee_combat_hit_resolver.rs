// melee_combat_hit_resolver.rs - Sferyczny hit-test oreza miecza, katowy filtr stozka ataku i fizyka odrzutu potworow (knockback)
use std::collections::HashSet;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitSphere {
    pub center: [f32; 3],
    pub radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeleeHitResult {
    pub victim_vid: u32,
    pub is_hit: bool,
    pub is_critical: bool,
    pub is_knockdown: bool,
    pub knockback_vector: [f32; 3],
    pub distance: f32,
}

#[derive(Debug, Default)]
pub struct MeleeSwingHitTracker {
    /// Lista VID potworow trafionych w biezacym machnieciu (zapobiega multi-hitom w jednej klatce ataku)
    hit_victims: HashSet<u32>,
    /// Biezacy indeks sekwencji combo
    pub current_combo_index: u32,
}

impl MeleeSwingHitTracker {
    pub fn new() -> Self {
        Self {
            hit_victims: HashSet::new(),
            current_combo_index: 0,
        }
    }

    /// Resetuje stan przy rozpoczeciu nowego uderzenia z sekwencji combo
    pub fn start_new_swing(&mut self, combo_index: u32) {
        self.hit_victims.clear();
        self.current_combo_index = combo_index;
    }

    /// Sprawdza czy cel zostal juz uderzony w tym zamachu
    pub fn has_already_hit(&self, victim_vid: u32) -> bool {
        self.hit_victims.contains(&victim_vid)
    }

    /// Rejestruje uderzenie w ofiare
    pub fn record_hit(&mut self, victim_vid: u32) {
        self.hit_victims.insert(victim_vid);
    }
}

/// Sprawdza czy cel znajduje sie w stozku ataku oraz w zasiegu broni
pub fn test_melee_attack_hit(
    attacker_pos: [f32; 3],
    attacker_rot_deg: f32,
    weapon_reach: f32,
    victim_vid: u32,
    victim_pos: [f32; 3],
    victim_radius: f32,
    tracker: &mut MeleeSwingHitTracker,
    is_critical: bool,
) -> Option<MeleeHitResult> {
    // 1. Sprawdzenie czy ofiara nie byla juz uderzona w tym zamachu
    if tracker.has_already_hit(victim_vid) {
        return None;
    }

    let dx = victim_pos[0] - attacker_pos[0];
    let dy = victim_pos[1] - attacker_pos[1];
    let dz = victim_pos[2] - attacker_pos[2];

    let dist_sq = dx * dx + dy * dy;
    let max_reach = weapon_reach + victim_radius;

    // Poza maksymalnym promieniem zasiegu
    if dist_sq > max_reach * max_reach {
        return None;
    }

    // Roznica wysokosci nie moze byc wieksza niz 150 jednostek
    if dz.abs() > 150.0 {
        return None;
    }

    let dist = dist_sq.sqrt();
    if dist < 0.001 {
        // Cel dokladnie w pozycji atakujacego -> bezposredni hit
        tracker.record_hit(victim_vid);
        return Some(MeleeHitResult {
            victim_vid,
            is_hit: true,
            is_critical,
            is_knockdown: is_critical,
            knockback_vector: [0.0, 100.0, 0.0],
            distance: 0.0,
        });
    }

    // 2. Filtr katowy stozka ataku (120 stopni przed postacia)
    // Kat patrzenia atakujacego w radianach (kierunek wektora w przod)
    let rot_rad = attacker_rot_deg.to_radians();
    let forward_x = -rot_rad.sin();
    let forward_y = rot_rad.cos();

    // Wektor do ofiary (znormalizowany)
    let dir_x = dx / dist;
    let dir_y = dy / dist;

    // Iloczyn skalarny
    let dot = forward_x * dir_x + forward_y * dir_y;

    // Cosinus kata 60 stopni (polowa stozka 120 stopni) = 0.5
    const CONE_COS_THRESHOLD: f32 = 0.5;
    if dot < CONE_COS_THRESHOLD {
        return None;
    }

    // 3. Obliczenie wektora odrzutu (knockback)
    let push_force = if is_critical { 250.0 } else { 120.0 };
    let knockback_x = dir_x * push_force;
    let knockback_y = dir_y * push_force;
    let knockback_z = if is_critical { 50.0 } else { 0.0 };

    tracker.record_hit(victim_vid);

    Some(MeleeHitResult {
        victim_vid,
        is_hit: true,
        is_critical,
        is_knockdown: is_critical,
        knockback_vector: [knockback_x, knockback_y, knockback_z],
        distance: dist,
    })
}

// ============================================================================
// FFI C-Bridge
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_combat_test_melee_hit(
    att_x: f32, att_y: f32, att_z: f32,
    att_rot_deg: f32,
    weapon_reach: f32,
    victim_vid: u32,
    vic_x: f32, vic_y: f32, vic_z: f32,
    vic_radius: f32,
    is_critical: bool,
    out_knockback_x: *mut f32,
    out_knockback_y: *mut f32,
) -> bool {
    let mut tracker = MeleeSwingHitTracker::new();
    let res = test_melee_attack_hit(
        [att_x, att_y, att_z],
        att_rot_deg,
        weapon_reach,
        victim_vid,
        [vic_x, vic_y, vic_z],
        vic_radius,
        &mut tracker,
        is_critical,
    );

    if let Some(hit) = res {
        if !out_knockback_x.is_null() && !out_knockback_y.is_null() {
            unsafe {
                *out_knockback_x = hit.knockback_vector[0];
                *out_knockback_y = hit.knockback_vector[1];
            }
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_melee_hit_in_front_within_reach() {
        let mut tracker = MeleeSwingHitTracker::new();
        // Atakujacy na (0,0,0) patrzy w strone +Y (rot = 0 deg)
        let hit = test_melee_attack_hit(
            [0.0, 0.0, 0.0],
            0.0,
            200.0, // weapon reach
            101,   // victim vid
            [0.0, 100.0, 0.0], // victim 100 units wprost przed graczem
            50.0,  // victim radius
            &mut tracker,
            false,
        );

        assert!(hit.is_some());
        let res = hit.unwrap();
        assert_eq!(res.victim_vid, 101);
        assert!(res.is_hit);
        assert_eq!(res.distance, 100.0);
        // Sprawdzenie czy wektor knockback jest skierowany w przod (+Y)
        assert!(res.knockback_vector[1] > 0.0);
    }

    #[test]
    fn test_melee_hit_behind_rejected_by_cone() {
        let mut tracker = MeleeSwingHitTracker::new();
        // Atakujacy na (0,0,0) patrzy w +Y (rot = 0 deg)
        // Ofiara z tylu (0, -100, 0)
        let hit = test_melee_attack_hit(
            [0.0, 0.0, 0.0],
            0.0,
            200.0,
            102,
            [0.0, -100.0, 0.0],
            50.0,
            &mut tracker,
            false,
        );

        assert!(hit.is_none(), "Cel z tylu musi zostac odrzucony przez stozek ataku");
    }

    #[test]
    fn test_melee_swing_prevents_duplicate_hit() {
        let mut tracker = MeleeSwingHitTracker::new();
        let hit1 = test_melee_attack_hit(
            [0.0, 0.0, 0.0],
            0.0,
            200.0,
            103,
            [0.0, 100.0, 0.0],
            50.0,
            &mut tracker,
            false,
        );
        assert!(hit1.is_some());

        // Drugie wywolanie w tym samym zamachu dla tej samej ofiary
        let hit2 = test_melee_attack_hit(
            [0.0, 0.0, 0.0],
            0.0,
            200.0,
            103,
            [0.0, 100.0, 0.0],
            50.0,
            &mut tracker,
            false,
        );
        assert!(hit2.is_none(), "Nie wolno zadac wielokrotnego trafienia w tym samym zamachu");

        // Nowy zamach combo -> odblokowanie celu
        tracker.start_new_swing(1);
        let hit3 = test_melee_attack_hit(
            [0.0, 0.0, 0.0],
            0.0,
            200.0,
            103,
            [0.0, 100.0, 0.0],
            50.0,
            &mut tracker,
            true, // critical
        );
        assert!(hit3.is_some());
        assert!(hit3.unwrap().is_knockdown);
    }
}
