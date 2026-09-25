use m2_client::phase_game_actor::{
    PacketGcCharacterAdd, PacketGcCharacterAdd2, PacketGcCharacterAdditionalInfo,
    PacketGcCharacterDelete, PacketGcCharacterUpdate,  PacketGcMove,
    PhaseGameActor, TYPE_PC,
};

fn str_to_array<const N: usize>(s: &str) -> [u8; N] {
    let mut arr = [0; N];
    let bytes = s.as_bytes();
    let len = bytes.len().min(N - 1);
    arr[..len].copy_from_slice(&bytes[..len]);
    arr
}

#[test]
fn test_integration_phase_game_actor() {
    let instance = PhaseGameActor::new();
    assert!(instance.is_ready());
}

#[test]
fn test_recv_character_append_packet() {
    let mut actor = PhaseGameActor::new();

    let packet = PacketGcCharacterAdd {
        header: 0,
        vid: 1234,
        angle: 45.0,
        x: 100,
        y: 200,
        z: 0,
        b_type: TYPE_PC,
        w_race_num: 0,
        b_moving_speed: 100,
        b_attack_speed: 100,
        b_state_flag: 0,
        dw_affect_flag: [0, 0],
    };

    assert!(actor.recv_character_append_packet(&packet));
    // TYPE_PC jest zapisywany tymczasowo, az do AdditionalInfo
    assert!(actor.temp_actor_data.is_some());
    assert!(actor.get_actor(1234).is_none());

    let mut packet_npc = packet;
    packet_npc.vid = 5678;
    packet_npc.b_type = 2; // NPC

    assert!(actor.recv_character_append_packet(&packet_npc));
    assert!(actor.get_actor(5678).is_some());
}

#[test]
fn test_recv_character_additional_info() {
    let mut actor = PhaseGameActor::new();

    let packet_add = PacketGcCharacterAdd {
        header: 0,
        vid: 1234,
        angle: 45.0,
        x: 100,
        y: 200,
        z: 0,
        b_type: TYPE_PC,
        w_race_num: 0,
        b_moving_speed: 100,
        b_attack_speed: 100,
        b_state_flag: 0,
        dw_affect_flag: [0, 0],
    };
    actor.recv_character_append_packet(&packet_add);

    let packet_info = PacketGcCharacterAdditionalInfo {
        header: 0,
        vid: 1234,
        name: str_to_array("TestGracz"),
        aw_part: [10, 20, 30, 40],
        b_empire: 1,
        dw_guild_id: 99,
        dw_level: 50,
        s_alignment: 1000,
        b_pk_mode: 0,
        dw_mount_vnum: 0,
    };

    assert!(actor.recv_character_additional_info(&packet_info));
    
    let a = actor.get_actor(1234).unwrap();
    assert_eq!(a.name, "TestGracz");
    assert_eq!(a.guild_id, 99);
    assert_eq!(a.level, 50);
    assert_eq!(a.armor, 10);
    assert_eq!(a.weapon, 20);
}

#[test]
fn test_recv_character_append_packet_new() {
    let mut actor = PhaseGameActor::new();

    let packet = PacketGcCharacterAdd2 {
        header: 0,
        vid: 8888,
        name: str_to_array("Gracz2"),
        angle: 90.0,
        x: 50,
        y: 50,
        z: 0,
        b_type: TYPE_PC,
        w_race_num: 1,
        aw_part: [1, 2, 3, 4],
        b_moving_speed: 150,
        b_attack_speed: 110,
        b_state_flag: 0,
        dw_affect_flag: [0, 0],
        b_empire: 2,
        dw_guild: 77,
        s_alignment: 500,
        b_pk_mode: 1,
        dw_mount_vnum: 20000,
    };

    assert!(actor.recv_character_append_packet_new(&packet));
    
    let a = actor.get_actor(8888).unwrap();
    assert_eq!(a.name, "Gracz2");
    assert_eq!(a.empire_id, 2);
    assert_eq!(a.guild_id, 77);
    assert_eq!(a.alignment, 500);
}

