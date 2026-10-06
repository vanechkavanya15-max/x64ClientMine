#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ItemRarity {
    Common = 0,
    Magic = 1,
    Rare = 2,
    Epic = 3,
    Legendary = 4,
}

impl ItemRarity {
    pub fn get_color(&self) -> [f32; 4] {
        match self {
            ItemRarity::Common => [1.0, 1.0, 1.0, 1.0],     // White
            ItemRarity::Magic => [0.0, 0.5, 1.0, 1.0],      // Blue
            ItemRarity::Rare => [1.0, 0.78, 0.20, 1.0],     // Gold
            ItemRarity::Epic => [0.8, 0.2, 1.0, 1.0],       // Purple
            ItemRarity::Legendary => [1.0, 0.2, 0.2, 1.0],  // Red
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Bonus {
    pub attr_type: u8,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct SocketData {
    pub vnum: u32,
}

#[derive(Debug, Clone)]
pub struct ItemData {
    pub name: String,
    pub rarity: ItemRarity,
    pub level_limit: u32,
    pub class_flags: u32, // Bitmask for class requirements (e.g. 1<<2 for warrior, 1<<3 for assassin, etc., representing banned classes)
    pub attack_min: i32,
    pub attack_max: i32,
    pub magic_attack_min: i32,
    pub magic_attack_max: i32,
    pub defense: i32,
    pub main_bonuses: Vec<Bonus>, // max 5
    pub extra_bonuses: Vec<Bonus>, // max 2
    pub sockets: [Option<SocketData>; 3], // 3 sockets, soul stones
    pub duration_seconds: Option<u32>,
}

impl Default for ItemData {
    fn default() -> Self {
        Self {
            name: String::new(),
            rarity: ItemRarity::Common,
            level_limit: 0,
            class_flags: 0,
            attack_min: 0,
            attack_max: 0,
            magic_attack_min: 0,
            magic_attack_max: 0,
            defense: 0,
            main_bonuses: Vec::new(),
            extra_bonuses: Vec::new(),
            sockets: [None, None, None],
            duration_seconds: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TooltipLine {
    pub text: String,
    pub color: [f32; 4],
}

pub struct TooltipCompositeBuilderV2 {
    pub lines: Vec<TooltipLine>,
}

impl TooltipCompositeBuilderV2 {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn build(
        &mut self,
        data: &ItemData,
        player_level: u32,
        player_job: u8,
    ) -> Result<(), &'static str> {
        self.lines.clear();

        if data.main_bonuses.len() > 5 {
            return Err("Too many main bonuses (max 5)");
        }
        if data.extra_bonuses.len() > 2 {
            return Err("Too many extra bonuses (max 2)");
        }

        // 1. Name with rarity color
        self.lines.push(TooltipLine {
            text: data.name.clone(),
            color: data.rarity.get_color(),
        });

        // 2. Class Requirements
        let is_warrior_banned = (data.class_flags & (1 << 2)) != 0;
        let is_assassin_banned = (data.class_flags & (1 << 3)) != 0;
        let is_sura_banned = (data.class_flags & (1 << 4)) != 0;
        let is_shaman_banned = (data.class_flags & (1 << 5)) != 0;

        let mut class_names = Vec::new();
        if !is_warrior_banned {
            class_names.push("Wojownik");
        }
        if !is_assassin_banned {
            class_names.push("Ninja");
        }
        if !is_sura_banned {
            class_names.push("Sura");
        }
        if !is_shaman_banned {
            class_names.push("Szaman");
        }

        if !class_names.is_empty() && class_names.len() < 4 {
            let classes_str = class_names.join(", ");
            let is_allowed = match player_job {
                0 => !is_warrior_banned,
                1 => !is_assassin_banned,
                2 => !is_sura_banned,
                3 => !is_shaman_banned,
                _ => false,
            };
            let color = if is_allowed {
                [0.6, 0.6, 0.6, 1.0] // COLOR_CLASS_MET
            } else {
                [1.0, 0.1, 0.1, 1.0] // COLOR_CLASS_UNMET
            };
            self.lines.push(TooltipLine {
                text: format!("Klasy: {}", classes_str),
                color,
            });
        }

        // 3. Level Limit
        if data.level_limit > 0 {
            let color = if player_level >= data.level_limit {
                [0.6, 0.6, 0.6, 1.0] // COLOR_LEVEL_GREY
            } else {
                [1.0, 0.1, 0.1, 1.0] // COLOR_LEVEL_RED
            };
            self.lines.push(TooltipLine {
                text: format!("Od poziomu: {}", data.level_limit),
                color,
            });
        }

        // 4. Attack/Defense Values
        let color_damage = [0.8, 0.8, 0.8, 1.0];
        if data.attack_min > 0 || data.attack_max > 0 {
            self.lines.push(TooltipLine {
                text: format!("Wartość ataku {}-{}", data.attack_min, data.attack_max),
                color: color_damage,
            });
        }
        if data.magic_attack_min > 0 || data.magic_attack_max > 0 {
            self.lines.push(TooltipLine {
                text: format!(
                    "Wartość magicznego ataku {}-{}",
                    data.magic_attack_min, data.magic_attack_max
                ),
                color: color_damage,
            });
        }
        if data.defense > 0 {
            self.lines.push(TooltipLine {
                text: format!("Obrona {}", data.defense),
                color: color_damage,
            });
        }

        // 5. Main Bonuses
        let color_bonus = [0.8, 0.8, 0.8, 1.0];
        for bonus in &data.main_bonuses {
            if bonus.attr_type == 0 || bonus.value == 0 {
                continue;
            }
            let name = Self::get_apply_name(bonus.attr_type);
            let sign = if bonus.value >= 0 { "+" } else { "" };
            self.lines.push(TooltipLine {
                text: format!("{}: {}{}", name, sign, bonus.value),
                color: color_bonus,
            });
        }

        // 6. Extra Bonuses
        let color_extra = [0.2, 1.0, 0.2, 1.0]; // Green for attributes/extra
        for bonus in &data.extra_bonuses {
            if bonus.attr_type == 0 || bonus.value == 0 {
                continue;
            }
            let name = Self::get_apply_name(bonus.attr_type);
            let sign = if bonus.value >= 0 { "+" } else { "" };
            self.lines.push(TooltipLine {
                text: format!("{}: {}{}", name, sign, bonus.value),
                color: color_extra,
            });
        }

        // 7. Sockets (Soul Stones)
        let color_socket_blue = [0.2, 0.5, 1.0, 1.0];
        let color_socket_empty = [0.5, 0.5, 0.5, 1.0];
        for socket in &data.sockets {
            match socket {
                Some(s) => {
                    self.lines.push(TooltipLine {
                        text: format!("Kamień Duszy (Vnum: {})", s.vnum),
                        color: color_socket_blue,
                    });
                }
                None => {
                    self.lines.push(TooltipLine {
                        text: "Puste gniazdo".to_string(),
                        color: color_socket_empty,
                    });
                }
            }
        }

        // 8. Duration
        if let Some(secs) = data.duration_seconds {
            let mins = secs / 60;
            let hours = mins / 60;
            let days = hours / 24;

            let duration_str = if days > 0 {
                format!("Pozostały czas: {} dni", days)
            } else if hours > 0 {
                format!("Pozostały czas: {} godz.", hours)
            } else if mins > 0 {
                format!("Pozostały czas: {} min.", mins)
            } else {
                format!("Pozostały czas: {} sek.", secs)
            };

            self.lines.push(TooltipLine {
                text: duration_str,
                color: [0.9, 0.9, 0.1, 1.0], // Yellowish for time
            });
        }

        Ok(())
    }

    fn get_apply_name(apply_type: u8) -> &'static str {
        match apply_type {
            1 => "Max PŻ",
            2 => "Max PE",
            3 => "Witalność",
            4 => "Inteligencja",
            5 => "Siła",
            6 => "Zręczność",
            7 => "Szybkość ataku",
            8 => "Szybkość ruchu",
            9 => "Szybkość zaklęcia",
            10 => "Regeneracja PŻ",
            11 => "Regeneracja PE",
            12 => "Szansa na otrucie",
            13 => "Szansa na omdlenie",
            14 => "Szansa na spowolnienie",
            15 => "Szansa na cios krytyczny",
            16 => "Szansa na przeszywające uderzenie",
            17 => "Silny przeciwko ludziom",
            18 => "Silny przeciwko zwierzętom",
            19 => "Silny przeciwko orkom",
            20 => "Silny przeciwko mistykom",
            21 => "Silny przeciwko nieumarłym",
            22 => "Silny przeciwko diabłom",
            _ => "Nieznany bonus",
        }
    }
}
