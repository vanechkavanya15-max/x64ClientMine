use m2_client::phase_loading::*;
use std::mem::size_of;

#[test]
fn test_integration_phase_loading() {
    let instance = PhaseLoading::new();
    assert!(instance.is_ready());
}

#[test]
fn test_packet_sizes() {
    // Sprawdzamy rozmiary w oparciu o padding C++ "pack(1)" czyli suma bajtów.
    // TPacketGCMainCharacter
    // header(1) + dwVID(4) + wRaceNum(2) + szName(25) + lX(4) + lY(4) + lZ(4) + bySkillGroup(1) = 45
    assert_eq!(size_of::<TPacketGCMainCharacter>(), 45);

    // TPacketGCMainCharacter2_EMPIRE
    // header(1) + dwVID(4) + wRaceNum(2) + szName(25) + lX(4) + lY(4) + lZ(4) + byEmpire(1) + bySkillGroup(1) = 46
    assert_eq!(size_of::<TPacketGCMainCharacter2_EMPIRE>(), 46);

    // TPacketGCMainCharacter3_BGM
    // header(1) + dwVID(4) + wRaceNum(2) + szUserName(25) + szBGMName(25) + lX(4) + lY(4) + lZ(4) + byEmpire(1) + bySkillGroup(1) = 71
    assert_eq!(size_of::<TPacketGCMainCharacter3_BGM>(), 71);

    // TPacketGCMainCharacter4_BGM_VOL
    // header(1) + dwVID(4) + wRaceNum(2) + szUserName(25) + szBGMName(25) + fBGMVol(4) + lX(4) + lY(4) + lZ(4) + byEmpire(1) + bySkillGroup(1) = 75
    assert_eq!(size_of::<TPacketGCMainCharacter4_BGM_VOL>(), 75);

    // TPacketGCPoints
    // header(1) + points(4 * 255) = 1 + 1020 = 1021
    assert_eq!(size_of::<TPacketGCPoints>(), 1021);

    // TPacketCGEnterFrontGame
    // header(1)
    assert_eq!(size_of::<TPacketCGEnterFrontGame>(), 1);
}

#[test]
fn test_recv_main_character() {
    let mut phase = PhaseLoading::new();
    let mut name_arr = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let name_bytes = b"TestPlayer\0";
    name_arr[..name_bytes.len()].copy_from_slice(name_bytes);

    let packet = TPacketGCMainCharacter {
        header: HEADER_GC_MAIN_CHARACTER,
        dwVID: 1234,
        wRaceNum: 3,
        szName: name_arr,
        lX: 100,
        lY: 200,
        lZ: 300,
        bySkillGroup: 2,
    };

    assert!(phase.recv_main_character(&packet));
    assert_eq!(phase.main_actor_vid, 1234);
    assert_eq!(phase.main_actor_race, 3);
    assert_eq!(phase.main_actor_skill_group, 2);
    assert_eq!(phase.main_actor_empire, 0);
    assert_eq!(phase.x, 100);
    assert_eq!(phase.y, 200);
    assert_eq!(phase.z, 300);
    assert_eq!(phase.main_actor_name, "TestPlayer");
}

#[test]
fn test_recv_main_character2_empire() {
    let mut phase = PhaseLoading::new();
    let mut name_arr = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let name_bytes = b"Warrior\0";
    name_arr[..name_bytes.len()].copy_from_slice(name_bytes);

    let packet = TPacketGCMainCharacter2_EMPIRE {
        header: 113,
        dwVID: 999,
        wRaceNum: 0,
        szName: name_arr,
        lX: 0,
        lY: 0,
        lZ: 0,
        byEmpire: 1,
        bySkillGroup: 1,
    };

    assert!(phase.recv_main_character2_empire(&packet));
    assert_eq!(phase.main_actor_vid, 999);
    assert_eq!(phase.main_actor_empire, 1);
    assert_eq!(phase.main_actor_name, "Warrior");
}

#[test]
fn test_recv_main_character3_bgm() {
    let mut phase = PhaseLoading::new();
    let mut name_arr = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let name_bytes = b"Shaman\0";
    name_arr[..name_bytes.len()].copy_from_slice(name_bytes);
    
    let mut bgm_arr = [0u8; MUSIC_NAME_MAX_LEN + 1];
    let bgm_bytes = b"bgm.mp3\0";
    bgm_arr[..bgm_bytes.len()].copy_from_slice(bgm_bytes);

    let packet = TPacketGCMainCharacter3_BGM {
        header: 137,
        dwVID: 444,
        wRaceNum: 1,
        szUserName: name_arr,
        szBGMName: bgm_arr,
        lX: 0,
        lY: 0,
        lZ: 0,
        byEmpire: 2,
        bySkillGroup: 0,
    };

    assert!(phase.recv_main_character3_bgm(&packet));
    assert_eq!(phase.main_actor_vid, 444);
    assert_eq!(phase.main_actor_empire, 2);
    assert_eq!(phase.main_actor_name, "Shaman");
    assert_eq!(phase.field_music_file_name, "bgm.mp3");
}

#[test]
fn test_recv_main_character4_bgm_vol() {
    let mut phase = PhaseLoading::new();
    let mut name_arr = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let name_bytes = b"Ninja\0";
    name_arr[..name_bytes.len()].copy_from_slice(name_bytes);
    
    let mut bgm_arr = [0u8; MUSIC_NAME_MAX_LEN + 1];
    let bgm_bytes = b"city.mp3\0";
    bgm_arr[..bgm_bytes.len()].copy_from_slice(bgm_bytes);

    let packet = TPacketGCMainCharacter4_BGM_VOL {
        header: 138,
        dwVID: 777,
        wRaceNum: 2,
        szUserName: name_arr,
        szBGMName: bgm_arr,
        fBGMVol: 0.5,
        lX: 0,
        lY: 0,
        lZ: 0,
        byEmpire: 3,
        bySkillGroup: 2,
    };

    assert!(phase.recv_main_character4_bgm_vol(&packet));
    assert_eq!(phase.main_actor_vid, 777);
    assert_eq!(phase.main_actor_empire, 3);
    assert_eq!(phase.main_actor_name, "Ninja");
    assert_eq!(phase.field_music_file_name, "city.mp3");
    assert_eq!(phase.field_music_volume, 0.5);
}

#[test]
fn test_phase_methods() {
    let mut phase = PhaseLoading::new();
    phase.set_field_music_file_name("test.mp3".to_string());
    assert_eq!(phase.get_field_music_file_name(), "test.mp3");

    phase.set_field_music_file_info("test2.mp3".to_string(), 0.8);
    assert_eq!(phase.get_field_music_file_name(), "test2.mp3");
    assert_eq!(phase.get_field_music_volume(), 0.8);

    assert_eq!(phase.is_start_game, false);
    phase.start_game();
    assert_eq!(phase.is_start_game, true);

    let enter_game = phase.send_enter_game();
    assert_eq!(enter_game.header, HEADER_CG_ENTERGAME);
}

#[test]
fn test_progress() {
    let mut phase = PhaseLoading::new();
    assert_eq!(phase.get_progress(), 0);
    phase.set_progress(50);
    assert_eq!(phase.get_progress(), 50);
    phase.set_progress(150);
    assert_eq!(phase.get_progress(), 100);
}
