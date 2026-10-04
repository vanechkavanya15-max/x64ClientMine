use std::mem::size_of;

pub const HORSE_SKILL_SLASH: u32 = 121;
pub const HORSE_SKILL_STOMP: u32 = 122;
pub const HORSE_SKILL_CHARGE: u32 = 123;
pub const HORSE_SKILL_ARROW_SHOWER: u32 = 124;

pub const HEADER_CG_USE_SKILL: u16 = 0x0402;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGUseSkill {
    pub header: u16,
    pub length: u16,
    pub dw_vnum: u32,
    pub dw_target_vid: u32,
}

pub struct HorseCombatSkillEngine;

impl HorseCombatSkillEngine {
    /// Weryfikuje czy gracz ma odpowiedni poziom konia do uzywania umiejetnosci specjalnych
    /// Wymagany jest Kon Militarny (poziom 21+)
    pub fn can_use_horse_skill(horse_level: u32, skill_vnum: u32) -> bool {
        // Skille z siodla: 121 (Ciecie), 122 (Tapanie), 123 (Szarza), 124 (Grad Strzal)
        let is_horse_skill = match skill_vnum {
            HORSE_SKILL_SLASH | HORSE_SKILL_STOMP | HORSE_SKILL_CHARGE | HORSE_SKILL_ARROW_SHOWER => true,
            _ => false,
        };

        if !is_horse_skill {
            return false;
        }

        // Tylko Militarny kon (level 21+) moze uzywac skilli, bojowy (11-20) nie.
        // Zgodnie z pamiecia: "Kon bojowy 11-20 vs Kon militarny 21+"
        horse_level >= 21
    }

    /// Oblicza fizyke odepchniecia wrogow w stozku szarzy.
    /// Zwraca (odleglosc, wektor_x, wektor_y)
    pub fn calculate_charge_knockback(
        attacker_x: f32, attacker_y: f32, attacker_rot: f32,
        target_x: f32, target_y: f32,
        charge_distance: f32, charge_cone_angle_degrees: f32,
        impulse_strength: f32
    ) -> Option<(f32, f32, f32)> {
        let dx = target_x - attacker_x;
        let dy = target_y - attacker_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > charge_distance {
            return None; // Cel poza zasiegiem szarzy
        }
        if distance == 0.0 {
            return None; // Ten sam punkt
        }

        // Kąt w radianach (kierunek patrzena atakujacego)
        // W Metin2, 0 stopni to oś -Y (lub +Y w zaleznosci od konwencji), 
        // przeliczamy rotacje na radian i wektor kierunku.
        // Konwencja matematyczna: rot_rad = (270.0 - rot) * PI / 180.0
        let rot_rad = (270.0 - attacker_rot) * std::f32::consts::PI / 180.0;
        let dir_x = rot_rad.cos();
        let dir_y = rot_rad.sin();

        // Znormalizowany wektor do celu
        let to_target_x = dx / distance;
        let to_target_y = dy / distance;

        // Iloczyn skalarny miedzy kierunkiem ataku a celem
        let dot_product = dir_x * to_target_x + dir_y * to_target_y;
        let angle_between = dot_product.clamp(-1.0, 1.0).acos(); // w radianach

        let cone_angle_rad = (charge_cone_angle_degrees / 2.0) * std::f32::consts::PI / 180.0;

        if angle_between <= cone_angle_rad {
            // Wewnatrz stozka - odepchniecie w kierunku `to_target`
            let knockback_x = to_target_x * impulse_strength;
            let knockback_y = to_target_y * impulse_strength;
            return Some((distance, knockback_x, knockback_y));
        }

        None // Cel nie miesci sie w stozku
    }

    /// Buduje pakiet `HEADER_CG_USE_SKILL` (VNUM 121, 122, 123, 124)
    pub fn build_horse_skill_packet(skill_vnum: u32, target_vid: u32) -> Option<Vec<u8>> {
        let is_horse_skill = match skill_vnum {
            HORSE_SKILL_SLASH | HORSE_SKILL_STOMP | HORSE_SKILL_CHARGE | HORSE_SKILL_ARROW_SHOWER => true,
            _ => false,
        };

        if !is_horse_skill {
            return None;
        }

        let packet = TPacketCGUseSkill {
            header: HEADER_CG_USE_SKILL,
            length: size_of::<TPacketCGUseSkill>() as u16,
            dw_vnum: skill_vnum,
            dw_target_vid: target_vid,
        };

        let bytes: [u8; size_of::<TPacketCGUseSkill>()] = unsafe { std::mem::transmute(packet) };
        Some(bytes.to_vec())
    }
}
