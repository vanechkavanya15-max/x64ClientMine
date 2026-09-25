use m2_client_granny::gr2_morph_targets::{Gr2MorphTargets, MorphVertexDelta};

#[test]
fn test_integration_gr2_morph_targets() {
    let instance = Gr2MorphTargets::new();
    assert!(instance.is_ready());
}

#[test]
fn test_add_and_get_target_weight() {
    let mut obj = Gr2MorphTargets::new();
    obj.add_target("smile".to_string(), vec![]);
    
    assert_eq!(obj.get_weight("smile"), Some(0.0));
    assert_eq!(obj.get_weight("frown"), None);
}

#[test]
fn test_set_target_weight() {
    let mut obj = Gr2MorphTargets::new();
    obj.add_target("smile".to_string(), vec![]);
    
    let res1 = obj.set_weight("smile", 0.5);
    assert!(res1.is_ok());
    assert_eq!(obj.get_weight("smile"), Some(0.5));
    
    let res2 = obj.set_weight("smile", 1.5);
    assert!(res2.is_ok());
    assert_eq!(obj.get_weight("smile"), Some(1.0)); // clamping

    let res3 = obj.set_weight("smile", -0.5);
    assert!(res3.is_ok());
    assert_eq!(obj.get_weight("smile"), Some(0.0)); // clamping
    
    let res4 = obj.set_weight("frown", 0.5);
    assert!(res4.is_err());
}

#[test]
fn test_evaluate_morph_targets() {
    let mut obj = Gr2MorphTargets::new();
    
    // Target 1: smile
    obj.add_target("smile".to_string(), vec![
        MorphVertexDelta {
            vertex_index: 0,
            delta_position: [1.0, 0.0, 0.0],
            delta_normal: [0.0, 1.0, 0.0],
        },
        MorphVertexDelta {
            vertex_index: 1,
            delta_position: [0.0, 2.0, 0.0],
            delta_normal: [0.0, 0.0, 1.0],
        }
    ]);
    
    // Target 2: frown
    obj.add_target("frown".to_string(), vec![
        MorphVertexDelta {
            vertex_index: 0,
            delta_position: [-0.5, 0.0, 0.0],
            delta_normal: [0.0, -0.5, 0.0],
        }
    ]);
    
    obj.set_weight("smile", 0.5).unwrap();
    obj.set_weight("frown", 1.0).unwrap();
    
    let evaluated = obj.evaluate();
    
    // Vertex 0: smile (w=0.5) + frown (w=1.0)
    // d_pos: [1.0*0.5 + -0.5*1.0, 0, 0] = [0.0, 0.0, 0.0]
    // d_nor: [0, 1.0*0.5 + -0.5*1.0, 0] = [0.0, 0.0, 0.0]
    let v0 = evaluated.get(&0).unwrap();
    assert_eq!(v0.delta_position, [0.0, 0.0, 0.0]);
    assert_eq!(v0.delta_normal, [0.0, 0.0, 0.0]);
    
    // Vertex 1: smile (w=0.5)
    // d_pos: [0, 2.0*0.5, 0] = [0.0, 1.0, 0.0]
    // d_nor: [0, 0, 1.0*0.5] = [0.0, 0.0, 0.5]
    let v1 = evaluated.get(&1).unwrap();
    assert_eq!(v1.delta_position, [0.0, 1.0, 0.0]);
    assert_eq!(v1.delta_normal, [0.0, 0.0, 0.5]);
    
    // Vertex 2: non-existent
    assert!(evaluated.get(&2).is_none());
}