#[test]
fn test_recv_character_update_packet() {
    let mut actor = PhaseGameActor::new();

    let packet_add = PacketGcCharacterAdd2 {
        header: 0,
        vid: 9999,
        name: str_to_array("UpdateMe"),
        angle: 0.0,
        x: 0,
        y: 0,
        z: 0,
        b_type: TYPE_PC,
        w_race_num: 0,
        aw_part: [0, 0, 0, 0],
        b_moving_speed: 100,
        b_attack_speed: 100,
        b_state_flag: 0,
        dw_affect_flag: [0, 0],
        b_empire: 1,
        dw_guild: 0,
        s_alignment: 0,
        b_pk_mode: 0,
        dw_mount_vnum: 0,
    };
    actor.recv_character_append_packet_new(&packet_add);

    let packet_update = PacketGcCharacterUpdate {
        header: 0,
        vid: 9999,
        aw_part: [5, 6, 7, 8],
        b_moving_speed: 120,
        b_attack_speed: 115,
        b_state_flag: 1,
        dw_affect_flag: [1, 0],
        dw_guild_id: 2,
        s_alignment: 10,
        b_pk_mode: 2,
        dw_mount_vnum: 20001,
    };

    assert!(actor.recv_character_update_packet(&packet_update));

    let a = actor.get_actor(9999).unwrap();
    assert_eq!(a.armor, 5);
    assert_eq!(a.weapon, 6);
    assert_eq!(a.mov_spd, 120);
    assert_eq!(a.mount_vnum, 20001);
}

#[test]
fn test_recv_character_delete_packet() {
    let mut actor = PhaseGameActor::new();

    let packet_add = PacketGcCharacterAdd2 {
        header: 0,
        vid: 1111,
        name: str_to_array("ToDelete"),
        angle: 0.0,
        x: 0,
        y: 0,
        z: 0,
        b_type: TYPE_PC,
        w_race_num: 0,
        aw_part: [0, 0, 0, 0],
        b_moving_speed: 100,
        b_attack_speed: 100,
        b_state_flag: 0,
        dw_affect_flag: [0, 0],
        b_empire: 1,
        dw_guild: 0,
        s_alignment: 0,
        b_pk_mode: 0,
        dw_mount_vnum: 0,
    };
    actor.recv_character_append_packet_new(&packet_add);
    assert!(actor.get_actor(1111).is_some());

    let packet_delete = PacketGcCharacterDelete {
        header: 0,
        vid: 1111,
    };
    assert!(actor.recv_character_delete_packet(&packet_delete));
    assert!(actor.get_actor(1111).is_none());
}

#[test]
fn test_recv_character_move_packet() {
    let mut actor = PhaseGameActor::new();

    let packet_add = PacketGcCharacterAdd2 {
        header: 0,
        vid: 2222,
        name: str_to_array("ToMove"),
        angle: 0.0,
        x: 100,
        y: 100,
        z: 0,
        b_type: TYPE_PC,
        w_race_num: 0,
        aw_part: [0, 0, 0, 0],
        b_moving_speed: 100,
        b_attack_speed: 100,
        b_state_flag: 0,
        dw_affect_flag: [0, 0],
        b_empire: 1,
        dw_guild: 0,
        s_alignment: 0,
        b_pk_mode: 0,
        dw_mount_vnum: 0,
    };
    actor.recv_character_append_packet_new(&packet_add);

    let packet_move = PacketGcMove {
        header: 0,
        b_func: 1,
        b_arg: 2,
        b_rot: 10,
        vid: 2222,
        x: 500,
        y: 600,
        dw_time: 12345,
        dw_duration: 5,
    };
    assert!(actor.recv_character_move_packet(&packet_move));

    let a = actor.get_actor(2222).unwrap();
    assert_eq!(a.cur_x, 500);
    assert_eq!(a.cur_y, 600);
    assert_eq!(a.rot, 50.0); // 10 * 5.0
}
