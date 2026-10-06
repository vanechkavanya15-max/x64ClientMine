#[path = "../src/recv_char_update_extended.rs"]
mod recv_char_update_extended;

use recv_char_update_extended::{
    AbstractPlayer, CharacterUpdateHandler, NetworkStream, NetworkActorManager,
    SNetworkUpdateActorData, TPacketGCCharacterUpdate,
};

#[derive(Default)]
struct MockActorManager {
    pub updated_vid: u32,
}

impl NetworkActorManager for MockActorManager {
    fn update_actor(&mut self, data: &SNetworkUpdateActorData) {
        self.updated_vid = data.m_dwVID;
    }
}

#[derive(Default)]
struct MockPlayer {
    pub is_main: bool,
    pub notified_vid: u32,
    pub weapon_power: u32,
}

impl AbstractPlayer for MockPlayer {
    fn is_main_character_index(&self, _vid: u32) -> bool {
        self.is_main
    }

    fn notify_character_update(&mut self, vid: u32) {
        self.notified_vid = vid;
    }

    fn set_weapon_power(&mut self, weapon: u32) {
        self.weapon_power = weapon;
    }
}

#[derive(Default)]
struct MockNetworkStream {
    pub guild_id: u32,
    pub status_refreshed: bool,
    pub alignment_refreshed: bool,
    pub equipment_refreshed: bool,
    pub inventory_refreshed: bool,
    pub selected_index: usize,
    pub updated_hair: u32,
    pub updated_armor: u32,
}

impl NetworkStream for MockNetworkStream {
    fn set_guild_id(&mut self, guild_id: u32) {
        self.guild_id = guild_id;
    }

    fn refresh_status(&mut self) {
        self.status_refreshed = true;
    }

    fn refresh_alignment_window(&mut self) {
        self.alignment_refreshed = true;
    }

    fn refresh_equipment_window(&mut self) {
        self.equipment_refreshed = true;
    }

    fn refresh_inventory_window(&mut self) {
        self.inventory_refreshed = true;
    }

    fn update_simple_player_info(&mut self, index: usize, hair: u32, armor: u32) {
        assert_eq!(index, self.selected_index);
        self.updated_hair = hair;
        self.updated_armor = armor;
    }

    fn get_selected_character_index(&self) -> usize {
        self.selected_index
    }
}

#[test]
fn test_packet_size_and_alignment() {
    let size = core::mem::size_of::<TPacketGCCharacterUpdate>();
    // header(2) + length(2) + dwVID(4) + awPart(2*4) + bMovingSpeed(1) + bAttackSpeed(1) + 
    // bStateFlag(1) + dwAffectFlag(4*2) + dwGuildID(4) + sAlignment(2) + bPKMode(1) + dwMountVnum(4)
    // 2 + 2 + 4 + 8 + 1 + 1 + 1 + 8 + 4 + 2 + 1 + 4 = 38
    assert_eq!(size, 38, "TPacketGCCharacterUpdate size must be strictly 38 bytes when packed");
}

#[test]
fn test_handle_packet_main_character() {
    let mut actor_mgr = MockActorManager::default();
    let mut player = MockPlayer {
        is_main: true,
        ..Default::default()
    };
    let mut network_stream = MockNetworkStream {
        selected_index: 0,
        ..Default::default()
    };

    let mut handler = CharacterUpdateHandler::new(&mut actor_mgr, &mut player, &mut network_stream);

    let packet = TPacketGCCharacterUpdate {
        header: 0xDEAD,
        length: 38,
        dwVID: 12345,
        awPart: [100, 200, 300, 400],
        bMovingSpeed: 10,
        bAttackSpeed: 20,
        bStateFlag: 1,
        dwAffectFlag: [123, 456],
        dwGuildID: 999,
        sAlignment: -100,
        bPKMode: 2,
        dwMountVnum: 20110,
    };

    let bytes = unsafe {
        core::slice::from_raw_parts(
            &packet as *const _ as *const u8,
            core::mem::size_of::<TPacketGCCharacterUpdate>(),
        )
    };

    let result = handler.handle_packet(bytes);
    assert!(result.is_ok());

    assert_eq!(actor_mgr.updated_vid, 12345);
    
    // Check main character logic
    assert_eq!(network_stream.guild_id, 999);
    assert_eq!(player.weapon_power, 200); // Check AbstractPlayer for weapon_power
    assert!(network_stream.status_refreshed);
    assert!(network_stream.alignment_refreshed);
    assert!(network_stream.equipment_refreshed);
    assert!(network_stream.inventory_refreshed);
    assert_eq!(network_stream.updated_hair, 400);
    assert_eq!(network_stream.updated_armor, 100);

    // Should not notify
    assert_eq!(player.notified_vid, 0);
}

