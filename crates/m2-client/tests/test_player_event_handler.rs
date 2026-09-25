use m2_client::player_event_handler::*;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct MockNetworkStream {
    pub sent_state_packets: Mutex<Vec<(PixelPosition, f32, u32, u32)>>,
    pub sent_attack_packets: Mutex<Vec<(u32, u32)>>,
    pub sent_sync_elements: Mutex<Vec<(u32, i32, i32)>>,
    pub sent_sequences: Mutex<u32>,
    pub sent_fly_targets: Mutex<Vec<(u32, PixelPosition)>>,
    pub sent_shoots: Mutex<Vec<u32>>,
    pub sent_raw: Mutex<Vec<(usize, Vec<u8>)>>,
}

impl NetworkStream for MockNetworkStream {
    fn send_character_state_packet(&self, pos: PixelPosition, rot: f32, func: u32, arg: u32) {
        self.sent_state_packets.lock().unwrap().push((pos, rot, func, arg));
    }
    fn send_attack_packet(&self, skill_index: u32, vid: u32) {
        self.sent_attack_packets.lock().unwrap().push((skill_index, vid));
    }
    fn send_sync_position_element_packet(&self, vid: u32, x: i32, y: i32) {
        self.sent_sync_elements.lock().unwrap().push((vid, x, y));
    }
    fn send_sequence(&self) {
        *self.sent_sequences.lock().unwrap() += 1;
    }
    fn send_fly_targeting_packet(&self, vid: u32, pos: PixelPosition) {
        self.sent_fly_targets.lock().unwrap().push((vid, pos));
    }
    fn send_shoot_packet(&self, skill_index: u32) {
        self.sent_shoots.lock().unwrap().push(skill_index);
    }
    fn send(&self, size: usize, data: &[u8]) {
        self.sent_raw.lock().unwrap().push((size, data.to_vec()));
    }
}

#[derive(Default)]
struct MockPlayer {
    pub clear_affects_called: Mutex<bool>,
    pub set_affect_called: Mutex<Option<u32>>,
    pub reset_affect_called: Mutex<Option<u32>>,
    pub new_stop_called: Mutex<bool>,
    pub set_target_called: Mutex<Option<(u32, bool)>>,
}

impl Player for MockPlayer {
    fn clear_affects(&self) { *self.clear_affects_called.lock().unwrap() = true; }
    fn set_affect(&self, affect: u32) { *self.set_affect_called.lock().unwrap() = Some(affect); }
    fn reset_affect(&self, affect: u32) { *self.reset_affect_called.lock().unwrap() = Some(affect); }
    fn new_stop(&self) { *self.new_stop_called.lock().unwrap() = true; }
    fn set_target(&self, vid: u32, is_pc: bool) { *self.set_target_called.lock().unwrap() = Some((vid, is_pc)); }
}

#[derive(Default)]
struct MockCharacterManager {
    pub adjust_collision_called: Mutex<bool>,
}

impl CharacterManager for MockCharacterManager {
    fn adjust_collision_with_other_objects(&self, _actor: &dyn ActorInstance) {
        *self.adjust_collision_called.lock().unwrap() = true;
    }
    fn get_instance_ptr(&self, _vid: u32) -> Option<Rc<RefCell<dyn InstanceBase>>> { None }
}

#[derive(Default, Clone)]
struct MockActorInstance {
    pub vid: u32,
    pub pushing: bool,
    pub race: u32,
    pub last_pos: PixelPosition,
    pub temp_push_called: Rc<RefCell<Option<(i32, i32)>>>,
}

impl ActorInstance for MockActorInstance {
    fn get_virtual_id(&self) -> u32 { self.vid }
    fn is_pushing(&self) -> bool { self.pushing }
    fn get_race(&self) -> u32 { self.race }
    fn new_get_last_pixel_position_ref(&self) -> PixelPosition { self.last_pos }
    fn temp_push(&self, x: i32, y: i32) { *self.temp_push_called.borrow_mut() = Some((x, y)); }
}

fn create_handler() -> (PlayerEventHandler, Arc<MockNetworkStream>, Arc<MockPlayer>, Arc<MockCharacterManager>) {
    let ns = Arc::new(MockNetworkStream::default());
    let pl = Arc::new(MockPlayer::default());
    let cm = Arc::new(MockCharacterManager::default());
    let handler = PlayerEventHandler::with_dependencies(ns.clone(), pl.clone(), cm.clone());
    (handler, ns, pl, cm)
}

#[test]
fn test_integration_player_event_handler() {
    let (handler, _, _, _) = create_handler();
    assert!(handler.is_ready());
}

#[test]
fn test_on_syncing() {
    let (mut handler, _, _, _) = create_handler();
    let state = State { p_pos_self: PixelPosition { x: 10.0, y: 20.0, z: 0.0 }, adv_rot_self: 0.0 };
    handler.on_syncing(&state);
    assert_eq!(handler.p_pos_prev_waiting, state.p_pos_self);
}

