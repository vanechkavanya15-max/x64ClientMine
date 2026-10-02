use std::cmp;

// Constants for Tooltip rendering based on PythonTooltip.cpp / PythonItem.cpp
pub const COLOR_TITLE_GOLD: [f32; 4] = [1.0, 0.78, 0.20, 1.0];
pub const COLOR_TITLE_WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const COLOR_LEVEL_RED: [f32; 4] = [1.0, 0.1, 0.1, 1.0];
pub const COLOR_LEVEL_GREY: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
pub const COLOR_DAMAGE: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
pub const COLOR_BASE_BONUS: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
pub const COLOR_ATTRIBUTE: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
pub const COLOR_SOCKET_BLUE: [f32; 4] = [0.2, 0.5, 1.0, 1.0];
pub const COLOR_SOCKET_EMPTY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
pub const COLOR_CLASS_UNMET: [f32; 4] = [1.0, 0.1, 0.1, 1.0];
pub const COLOR_CLASS_MET: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
pub const COLOR_NORMAL_TEXT: [f32; 4] = [0.7, 0.7, 0.7, 1.0];

#[derive(Debug, Clone, PartialEq)]
pub enum TooltipElement {
    Text { text: String, color: [f32; 4] },
    Image { image_path: String, width: i32, height: i32 },
}

#[derive(Debug, Clone, Default)]
pub struct ItemTooltipRenderer {
    pub elements: Vec<TooltipElement>,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

impl ItemTooltipRenderer {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            width: 0,
            height: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.width = 0;
        self.height = 0;
    }

    pub fn append_line(&mut self, text: String, color: [f32; 4]) {
        self.elements.push(TooltipElement::Text { text, color });
    }

    pub fn append_image(&mut self, image_path: String, width: i32, height: i32) {
        self.elements.push(TooltipElement::Image { image_path, width, height });
    }

    pub fn add_title(&mut self, item_name: &str) {
        let color = if item_name.contains("+9") {
            COLOR_TITLE_GOLD
        } else {
            COLOR_TITLE_WHITE
        };
        self.append_line(item_name.to_string(), color);
    }

    pub fn add_item_type(&mut self, item_type: u8, item_sub_type: u8) {
        let type_name = match item_type {
            1 => match item_sub_type {
                0 => "Miecz",
                1 => "Sztylet",
                2 => "Łuk",
                3 => "Broń dwuręczna",
                4 => "Dzwon",
                5 => "Wachlarz",
                _ => "Broń",
            },
            2 => "Zbroja",
            _ => "Przedmiot",
        };
        self.append_line(type_name.to_string(), COLOR_NORMAL_TEXT);
    }

    pub fn add_requirements(
        &mut self,
        limit_level: u32,
        anti_flags: u32,
        player_level: u32,
        player_job: u8,
    ) {
        if limit_level > 0 {
            let color = if player_level >= limit_level {
                COLOR_CLASS_MET
            } else {
                COLOR_CLASS_UNMET
            };
            self.append_line(format!("Od poziomu: {}", limit_level), color);
        }

        let mut class_names = Vec::new();
        // anti_flags bitmask:
        // ITEM_ANTIFLAG_WARRIOR = 1 << 2
        // ITEM_ANTIFLAG_ASSASSIN = 1 << 3
        // ITEM_ANTIFLAG_SURA = 1 << 4
        // ITEM_ANTIFLAG_SHAMAN = 1 << 5

        let is_warrior_banned = (anti_flags & (1 << 2)) != 0;
        let is_assassin_banned = (anti_flags & (1 << 3)) != 0;
        let is_sura_banned = (anti_flags & (1 << 4)) != 0;
        let is_shaman_banned = (anti_flags & (1 << 5)) != 0;

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
                COLOR_CLASS_MET
            } else {
                COLOR_CLASS_UNMET
            };
            self.append_line(format!("Klasy: {}", classes_str), color);
        }
    }

    pub fn add_battle_stats(
        &mut self,
        attack_min: i32,
        attack_max: i32,
        magic_attack_min: i32,
        magic_attack_max: i32,
        defense: i32,
        attack_speed: i32,
    ) {
        if attack_min > 0 || attack_max > 0 {
            self.append_line(
                format!("Wartość ataku {}-{}", attack_min, attack_max),
                COLOR_DAMAGE,
            );
        }
        if magic_attack_min > 0 || magic_attack_max > 0 {
            self.append_line(
                format!("Wartość magicznego ataku {}-{}", magic_attack_min, magic_attack_max),
                COLOR_DAMAGE,
            );
        }
        if attack_speed > 0 {
            self.append_line(format!("Szybkość ataku {}", attack_speed), COLOR_DAMAGE);
        }
        if defense > 0 {
            self.append_line(format!("Obrona {}", defense), COLOR_DAMAGE);
        }
    }

    pub fn add_applies(&mut self, applies: &[(u8, i32)]) {
        for &(apply_type, value) in applies {
            if apply_type == 0 || value == 0 {
                continue;
            }
            let name = Self::get_apply_name(apply_type);
            let sign = if value >= 0 { "+" } else { "" };
            self.append_line(format!("{}: {}{}", name, sign, value), COLOR_BASE_BONUS);
        }
    }

    pub fn add_attributes(&mut self, attributes: &[(u8, i16)]) {
        for &(attr_type, value) in attributes {
            if attr_type == 0 || value == 0 {
                continue;
            }
            let name = Self::get_apply_name(attr_type);
            let sign = if value >= 0 { "+" } else { "" };
            self.append_line(format!("{}: {}{}", name, sign, value), COLOR_ATTRIBUTE);
        }
    }

    pub fn add_sockets(&mut self, sockets: &[Option<u32>]) {
        for socket in sockets {
            match socket {
                Some(vnum) => {
                    let image_path = format!("d:/ymir work/ui/items/{}.tga", vnum);
                    self.append_image(image_path, 32, 32);
                    self.append_line(format!("Kamień Duszy (Vnum: {})", vnum), COLOR_SOCKET_BLUE);
                }
                None => {
                    self.append_line("Puste gniazdo".to_string(), COLOR_SOCKET_EMPTY);
                }
            }
        }
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

    pub fn finalize_dimensions(&mut self) {
        // Approximate character width and line height
        const CHAR_WIDTH: i32 = 6;
        const LINE_HEIGHT: i32 = 15;
        const PADDING_X: i32 = 20; // 10px on each side
        const PADDING_Y: i32 = 20; // 10px top, 10px bottom

        let mut max_width = 0;
        let mut total_height = 0;

        for element in &self.elements {
            match element {
                TooltipElement::Text { text, .. } => {
                    let line_width = text.chars().count() as i32 * CHAR_WIDTH;
                    if line_width > max_width {
                        max_width = line_width;
                    }
                    total_height += LINE_HEIGHT;
                }
                TooltipElement::Image { width, height, .. } => {
                    if *width > max_width {
                        max_width = *width;
                    }
                    total_height += height;
                }
            }
        }

        self.width = max_width + PADDING_X;
        self.height = total_height + PADDING_Y;
    }

    pub fn update_position(&mut self, mouse_x: i32, mouse_y: i32, screen_width: i32, screen_height: i32) {
        let offset_x = 15;
        let offset_y = 15;

        self.x = mouse_x + offset_x;
        self.y = mouse_y + offset_y;

        if self.x + self.width > screen_width {
            self.x = screen_width - self.width;
        }
        if self.y + self.height > screen_height {
            self.y = screen_height - self.height;
        }
        
        self.x = cmp::max(0, self.x);
        self.y = cmp::max(0, self.y);
    }
}
