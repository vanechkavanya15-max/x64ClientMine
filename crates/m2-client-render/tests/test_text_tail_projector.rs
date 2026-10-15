use m2_client_render::text_tail_projector::{TextTailProjector, Rank, TextTailColor};
use nalgebra::{Matrix4, Point3};

#[test]
fn test_project_in_front_of_camera() {
    let view_proj = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
    );
    let projector = TextTailProjector::new(view_proj, 800.0, 600.0, 1.0);
    
    let pos = Point3::new(0.0, 0.0, 10.0);
    let result = projector.project_position(&pos, 0.0, 0.0);
    
    assert!(result.is_some());
}

#[test]
fn test_project_behind_camera() {
    let view_proj = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, -1.0, 0.0,
    );
    let projector = TextTailProjector::new(view_proj, 800.0, 600.0, 1.0);
    
    let pos = Point3::new(0.0, 0.0, 10.0);
    let result = projector.project_position(&pos, 0.0, 0.0);
    
    assert!(result.is_none());
}

#[test]
fn test_rank_colors() {
    assert_eq!(Rank::Knight.get_color(), TextTailColor { r: 0.2, g: 0.2, b: 1.0, a: 1.0 });
    assert_eq!(Rank::Noble.get_color(), TextTailColor { r: 0.2, g: 1.0, b: 0.2, a: 1.0 });
    assert_eq!(Rank::Cruel.get_color(), TextTailColor { r: 1.0, g: 0.2, b: 0.2, a: 1.0 });
}
