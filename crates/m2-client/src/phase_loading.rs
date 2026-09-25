//! Modul `phase_loading` — Obsluga fazy ladowania mapy i wejscia do swiata LoadingPhase
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseLoading.cpp`
//! Opis: Odwzorowanie LoadingPhase: postep ladowania zasobow mapy (0..100%), odbior HEADER_GC_MAIN_CHARACTER (VID gracza, pozycja X/Y/Z, rasa), wysylanie HEADER_CG_ENTERGAME.

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const MUSIC_NAME_MAX_LEN: usize = 24;
pub const POINT_MAX_NUM: usize = 255;

pub const HEADER_CG_ENTERGAME: u8 = 10;
pub const HEADER_GC_MAIN_CHARACTER: u8 = 15;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCMainCharacter {
    pub header: u8,
    pub dwVID: u32,
    pub wRaceNum: u16,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub bySkillGroup: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCMainCharacter2_EMPIRE {
    pub header: u8,
    pub dwVID: u32,
    pub wRaceNum: u16,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub byEmpire: u8,
    pub bySkillGroup: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCMainCharacter3_BGM {
    pub header: u8,
    pub dwVID: u32,
    pub wRaceNum: u16,
    pub szUserName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub szBGMName: [u8; MUSIC_NAME_MAX_LEN + 1],
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub byEmpire: u8,
    pub bySkillGroup: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCMainCharacter4_BGM_VOL {
    pub header: u8,
    pub dwVID: u32,
    pub wRaceNum: u16,
    pub szUserName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub szBGMName: [u8; MUSIC_NAME_MAX_LEN + 1],
    pub fBGMVol: f32,
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub byEmpire: u8,
    pub bySkillGroup: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCPoints {
    pub header: u8,
    pub points: [i32; POINT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketCGEnterFrontGame {
    pub header: u8,
}

#[derive(Debug, Clone)]
pub struct PhaseLoading {
    pub initialized: bool,
    pub main_actor_vid: u32,
    pub main_actor_race: u16,
    pub main_actor_empire: u8,
    pub main_actor_skill_group: u8,
    pub main_actor_name: String,
    pub field_music_file_name: String,
    pub field_music_volume: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub progress: u8,
    pub is_start_game: bool,
}

impl Default for PhaseLoading {
    fn default() -> Self {
        Self {
            initialized: true,
            main_actor_vid: 0,
            main_actor_race: 0,
            main_actor_empire: 0,
            main_actor_skill_group: 0,
            main_actor_name: String::new(),
            field_music_file_name: String::new(),
            field_music_volume: 1.0 / 5.0 * 0.1,
            x: 0,
            y: 0,
            z: 0,
            progress: 0,
            is_start_game: false,
        }
    }
}

impl PhaseLoading {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    fn parse_string(buffer: &[u8]) -> String {
        let null_pos = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
        String::from_utf8_lossy(&buffer[..null_pos]).into_owned()
    }

    pub fn recv_main_character(&mut self, packet: &TPacketGCMainCharacter) -> bool {
        let vid = std::ptr::addr_of!(packet.dwVID);
        let race = std::ptr::addr_of!(packet.wRaceNum);
        let skill_group = std::ptr::addr_of!(packet.bySkillGroup);
        
                let x = std::ptr::addr_of!(packet.lX);
        let y = std::ptr::addr_of!(packet.lY);
        let z = std::ptr::addr_of!(packet.lZ);
        
        unsafe {
            self.main_actor_vid = vid.read_unaligned();
            self.main_actor_race = race.read_unaligned();
            self.main_actor_skill_group = skill_group.read_unaligned();
            self.x = x.read_unaligned();
            self.y = y.read_unaligned();
            self.z = z.read_unaligned();
        }
        self.main_actor_empire = 0;
        self.main_actor_name = Self::parse_string(&packet.szName);
        true
    }

    pub fn recv_main_character2_empire(&mut self, packet: &TPacketGCMainCharacter2_EMPIRE) -> bool {
        let vid = std::ptr::addr_of!(packet.dwVID);
        let race = std::ptr::addr_of!(packet.wRaceNum);
        let empire = std::ptr::addr_of!(packet.byEmpire);
        let skill_group = std::ptr::addr_of!(packet.bySkillGroup);
        
                let x = std::ptr::addr_of!(packet.lX);
        let y = std::ptr::addr_of!(packet.lY);
        let z = std::ptr::addr_of!(packet.lZ);
        
        unsafe {
            self.main_actor_vid = vid.read_unaligned();
            self.main_actor_race = race.read_unaligned();
            self.main_actor_empire = empire.read_unaligned();
            self.main_actor_skill_group = skill_group.read_unaligned();
            self.x = x.read_unaligned();
            self.y = y.read_unaligned();
            self.z = z.read_unaligned();
        }
        self.main_actor_name = Self::parse_string(&packet.szName);
        true
    }

    pub fn recv_main_character3_bgm(&mut self, packet: &TPacketGCMainCharacter3_BGM) -> bool {
        let vid = std::ptr::addr_of!(packet.dwVID);
        let race = std::ptr::addr_of!(packet.wRaceNum);
        let empire = std::ptr::addr_of!(packet.byEmpire);
        let skill_group = std::ptr::addr_of!(packet.bySkillGroup);
        
                let x = std::ptr::addr_of!(packet.lX);
        let y = std::ptr::addr_of!(packet.lY);
        let z = std::ptr::addr_of!(packet.lZ);
        
        unsafe {
            self.main_actor_vid = vid.read_unaligned();
            self.main_actor_race = race.read_unaligned();
            self.main_actor_empire = empire.read_unaligned();
            self.main_actor_skill_group = skill_group.read_unaligned();
            self.x = x.read_unaligned();
            self.y = y.read_unaligned();
            self.z = z.read_unaligned();
        }
        self.main_actor_name = Self::parse_string(&packet.szUserName);
        self.field_music_file_name = Self::parse_string(&packet.szBGMName);
        true
    }

    pub fn recv_main_character4_bgm_vol(&mut self, packet: &TPacketGCMainCharacter4_BGM_VOL) -> bool {
        let vid = std::ptr::addr_of!(packet.dwVID);
        let race = std::ptr::addr_of!(packet.wRaceNum);
        let empire = std::ptr::addr_of!(packet.byEmpire);
        let skill_group = std::ptr::addr_of!(packet.bySkillGroup);
        let vol = std::ptr::addr_of!(packet.fBGMVol);
        
        unsafe {
            self.main_actor_vid = vid.read_unaligned();
            self.main_actor_race = race.read_unaligned();
            self.main_actor_empire = empire.read_unaligned();
            self.main_actor_skill_group = skill_group.read_unaligned();
            self.field_music_volume = vol.read_unaligned();
        }
        self.main_actor_name = Self::parse_string(&packet.szUserName);
        self.field_music_file_name = Self::parse_string(&packet.szBGMName);
        true
    }

    pub fn recv_player_points(&mut self, _packet: &TPacketGCPoints) -> bool {
        true
    }

    pub fn send_enter_game(&self) -> TPacketCGEnterFrontGame {
        TPacketCGEnterFrontGame {
            header: HEADER_CG_ENTERGAME,
        }
    }

    pub fn set_field_music_file_name(&mut self, name: String) {
        self.field_music_file_name = name;
    }

    pub fn set_field_music_file_info(&mut self, name: String, vol: f32) {
        self.field_music_file_name = name;
        self.field_music_volume = vol;
    }

    pub fn get_field_music_file_name(&self) -> &str {
        &self.field_music_file_name
    }

    pub fn get_field_music_volume(&self) -> f32 {
        self.field_music_volume
    }

    
    pub fn set_progress(&mut self, progress: u8) {
        self.progress = progress.clamp(0, 100);
    }

    pub fn get_progress(&self) -> u8 {
        self.progress
    }

    pub fn start_game(&mut self) {
        self.is_start_game = true;
    }
}
