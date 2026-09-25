//! Modul `player_input_logic` — Automatyka podchodzenia do celu, auto-ataku i zbierania dropu
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayerInput.cpp`
//! Opis: Odwzorowanie __Update_AutoAttack oraz PickCloseItem: automatyczny bieg do wskazanego wroga i rozpoczecie ataku po wejsciu w zasieg broni, wyszukiwanie najblizszego przedmiotu na ziemi.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReservedMode {
    #[default]
    None,
    UseSkill,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait InstanceBase {
    fn is_mounting_horse(&self) -> bool;
    fn can_attack_horse_level(&self) -> bool;
    fn is_dead(&self) -> bool;
    fn is_sleep(&self) -> bool;
    fn is_attackable_instance(&self, victim: &dyn InstanceBase) -> bool;
    fn get_pixel_position(&self) -> PixelPosition;
}

pub trait GameWorld {
    fn get_main_actor(&self) -> Option<&dyn InstanceBase>;
    fn find_actor(&self, vid: u32) -> Option<&dyn InstanceBase>;
    fn check_dash_affect(&self, actor: &dyn InstanceBase) -> bool;
    fn is_charge_skill(&self, skill_index: u32) -> bool;
    fn get_close_item(&self, pos: PixelPosition, distance: u32) -> Option<u32>;
    fn send_click_item_packet(&mut self, item_id: u32);
    fn reserve_click_actor(&mut self, vid: u32);
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInputLogic {
    pub initialized: bool,
    pub auto_attack_target_vid: u32,
    pub reserved_mode: ReservedMode,
    pub reserved_skill_slot_index: u32,
}

impl PlayerInputLogic {
    pub fn new() -> Self {
        Self {
            initialized: true,
            auto_attack_target_vid: 0,
            reserved_mode: ReservedMode::None,
            reserved_skill_slot_index: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn get_pickable_distance(&self, world: &dyn GameWorld) -> u32 {
        if let Some(main_actor) = world.get_main_actor() {
            if main_actor.is_mounting_horse() {
                return 500;
            }
        }
        300
    }

    pub fn pick_close_item(&self, world: &mut dyn GameWorld) {
        if let Some(main_actor) = world.get_main_actor() {
            let pos = main_actor.get_pixel_position();
            let distance = self.get_pickable_distance(world);

            if let Some(item_id) = world.get_close_item(pos, distance) {
                world.send_click_item_packet(item_id);
            }
        }
    }

    pub fn clear_auto_attack_target_actor_id(&mut self) {
        self.auto_attack_target_vid = 0;
    }

    pub fn is_using_charge_skill(&self, world: &dyn GameWorld) -> bool {
        let main_actor = match world.get_main_actor() {
            Some(actor) => actor,
            None => return false,
        };

        if world.check_dash_affect(main_actor) {
            return true;
        }

        if self.reserved_mode != ReservedMode::UseSkill {
            return false;
        }

        if self.reserved_skill_slot_index >= 255 {
            return false; // SKILL_MAX_NUM = 255 zakladajac standardy
        }

        world.is_charge_skill(self.reserved_skill_slot_index)
    }

    pub fn update_auto_attack(&mut self, world: &mut dyn GameWorld) {
        if self.auto_attack_target_vid == 0 {
            return;
        }

        let main_actor = match world.get_main_actor() {
            Some(actor) => actor,
            None => return,
        };

        if self.is_using_charge_skill(world) {
            return;
        }

        let victim = world.find_actor(self.auto_attack_target_vid);
        
        match victim {
            None => {
                self.clear_auto_attack_target_actor_id();
            }
            Some(victim) => {
                if victim.is_dead() {
                    self.clear_auto_attack_target_actor_id();
                } else if main_actor.is_mounting_horse() && !main_actor.can_attack_horse_level() {
                    self.clear_auto_attack_target_actor_id();
                } else if main_actor.is_attackable_instance(victim) {
                    if victim.is_sleep() || main_actor.is_sleep() {
                        // Nie mozna atakowac w snie, przerywamy albo czekamy
                    } else {
                        world.reserve_click_actor(self.auto_attack_target_vid);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_player_input_logic() {
        let instance = PlayerInputLogic::new();
        assert!(instance.is_ready());
    }
}