#[test]
fn test_handle_packet_other_character() {
    let mut actor_mgr = MockActorManager::default();
    let mut player = MockPlayer {
        is_main: false,
        ..Default::default()
    };
    let mut network_stream = MockNetworkStream::default();

    let mut handler = CharacterUpdateHandler::new(&mut actor_mgr, &mut player, &mut network_stream);

    let packet = TPacketGCCharacterUpdate {
        header: 0xDEAD,
        length: 38,
        dwVID: 54321,
        awPart: [1, 2, 3, 4],
        bMovingSpeed: 0,
        bAttackSpeed: 0,
        bStateFlag: 0,
        dwAffectFlag: [0, 0],
        dwGuildID: 1,
        sAlignment: 0,
        bPKMode: 0,
        dwMountVnum: 0,
    };

    let bytes = unsafe {
        core::slice::from_raw_parts(
            &packet as *const _ as *const u8,
            core::mem::size_of::<TPacketGCCharacterUpdate>(),
        )
    };

    let result = handler.handle_packet(bytes);
    assert!(result.is_ok());

    assert_eq!(actor_mgr.updated_vid, 54321);
    
    // Should NOT refresh anything
    assert_eq!(network_stream.guild_id, 0);
    assert!(!network_stream.status_refreshed);
    assert!(!network_stream.alignment_refreshed);
    
    // Should notify
    assert_eq!(player.notified_vid, 54321);
}

#[test]
fn test_handle_packet_too_small() {
    let mut actor_mgr = MockActorManager::default();
    let mut player = MockPlayer::default();
    let mut network_stream = MockNetworkStream::default();
    
    let mut handler = CharacterUpdateHandler::new(&mut actor_mgr, &mut player, &mut network_stream);

    let bytes = [0u8; 10]; // Too small
    let result = handler.handle_packet(&bytes);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Packet too small");
}

#[test]
fn test_snetworkupdateactordata_conversion() {
    let packet = TPacketGCCharacterUpdate {
        header: 0,
        length: 0,
        dwVID: 777,
        awPart: [10, 20, 30, 40],
        bMovingSpeed: 100,
        bAttackSpeed: 110,
        bStateFlag: 5,
        dwAffectFlag: [111, 222],
        dwGuildID: 333,
        sAlignment: -500,
        bPKMode: 3,
        dwMountVnum: 12345,
    };

    let actor_data = SNetworkUpdateActorData::from(&packet);

    assert_eq!(actor_data.m_dwVID, { packet.dwVID });
    assert_eq!(actor_data.m_dwGuildID, { packet.dwGuildID });
    assert_eq!(actor_data.m_dwArmor, 10);
    assert_eq!(actor_data.m_dwWeapon, 20);
    assert_eq!(actor_data.m_dwHair, 40);
    assert_eq!(actor_data.m_dwMovSpd, 100);
    assert_eq!(actor_data.m_dwAtkSpd, 110);
    assert_eq!(actor_data.m_sAlignment, { packet.sAlignment });
    assert_eq!(actor_data.m_byPKMode, { packet.bPKMode });
    assert_eq!(actor_data.m_dwMountVnum, { packet.dwMountVnum });
    assert_eq!(actor_data.m_dwStateFlags, 5);
    let affect = { packet.dwAffectFlag };
    assert_eq!(actor_data.m_kAffectFlags, affect);
}
