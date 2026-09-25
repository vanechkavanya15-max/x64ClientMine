use m2_client::input_mouse::*;

#[derive(Default)]
pub struct MockContext {
    pub can_move: bool,
    pub main_actor_ptr: bool,
    pub is_open_private_shop: bool,
    pub is_processing_emotion: bool,
    pub is_sleep: bool,
    pub is_lock: bool,
    pub is_movie_mode: bool,

    pub mouse_pos: (i32, i32),
    pub screen_size: (i32, i32),
    pub mouse_dir_rot: f32,

    pub move_to_direction_called_with: Option<f32>,
    pub stop_called: bool,

    pub has_current_camera: bool,
    pub begin_camera_drag_called: bool,
    pub end_camera_drag_called: bool,
    pub is_map_ready: bool,
    pub cursor_num_set: Option<i32>,
    pub cursor_mode: i32,
    pub cursor_visible_set: Option<(bool, bool)>,

    pub change_target_called: bool,
    pub picked_actor_ptr: Option<u32>,
    pub open_character_menu_called_with: Option<u32>,
    
    pub picked_item_id: Option<u32>,
    pub picked_actor_id: Option<u32>,
    pub picked_ground_pos: Option<(f32, f32, f32)>,
    pub main_character_index: u32,

    pub press_item_called: Option<u32>,
    pub press_actor_called: Option<(u32, bool)>,
    pub press_ground_called: Option<(f32, f32, f32)>,
    pub press_screen_called: bool,

    pub click_item_called: Option<u32>,
    pub click_actor_called: Option<(u32, bool)>,
    pub click_ground_called: Option<(f32, f32, f32)>,

    pub show_effect_called: Option<(i32, (f32, f32, f32))>,

    pub pressed_keys: Vec<i32>,
    pub attack_called: bool,
    pub use_current_skill_called: bool,

    pub ground_item_pos: Option<(f32, f32, f32)>,
    pub dist_from_dest_pos: f32,
    pub pixel_position: (f32, f32, f32),
    pub rotation: f32,
    pub send_char_state_called: bool,
    pub send_click_item_called: Option<u32>,
    pub move_to_dest_pixel_pos_called: Option<(f32, f32, f32)>,
    
    pub empty_reserved_delay_time: bool,
    pub global_elapsed_time: f32,
    
    pub reserve_process_click_actor_called: bool,
    pub actor_ptr_found: bool,
    pub distance: f32,
    pub is_huge_race: bool,
    pub set_target_called: Option<u32>,
    pub use_skill_res: bool,
    pub use_skill_called: Option<u32>,
    pub move_to_dest_instance_dir_called: Option<u32>,
    pub set_multi_dir_key_state_called: Option<(bool, bool, bool, bool)>,
    pub combo_index: i32,
    
    pub clear_reserved_action_called: bool,
    pub target_vid: u32,
    pub dash_affect: bool,
    pub attackable: bool,
    pub can_attack: bool,
    pub clickable_distance: bool,
    pub npc: bool,
    pub click_actor_packet_called: Option<u32>,
    pub auto_attack_target: Option<u32>,
    pub bow_mode: bool,
    pub can_shot: bool,
    pub attack_to_dest_called: Option<u32>,
    pub movable_ground_distance: bool,
    pub move_dest_pixel_bool_res: bool,
    pub move_dest_pixel_bool_called: Option<(f32, f32, f32)>,
    pub clear_auto_attack_called: bool,
    pub cancel_fishing_res: bool,
}

impl InputMouseContext for MockContext {
    fn can_move(&self) -> bool { self.can_move }
    fn get_main_actor_ptr(&self) -> bool { self.main_actor_ptr }
    fn is_open_private_shop(&self) -> bool { self.is_open_private_shop }
    fn is_processing_emotion(&self) -> bool { self.is_processing_emotion }
    fn is_sleep(&self) -> bool { self.is_sleep }
    fn is_lock(&self) -> bool { self.is_lock }
    fn is_movie_mode(&self) -> bool { self.is_movie_mode }

    fn get_mouse_position(&self) -> (i32, i32) { self.mouse_pos }
    fn get_screen_size(&self) -> (i32, i32) { self.screen_size }
    fn get_mouse_dir_rotation(&self, _x: f32, _y: f32) -> f32 { self.mouse_dir_rot }
    fn move_to_direction(&mut self, dir_rot: f32) -> bool {
        self.move_to_direction_called_with = Some(dir_rot);
        true
    }
    fn stop(&mut self) { self.stop_called = true; }

