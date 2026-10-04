use glam::{Mat4, Vec3};

#[path = "../src/actor_texttail_render_cache.rs"]
mod actor_texttail_render_cache;

use actor_texttail_render_cache::{
    ActorTexttailRenderCache, AlignmentGrade, Empire, TextTailData, 
    COLOR_BLUE, COLOR_BROWN, COLOR_GM, COLOR_RED, COLOR_WHITE, COLOR_GUILD
};

#[test]
fn test_world_to_screen_projection() {
    let mut cache = ActorTexttailRenderCache::new();

    let data = TextTailData {
        virtual_id: 1,
        name: "TestPlayer".to_string(),
        guild_name: None,
        level: Some(10),
        alignment: 1000,
        is_gm: false,
        is_monster: false,
        empire: Empire::EmpireA,
        world_pos: Vec3::new(100.0, 200.0, 300.0),
        height_offset: 50.0,
        hp_percent: None,
    };
    cache.add_or_update(data);

    // Identity projection should map coords directly
    let view_proj = Mat4::IDENTITY;
    let screen_w = 800.0;
    let screen_h = 600.0;

    cache.calculate_screen_positions(&view_proj, screen_w, screen_h, Empire::EmpireA);

    let res = cache.render_results.get(&1).unwrap();
    
    // clip_coords: x=100.0, y=200.0, z=350.0, w=1.0
    // ndc: x=100.0, y=200.0
    // screen_x = (100.0 + 1.0) * 0.5 * 800.0 = 40400.0
    // screen_y = (1.0 - 200.0) * 0.5 * 600.0 = -59700.0
    assert!(res.is_visible);
    assert_eq!(res.screen_x, 40400.0);
    assert_eq!(res.screen_y, -59700.0);
}

#[test]
fn test_alignment_titles_and_colors() {
    // Check mapping values
    assert_eq!(AlignmentGrade::from_alignment(12000), AlignmentGrade::Grade0);
    assert_eq!(AlignmentGrade::from_alignment(1000), AlignmentGrade::Grade3);
    assert_eq!(AlignmentGrade::from_alignment(0), AlignmentGrade::Grade4);
    assert_eq!(AlignmentGrade::from_alignment(-4000), AlignmentGrade::Grade6); // > -4000 is Grade5, so -4000 is Grade6
    assert_eq!(AlignmentGrade::from_alignment(-3999), AlignmentGrade::Grade5);
    assert_eq!(AlignmentGrade::from_alignment(-12000), AlignmentGrade::Grade8);
    
    // Check titles
    assert_eq!(AlignmentGrade::Grade0.to_title_string(), Some("Chivalric".to_string()));
    assert_eq!(AlignmentGrade::Grade4.to_title_string(), None);
    assert_eq!(AlignmentGrade::Grade8.to_title_string(), Some("Cruel".to_string()));
    
    // Check colors
    assert_eq!(AlignmentGrade::Grade0.get_color(), COLOR_BLUE);
    assert_eq!(AlignmentGrade::Grade4.get_color(), COLOR_WHITE);
    assert_eq!(AlignmentGrade::Grade8.get_color(), COLOR_BROWN);
}

#[test]
fn test_name_coloring_logic() {
    let mut cache = ActorTexttailRenderCache::new();

    // Player is EmpireA
    let player_empire = Empire::EmpireA;
    let view_proj = Mat4::IDENTITY;

    // 1. GM
    cache.add_or_update(TextTailData {
        virtual_id: 1,
        name: "GM_Player".to_string(),
        guild_name: None,
        level: None,
        alignment: 20000,
        is_gm: true,
        is_monster: false,
        empire: Empire::EmpireB, // Even if different empire, GM should be colored GM
        world_pos: Vec3::ZERO,
        height_offset: 0.0,
        hp_percent: None,
    });

    // 2. Monster
    cache.add_or_update(TextTailData {
        virtual_id: 2,
        name: "Wild Dog".to_string(),
        guild_name: None,
        level: None,
        alignment: -1000,
        is_gm: false,
        is_monster: true,
        empire: Empire::None,
        world_pos: Vec3::ZERO,
        height_offset: 0.0,
        hp_percent: None,
    });

    // 3. Enemy Player
    cache.add_or_update(TextTailData {
        virtual_id: 3,
        name: "Enemy".to_string(),
        guild_name: None,
        level: None,
        alignment: 10000,
        is_gm: false,
        is_monster: false,
        empire: Empire::EmpireB,
        world_pos: Vec3::ZERO,
        height_offset: 0.0,
        hp_percent: None,
    });

    // 4. Ally Player
    cache.add_or_update(TextTailData {
        virtual_id: 4,
        name: "Ally".to_string(),
        guild_name: None,
        level: None,
        alignment: 1000,
        is_gm: false,
        is_monster: false,
        empire: Empire::EmpireA,
        world_pos: Vec3::ZERO,
        height_offset: 0.0,
        hp_percent: None,
    });

    cache.calculate_screen_positions(&view_proj, 800.0, 600.0, player_empire);

    let res_gm = cache.render_results.get(&1).unwrap();
    assert_eq!(res_gm.name_color, COLOR_GM);

    let res_monster = cache.render_results.get(&2).unwrap();
    assert_eq!(res_monster.name_color, COLOR_RED);

    let res_enemy = cache.render_results.get(&3).unwrap();
    assert_eq!(res_enemy.name_color, COLOR_RED);

    let res_ally = cache.render_results.get(&4).unwrap();
    assert_eq!(res_ally.name_color, COLOR_BLUE);
}

#[test]
fn test_render_cache_management() {
    let mut cache = ActorTexttailRenderCache::new();

    // Insert
    cache.add_or_update(TextTailData {
        virtual_id: 1,
        name: "Test".to_string(),
        guild_name: None,
        level: None,
        alignment: 0,
        is_gm: false,
        is_monster: false,
        empire: Empire::EmpireA,
        world_pos: Vec3::new(10.0, 10.0, 10.0),
        height_offset: 15.0,
        hp_percent: None,
    });

    assert_eq!(cache.tails.len(), 1);
    
    // Update HP - Show
    cache.update_hp(1, 95);
    assert_eq!(cache.tails.get(&1).unwrap().hp_percent, Some(95));

    // Update HP - Hide (full HP)
    cache.update_hp(1, 100);
    assert_eq!(cache.tails.get(&1).unwrap().hp_percent, None);

    // Update position
    cache.update_position(1, Vec3::new(20.0, 20.0, 20.0));
    assert_eq!(cache.tails.get(&1).unwrap().world_pos, Vec3::new(20.0, 20.0, 20.0));

    // Calculate (populates render_results)
    cache.calculate_screen_positions(&Mat4::IDENTITY, 800.0, 600.0, Empire::EmpireA);
    assert_eq!(cache.render_results.len(), 1);

    // Remove
    cache.remove(1);
    assert_eq!(cache.tails.len(), 0);
    assert_eq!(cache.render_results.len(), 0);
}
