use m2_client::actor_sync::{ActorSync, IPhysicsWorld, Vector3, TPixelPosition, NAME_DAMAGE_FLYING, NAME_STAND_UP};

struct MockPhysicsWorld {
    collision_at: Option<Vector3>,
}

impl IPhysicsWorld for MockPhysicsWorld {
    fn is_physical_collision(&self, pos: &Vector3) -> bool {
        if let Some(col) = self.collision_at {
            // Proste sprawdzenie bliskosci dla mocka
            let dx = pos.x - col.x;
            let dy = pos.y - col.y;
            let dz = pos.z - col.z;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            if dist_sq < 1.0 {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_integration_actor_sync_initialization() {
    let mut instance = ActorSync::new();
    assert!(instance.is_ready());
    instance.initialize_position_data();
    
    assert_eq!(instance.x, 0.0);
    assert_eq!(instance.y, 0.0);
    assert_eq!(instance.z, 0.0);
    assert_eq!(instance.need_update_collision, false);
}

#[test]
fn test_integration_actor_sync_push_resist_fallen() {
    let mut instance = ActorSync::new();
    instance.resist_fallen = true;
    let world = MockPhysicsWorld { collision_at: None };
    
    instance.push(Some(&world), 100, 100);
    assert!(!instance.blending);
}

#[test]
fn test_integration_actor_sync_push_collision() {
    let mut instance = ActorSync::new();
    instance.initialize_position_data();
    
    // Ustawiamy zeby srodek trasy mial kolizje
    let world = MockPhysicsWorld { collision_at: Some(Vector3::new(50.0, -50.0, 0.0)) };
    
    instance.push(Some(&world), 100, 100); // docelowo (100, -100, 0)
    
    assert!(!instance.blending);
    assert_eq!(instance.blending_position.x, 0.0);
    assert_eq!(instance.blending_position.y, 0.0);
}

#[test]
fn test_integration_actor_sync_push_success_no_skill() {
    let mut instance = ActorSync::new();
    instance.initialize_position_data();
    instance.using_skill = false;
    
    let world = MockPhysicsWorld { collision_at: None };
    
    // Daleki odrzut
    instance.push(Some(&world), 200, 200);
    
    assert!(instance.blending);
    assert_eq!(instance.blending_position.x, 200.0);
    assert_eq!(instance.blending_position.y, -200.0);
    
    // Sprawdzamy kolejke ruchu (distance > 150)
    assert_eq!(instance.motion_queue.len(), 2);
    assert_eq!(instance.motion_queue[0], NAME_DAMAGE_FLYING);
    assert_eq!(instance.motion_queue[1], NAME_STAND_UP);
}

#[test]
fn test_integration_actor_sync_push_success_with_skill() {
    let mut instance = ActorSync::new();
    instance.initialize_position_data();
    instance.using_skill = true;
    
    let world = MockPhysicsWorld { collision_at: None };
    
    // Daleki odrzut
    instance.push(Some(&world), 200, 200);
    
    assert!(instance.blending);
    assert_eq!(instance.blending_position.x, 200.0);
    assert_eq!(instance.blending_position.y, -200.0);
    
    // Sprawdzamy kolejke ruchu, przy skillu nie dodaje motow
    assert!(instance.motion_queue.is_empty());
}

#[test]
fn test_integration_actor_sync_is_syncing() {
    let mut instance = ActorSync::new();
    
    assert!(!instance.is_syncing());
    
    instance.dead = true;
    assert!(instance.is_syncing());
    instance.dead = false;
    
    instance.stun = true;
    assert!(instance.is_syncing());
    instance.stun = false;
    
    instance.blending = true;
    assert!(instance.is_syncing());
}

#[test]
fn test_integration_actor_sync_positions() {
    let mut instance = ActorSync::new();
    instance.initialize_position_data();
    
    let pos = TPixelPosition::new(10.0, 20.0, 30.0);
    instance.set_cur_pixel_position(&pos);
    
    // set_cur_pixel_position odwraca y
    assert_eq!(instance.x, 10.0);
    assert_eq!(instance.y, -20.0);
    assert_eq!(instance.z, 30.0);
    assert!(instance.need_update_collision);
    
    let cur_pos = instance.new_get_cur_pixel_position_ref();
    assert_eq!(cur_pos.x, 10.0);
    assert_eq!(cur_pos.y, 20.0); // Odwraca znowu
    assert_eq!(cur_pos.z, 30.0);
    
    let mut pixel_pos = TPixelPosition::default();
    instance.get_pixel_position(&mut pixel_pos);
    assert_eq!(pixel_pos.x, 10.0);
    assert_eq!(pixel_pos.y, -20.0);
    assert_eq!(pixel_pos.z, 30.0);
}
