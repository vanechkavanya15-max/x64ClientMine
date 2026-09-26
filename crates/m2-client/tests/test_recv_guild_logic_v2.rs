use std::sync::{Mutex, OnceLock};

#[path = "../src/recv_guild_logic_v2.rs"]
mod recv_guild_logic_v2;

use recv_guild_logic_v2::*;

// Ensure tests run sequentially to avoid state collisions
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
fn acquire_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn clear_state() {
    test_clear_guild_state();
}

#[test]
fn test_parse_info_packet() {
    let _lock = acquire_lock();
    clear_state();

    let mut data = vec![0u8; std::mem::size_of::<PacketGcGuild>() + std::mem::size_of::<PacketGcGuildInfo>()];
    
    // Header setup
    let header_ptr = data.as_mut_ptr() as *mut PacketGcGuild;
    unsafe {
        (*header_ptr).header = 0x1234;
        (*header_ptr).length = data.len() as u16;
        (*header_ptr).subheader = GuildSubGc::Info as u8;
    }

    // Info payload setup
    let info_ptr = unsafe { data.as_mut_ptr().add(std::mem::size_of::<PacketGcGuild>()) as *mut PacketGcGuildInfo };
    unsafe {
        (*info_ptr).member_count = 10;
        (*info_ptr).max_member_count = 50;
        (*info_ptr).guild_id = 999;
        (*info_ptr).master_pid = 42;
        (*info_ptr).exp = 1500;
        (*info_ptr).level = 10;
        (*info_ptr).gold = 10000;
        (*info_ptr).has_land = 1;
        
        let name = b"EliteGuild\0\0\0";
        (*info_ptr).name.copy_from_slice(name);
    }

    let result = parse_guild_packet(&data);
    assert!(result.is_ok());

    let state = get_guild_state().lock().unwrap();
    assert_eq!(state.is_enabled, true);
    assert_eq!(state.member_count, 10);
    assert_eq!(state.max_member_count, 50);
    assert_eq!(state.guild_id, 999);
    assert_eq!(state.master_pid, 42);
    assert_eq!(state.exp, 1500);
    assert_eq!(state.level, 10);
    assert_eq!(state.name, "EliteGuild");
    assert_eq!(state.gold, 10000);
    assert_eq!(state.has_land, 1);
}

#[test]
fn test_parse_list_packet() {
    let _lock = acquire_lock();
    clear_state();

    let packet_size = std::mem::size_of::<PacketGcGuild>() 
        + std::mem::size_of::<PacketGcGuildSubMember>() 
        + CHARACTER_NAME_MAX_LEN + 1; // 1 member with name flag
        
    let mut data = vec![0u8; packet_size];
    
    // Header setup
    let header_ptr = data.as_mut_ptr() as *mut PacketGcGuild;
    unsafe {
        (*header_ptr).subheader = GuildSubGc::List as u8;
    }

    // Member setup
    let mem_ptr = unsafe { data.as_mut_ptr().add(std::mem::size_of::<PacketGcGuild>()) as *mut PacketGcGuildSubMember };
    unsafe {
        (*mem_ptr).pid = 1234;
        (*mem_ptr).by_grade = 1;
        (*mem_ptr).by_is_general = 0;
        (*mem_ptr).by_job = 2;
        (*mem_ptr).by_level = 50;
        (*mem_ptr).dw_offer = 100;
        (*mem_ptr).by_name_flag = 1;
        
        // Copy name immediately after
        let name_start = (mem_ptr as usize + std::mem::size_of::<PacketGcGuildSubMember>()) as *mut u8;
        let test_name = b"WarriorPlayer123\0";
        std::ptr::copy_nonoverlapping(test_name.as_ptr(), name_start, test_name.len());
    }

    let result = parse_guild_packet(&data);
    assert!(result.is_ok());

    let state = get_guild_state().lock().unwrap();
    assert_eq!(state.members.len(), 1);
    
    let mem = state.members.get(&1234).unwrap();
    assert_eq!(mem.grade, 1);
    assert_eq!(mem.level, 50);
    assert_eq!(mem.name, "WarriorPlayer123");
}

