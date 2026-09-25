use m2_client::player_input_logic::*;
use std::collections::HashMap;

struct MockActor {
    mounting_horse: bool,
    can_attack_horse_level: bool,
    dead: bool,
    sleep: bool,
    attackable: bool,
    pos: PixelPosition,
}

impl InstanceBase for MockActor {
    fn is_mounting_horse(&self) -> bool { self.mounting_horse }
    fn can_attack_horse_level(&self) -> bool { self.can_attack_horse_level }
    fn is_dead(&self) -> bool { self.dead }
    fn is_sleep(&self) -> bool { self.sleep }
    fn is_attackable_instance(&self, _victim: &dyn InstanceBase) -> bool { self.attackable }
    fn get_pixel_position(&self) -> PixelPosition { self.pos }
}

impl Default for MockActor {
    fn default() -> Self {
        Self {
            mounting_horse: false,
            can_attack_horse_level: true,
            dead: false,
            sleep: false,
            attackable: true,
            pos: PixelPosition { x: 0.0, y: 0.0, z: 0.0 },
        }
    }
}

struct MockGameWorld {
    main_actor: Option<MockActor>,
    actors: HashMap<u32, MockActor>,
    dash_affect: bool,
    charge_skill: bool,
    close_item: Option<u32>,
    clicked_items: Vec<u32>,
    reserved_actors: Vec<u32>,
}

impl Default for MockGameWorld {
    fn default() -> Self {
        Self {
            main_actor: Some(MockActor::default()),
            actors: HashMap::new(),
            dash_affect: false,
            charge_skill: false,
            close_item: None,
            clicked_items: Vec::new(),
            reserved_actors: Vec::new(),
        }
    }
}

impl GameWorld for MockGameWorld {
    fn get_main_actor(&self) -> Option<&dyn InstanceBase> {
        self.main_actor.as_ref().map(|a| a as &dyn InstanceBase)
    }

    fn find_actor(&self, vid: u32) -> Option<&dyn InstanceBase> {
        self.actors.get(&vid).map(|a| a as &dyn InstanceBase)
    }

    fn check_dash_affect(&self, _actor: &dyn InstanceBase) -> bool {
        self.dash_affect
    }

    fn is_charge_skill(&self, _skill_index: u32) -> bool {
        self.charge_skill
    }

    fn get_close_item(&self, _pos: PixelPosition, _distance: u32) -> Option<u32> {
        self.close_item
    }

    fn send_click_item_packet(&mut self, item_id: u32) {
        self.clicked_items.push(item_id);
    }

    fn reserve_click_actor(&mut self, vid: u32) {
        self.reserved_actors.push(vid);
    }
}

#[test]
fn test_integration_player_input_logic() {
    let instance = PlayerInputLogic::new();
    assert!(instance.is_ready());
}

#[test]
fn test_get_pickable_distance() {
    let logic = PlayerInputLogic::new();
    let mut world = MockGameWorld::default();

    // Default distance on foot is 300
    assert_eq!(logic.get_pickable_distance(&world), 300);

    // Distance on horseback is 500
    world.main_actor.as_mut().unwrap().mounting_horse = true;
    assert_eq!(logic.get_pickable_distance(&world), 500);

    // Distance when no main actor is 300
    world.main_actor = None;
    assert_eq!(logic.get_pickable_distance(&world), 300);
}

#[test]
fn test_pick_close_item() {
    let logic = PlayerInputLogic::new();
    let mut world = MockGameWorld::default();
    
    // No close item
    logic.pick_close_item(&mut world);
    assert!(world.clicked_items.is_empty());

    // Found close item
    world.close_item = Some(42);
    logic.pick_close_item(&mut world);
    assert_eq!(world.clicked_items.len(), 1);
    assert_eq!(world.clicked_items[0], 42);
}

#[test]
fn test_is_using_charge_skill() {
    let mut logic = PlayerInputLogic::new();
    let mut world = MockGameWorld::default();

    assert!(!logic.is_using_charge_skill(&world));

    // True because of dash affect
    world.dash_affect = true;
    assert!(logic.is_using_charge_skill(&world));
    world.dash_affect = false;

    // True because of reserved charge skill
    logic.reserved_mode = ReservedMode::UseSkill;
    logic.reserved_skill_slot_index = 10;
    world.charge_skill = true;
    assert!(logic.is_using_charge_skill(&world));

    // False if reserved mode is None
    logic.reserved_mode = ReservedMode::None;
    assert!(!logic.is_using_charge_skill(&world));
}

#[test]
fn test_update_auto_attack() {
    let mut logic = PlayerInputLogic::new();
    let mut world = MockGameWorld::default();

    // Do nothing if no target
    logic.update_auto_attack(&mut world);
    assert!(world.reserved_actors.is_empty());

    // Do nothing if using charge skill (dash_affect)
    logic.auto_attack_target_vid = 100;
    world.dash_affect = true;
    logic.update_auto_attack(&mut world);
    assert_eq!(logic.auto_attack_target_vid, 100); // Should not clear target
    assert!(world.reserved_actors.is_empty());
    world.dash_affect = false;

    // Clear target if victim not found
    logic.update_auto_attack(&mut world);
    assert_eq!(logic.auto_attack_target_vid, 0);

    // Clear target if victim is dead
    logic.auto_attack_target_vid = 100;
    let mut victim = MockActor::default();
    victim.dead = true;
    world.actors.insert(100, victim);
    logic.update_auto_attack(&mut world);
    assert_eq!(logic.auto_attack_target_vid, 0);

    // Clear target if main actor is on horse but can't attack
    logic.auto_attack_target_vid = 100;
    let mut victim = MockActor::default();
    victim.dead = false;
    world.actors.insert(100, victim);
    world.main_actor.as_mut().unwrap().mounting_horse = true;
    world.main_actor.as_mut().unwrap().can_attack_horse_level = false;
    logic.update_auto_attack(&mut world);
    assert_eq!(logic.auto_attack_target_vid, 0);

    // Do not clear, but do not attack if in sleep
    world.main_actor.as_mut().unwrap().mounting_horse = false;
    world.main_actor.as_mut().unwrap().can_attack_horse_level = true;
    logic.auto_attack_target_vid = 100;
    world.main_actor.as_mut().unwrap().sleep = true;
    logic.update_auto_attack(&mut world);
    assert_eq!(logic.auto_attack_target_vid, 100);
    assert!(world.reserved_actors.is_empty());

    // Successfully reserve click actor
    world.main_actor.as_mut().unwrap().sleep = false;
    logic.update_auto_attack(&mut world);
    assert_eq!(world.reserved_actors.len(), 1);
    assert_eq!(world.reserved_actors[0], 100);
}