#[test]
fn test_on_waiting() {
    let (mut handler, ns, _, _) = create_handler();
    handler.next_waiting_notify_time = 0;
    
    let state = State { p_pos_self: PixelPosition { x: 10.0, y: 20.0, z: 0.0 }, adv_rot_self: 0.0 };
    handler.p_pos_prev_waiting = PixelPosition { x: 15.0, y: 20.0, z: 0.0 }; // dist = 5.0
    
    handler.on_waiting(&state);
    assert_eq!(ns.sent_state_packets.lock().unwrap().len(), 1);
    
    // Test dist < 1.0
    handler.next_waiting_notify_time = 0;
    handler.p_pos_prev_waiting = PixelPosition { x: 10.5, y: 20.0, z: 0.0 }; // dist = 0.5
    handler.on_waiting(&state);
    assert_eq!(ns.sent_state_packets.lock().unwrap().len(), 1); // no new packet
}

#[test]
fn test_on_moving() {
    let (mut handler, ns, _, _) = create_handler();
    handler.next_moving_notify_time = 0;
    let state = State { p_pos_self: PixelPosition { x: 10.0, y: 20.0, z: 0.0 }, adv_rot_self: 0.0 };
    handler.on_moving(&state);
    assert_eq!(ns.sent_state_packets.lock().unwrap().len(), 1);
}

#[test]
fn test_on_move() {
    let (mut handler, ns, _, _) = create_handler();
    let state = State { p_pos_self: PixelPosition { x: 10.0, y: 20.0, z: 0.0 }, adv_rot_self: 0.0 };
    handler.on_move(&state);
    assert_eq!(ns.sent_state_packets.lock().unwrap().len(), 1);
}

#[test]
fn test_on_stop() {
    let (mut handler, ns, _, _) = create_handler();
    let state = State { p_pos_self: PixelPosition { x: 10.0, y: 20.0, z: 0.0 }, adv_rot_self: 0.0 };
    handler.on_stop(&state);
    assert_eq!(ns.sent_state_packets.lock().unwrap().len(), 1);
}

#[test]
fn test_on_warp() {
    let (mut handler, ns, _, _) = create_handler();
    let state = State { p_pos_self: PixelPosition { x: 10.0, y: 20.0, z: 0.0 }, adv_rot_self: 0.0 };
    handler.on_warp(&state);
    assert_eq!(ns.sent_state_packets.lock().unwrap().len(), 1);
}


#[derive(Default)]
struct MockGraphicThingInstance {
    pub target_rot: f32,
    pub fly_target_pos: PixelPosition,
    pub shoot_damage_called: RefCell<bool>,
}

impl GraphicThingInstance for MockGraphicThingInstance {
    fn get_target_rotation(&self) -> f32 { self.target_rot }
    fn on_get_fly_target_position(&self) -> PixelPosition { self.fly_target_pos }
    fn on_shoot_damage(&self) { *self.shoot_damage_called.borrow_mut() = true; }
}

#[derive(Default)]
struct MockInstanceBase {
    pub vid: u32,
    pub pos: PixelPosition,
    pub graphic_thing: Rc<RefCell<MockGraphicThingInstance>>,
}

impl InstanceBase for MockInstanceBase {
    fn new_get_pixel_position(&self) -> PixelPosition { self.pos }
    fn get_graphic_thing_instance_ptr(&self) -> Rc<RefCell<dyn GraphicThingInstance>> { self.graphic_thing.clone() }
    fn get_virtual_id(&self) -> u32 { self.vid }
}

#[test]
fn test_on_attack() {
    let (mut handler, ns, _, _) = create_handler();
    let state = State { p_pos_self: PixelPosition::default(), adv_rot_self: 0.0 };
    handler.on_attack(&state, 5);
    let packets = ns.sent_state_packets.lock().unwrap();
    assert_eq!(packets.len(), 1);
    assert_eq!(packets[0].2, FUNC_COMBO);
    assert_eq!(packets[0].3, 5);
}

#[test]
fn test_on_use_skill() {
    let (mut handler, ns, _, _) = create_handler();
    let state = State { p_pos_self: PixelPosition::default(), adv_rot_self: 0.0 };
    handler.on_use_skill(&state, 3, 100);
    let packets = ns.sent_state_packets.lock().unwrap();
    assert_eq!(packets.len(), 1);
    assert_eq!(packets[0].2, FUNC_SKILL | 3);
    assert_eq!(packets[0].3, 100);
}

