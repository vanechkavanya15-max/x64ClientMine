use m2_client_granny::gr2_anim_track::Gr2AnimTrack;

#[test]
fn test_integration_gr2_anim_track() {
    let instance = Gr2AnimTrack::new();
    assert!(instance.is_ready());
}
