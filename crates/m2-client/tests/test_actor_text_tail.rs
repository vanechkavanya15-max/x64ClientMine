#[path = "../src/actor_text_tail.rs"]
mod actor_text_tail;

use actor_text_tail::{AlignmentGrade, Color, Empire, Matrix4, TextTail, Vector3, Vector4, get_name_color};

#[test]
fn test_alignment_grades() {
    assert_eq!(AlignmentGrade::from_alignment(12000), AlignmentGrade::Grade0);
    assert_eq!(AlignmentGrade::from_alignment(8000), AlignmentGrade::Grade1);
    assert_eq!(AlignmentGrade::from_alignment(4000), AlignmentGrade::Grade2);
    assert_eq!(AlignmentGrade::from_alignment(1000), AlignmentGrade::Grade3);
    assert_eq!(AlignmentGrade::from_alignment(0), AlignmentGrade::Grade4);
    assert_eq!(AlignmentGrade::from_alignment(-3999), AlignmentGrade::Grade5);
    assert_eq!(AlignmentGrade::from_alignment(-7999), AlignmentGrade::Grade6);
    assert_eq!(AlignmentGrade::from_alignment(-11999), AlignmentGrade::Grade7);
    assert_eq!(AlignmentGrade::from_alignment(-12000), AlignmentGrade::Grade8);
}

#[test]
fn test_title_string_generation() {
    assert_eq!(AlignmentGrade::Grade0.to_title_string(), Some("Chivalric".to_string()));
    assert_eq!(AlignmentGrade::Grade4.to_title_string(), None);
    assert_eq!(AlignmentGrade::Grade8.to_title_string(), Some("Cruel".to_string()));
}

#[test]
fn test_color_by_empire_and_alignment() {
    // Enemy empire -> Red
    let color_enemy = get_name_color(Empire::EmpireA, Empire::EmpireB, &AlignmentGrade::Grade0);
    assert_eq!(color_enemy, Color::new(1.0, 0.0, 0.0, 1.0));

    // Same empire, positive alignment -> Blue
    let color_ally_positive = get_name_color(Empire::EmpireA, Empire::EmpireA, &AlignmentGrade::Grade0);
    assert_eq!(color_ally_positive, Color::new(0.0, 0.0, 1.0, 1.0));

    // Same empire, neutral alignment -> White
    let color_ally_neutral = get_name_color(Empire::EmpireB, Empire::EmpireB, &AlignmentGrade::Grade4);
    assert_eq!(color_ally_neutral, Color::new(1.0, 1.0, 1.0, 1.0));

    // Same empire, negative alignment -> Brown
    let color_ally_negative = get_name_color(Empire::EmpireC, Empire::EmpireC, &AlignmentGrade::Grade8);
    assert_eq!(color_ally_negative, Color::new(0.6, 0.3, 0.0, 1.0));
}

#[test]
fn test_hp_bar_visibility() {
    // 100% HP -> Hidden
    let tail_full_hp = TextTail::new(
        "Player1".to_string(),
        None,
        0,
        Empire::EmpireA,
        Empire::EmpireA,
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        10.0,
        100,
    );
    assert_eq!(tail_full_hp.hp_bar_pct, None);

    // 99% HP -> Visible
    let tail_damaged = TextTail::new(
        "Player2".to_string(),
        None,
        0,
        Empire::EmpireA,
        Empire::EmpireA,
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        10.0,
        99,
    );
    assert_eq!(tail_damaged.hp_bar_pct, Some(99));

    // 0% HP -> Visible
    let tail_dead = TextTail::new(
        "Player3".to_string(),
        None,
        0,
        Empire::EmpireA,
        Empire::EmpireA,
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        10.0,
        0,
    );
    assert_eq!(tail_dead.hp_bar_pct, Some(0));
}

#[test]
fn test_transform_to_screen() {
    let tail = TextTail::new(
        "Player".to_string(),
        None,
        0,
        Empire::EmpireA,
        Empire::EmpireA,
        Vector3 { x: 10.0, y: 20.0, z: 30.0 },
        10.0, // Total Z will be 40.0
        100,
    );

    // Create a dummy identity-like projection matrix that maps coordinates directly
    let view_proj = Matrix4 {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0], // w will be 1.0
        ]
    };

    let screen_width = 800.0;
    let screen_height = 600.0;

    let result = tail.transform_to_screen(&view_proj, screen_width, screen_height);
    
    // clip_coords: x=10, y=20, z=40, w=1
    // ndc: x=10, y=20
    // screen_x = (10 + 1) * 0.5 * 800 = 11 * 400 = 4400
    // screen_y = (1 - 20) * 0.5 * 600 = -19 * 300 = -5700
    assert!(result.is_some());
    let (sx, sy) = result.unwrap();
    assert_eq!(sx, 4400.0);
    assert_eq!(sy, -5700.0);
}

#[test]
fn test_transform_to_screen_behind_camera() {
    let tail = TextTail::new(
        "Player".to_string(),
        None,
        0,
        Empire::EmpireA,
        Empire::EmpireA,
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        10.0,
        100,
    );

    // Create a matrix that results in w <= 0.0
    let view_proj = Matrix4 {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, -1.0], // w will be -1.0 * w = -1.0
        ]
    };

    let result = tail.transform_to_screen(&view_proj, 800.0, 600.0);
    assert!(result.is_none());
}