#[test]
fn test_on_hit() {
    let (mut handler, ns, pl, cm) = create_handler();
    let actor = MockActorInstance {
        vid: 1234,
        pushing: true,
        race: 1, // not huge
        last_pos: PixelPosition { x: 50.0, y: 60.0, z: 0.0 },
        temp_push_called: Rc::new(RefCell::new(None)),
    };

    handler.on_hit(42, &actor, true);

    // Verify Target
    assert_eq!(*pl.set_target_called.lock().unwrap(), Some((1234, false)));
    
    // Verify Network Packet
    let attacks = ns.sent_attack_packets.lock().unwrap();
    assert_eq!(attacks.len(), 1);
    assert_eq!(attacks[0], (42, 1234));

    // Verify Collision & Push
    assert_eq!(*cm.adjust_collision_called.lock().unwrap(), true);
    assert_eq!(*actor.temp_push_called.borrow(), Some((50, 60)));

    // Verify Victim List
    assert_eq!(handler.victims.len(), 1);
    assert_eq!(handler.victims[0].vid, 1234);
    assert_eq!(handler.victims[0].pixel_x, 50);
}

#[test]
fn test_on_hit_huge_race() {
    let (mut handler, ns, pl, _) = create_handler();
    let actor = MockActorInstance {
        vid: 1234,
        pushing: true,
        race: 8000, // huge race
        last_pos: PixelPosition::default(),
        temp_push_called: Rc::new(RefCell::new(None)),
    };
    
    handler.on_hit(42, &actor, false);
    
    assert_eq!(*pl.set_target_called.lock().unwrap(), Some((1234, false)));
    assert_eq!(ns.sent_attack_packets.lock().unwrap().len(), 0); // isSendPacket = false
    assert_eq!(handler.victims.len(), 0); // huge race shouldn't push victim
}

#[test]
fn test_flush_victim_list() {
    let (mut handler, ns, _, _) = create_handler();
    handler.victims.push(SVictim { vid: 1, pixel_x: 10, pixel_y: 20 });
    handler.victims.push(SVictim { vid: 2, pixel_x: 30, pixel_y: 40 });
    
    handler.flush_victim_list();
    
    assert_eq!(handler.victims.len(), 0);
    assert_eq!(ns.sent_raw.lock().unwrap().len(), 1);
    assert_eq!(ns.sent_sync_elements.lock().unwrap().len(), 2);
    assert_eq!(*ns.sent_sequences.lock().unwrap(), 1);
}

#[test]
fn test_player_methods() {
    let (handler, _, pl, _) = create_handler();
    handler.on_clear_affects();
    assert_eq!(*pl.clear_affects_called.lock().unwrap(), true);
    
    handler.on_set_affect(5);
    assert_eq!(*pl.set_affect_called.lock().unwrap(), Some(5));
    
    handler.on_reset_affect(3);
    assert_eq!(*pl.reset_affect_called.lock().unwrap(), Some(3));
    
    handler.on_change_shape();
    assert_eq!(*pl.new_stop_called.lock().unwrap(), true);
    
    handler.on_update();
}

#[test]
fn test_fly_event_handler() {
    let (handler, ns, _, _) = create_handler();
    
    let main_graphic = Rc::new(RefCell::new(MockGraphicThingInstance {
        target_rot: 1.5,
        ..Default::default()
    }));
    let main_inst = Rc::new(RefCell::new(MockInstanceBase {
        vid: 10,
        pos: PixelPosition { x: 1.0, y: 2.0, z: 3.0 },
        graphic_thing: main_graphic,
    }));
    
    let target_graphic = Rc::new(RefCell::new(MockGraphicThingInstance {
        fly_target_pos: PixelPosition { x: 4.0, y: 5.0, z: 6.0 },
        ..Default::default()
    }));
    let target_inst = Rc::new(RefCell::new(MockInstanceBase {
        vid: 20,
        pos: PixelPosition::default(),
        graphic_thing: target_graphic,
    }));
    
    let mut fly_handler = handler.get_normal_bow_attack_fly_event_handler(main_inst, target_inst.clone());
    
    fly_handler.on_set_fly_target();
    {
        let fly_targets = ns.sent_fly_targets.lock().unwrap();
        assert_eq!(fly_targets.len(), 1);
        assert_eq!(fly_targets[0].0, 20);
        assert_eq!(fly_targets[0].1.x, 4.0);
    }
    
    fly_handler.on_shoot(123);
    assert_eq!(*ns.sent_shoots.lock().unwrap(), vec![123]);
    
    // Test remaining empty methods (just to verify they don't panic)
    fly_handler.on_no_target();
    fly_handler.on_exploding_out_of_range();
    fly_handler.on_exploding_at_background();
    fly_handler.on_exploding_at_another_target(1, 2);
    fly_handler.on_exploding_at_target(3);
    
    // Test Change Fly Target
    let target_inst2 = Rc::new(RefCell::new(MockInstanceBase {
        vid: 30,
        ..Default::default()
    }));
    PlayerEventHandler::change_fly_target(&mut fly_handler, target_inst2);
    fly_handler.on_set_fly_target();
    assert_eq!(ns.sent_fly_targets.lock().unwrap().len(), 2);
    assert_eq!(ns.sent_fly_targets.lock().unwrap()[1].0, 30);
}
