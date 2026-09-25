//! Modul `instance_base` — Klasa bazowa instancji postaci w swiecie gry CInstanceBase
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBase.cpp`
//! Opis: Odwzorowanie CInstanceBase: VID, nazwa, rasa, krolestwo (Shinsoo/Chunjo/Jinno), gildia, ranga (alignment), PK mode, zalozona zbroja, bron, fryzura, wierzchowiec.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North = 0,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    MaxNum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Func {
    Wait = 0,
    Move = 1,
    Attack = 2,
    Combo = 3,
    MobSkill = 4,
    Emotion = 5,
    Skill = 0x80,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Affect {
    Ymir = 0,
    Invisibility = 1,
    Spawn = 2,
    Poison = 3,
    Slow = 4,
    Stun = 5,
    DungeonReady = 6,
    ShowAlways = 7,
    BuildingConstructionSmall = 8,
    BuildingConstructionLarge = 9,
    BuildingUpgrade = 10,
    MovSpeedPotion = 11,
    AttSpeedPotion = 12,
    FishMind = 13,
    Jeongwi = 14,
    Geomgyeong = 15,
    Cheongeun = 16,
    Gyeonggong = 17,
    Eunhyeong = 18,
    Gwigeom = 19,
    Gongpo = 20,
    Jumagap = 21,
    Hosin = 22,
    Boho = 23,
    Kwaesok = 24,
    Heuksin = 25,
    Muyeong = 26,
    ReviveInvisibility = 27,
    Fire = 28,
    Gicheon = 29,
    Jeungryeok = 30,
    Dash = 31,
    Pabeop = 32,
    FallenCheongeun = 33,
    Polymorph = 34,
    WarFlag1 = 35,
    WarFlag2 = 36,
    WarFlag3 = 37,
    ChinaFirework = 38,
    PremiumSilver = 39,
    PremiumGold = 40,
    RamadanRing = 41,
    Num = 64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    DualHand = 0,
    OneHand = 1,
    TwoHand = 2,
    Num = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Empire {
    None = 0,
    A = 1,
    B = 2,
    C = 3,
    Num = 4,
}

#[derive(Debug, Clone, Default)]
pub struct CreateData {
    pub type_: u8,
    pub state_flags: u32,
    pub empire_id: u32,
    pub guild_id: u32,
    pub level: u32,
    pub vid: u32,
    pub race: u32,
    pub mov_spd: u32,
    pub atk_spd: u32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub rot: f32,
    pub armor: u32,
    pub weapon: u32,
    pub hair: u32,
    pub mount_vnum: u32,
    pub alignment: i16,
    pub pk_mode: u8,
    pub name: String,
    pub is_main: bool,
}

#[derive(Debug, Clone, Default)]
pub struct InstanceBase {
    pub initialized: bool,
    pub vid: u32,
    pub name: String,
    pub race: u32,
    pub empire_id: u32,
    pub guild_id: u32,
    pub level: u32,
    pub alignment: i16,
    pub pk_mode: u8,
    pub armor: u32,
    pub weapon: u32,
    pub hair: u32,
    pub mount_vnum: u32,
    pub type_: u8,
    pub state_flags: u32,
    pub mov_spd: u32,
    pub atk_spd: u32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub rot: f32,
    pub is_main: bool,
}

impl InstanceBase {
    pub fn new() -> Self {
        Self {
            initialized: true,
            ..Default::default()
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn create(&mut self, data: CreateData) {
        self.type_ = data.type_;
        self.state_flags = data.state_flags;
        self.empire_id = data.empire_id;
        self.guild_id = data.guild_id;
        self.level = data.level;
        self.vid = data.vid;
        self.race = data.race;
        self.mov_spd = data.mov_spd;
        self.atk_spd = data.atk_spd;
        self.pos_x = data.pos_x;
        self.pos_y = data.pos_y;
        self.rot = data.rot;
        self.armor = data.armor;
        self.weapon = data.weapon;
        self.hair = data.hair;
        self.mount_vnum = data.mount_vnum;
        self.alignment = data.alignment;
        self.pk_mode = data.pk_mode;
        self.name = data.name;
        self.is_main = data.is_main;
        self.initialized = true;
    }

    pub fn set_vid(&mut self, vid: u32) {
        self.vid = vid;
    }

    pub fn get_vid(&self) -> u32 {
        self.vid
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_race(&mut self, race: u32) {
        self.race = race;
    }

    pub fn get_race(&self) -> u32 {
        self.race
    }

    pub fn set_empire_id(&mut self, empire_id: u32) {
        self.empire_id = empire_id;
    }

    pub fn get_empire_id(&self) -> u32 {
        self.empire_id
    }

    pub fn set_guild_id(&mut self, guild_id: u32) {
        self.guild_id = guild_id;
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn set_alignment(&mut self, alignment: i16) {
        self.alignment = alignment;
    }

    pub fn get_alignment(&self) -> i16 {
        self.alignment
    }

    pub fn set_pk_mode(&mut self, pk_mode: u8) {
        self.pk_mode = pk_mode;
    }

    pub fn get_pk_mode(&self) -> u8 {
        self.pk_mode
    }

    pub fn set_armor(&mut self, armor: u32) {
        self.armor = armor;
    }

    pub fn get_armor(&self) -> u32 {
        self.armor
    }

    pub fn set_weapon(&mut self, weapon: u32) {
        self.weapon = weapon;
    }

    pub fn get_weapon(&self) -> u32 {
        self.weapon
    }

    pub fn set_hair(&mut self, hair: u32) {
        self.hair = hair;
    }

    pub fn get_hair(&self) -> u32 {
        self.hair
    }

    pub fn set_mount_vnum(&mut self, mount_vnum: u32) {
        self.mount_vnum = mount_vnum;
    }

    pub fn get_mount_vnum(&self) -> u32 {
        self.mount_vnum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_instance_base() {
        let instance = InstanceBase::new();
        assert!(instance.is_ready());
    }

    #[test]
    fn test_create_instance_base() {
        let mut instance = InstanceBase::new();
        let data = CreateData {
            vid: 1234,
            name: "TestPlayer".to_string(),
            race: 0,
            empire_id: 1,
            level: 50,
            ..Default::default()
        };
        instance.create(data);
        assert_eq!(instance.get_vid(), 1234);
        assert_eq!(instance.get_name(), "TestPlayer");
        assert_eq!(instance.get_race(), 0);
        assert_eq!(instance.get_empire_id(), 1);
        assert_eq!(instance.get_level(), 50);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PixelPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub chk_time: u32,
    pub cmd_time: u32,
    pub dst_rot: f32,
    pub func: u32,
    pub arg: u32,
    pub target_vid: u32,
    pub dst_pos: PixelPosition,
}

impl InstanceBase {
    pub fn push_command(&mut self, _cmd: Command) {
        // Here we would push it to a command queue, for now we will just simulate StateProcess
    }

    pub fn state_process(&mut self, command: Command) {
        if !self.can_process_network_state_packet() {
            return;
        }

        if !self.is_enable_tcp_process(command.func) {
            return;
        }
        
        let func = command.func;
        if func == Func::Wait as u32 {
            self.set_dst_rot(command.dst_rot);
            self.start_walking();
        } else if func == Func::Move as u32 {
            self.set_dst_rot(command.dst_rot);
            self.start_walking();
        } else if func == Func::Attack as u32 {
            self.attack(command.arg);
        } else if func == Func::Combo as u32 {
            self.combo_attack(command.arg);
        } else if func == Func::MobSkill as u32 {
            self.mob_skill(command.arg);
        } else if func == Func::Emotion as u32 {
            self.emotion(command.arg, command.target_vid, command.dst_pos);
        } else if (func & Func::Skill as u32) != 0 {
            self.skill(command.arg);
        }
    }

    pub fn can_process_network_state_packet(&self) -> bool {
        // Will evaluate if character is dead, knocked down etc.
        true
    }

    pub fn is_enable_tcp_process(&self, _func: u32) -> bool {
        // Will check if tcp process is enabled
        true
    }

    pub fn set_dst_rot(&mut self, rot: f32) {
        self.rot = rot;
    }

    pub fn start_walking(&mut self) {
        // Simulates switching into walking mode
    }

    pub fn attack(&mut self, _arg: u32) {
        // Simulates attack
    }

    pub fn combo_attack(&mut self, _arg: u32) {
        // Simulates combo
    }

    pub fn mob_skill(&mut self, _arg: u32) {
        // Simulates mob skill
    }

    pub fn emotion(&mut self, _arg: u32, _target_vid: u32, _dst_pos: PixelPosition) {
        // Simulates emotion
    }

    pub fn skill(&mut self, _arg: u32) {
        // Simulates skill
    }
}
