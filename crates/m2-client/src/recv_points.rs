use m2_client_bridge::net::gc::char_status::{TPacketGCPoints, TPacketGCPointChange, POINT_MAX_NUM};

pub const POINT_LEVEL: u8 = 1;
pub const POINT_EXP: u8 = 3;
pub const POINT_NEXT_EXP: u8 = 4;
pub const POINT_HP: u8 = 5;
pub const POINT_MAX_HP: u8 = 6;
pub const POINT_SP: u8 = 7;
pub const POINT_MAX_SP: u8 = 8;
pub const POINT_GOLD: u8 = 11;
pub const POINT_ST: u8 = 12;
pub const POINT_HT: u8 = 13;
pub const POINT_DX: u8 = 14;
pub const POINT_IQ: u8 = 15;
pub const POINT_PLAYTIME: u8 = 31;
pub const POINT_STAT_RESET_COUNT: u8 = 112; 
pub const POINT_SKILL: u8 = 28;
pub const POINT_SUB_SKILL: u8 = 27;
pub const POINT_HORSE_SKILL: u8 = 109; 
pub const POINT_ENERGY: u8 = 128; 
pub const POINT_ENERGY_END_TIME: u8 = 129; 

pub struct GlobalGameState {
    pub points: [i32; POINT_MAX_NUM],
    pub level: i32,
    pub play_time: i32,
    pub gold: i32,
    pub exp: i32,
    pub next_exp: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub sp: i32,
    pub max_sp: i32,
    pub st: i32,
    pub ht: i32,
    pub dx: i32,
    pub iq: i32,
    pub skill_point: i32,
    pub sub_skill_point: i32,
    pub horse_skill_point: i32,
    pub vid: u32,
    pub money_picked: i32,
}

impl GlobalGameState {
    pub fn new() -> Self {
        Self {
            points: [0; POINT_MAX_NUM],
            level: 0,
            play_time: 0,
            gold: 0,
            exp: 0,
            next_exp: 0,
            hp: 0,
            max_hp: 0,
            sp: 0,
            max_sp: 0,
            st: 0,
            ht: 0,
            dx: 0,
            iq: 0,
            skill_point: 0,
            sub_skill_point: 0,
            horse_skill_point: 0,
            vid: 0,
            money_picked: 0,
        }
    }

    pub fn handle_points_packet(&mut self, packet: &TPacketGCPoints) -> bool {
        for i in 0..POINT_MAX_NUM {
            self.set_status(i as u8, packet.points[i]);
        }
        true
    }

    pub fn handle_point_change_packet(&mut self, packet: &TPacketGCPointChange, current_vid: u32) -> bool {
        if packet.Type as usize >= POINT_MAX_NUM {
            return false;
        }

        if packet.dwVID == current_vid {
            self.set_status(packet.Type, packet.value);
            
            if packet.Type == POINT_ENERGY && packet.value == 0 {
                self.set_status(POINT_ENERGY_END_TIME, 0);
            }
            
            if packet.Type == POINT_GOLD {
                if packet.amount > 0 {
                    self.money_picked = packet.amount;
                }
            }
        } else if packet.Type == POINT_LEVEL {
            // It modifies instance level directly for other instances in C++
            // Here we can just consider the parsing as successful and not modifying the main player struct
            return true;
        }
        
        true
    }

    pub fn set_status(&mut self, point_type: u8, value: i32) {
        if (point_type as usize) >= POINT_MAX_NUM {
            return;
        }

        self.points[point_type as usize] = value;
            
        match point_type {
            POINT_LEVEL => self.level = value,
            POINT_ST => self.st = value,
            POINT_HT => self.ht = value,
            POINT_DX => self.dx = value,
            POINT_IQ => self.iq = value,
            POINT_PLAYTIME | POINT_STAT_RESET_COUNT => self.play_time = value,
            POINT_GOLD => self.gold = value,
            POINT_EXP => self.exp = value,
            POINT_NEXT_EXP => self.next_exp = value,
            POINT_HP => self.hp = value,
            POINT_MAX_HP => self.max_hp = value,
            POINT_SP => self.sp = value,
            POINT_MAX_SP => self.max_sp = value,
            POINT_SKILL => self.skill_point = value,
            POINT_SUB_SKILL => self.sub_skill_point = value,
            POINT_HORSE_SKILL => self.horse_skill_point = value,
            _ => {}
        }
    }
}
