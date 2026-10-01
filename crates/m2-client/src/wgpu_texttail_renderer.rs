use glam::{Mat4, Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentGrade {
    Chivalric,  // 12000 ~ 20000
    Noble,      // 8000 ~ 11999
    Good,       // 4000 ~ 7999
    Friendly,   // 1000 ~ 3999
    Neutral,    // 0 ~ 999
    Aggressive, // -1 ~ -3999
    Fraudulent, // -4000 ~ -7999
    Malicious,  // -8000 ~ -11999
    Cruel,      // -12000 ~ -20000
}

impl AlignmentGrade {
    pub fn from_alignment(alignment: i32) -> Self {
        if alignment >= 12000 {
            Self::Chivalric
        } else if alignment >= 8000 {
            Self::Noble
        } else if alignment >= 4000 {
            Self::Good
        } else if alignment >= 1000 {
            Self::Friendly
        } else if alignment >= 0 {
            Self::Neutral
        } else if alignment > -4000 {
            Self::Aggressive
        } else if alignment > -8000 {
            Self::Fraudulent
        } else if alignment > -12000 {
            Self::Malicious
        } else {
            Self::Cruel
        }
    }

    pub fn to_title_string(&self) -> Option<&'static str> {
        match self {
            Self::Chivalric => Some("Rycerski"),
            Self::Noble => Some("Szlachetny"),
            Self::Good => Some("Dobry"),
            Self::Friendly => Some("Przyjazny"),
            Self::Neutral => None,
            Self::Aggressive => Some("Agresywny"),
            Self::Fraudulent => Some("Nieuczciwy"),
            Self::Malicious => Some("Zlosliwy"),
            Self::Cruel => Some("Okrutny"),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Chivalric | Self::Noble | Self::Good | Self::Friendly => {
                Color::new(0.0, 0.0, 1.0, 1.0) // Niebieski
            }
            Self::Neutral => {
                Color::new(1.0, 1.0, 0.0, 1.0) // Zolty
            }
            Self::Aggressive | Self::Fraudulent | Self::Malicious | Self::Cruel => {
                Color::new(1.0, 0.0, 0.0, 1.0) // Czerwony
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextTail {
    pub virtual_id: u32,
    pub name: String,
    pub level: Option<u32>,
    pub guild_name: Option<String>,
    pub alignment: i32,

    pub position: Vec3,
    pub height: f32,
    pub distance_from_player: f32,

    pub hp_percent: Option<u8>, // 0-100, Some if shown (e.g. mob/enemy)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageFlag {
    Normal,
    Critical,
    Poison,
    Miss,
    Dodge,
    Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DamageNumber {
    pub virtual_id: u32,
    pub damage: u32,
    pub flag: DamageFlag,

    pub position: Vec3,
    pub start_time: f32,  // Time when damage appeared
    pub living_time: f32, // How long it has been alive
}

pub struct TextTailRenderer {
    pub text_tails: std::collections::HashMap<u32, TextTail>,
    pub damage_numbers: Vec<DamageNumber>,
}

impl TextTailRenderer {
    pub fn new() -> Self {
        Self {
            text_tails: std::collections::HashMap::new(),
            damage_numbers: Vec::new(),
        }
    }

    /// Projects 3D world position to 2D screen position.
    /// Returns (screen_x, screen_y, screen_z) if the point is in front of the camera, None otherwise.
    /// screen_z is used for depth sorting (closer objects have smaller z).
    pub fn transform_to_screen(
        world_pos: Vec3,
        height_offset: f32,
        view_proj: &Mat4,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Option<(f32, f32, f32)> {
        let pos_3d = Vec4::new(world_pos.x, world_pos.y, world_pos.z + height_offset, 1.0);
        let clip_coords = *view_proj * pos_3d;

        if clip_coords.w <= 0.0 {
            return None; // Behind camera
        }

        let ndc_x = clip_coords.x / clip_coords.w;
        let ndc_y = clip_coords.y / clip_coords.w;
        let ndc_z = clip_coords.z / clip_coords.w;

        // Map NDC (-1 to +1) to screen space (0 to width/height)
        // Y is inverted because NDC Y goes up, screen Y goes down.
        let screen_x = (ndc_x + 1.0) * 0.5 * viewport_width;
        let screen_y = (1.0 - ndc_y) * 0.5 * viewport_height;

        Some((screen_x.floor(), screen_y.floor(), ndc_z))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderPayload {
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        z: f32,
    },
    Text {
        x: f32,
        y: f32,
        text: String,
        color: Color,
        z: f32,
        is_centered: bool,
    },
    Texture {
        x: f32,
        y: f32,
        texture_path: String, // Or ID
        z: f32,
    },
}

impl TextTailRenderer {
    pub fn register_text_tail(&mut self, text_tail: TextTail) {
        self.text_tails.insert(text_tail.virtual_id, text_tail);
    }

    pub fn delete_text_tail(&mut self, virtual_id: u32) {
        self.text_tails.remove(&virtual_id);
    }

    pub fn add_damage_number(&mut self, damage_number: DamageNumber) {
        self.damage_numbers.push(damage_number);
    }

    pub fn update_damage_numbers(&mut self, delta_time: f32) {
        for dmg in &mut self.damage_numbers {
            dmg.living_time += delta_time;
            // Float upwards
            dmg.position.z += 50.0 * delta_time;
        }
        // Remove old damage numbers (e.g. older than 1.5 seconds)
        self.damage_numbers.retain(|dmg| dmg.living_time < 1.5);
    }

    pub fn generate_render_payloads(
        &self,
        view_proj: &Mat4,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Result<Vec<RenderPayload>, String> {
        let mut payloads = Vec::new();

        // 1. Text Tails
        for text_tail in self.text_tails.values() {
            // Distance cull (e.g., 3500.0 from C++ ShowAllTextTail)
            if text_tail.distance_from_player > 3500.0 {
                continue;
            }

            if let Some((screen_x, screen_y, screen_z)) = Self::transform_to_screen(
                text_tail.position,
                text_tail.height,
                view_proj,
                viewport_width,
                viewport_height,
            ) {
                let mut current_y = screen_y;

                // Guild Name
                if let Some(guild_name) = &text_tail.guild_name {
                    payloads.push(RenderPayload::Text {
                        x: screen_x,
                        y: current_y,
                        text: guild_name.clone(),
                        color: Color::new(1.0, 1.0, 1.0, 1.0),
                        z: screen_z,
                        is_centered: true,
                    });
                    current_y += 15.0; // move down for next element
                }

                // Title + Level + Name
                let grade = AlignmentGrade::from_alignment(text_tail.alignment);
                let title = grade.to_title_string();
                let title_color = grade.color();

                let mut full_name = String::new();
                if let Some(t) = title {
                    full_name.push_str(t);
                    full_name.push(' ');
                }
                if let Some(lvl) = text_tail.level {
                    full_name.push_str(&format!("[Lv. {}] ", lvl));
                }
                full_name.push_str(&text_tail.name);

                payloads.push(RenderPayload::Text {
                    x: screen_x,
                    y: current_y,
                    text: full_name,
                    color: title_color, // Simplified: coloring whole line based on alignment
                    z: screen_z,
                    is_centered: true,
                });
                current_y += 15.0;

                // HP Bar
                if let Some(hp_pct) = text_tail.hp_percent {
                    let bar_width = 50.0;
                    let bar_height = 5.0;
                    let bar_x = screen_x - bar_width / 2.0;

                    // Background
                    payloads.push(RenderPayload::Rect {
                        x: bar_x,
                        y: current_y,
                        width: bar_width,
                        height: bar_height,
                        color: Color::new(0.0, 0.0, 0.0, 1.0),
                        z: screen_z,
                    });

                    // Fill
                    let fill_width = (hp_pct as f32 / 100.0) * bar_width;
                    payloads.push(RenderPayload::Rect {
                        x: bar_x,
                        y: current_y,
                        width: fill_width,
                        height: bar_height,
                        color: Color::new(1.0, 0.0, 0.0, 1.0), // Red fill
                        z: screen_z,
                    });
                }
            }
        }

        // 2. Damage Numbers
        for dmg in &self.damage_numbers {
            if let Some((screen_x, screen_y, screen_z)) = Self::transform_to_screen(
                dmg.position,
                0.0, // Damage numbers already float up via position.z
                view_proj,
                viewport_width,
                viewport_height,
            ) {
                // Formatting damage
                let prefix = match dmg.flag {
                    DamageFlag::Poison => "poison_",
                    DamageFlag::Miss | DamageFlag::Dodge | DamageFlag::Block => "miss_",
                    DamageFlag::Critical => "crit_",
                    _ => "damage_",
                };

                let text = match dmg.flag {
                    DamageFlag::Miss | DamageFlag::Dodge | DamageFlag::Block => "MISS".to_string(),
                    _ => format!("-{}", dmg.damage),
                };

                // Add text based payload (or ideally texture path strings for digits in C++)
                payloads.push(RenderPayload::Text {
                    x: screen_x,
                    y: screen_y,
                    text,
                    color: match dmg.flag {
                        DamageFlag::Poison => Color::new(0.0, 1.0, 0.0, 1.0),
                        DamageFlag::Critical => Color::new(1.0, 0.5, 0.0, 1.0),
                        _ => Color::new(1.0, 0.0, 0.0, 1.0),
                    },
                    z: screen_z,
                    is_centered: true,
                });
            }
        }

        Ok(payloads)
    }
}