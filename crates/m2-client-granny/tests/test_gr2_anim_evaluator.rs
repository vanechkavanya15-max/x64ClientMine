use m2_client_granny::gr2_anim_evaluator::Gr2AnimEvaluator;

#[test]
fn test_integration_gr2_anim_evaluator() {
    let instance = Gr2AnimEvaluator::new();
    assert!(instance.is_ready());
}
