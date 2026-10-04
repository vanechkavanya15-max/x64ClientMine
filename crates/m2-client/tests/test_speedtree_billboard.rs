use m2_client::wgpu_speedtree_billboard_lod::*;

#[test]
fn test_lod_evaluator_full3d() {
    let evaluator = SpeedTreeLodEvaluator::new(100.0, 500.0);
    let camera_pos = [0.0, 0.0, 0.0];
    let tree_pos = [50.0, 0.0, 0.0];

    let state = evaluator.evaluate(camera_pos, tree_pos);
    assert_eq!(state.mode, SpeedTreeRenderMode::Full3D);
    assert_eq!(state.lod_value, 0.0);
    assert_eq!(state.tree_alpha, 1.0);
    assert_eq!(state.billboard_alpha, 0.0);
}

#[test]
fn test_lod_evaluator_transition() {
    let evaluator = SpeedTreeLodEvaluator::new(100.0, 500.0);
    let camera_pos = [0.0, 0.0, 0.0];
    let tree_pos = [300.0, 0.0, 0.0];

    let state = evaluator.evaluate(camera_pos, tree_pos);
    assert_eq!(state.mode, SpeedTreeRenderMode::Transition);
    assert_eq!(state.lod_value, 0.5);
    assert_eq!(state.tree_alpha, 0.5);
    assert_eq!(state.billboard_alpha, 0.5);
}

#[test]
fn test_lod_evaluator_billboard() {
    let evaluator = SpeedTreeLodEvaluator::new(100.0, 500.0);
    let camera_pos = [0.0, 0.0, 0.0];
    let tree_pos = [600.0, 0.0, 0.0];

    let state = evaluator.evaluate(camera_pos, tree_pos);
    assert_eq!(state.mode, SpeedTreeRenderMode::Billboard);
    assert_eq!(state.lod_value, 1.0);
    assert_eq!(state.tree_alpha, 0.0);
    assert_eq!(state.billboard_alpha, 1.0);
}

#[test]
fn test_billboard_generator() {
    let generator = SpeedTreeBillboardGenerator::new(10.0, 20.0);
    let tree_pos = [100.0, 0.0, 0.0];
    let camera_pos = [0.0, 0.0, 0.0];

    let quad = generator.generate_billboard_quad(tree_pos, camera_pos);

    // dx = -100, dy = 0, dist = 100
    // right = (0, 1)

    assert_eq!(quad[0], [100.0, -5.0, 0.0]); // bottom_left
    assert_eq!(quad[1], [100.0, 5.0, 0.0]);  // bottom_right
    assert_eq!(quad[2], [100.0, 5.0, 20.0]); // top_right
    assert_eq!(quad[3], [100.0, -5.0, 20.0]);// top_left
}
