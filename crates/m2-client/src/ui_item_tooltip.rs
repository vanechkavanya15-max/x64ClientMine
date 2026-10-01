use std::cmp;

pub const COLOR_TITLE_GOLD: [f32; 4] = [1.0, 0.78, 0.20, 1.0];
pub const COLOR_TITLE_WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const COLOR_LEVEL_RED: [f32; 4] = [1.0, 0.1, 0.1, 1.0];
pub const COLOR_LEVEL_GREY: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
pub const COLOR_DAMAGE: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
pub const COLOR_BASE_BONUS: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
pub const COLOR_ATTRIBUTE: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
pub const COLOR_SOCKET_BLUE: [f32; 4] = [0.2, 0.5, 1.0, 1.0];
pub const COLOR_SOCKET_EMPTY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

#[derive(Debug, Clone, PartialEq)]
pub struct TooltipLine {
    pub text: String,
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Default)]
pub struct ItemTooltipData {
    pub name: String,
    pub item_type: u8, // e.g. 1 for weapon, 2 for armor, but using constants or raw u8
    pub level_limit: u8,
    pub damage_min: i32,
    pub damage_max: i32,
    pub defense: i32,
    pub base_bonuses: Vec<(String, i32)>,
    pub sockets: [Option<u32>; 3],
    pub attrs: Vec<(String, i32)>,
}

#[derive(Debug, Clone)]
pub struct ItemTooltip {
    pub lines: Vec<TooltipLine>,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

impl ItemTooltip {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            width: 0,
            height: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn build_tooltip(&mut self, data: &ItemTooltipData, player_level: u8) {
        self.lines.clear();

        // 1. Name
        let name_color = if data.name.contains("+") {
            COLOR_TITLE_GOLD
        } else {
            COLOR_TITLE_WHITE
        };
        self.lines.push(TooltipLine {
            text: data.name.clone(),
            color: name_color,
        });

        // 2. Level Limit
        if data.level_limit > 0 {
            let level_color = if player_level < data.level_limit {
                COLOR_LEVEL_RED
            } else {
                COLOR_LEVEL_GREY
            };
            self.lines.push(TooltipLine {
                text: format!("Od poziomu: {}", data.level_limit),
                color: level_color,
            });
        }

        // 3. Damage / Defense
        if data.damage_max > 0 || data.damage_min > 0 {
            self.lines.push(TooltipLine {
                text: format!("Wartość ataku {}-{}", data.damage_min, data.damage_max),
                color: COLOR_DAMAGE,
            });
        }
        if data.defense > 0 {
            self.lines.push(TooltipLine {
                text: format!("Obrona {}", data.defense),
                color: COLOR_DAMAGE,
            });
        }

        // 4. Base Bonuses
        for (name, val) in &data.base_bonuses {
            let sign = if *val >= 0 { "+" } else { "" };
            self.lines.push(TooltipLine {
                text: format!("{}: {}{}", name, sign, val),
                color: COLOR_BASE_BONUS,
            });
        }

        // 5. Attributes (Green)
        for (name, val) in &data.attrs {
            let sign = if *val >= 0 { "+" } else { "" };
            self.lines.push(TooltipLine {
                text: format!("{}: {}{}", name, sign, val),
                color: COLOR_ATTRIBUTE,
            });
        }

        // 6. Sockets
        for socket in &data.sockets {
            match socket {
                Some(vnum) => {
                    self.lines.push(TooltipLine {
                        text: format!("Kamień Duszy (Vnum: {})", vnum), // Simplified representation
                        color: COLOR_SOCKET_BLUE,
                    });
                }
                None => {
                    self.lines.push(TooltipLine {
                        text: "Puste gniazdo".to_string(),
                        color: COLOR_SOCKET_EMPTY,
                    });
                }
            }
        }

        // Calculate size based on lines (approximate)
        let line_height = 15;
        self.height = self.lines.len() as i32 * line_height + 10;
        self.width = 200; // Fixed for now
    }

    pub fn update_position(&mut self, mouse_x: i32, mouse_y: i32, screen_width: i32, screen_height: i32) {
        // Offset from mouse
        let offset_x = 15;
        let offset_y = 15;

        self.x = mouse_x + offset_x;
        self.y = mouse_y + offset_y;

        // Clamp to screen
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
