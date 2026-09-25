use m2_client::phase_login::*;

#[test]
fn test_integration_phase_login() {
    let instance = PhaseLogin::new();
    assert!(instance.is_ready());
}

#[test]
fn test_memory_layout_structs() {
    assert_eq!(std::mem::size_of::<TPacketCGLogin>(), 1 + 31 + 17);
    assert_eq!(std::mem::size_of::<TPacketCGLogin2>(), 1 + 31 + 4 + 16);
    assert_eq!(std::mem::size_of::<TPacketCGDirectEnter>(), 1 + 31 + 17 + 1);
    
    // TSimplePlayerInformation:
    // dwID (4) + szName (25) + byJob (1) + byLevel (1) + dwPlayMinutes (4)
    // + byST (1) + byHT (1) + byDX (1) + byIQ (1) + wMainPart (2)
    // + bChangeName (1) + wHairPart (2) + bDummy (4) + x (4) + y (4) + lAddr (4) + wPort (2) + bySkillGroup (1) = 63 bytes
    assert_eq!(std::mem::size_of::<TSimplePlayerInformation>(), 63);

    // TPacketGCLoginSuccess3: header (1) + akSimplePlayerInformation (63*3 = 189) + guild_id (4*3 = 12) + guild_name (13*3 = 39) + handle (4) + random_key (4) = 249
    assert_eq!(std::mem::size_of::<TPacketGCLoginSuccess3>(), 249);
    
    // TPacketGCLoginSuccess4: header (1) + akSimplePlayerInformation (63*4 = 252) + guild_id (4*4 = 16) + guild_name (13*4 = 52) + handle (4) + random_key (4) = 329
    assert_eq!(std::mem::size_of::<TPacketGCLoginSuccess4>(), 329);

    assert_eq!(std::mem::size_of::<TPacketGCLoginFailure>(), 1 + 9);
    assert_eq!(std::mem::size_of::<TPacketGCEmpire>(), 1 + 1);
    assert_eq!(std::mem::size_of::<TPacketGCLoginKey>(), 1 + 4);
}

#[test]
fn test_packet_creation() {
    let mut login = PhaseLogin::new();
    
    let packet = login.send_login_packet("test_user", "password123");
    assert_eq!(packet.header, HEADER_CG_LOGIN);
    assert_eq!(&packet.name[..9], b"test_user");
    assert_eq!(packet.name[9], 0); // null termination
    assert_eq!(&packet.pwd[..11], b"password123");
    assert_eq!(packet.pwd[11], 0);

    login.m_dwLoginKey = 12345;
    let packet2 = login.send_login_packet_new("user2", "pass");
    assert_eq!(packet2.header, HEADER_CG_LOGIN2);
    assert_eq!(&packet2.name[..5], b"user2");
    assert_eq!(packet2.name[5], 0);
    let login_key = packet2.login_key;
    assert_eq!(login_key, 12345);

    let packet_direct = login.send_direct_enter_packet("dir_user", "dir_pass", 2);
    assert_eq!(packet_direct.bHeader, HEADER_CG_DIRECT_ENTER);
    assert_eq!(&packet_direct.login[..8], b"dir_user");
    assert_eq!(packet_direct.login[8], 0);
    assert_eq!(&packet_direct.passwd[..8], b"dir_pass");
    assert_eq!(packet_direct.passwd[8], 0);
    assert_eq!(packet_direct.index, 2);
}

#[test]
fn test_packet_ingestion() {
    let mut login = PhaseLogin::new();

    let packet_empire = TPacketGCEmpire {
        bHeader: HEADER_GC_EMPIRE,
        bEmpire: 2,
    };
    login.recv_empire_packet(&packet_empire);
    assert_eq!(login.m_dwEmpireID, 2);

    let packet_login_key = TPacketGCLoginKey {
        bHeader: HEADER_GC_LOGIN_KEY,
        dwLoginKey: 54321,
    };
    login.m_isWaitLoginKey = true;
    login.recv_login_key_packet(&packet_login_key);
    assert_eq!(login.m_dwLoginKey, 54321);
    assert!(!login.m_isWaitLoginKey);

    let mut fail_status = [0; LOGIN_STATUS_MAX_LEN + 1];
    let err_str = b"WRONGPWD";
    fail_status[..8].copy_from_slice(err_str);
    let packet_fail = TPacketGCLoginFailure {
        header: HEADER_GC_LOGIN_FAILURE,
        szStatus: fail_status,
    };
    let fail_msg = login.recv_login_failure_packet(&packet_fail);
    assert_eq!(fail_msg, "WRONGPWD");

    login.m_stID = "test".to_string();
    login.m_stPassword = "test".to_string();
    login.set_login_phase();
    assert_eq!(login.m_strPhase, "Login");
    assert_eq!(login.m_stID, "");
    assert_eq!(login.m_stPassword, "");
}
