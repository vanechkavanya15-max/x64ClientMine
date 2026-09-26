pub struct InstanceBase {
    pub vid: u32,
    pub race: u32,
    pub name: String,
    pub empire: u8,
    pub guild_id: u32,
    pub alignment: i32,
    pub pk_mode: u8,
    pub pos: (f32, f32, f32),
    pub rot_deg: f32,
    pub cur_speed: f32,
    pub armor_vnum: u32,
    pub weapon_vnum: u32,
    pub hair_vnum: u32,
    pub is_alive: bool,
    pub is_stunned: bool,
    pub is_poisoned: bool,
}

impl InstanceBase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        vid: u32,
        race: u32,
        name: String,
        empire: u8,
        guild_id: u32,
        alignment: i32,
        pk_mode: u8,
        pos: (f32, f32, f32),
        rot_deg: f32,
        cur_speed: f32,
        armor_vnum: u32,
        weapon_vnum: u32,
        hair_vnum: u32,
    ) -> Self {
        Self {
            vid,
            race,
            name,
            empire,
            guild_id,
            alignment,
            pk_mode,
            pos,
            rot_deg,
            cur_speed,
            armor_vnum,
            weapon_vnum,
            hair_vnum,
            is_alive: true,
            is_stunned: false,
            is_poisoned: false,
        }
    }

    pub fn is_same_empire(&self, other: &Self) -> bool {
        if self.empire == 0 || other.empire == 0 {
            return false;
        }
        self.empire == other.empire
    }

    pub fn is_same_guild(&self, other: &Self) -> bool {
        if self.guild_id == 0 || other.guild_id == 0 {
            return false;
        }
        self.guild_id == other.guild_id
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive
    }

    pub fn set_dead(&mut self) {
        self.is_alive = false;
        self.cur_speed = 0.0;
        self.is_stunned = false;
        self.is_poisoned = false;
    }

    pub fn set_stunned(&mut self, stunned: bool) {
        if self.is_alive {
            self.is_stunned = stunned;
        }
    }
}
