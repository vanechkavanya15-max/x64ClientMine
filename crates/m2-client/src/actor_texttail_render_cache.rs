use glam::{Mat4, Vec3, Vec4};
use std::collections::HashMap;

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

pub const COLOR_WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const COLOR_RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
pub const COLOR_BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
pub const COLOR_BROWN: Color = Color::new(0.6, 0.3, 0.0, 1.0);
pub const COLOR_GM: Color = Color::new(1.0, 0.785, 0.785, 1.0);
pub const COLOR_GUILD: Color = Color::new(1.0, 0.937, 0.827, 1.0); // Derived from 0xFFEFD3FF approx

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Empire {
    None = 0,
    EmpireA = 1,
    EmpireB = 2,
    EmpireC = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentGrade {
    Grade0 = 0, // Chivalric (>= 12000)
    Grade1 = 1, // Noble (>= 8000)
    Grade2 = 2, // Good (>= 4000)
    Grade3 = 3, // Friendly (>= 1000)
    Grade4 = 4, // Neutral (0 - 999)
    Grade5 = 5, // Aggressive (-1 to -3999)
    Grade6 = 6, // Fraudulent (-4000 to -7999)
    Grade7 = 7, // Malicious (-8000 to -11999)
    Grade8 = 8, // Cruel (<= -12000)
}

impl AlignmentGrade {
    pub fn from_alignment(alignment: i32) -> Self {
        if alignment >= 12000 {
            AlignmentGrade::Grade0
        } else if alignment >= 8000 {
            AlignmentGrade::Grade1
        } else if alignment >= 4000 {
            AlignmentGrade::Grade2
        } else if alignment >= 1000 {
            AlignmentGrade::Grade3
        } else if alignment >= 0 {
            AlignmentGrade::Grade4
        } else if alignment > -4000 {
            AlignmentGrade::Grade5
        } else if alignment > -8000 {
            AlignmentGrade::Grade6
        } else if alignment > -12000 {
            AlignmentGrade::Grade7
        } else {
            AlignmentGrade::Grade8
        }
    }

    pub fn to_title_string(&self) -> Option<String> {
        match self {
            AlignmentGrade::Grade0 => Some("Chivalric".to_string()),
            AlignmentGrade::Grade1 => Some("Noble".to_string()),
            AlignmentGrade::Grade2 => Some("Good".to_string()),
            AlignmentGrade::Grade3 => Some("Friendly".to_string()),
            AlignmentGrade::Grade4 => None,
            AlignmentGrade::Grade5 => Some("Aggressive".to_string()),
            AlignmentGrade::Grade6 => Some("Fraudulent".to_string()),
            AlignmentGrade::Grade7 => Some("Malicious".to_string()),
            AlignmentGrade::Grade8 => Some("Cruel".to_string()),
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            AlignmentGrade::Grade0
            | AlignmentGrade::Grade1
            | AlignmentGrade::Grade2
            | AlignmentGrade::Grade3 => COLOR_BLUE,
            AlignmentGrade::Grade4 => COLOR_WHITE,
            AlignmentGrade::Grade5
            | AlignmentGrade::Grade6
            | AlignmentGrade::Grade7
            | AlignmentGrade::Grade8 => COLOR_BROWN,
        }
    }
}

pub fn get_name_color(
    player_empire: Empire,
    target_empire: Empire,
    target_alignment: &AlignmentGrade,
    is_gm: bool,
    is_monster: bool,
) -> Color {
    if is_gm {
        return COLOR_GM;
    }
    
    if is_monster {
        return COLOR_RED;
    }

    // Different valid empires -> Red
    if player_empire != target_empire
        && player_empire != Empire::None
        && target_empire != Empire::None
    {
        return COLOR_RED;
    }

    target_alignment.get_color()
}

#[derive(Debug, Clone)]
pub struct TextTailData {
    pub virtual_id: u32,
    pub name: String,
    pub guild_name: Option<String>,
    pub level: Option<u32>,
    pub alignment: i32,
    pub is_gm: bool,
    pub is_monster: bool,
    pub empire: Empire,
    pub world_pos: Vec3,
    pub height_offset: f32,
    pub hp_percent: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct TextTailRenderResult {
    pub screen_x: f32,
    pub screen_y: f32,
    pub depth: f32,
    pub is_visible: bool,
    pub name_color: Color,
    pub title_color: Color,
    pub guild_color: Color,
}

#[derive(Default)]
pub struct ActorTexttailRenderCache {
    pub tails: HashMap<u32, TextTailData>,
    pub render_results: HashMap<u32, TextTailRenderResult>,
}

impl ActorTexttailRenderCache {
    pub fn new() -> Self {
        Self {
            tails: HashMap::new(),
            render_results: HashMap::new(),
        }
    }

    pub fn add_or_update(&mut self, data: TextTailData) {
        self.tails.insert(data.virtual_id, data);
    }

    pub fn remove(&mut self, virtual_id: u32) {
        self.tails.remove(&virtual_id);
        self.render_results.remove(&virtual_id);
    }

    pub fn update_hp(&mut self, virtual_id: u32, hp_percent: u8) {
        if let Some(tail) = self.tails.get_mut(&virtual_id) {
            // Hide HP bar if full, else show
            if hp_percent >= 100 {
                tail.hp_percent = None;
            } else {
                tail.hp_percent = Some(hp_percent);
            }
        }
    }

    pub fn update_position(&mut self, virtual_id: u32, world_pos: Vec3) {
        if let Some(tail) = self.tails.get_mut(&virtual_id) {
            tail.world_pos = world_pos;
        }
    }

    pub fn calculate_screen_positions(
        &mut self,
        view_proj: &Mat4,
        screen_width: f32,
        screen_height: f32,
        player_empire: Empire,
    ) {
        self.render_results.clear();

        for (vid, tail) in &self.tails {
            let alignment_grade = AlignmentGrade::from_alignment(tail.alignment);
            let name_color = get_name_color(
                player_empire,
                tail.empire,
                &alignment_grade,
                tail.is_gm,
                tail.is_monster,
            );
            let title_color = alignment_grade.get_color();
            let guild_color = COLOR_GUILD;

            // Target world pos (head position)
            let target_pos = tail.world_pos + Vec3::new(0.0, 0.0, tail.height_offset);
            let pos_v4 = Vec4::new(target_pos.x, target_pos.y, target_pos.z, 1.0);
            
            // Transform to clip space
            let clip_coords = *view_proj * pos_v4;

            // If behind camera (w <= 0.0) or outside NDC limits slightly, mark not visible
            let is_visible = clip_coords.w > 0.0;
            
            let mut screen_x = 0.0;
            let mut screen_y = 0.0;
            let mut depth = 0.0;

            if is_visible {
                let ndc_x = clip_coords.x / clip_coords.w;
                let ndc_y = clip_coords.y / clip_coords.w;
                depth = clip_coords.z / clip_coords.w;
                
                // Map to screen (assuming origin top-left, y down)
                screen_x = (ndc_x + 1.0) * 0.5 * screen_width;
                screen_y = (1.0 - ndc_y) * 0.5 * screen_height;
            }

            self.render_results.insert(
                *vid,
                TextTailRenderResult {
                    screen_x,
                    screen_y,
                    depth,
                    is_visible,
                    name_color,
                    title_color,
                    guild_color,
                },
            );
        }
    }
}
