use m2_client::net_stream_core::*;
 
 #[test]
fn test_integration_net_stream_core_initial_state() {
     let instance = NetStreamCore::new();
     assert!(instance.is_ready());
    assert_eq!(instance.phase, Phase::OffLine);
}

#[test]
fn test_net_stream_core_phase_transitions() {
    let mut instance = NetStreamCore::new();
    
    instance.set_handshake_phase();
    assert_eq!(instance.phase, Phase::HandShake);
    
    instance.set_login_phase();
    assert_eq!(instance.phase, Phase::Login);
    
    instance.set_select_phase();
    assert_eq!(instance.phase, Phase::Select);
    
    instance.set_loading_phase();
    assert_eq!(instance.phase, Phase::Loading);
    
    instance.set_game_phase();
    assert_eq!(instance.phase, Phase::Game);
    
    instance.set_dead_phase();
    assert_eq!(instance.phase, Phase::Dead);

    instance.close_phase();
    assert_eq!(instance.phase, Phase::Close);
}

#[test]
fn test_net_stream_core_queues() {
    let mut instance = NetStreamCore::new();
    
    let packet_recv = Packet { header: HEADER_GC_PHASE, payload: vec![1] };
    let packet_send = Packet { header: 0xAA, payload: vec![2, 3] };
    
    instance.queue_recv_packet(packet_recv);
    instance.queue_send_packet(packet_send);
    
    assert_eq!(instance.recv_queue.len(), 1);
    assert_eq!(instance.send_queue.len(), 1);
    
    let popped_recv = instance.pop_recv_packet().unwrap();
    assert_eq!(popped_recv.header, HEADER_GC_PHASE);
    assert_eq!(popped_recv.payload, vec![1]);
    
    let popped_send = instance.pop_send_packet().unwrap();
    assert_eq!(popped_send.header, 0xAA);
    assert_eq!(popped_send.payload, vec![2, 3]);
}

#[test]
fn test_net_stream_core_process_transitions() {
    let mut instance = NetStreamCore::new();
    
    // Test przejscia OffLine -> HandShake przez pakiet HEADER_GC_HANDSHAKE
    instance.queue_recv_packet(Packet { header: HEADER_GC_HANDSHAKE, payload: vec![] });
    instance.process();
    assert_eq!(instance.phase, Phase::HandShake);
    
    // Test przejscia HandShake -> Login przez pakiet HEADER_GC_PHASE
    instance.queue_recv_packet(Packet { header: HEADER_GC_PHASE, payload: vec![2] });
    instance.process();
    assert_eq!(instance.phase, Phase::Login);
    
    // Test przejscia Login -> Select
    instance.queue_recv_packet(Packet { header: HEADER_GC_PHASE, payload: vec![3] });
    instance.process();
    assert_eq!(instance.phase, Phase::Select);
    
    // Test przejscia Select -> Loading
    instance.queue_recv_packet(Packet { header: HEADER_GC_PHASE, payload: vec![4] });
    instance.process();
    assert_eq!(instance.phase, Phase::Loading);
    
    // Test przejscia Loading -> Game
    instance.queue_recv_packet(Packet { header: HEADER_GC_PHASE, payload: vec![5] });
    instance.process();
    assert_eq!(instance.phase, Phase::Game);
    
    // Test przejscia Game -> Close
    instance.queue_recv_packet(Packet { header: HEADER_GC_PHASE, payload: vec![0] });
    instance.process();
    assert_eq!(instance.phase, Phase::Close);
}

#[test]
fn test_loading_phase_fallback() {
    let mut instance = NetStreamCore::new();
    instance.set_loading_phase();

    // 0xAA to niezidentyfikowany pakiet (nie pasuje do zadnego HEADER_GC_* w process_loading_phase),
    // wiec trafia do default `_ =>` ktory wywoluje set_game_phase() jako fallback
    instance.queue_recv_packet(Packet { header: 0xAA, payload: vec![] });
    instance.process();

    assert_eq!(instance.phase, Phase::Game);
}

#[test]
fn test_error_packet() {
    let mut instance = NetStreamCore::new();
    instance.set_login_phase();

    // 0xAA to niezidentyfikowany pakiet w Login Phase -> trafia do default -> recv_error_packet -> clear().
    // Drugi pakiet HEADER_GC_PHASE nigdy nie zostanie obsluzony, bo clear() go usunie.
    instance.queue_recv_packet(Packet { header: 0xAA, payload: vec![] });
    instance.queue_recv_packet(Packet { header: HEADER_GC_PHASE, payload: vec![3] });
    
    instance.process();

    // Faza zostaje Login bo recv_error_packet wyczyscil kolejke zanim HEADER_GC_PHASE zdazyl zmienic na Select.
    assert_eq!(instance.phase, Phase::Login);
    assert_eq!(instance.recv_queue.len(), 0);
}