    fn has_current_camera(&self) -> bool { self.has_current_camera }
    fn begin_camera_drag(&mut self, _x: i32, _y: i32) { self.begin_camera_drag_called = true; }
    fn end_camera_drag(&mut self) -> bool {
        self.end_camera_drag_called = true;
        true
    }
    fn is_map_ready(&self) -> bool { self.is_map_ready }
    fn set_cursor_num(&mut self, cursor_num: i32) { self.cursor_num_set = Some(cursor_num); }
    fn get_cursor_mode(&self) -> i32 { self.cursor_mode }
    fn set_cursor_visible(&mut self, visible: bool, force: bool) { self.cursor_visible_set = Some((visible, force)); }

    fn change_target_to_picked_instance(&mut self) { self.change_target_called = true; }
    fn get_picked_actor_ptr(&self) -> Option<u32> { self.picked_actor_ptr }
    fn open_character_menu(&mut self, virtual_id: u32) { self.open_character_menu_called_with = Some(virtual_id); }
    
    fn get_picked_item_id(&self) -> Option<u32> { self.picked_item_id }
    fn get_picked_actor_id(&self) -> Option<u32> { self.picked_actor_id }
    fn get_picked_ground_pos(&self) -> Option<(f32, f32, f32)> { self.picked_ground_pos }
    fn get_main_character_index(&self) -> u32 { self.main_character_index }

    fn on_press_item(&mut self, item_id: u32) { self.press_item_called = Some(item_id); }
    fn on_press_actor(&mut self, actor_id: u32, is_auto: bool) { self.press_actor_called = Some((actor_id, is_auto)); }
    fn on_press_ground(&mut self, pos: (f32, f32, f32)) { self.press_ground_called = Some(pos); }
    fn on_press_screen(&mut self) { self.press_screen_called = true; }

    fn on_click_item(&mut self, item_id: u32) { self.click_item_called = Some(item_id); }
    fn on_click_actor(&mut self, actor_id: u32, is_auto: bool) { self.click_actor_called = Some((actor_id, is_auto)); }
    fn on_click_ground(&mut self, pos: (f32, f32, f32)) { self.click_ground_called = Some(pos); }

    fn show_effect(&mut self, effect_id: i32, pos: (f32, f32, f32)) { self.show_effect_called = Some((effect_id, pos)); }

    fn is_pressed(&self, key: i32) -> bool { self.pressed_keys.contains(&key) }
    fn attack(&mut self) { self.attack_called = true; }
    fn use_current_skill(&mut self) { self.use_current_skill_called = true; }

    fn get_ground_item_position(&self, _iid: u32) -> Option<(f32, f32, f32)> { self.ground_item_pos }
    fn get_distance_from_dest_pixel_position(&self, _pos: (f32, f32, f32)) -> f32 { self.dist_from_dest_pos }
    fn get_pixel_position(&self) -> (f32, f32, f32) { self.pixel_position }
    fn get_rotation(&self) -> f32 { self.rotation }
    fn send_character_state_packet(&mut self, _pos: (f32, f32, f32), _rot: f32, _func: i32, _arg: i32) { self.send_char_state_called = true; }
    fn send_click_item_packet(&mut self, iid: u32) { self.send_click_item_called = Some(iid); }
    fn move_to_dest_pixel_position_direction(&mut self, pos: (f32, f32, f32)) { self.move_to_dest_pixel_pos_called = Some(pos); }
    fn is_empty_reserved_delay_time(&self, _elapsed_time: f32) -> bool { self.empty_reserved_delay_time }
    fn get_global_elapsed_time(&self) -> f32 { self.global_elapsed_time }
    
    fn reserve_process_click_actor(&mut self) { self.reserve_process_click_actor_called = true; }
    fn find_actor_ptr(&self, _vid: u32) -> bool { self.actor_ptr_found }
    fn get_distance(&self, _vid: u32) -> f32 { self.distance }
    fn is_huge_race(&self, _vid: u32) -> bool { self.is_huge_race }
    fn set_target(&mut self, vid: u32) { self.set_target_called = Some(vid); }
    fn use_skill(&mut self, slot_index: u32) -> bool { self.use_skill_called = Some(slot_index); self.use_skill_res }
    fn move_to_dest_instance_direction(&mut self, vid: u32) { self.move_to_dest_instance_dir_called = Some(vid); }
    fn set_multi_dir_key_state(&mut self, left: bool, right: bool, up: bool, down: bool) { self.set_multi_dir_key_state_called = Some((left, right, up, down)); }
    fn get_combo_index(&self) -> i32 { self.combo_index }
    
