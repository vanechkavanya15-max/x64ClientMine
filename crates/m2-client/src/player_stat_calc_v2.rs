use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone, Default)]
pub struct PlayerStatCalc {
    pub con: i32,
    pub int_: i32,
    pub str_: i32,
    pub dex: i32,
    pub level: u32,
    pub base_hp: i32,
    pub equip_hp: i32,
    pub base_sp: i32,
    pub equip_sp: i32,
    pub equip_atk: i32,
    pub equip_def: i32,
}

impl PlayerStatCalc {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_hp(&self) -> i64 {
        (self.con as i64)
            .saturating_mul(20)
            .saturating_add(self.base_hp as i64)
            .saturating_add(self.equip_hp as i64)
    }

    pub fn max_sp(&self) -> i64 {
        (self.int_ as i64)
            .saturating_mul(20)
            .saturating_add(self.base_sp as i64)
            .saturating_add(self.equip_sp as i64)
    }

    pub fn atk_grade(&self) -> i64 {
        let stat_atk = (self.str_ as i64).saturating_mul(2);
        let level_atk = (self.level as i64).saturating_mul(2);
        stat_atk
            .saturating_add(level_atk)
            .saturating_add(self.equip_atk as i64)
    }

    pub fn def_grade(&self) -> i64 {
        (self.con as i64).saturating_add(self.equip_def as i64)
    }

    pub fn mag_atk(&self) -> i64 {
        let stat_matk = (self.int_ as i64).saturating_mul(2);
        let level_matk = (self.level as i64).saturating_mul(2);
        stat_matk.saturating_add(level_matk)
    }

    pub fn evasion(&self) -> i64 {
        let d = self.dex.max(0) as i64 ;
        (30 * (2 * d + 5)) / (d + 95)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PlayerStatResult {
    pub hp: i64,
    pub sp: i64,
    pub atk: i64,
    pub def: i64,
}

#[no_mangle]
pub extern "C" fn m2_stat_calculate(con: i32, int_: i32, str_: i32, dex: i32, level: u32) -> PlayerStatResult {
    safe_ffi_call(PlayerStatResult::default(), || {
        let calc = PlayerStatCalc {
            con,
            int_,
            str_,
            dex,
            level,
            ..Default::default()
        };
        PlayerStatResult {
            hp: calc.max_hp(),
            sp: calc.max_sp(),
            atk: calc.atk_grade(),
            def: calc.def_grade(),
        }
    })
}