#[test]
fn test_parse_skill_info() {
    let _lock = acquire_lock();
    clear_state();

    let sub_size = 1 + GUILD_SKILL_MAX_NUM + 2 + 2;
    let mut data = vec![0u8; std::mem::size_of::<PacketGcGuild>() + sub_size];
    
    // Header
    let header_ptr = data.as_mut_ptr() as *mut PacketGcGuild;
    unsafe {
        (*header_ptr).subheader = GuildSubGc::SkillInfo as u8;
    }

    // Payload
    let sub = &mut data[std::mem::size_of::<PacketGcGuild>()..];
    sub[0] = 5; // skill_point
    sub[1] = 2; // skill_level[0]
    sub[2] = 3; // skill_level[1]
    
    // wGuildPoint = 100, wMaxGuildPoint = 200
    sub[1 + GUILD_SKILL_MAX_NUM] = 100;
    sub[1 + GUILD_SKILL_MAX_NUM + 1] = 0;
    sub[1 + GUILD_SKILL_MAX_NUM + 2] = 200;
    sub[1 + GUILD_SKILL_MAX_NUM + 3] = 0;

    let result = parse_guild_packet(&data);
    assert!(result.is_ok());

    let state = get_guild_state().lock().unwrap();
    assert_eq!(state.skills.skill_point, 5);
    assert_eq!(state.skills.skill_level[0], 2);
    assert_eq!(state.skills.skill_level[1], 3);
    assert_eq!(state.skills.guild_point, 100);
    assert_eq!(state.skills.max_guild_point, 200);
}

#[test]
fn test_war_state_transitions() {
    let _lock = acquire_lock();
    clear_state();

    let mut data = vec![0u8; std::mem::size_of::<PacketGcGuild>() + std::mem::size_of::<PacketGcGuildWar>()];
    
    // Header
    let header_ptr = data.as_mut_ptr() as *mut PacketGcGuild;
    unsafe {
        (*header_ptr).subheader = GuildSubGc::War as u8;
    }

    // OnWar setup
    let war_ptr = unsafe { data.as_mut_ptr().add(std::mem::size_of::<PacketGcGuild>()) as *mut PacketGcGuildWar };
    unsafe {
        (*war_ptr).dw_guild_self = 1;
        (*war_ptr).dw_guild_opp = 2;
        (*war_ptr).b_type = 0;
        (*war_ptr).b_war_state = GuildWarState::OnWar as u8;
    }

    let result = parse_guild_packet(&data);
    assert!(result.is_ok());

    {
        let state = get_guild_state().lock().unwrap();
        assert_eq!(state.wars.len(), 1);
        assert_eq!(state.wars[0], 2);
    }

    // EndWar setup
    unsafe {
        (*war_ptr).b_war_state = GuildWarState::End as u8;
    }

    let result2 = parse_guild_packet(&data);
    assert!(result2.is_ok());

    {
        let state = get_guild_state().lock().unwrap();
        assert_eq!(state.wars.len(), 0);
    }
}

#[test]
fn test_ffi_get_info() {
    let _lock = acquire_lock();
    clear_state();

    {
        let mut state = get_guild_state().lock().unwrap();
        state.is_enabled = true;
        state.guild_id = 111;
        state.level = 20;
        state.gold = 50000;
    }

    let mut out_id = 0;
    let mut out_level = 0;
    let mut out_gold = 0;

    let res = m2_guild_get_info(&mut out_id, &mut out_level, &mut out_gold);
    assert!(res);
    assert_eq!(out_id, 111);
    assert_eq!(out_level, 20);
    assert_eq!(out_gold, 50000);
}

#[test]
fn test_ffi_get_info_disabled() {
    let _lock = acquire_lock();
    clear_state(); // is_enabled is false by default

    let mut out_id = 0;
    let mut out_level = 0;
    let mut out_gold = 0;

    let res = m2_guild_get_info(&mut out_id, &mut out_level, &mut out_gold);
    assert!(!res);
}

#[test]
fn test_invalid_packet_length() {
    let _lock = acquire_lock();
    clear_state();

    let data = vec![0u8; 1]; // too small even for header
    let result = parse_guild_packet(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Packet too small for TPacketGCGuild");
}