    fn clear_reserved_action(&mut self) { self.clear_reserved_action_called = true; }
    fn get_target_vid(&self) -> u32 { self.target_vid }
    fn check_dash_affect(&self) -> bool { self.dash_affect }
    fn is_attackable_instance(&self, _vid: u32) -> bool { self.attackable }
    fn can_attack(&self) -> bool { self.can_attack }
    fn is_clickable_distance_dest_instance(&self, _vid: u32) -> bool { self.clickable_distance }
    fn is_npc(&self, _vid: u32) -> bool { self.npc }
    fn send_click_actor_packet(&mut self, vid: u32) { self.click_actor_packet_called = Some(vid); }
    fn set_auto_attack_target_actor_id(&mut self, vid: u32) { self.auto_attack_target = Some(vid); }
    fn is_bow_mode(&self) -> bool { self.bow_mode }
    fn can_shot(&self, _vid: u32) -> bool { self.can_shot }
    fn attack_to_dest_instance_direction(&mut self, vid: u32) { self.attack_to_dest_called = Some(vid); }
    fn is_movable_ground_distance(&self, _pos: (f32, f32, f32)) -> bool { self.movable_ground_distance }
    fn move_to_dest_pixel_position_direction_bool(&mut self, pos: (f32, f32, f32)) -> bool { 
        self.move_dest_pixel_bool_called = Some(pos);
        self.move_dest_pixel_bool_res 
    }
    fn clear_auto_attack_target_actor_id(&mut self) { self.clear_auto_attack_called = true; }
    fn cancel_fishing(&mut self) -> bool { self.cancel_fishing_res }
}

#[test]
fn test_integration_input_mouse_new() {
    let instance = InputMouse::new();
    assert!(instance.is_ready());
}

#[test]
fn test_set_mouse_move_state_press() {
    let mut mouse = InputMouse::new();
    let mut ctx = MockContext::default();
    ctx.mouse_pos = (100, 200);
    ctx.screen_size = (800, 600);
    ctx.mouse_dir_rot = 90.0;
    
    mouse.set_mouse_move_state(&mut ctx, MouseButtonState::Press);
    
    assert!(mouse.m_is_dir_mov);
    assert_eq!(ctx.move_to_direction_called_with, Some(90.0));
}

#[test]
fn test_set_mouse_camera_state() {
    let mut mouse = InputMouse::new();
    let mut ctx = MockContext::default();
    ctx.has_current_camera = true;
    ctx.is_map_ready = true;
    
    mouse.set_mouse_camera_state(&mut ctx, MouseButtonState::Press);
    assert!(ctx.begin_camera_drag_called);
}

#[test]
fn test_set_mouse_smart_state_click_ground() {
    let mut mouse = InputMouse::new();
    let mut ctx = MockContext::default();
    ctx.main_actor_ptr = true;
    ctx.picked_ground_pos = Some((10.0, 20.0, 30.0));
    ctx.movable_ground_distance = true;
    ctx.move_dest_pixel_bool_res = true;
    
    mouse.set_mouse_smart_state(&mut ctx, MouseButtonState::Click, false);
    
    assert!(!mouse.m_is_smt_mov);
    assert_eq!(ctx.move_dest_pixel_bool_called, Some((10.0, 20.0, 30.0)));
}

#[test]
fn test_set_mouse_smart_state_press_actor() {
    let mut mouse = InputMouse::new();
    let mut ctx = MockContext::default();
    ctx.main_actor_ptr = true;
    ctx.picked_actor_id = Some(123);
    ctx.main_character_index = 456;
    ctx.actor_ptr_found = true;
    ctx.can_attack = true;
    ctx.attackable = true;
    ctx.clickable_distance = true;
    
    mouse.set_mouse_smart_state(&mut ctx, MouseButtonState::Press, true);
    
    assert!(mouse.m_is_smt_mov);
    assert_eq!(ctx.auto_attack_target, Some(123));
    assert_eq!(ctx.attack_to_dest_called, Some(123));
}
