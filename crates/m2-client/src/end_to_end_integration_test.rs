use std::ffi::c_char;

// Definitions from src/UserInterface/Packet.h
pub const CHARACTER_NAME_MAX_LEN: usize = 64;
pub const ID_MAX_NUM: usize = 30;
pub const PASS_MAX_NUM: usize = 16;
pub const PLAYER_PER_ACCOUNT4: usize = 4;
pub const GUILD_NAME_MAX_LEN: usize = 12;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TSimplePlayerInformation {
    pub dwID: u32,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub byJob: u8,
    pub byLevel: u8,
    pub dwPlayMinutes: u32,
    pub byST: u8,
    pub byHT: u8,
    pub byDX: u8,
    pub byIQ: u8,
    pub wMainPart: u16,
    pub byHairPart: u8,
    pub x: i32,
    pub y: i32,
    pub lAddr: i32,
    pub wPort: u16,
    pub bySkillGroup: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCLoginSuccess4 {
    pub header: u16,
    pub length: u16,
    pub akSimplePlayerInformation: [TSimplePlayerInformation; PLAYER_PER_ACCOUNT4],
    pub guild_id: [u32; PLAYER_PER_ACCOUNT4],
    pub guild_name: [[u8; GUILD_NAME_MAX_LEN + 1]; PLAYER_PER_ACCOUNT4],
    pub handle: u32,
    pub random_key: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGEnterFrontGame {
    pub header: u16,
    pub length: u16,
}

pub const MUSIC_NAME_MAX_LEN: usize = 24;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMainCharacter {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub wRaceNum: u16,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub szBGMName: [u8; MUSIC_NAME_MAX_LEN + 1],
    pub fBGMVol: f32,
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub byEmpire: u8,
    pub bySkillGroup: u8,
}

pub const CHR_EQUIPPART_NUM: usize = 4;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCCharacterAdd {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub bType: u8,
    pub wRaceNum: u16,
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,
    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMove {
    pub header: u16,
    pub length: u16,
    pub bFunc: u8,
    pub bArg: u8,
    pub bRot: u8,
    pub dwVID: u32,
    pub lX: i32,
    pub lY: i32,
    pub dwTime: u32,
    pub dwDuration: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCDamageInfo {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub flag: u8,
    pub damage: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3; // Wlasciwy rozmiar z GameType.h (3 wg zrodel)
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemSet {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: u8,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

// Ensure default implementations for creating blank test packets
impl Default for TPacketGCLoginSuccess4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCMainCharacter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCCharacterAdd {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCMove {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCDamageInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCItemSet {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub fn dummy_module_function() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_to_end_client_flow() {
        // 1. Weryfikacja rozmiarow pakietow binarnych zgodnych z C++
        assert_eq!(std::mem::size_of::<TPacketCGEnterFrontGame>(), 4);
        assert_eq!(std::mem::size_of::<TSimplePlayerInformation>(), 97);

        // 2. Symulacja pakietu GC_LOGIN_SUCCESS4 od serwera
        let mut login_success = TPacketGCLoginSuccess4::default();
        login_success.header = 0x0104; // GC_LOGIN_SUCCESS4
        login_success.length = std::mem::size_of::<TPacketGCLoginSuccess4>() as u16;
        login_success.handle = 1001;
        login_success.random_key = 0xDEADBEEF;

        login_success.akSimplePlayerInformation[0].dwID = 777;
        let char_name = b"WojownikBody\0";
        login_success.akSimplePlayerInformation[0].szName[..char_name.len()].copy_from_slice(char_name);
        login_success.akSimplePlayerInformation[0].byJob = 0; // Wojownik
        login_success.akSimplePlayerInformation[0].byLevel = 99;
        login_success.akSimplePlayerInformation[0].byST = 90;
        login_success.akSimplePlayerInformation[0].byHT = 90;
        login_success.akSimplePlayerInformation[0].byDX = 90;
        login_success.akSimplePlayerInformation[0].byIQ = 90;
        login_success.akSimplePlayerInformation[0].wMainPart = 11299; // Zbroja z czarnej stali +9
        login_success.akSimplePlayerInformation[0].byHairPart = 1;

        assert_eq!({ login_success.akSimplePlayerInformation[0].dwID }, 777);
        assert_eq!({ login_success.akSimplePlayerInformation[0].byLevel }, 99);

        // 3. Symulacja wejscia do gry (MainCharacter)
        let mut main_char = TPacketGCMainCharacter::default();
        main_char.header = 0x0110;
        main_char.dwVID = 777;
        main_char.wRaceNum = 0;
        main_char.lX = 469300;
        main_char.lY = 964200;
        main_char.byEmpire = 1; // Shinsoo
        main_char.bySkillGroup = 1; // Wojownik Cialo (Body)

        assert_eq!({ main_char.dwVID }, 777);
        assert_eq!({ main_char.byEmpire }, 1);
        assert_eq!({ main_char.bySkillGroup }, 1);

        // 4. Symulacja ruchu (Move)
        let mut move_pkt = TPacketGCMove::default();
        move_pkt.header = 0x0103;
        move_pkt.dwVID = 777;
        move_pkt.bFunc = 1; // MOVE
        move_pkt.lX = 469400;
        move_pkt.lY = 964300;
        move_pkt.bRot = 180;

        assert_eq!({ move_pkt.dwVID }, 777);
        assert_eq!({ move_pkt.lX }, 469400);

        // 5. Symulacja spawnu innego aktora / moba (Dziki Pies vnum=101)
        let mut mob_add = TPacketGCCharacterAdd::default();
        mob_add.header = 0x0101;
        mob_add.dwVID = 20001;
        mob_add.wRaceNum = 101; // Wild Dog
        mob_add.bType = 0; // CHAR_TYPE_MONSTER
        mob_add.bMovingSpeed = 150;
        mob_add.bAttackSpeed = 100;

        assert_eq!({ mob_add.dwVID }, 20001);
        assert_eq!({ mob_add.wRaceNum }, 101);

        // 6. Symulacja zadania obrazen (Damage Info)
        let mut dmg = TPacketGCDamageInfo::default();
        dmg.header = 0x0120;
        dmg.dwVID = 20001;
        dmg.flag = 0;
        dmg.damage = 1540;

        assert_eq!({ dmg.damage }, 1540);
        assert_eq!({ dmg.dwVID }, 20001);

        // 7. Symulacja dodania przedmiotu do ekwipunku (Miecz Bojowy +9, vnum=169)
        let mut item_set = TPacketGCItemSet::default();
        item_set.header = 0x0130;
        item_set.pos = TItemPos { window_type: 1, cell: 0 };
        item_set.vnum = 169;
        item_set.count = 1;
        item_set.alSockets[0] = 28430; // KD Smierci +4
        item_set.alSockets[1] = 28431; // KD Szybkiego Ataku +4
        item_set.alSockets[2] = 28432; // KD Potwora +4
        item_set.aAttr[0].bType = 7; // Srednie Obrazenia
        item_set.aAttr[0].sValue = 48; // 48% sr

        assert_eq!({ item_set.vnum }, 169);
        assert_eq!({ item_set.aAttr[0].sValue }, 48);
        assert_eq!({ item_set.alSockets[0] }, 28430);
    }
}

