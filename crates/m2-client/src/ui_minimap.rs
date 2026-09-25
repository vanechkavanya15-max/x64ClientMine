//! Modul `ui_minimap` — Radar i minimapa swiata z punktami postaci CPythonMiniMap
//! Wzorzec referencyjny C++: `source/UserInterface/PythonMiniMap.cpp`
//! Opis: Odwzorowanie CPythonMiniMap: okragly radar w prawym gornym rogu, punkty orientacyjne (zielone NPC, czerwone potwory, zolte gracze, niebieskie grupa), wspolrzedne (X, Y).

pub const EMPIRE_NUM: u8 = 4;
pub const MINI_WAYPOINT_IMAGE_COUNT: usize = 12;
pub const WAYPOINT_IMAGE_COUNT: usize = 15;
pub const TARGET_MARK_IMAGE_COUNT: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkType {
    Opc,
    OpcPvp,
    OpcPvpSelf,
    Npc,
    Monster,
    Warp,
    WayPoint,
    Party,
    Empire(u8),
    Target,
}

#[derive(Debug, Clone)]
pub struct AtlasMarkInfo {
    pub mark_type: MarkType,
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub minimap_x: f32,
    pub minimap_y: f32,
    pub chr_vid: u32,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct GuildAreaInfo {
    pub guild_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub sx_render: f32,
    pub sy_render: f32,
    pub ex_render: f32,
    pub ey_render: f32,
}

#[derive(Debug, Clone)]
pub struct Observer {
    pub cur_x: f32,
    pub cur_y: f32,
    pub src_x: f32,
    pub src_y: f32,
    pub dst_x: f32,
    pub dst_y: f32,
    pub src_time: u32,
    pub dst_time: u32,
}

#[derive(Debug, Clone)]
pub struct MarkPosition {
    pub x: f32,
    pub y: f32,
    pub name_color: u32,
}

#[derive(Debug, Clone)]
pub struct SignalPoint {
    pub pos: [f32; 2],
    pub id: u32,
}

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct UiMinimap {
    pub initialized: bool,
    
    pub width: f32,
    pub height: f32,
    
    pub scale: f32,
    
    pub center_x: f32,
    pub center_y: f32,
    
    pub center_cell_x: f32,
    pub center_cell_y: f32,
    
    pub screen_x: f32,
    pub screen_y: f32,
    
    pub minimap_radius: f32,
    
    pub party_pc_position_vector: Vec<MarkPosition>,
    pub other_pc_position_vector: Vec<MarkPosition>,
    pub npc_position_vector: Vec<MarkPosition>,
    pub monster_position_vector: Vec<MarkPosition>,
    pub warp_position_vector: Vec<MarkPosition>,
    
    pub map_vid_observer: HashMap<u32, Observer>,
    
    pub is_atlas: bool,
    pub is_show: bool,
    pub is_show_atlas: bool,
    
    pub atlas_screen_x: f32,
    pub atlas_screen_y: f32,
    pub atlas_base_x: u32,
    pub atlas_base_y: u32,
    pub atlas_max_x: f32,
    pub atlas_max_y: f32,
    pub atlas_image_size_x: f32,
    pub atlas_image_size_y: f32,
    
    pub atlas_npc_info_vector: Vec<AtlasMarkInfo>,
    pub atlas_warp_info_vector: Vec<AtlasMarkInfo>,
    pub atlas_waypoint_info_vector: Vec<AtlasMarkInfo>,
    pub guild_area_info_vector: Vec<GuildAreaInfo>,
    
    pub signal_point_vector: Vec<SignalPoint>,
}

impl UiMinimap {
    pub fn new() -> Self {
        let mut instance = Self::default();
        instance.initialized = true;
        instance.scale = 1.0;
        instance
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn destroy(&mut self) {
        self.initialized = false;
        *self = Self::default();
    }

    pub fn create(&mut self) -> bool {
        self.initialized = true;
        self.scale = 1.0;
        self.is_atlas = false;
        self.is_show = false;
        self.is_show_atlas = false;
        true
    }

    pub fn is_atlas(&self) -> bool {
        self.is_atlas
    }

    pub fn can_show(&self) -> bool {
        self.is_show
    }

    pub fn can_show_atlas(&self) -> bool {
        self.is_show_atlas
    }

    pub fn set_minimap_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.update_minimap_radius();
    }

    pub fn set_scale(&mut self, mut scale: f32) {
        if scale >= 4.0 {
            scale = 4.0;
        }
        if scale <= 0.5 {
            scale = 0.5;
        }
        self.scale = scale;
        self.update_minimap_radius();
    }

    pub fn scale_up(&mut self) {
        self.scale *= 2.0;
        if self.scale >= 4.0 {
            self.scale = 4.0;
        }
        self.update_minimap_radius();
    }

    pub fn scale_down(&mut self) {
        self.scale *= 0.5;
        if self.scale <= 0.5 {
            self.scale = 0.5;
        }
        self.update_minimap_radius();
    }

    pub fn set_center_position(&mut self, center_x: f32, center_y: f32) {
        self.center_x = center_x;
        self.center_y = center_y;
    }

    fn update_minimap_radius(&mut self) {
        // Oblicza promien minimapy z ograniczeniem (CTerrainImpl::CELLSCALE w Metin2 wynosi 100.0f)
        let calculated_radius = (6400.0 / 100.0) * self.scale;
        self.minimap_radius = if calculated_radius < 64.0 { calculated_radius } else { 64.0 };
    }

    pub fn add_entity_mark(&mut self, entity_x: f32, entity_y: f32, mark_type: MarkType, name_color: u32) {
        let cell_scale = 1.0 / 100.0;
        let dist_x = (entity_x - self.center_x) * cell_scale * self.scale;
        let dist_y = (entity_y - self.center_y) * cell_scale * self.scale;

        if dist_x.abs() >= self.minimap_radius || dist_y.abs() >= self.minimap_radius {
            return;
        }

        let dist_center = (dist_x * dist_x + dist_y * dist_y).sqrt();
        if dist_center >= self.minimap_radius {
            return;
        }

        let mark_position = MarkPosition {
            x: (self.width - 16.0) / 2.0 + dist_x + self.screen_x,
            y: (self.height - 16.0) / 2.0 + dist_y + self.screen_y,
            name_color,
        };

        match mark_type {
            MarkType::Opc | MarkType::OpcPvp | MarkType::OpcPvpSelf | MarkType::Empire(_) => {
                self.other_pc_position_vector.push(mark_position);
            },
            MarkType::Party => {
                self.party_pc_position_vector.push(mark_position);
            },
            MarkType::Npc => {
                self.npc_position_vector.push(mark_position);
            },
            MarkType::Monster => {
                self.monster_position_vector.push(mark_position);
            },
            MarkType::Warp => {
                self.warp_position_vector.push(mark_position);
            },
            _ => {}
        }
    }

    pub fn update(&mut self, center_x: f32, center_y: f32, cur_time: u32) {
        if self.center_x != center_x || self.center_y != center_y {
            self.set_center_position(center_x, center_y);
        }
        self.other_pc_position_vector.clear();
        self.party_pc_position_vector.clear();
        self.npc_position_vector.clear();
        self.monster_position_vector.clear();
        self.warp_position_vector.clear();

        // Interpolacja obserwatorow w czasie
        let mut to_add = Vec::new();
        for observer in self.map_vid_observer.values_mut() {
            let src_time = observer.src_time;
            let dst_time = observer.dst_time;
            
            let mut pos = if dst_time > src_time {
                (cur_time.saturating_sub(src_time)) as f32 / (dst_time - src_time) as f32
            } else {
                1.0
            };

            if pos < 0.0 { pos = 0.0; }
            else if pos > 1.0 { pos = 1.0; }

            observer.cur_x = (observer.dst_x - observer.src_x) * pos + observer.src_x;
            observer.cur_y = (observer.dst_y - observer.src_y) * pos + observer.src_y;

            let cell_scale = 1.0 / 100.0; // Skala siatki terenu
            let dist_x = (observer.cur_x - self.center_x) * cell_scale * self.scale;
            let dist_y = (observer.cur_y - self.center_y) * cell_scale * self.scale;

            if dist_x.abs() >= self.minimap_radius || dist_y.abs() >= self.minimap_radius {
                continue;
            }

            let dist_center = (dist_x * dist_x + dist_y * dist_y).sqrt();
            if dist_center >= self.minimap_radius {
                continue;
            }

            to_add.push(MarkPosition {
                x: (self.width - 16.0) / 2.0 + dist_x + self.screen_x,
                y: (self.height - 16.0) / 2.0 + dist_y + self.screen_y,
                name_color: 0, // Zastepczy kolor grupy
            });
        }
        self.party_pc_position_vector.extend(to_add);
    }

    pub fn render(&mut self, screen_x: f32, screen_y: f32) {
        if !self.is_show {
            return;
        }
        if self.screen_x != screen_x || self.screen_y != screen_y {
            self.screen_x = screen_x;
            self.screen_y = screen_y;
        }
    }

    pub fn show(&mut self) {
        self.is_show = true;
    }

    pub fn hide(&mut self) {
        self.is_show = false;
    }

    pub fn load_atlas(&mut self) -> bool {
        self.is_atlas = true;
        true
    }

    pub fn update_atlas(&mut self, player_x: f32, player_y: f32, player_rotation: f32) {
        if self.atlas_max_x <= 0.0 || self.atlas_max_y <= 0.0 {
            return;
        }

        // Mocking CInstanceBase player rotation updates to update_atlas parameter
        let mut rotation = 540.0 - player_rotation;
        while rotation > 360.0 { rotation -= 360.0; }
        while rotation < 0.0 { rotation += 360.0; }

        let _atlas_player_x = player_x / self.atlas_max_x * self.atlas_image_size_x;
        let _atlas_player_y = player_y / self.atlas_max_y * self.atlas_image_size_y;
        
        let atlas_max_x = self.atlas_max_x;
        let atlas_max_y = self.atlas_max_y;
        let atlas_image_size_x = self.atlas_image_size_x;
        let atlas_image_size_y = self.atlas_image_size_y;

        for info in &mut self.guild_area_info_vector {
            let global_to_atlas = |x: f32, y: f32| -> (f32, f32) {
                if atlas_max_x <= 0.0 || atlas_max_y <= 0.0 {
                    (0.0, 0.0)
                } else {
                    (
                        x / atlas_max_x * atlas_image_size_x,
                        y / atlas_max_y * atlas_image_size_y
                    )
                }
            };
            
            let (sx, sy) = global_to_atlas(info.x as f32, info.y as f32);
            info.sx_render = sx;
            info.sy_render = sy;
            
            let (ex, ey) = global_to_atlas((info.x + info.width) as f32, (info.y + info.height) as f32);
            info.ex_render = ex;
            info.ey_render = ey;
        }
    }

    pub fn global_position_to_atlas_position(&self, x: f32, y: f32) -> (f32, f32) {
        if self.atlas_max_x <= 0.0 || self.atlas_max_y <= 0.0 {
            return (0.0, 0.0);
        }
        (
            x / self.atlas_max_x * self.atlas_image_size_x,
            y / self.atlas_max_y * self.atlas_image_size_y
        )
    }

    pub fn render_atlas(&mut self, screen_x: f32, screen_y: f32) {
        if !self.is_show_atlas {
            return;
        }
        if self.atlas_screen_x != screen_x || self.atlas_screen_y != screen_y {
            self.atlas_screen_x = screen_x;
            self.atlas_screen_y = screen_y;
        }
    }

    pub fn show_atlas(&mut self) {
        self.is_show_atlas = true;
    }

    pub fn hide_atlas(&mut self) {
        self.is_show_atlas = false;
    }

    pub fn add_observer(&mut self, vid: u32, src_x: f32, src_y: f32) {
        let entry = self.map_vid_observer.entry(vid).or_insert(Observer {
            cur_x: src_x,
            cur_y: src_y,
            src_x,
            src_y,
            dst_x: src_x,
            dst_y: src_y,
            src_time: 0,
            dst_time: 1000,
        });
        
        entry.src_time = 0;
        entry.dst_time = 1000;
        entry.src_x = src_x;
        entry.src_y = src_y;
        entry.dst_x = src_x;
        entry.dst_y = src_y;
        entry.cur_x = src_x;
        entry.cur_y = src_y;
    }

    pub fn move_observer(&mut self, vid: u32, dst_x: f32, dst_y: f32) {
        if let Some(observer) = self.map_vid_observer.get_mut(&vid) {
            observer.src_x = observer.cur_x;
            observer.src_y = observer.cur_y;
            observer.dst_x = dst_x;
            observer.dst_y = dst_y;
            observer.src_time = 0;
            observer.dst_time = 1000;
        }
    }

    pub fn remove_observer(&mut self, vid: u32) {
        self.map_vid_observer.remove(&vid);
    }

    pub fn add_way_point(&mut self, mark_type: MarkType, id: u32, x: f32, y: f32, text: String, chr_vid: u32) {
        let info = AtlasMarkInfo {
            mark_type,
            id,
            x,
            y,
            screen_x: 0.0,
            screen_y: 0.0,
            minimap_x: 0.0,
            minimap_y: 0.0,
            chr_vid,
            text,
        };
        self.atlas_waypoint_info_vector.push(info);
    }

    pub fn remove_way_point(&mut self, id: u32) {
        self.atlas_waypoint_info_vector.retain(|info| info.id != id);
    }

    pub fn add_signal_point(&mut self, x: f32, y: f32) {
        let id = self.signal_point_vector.len() as u32 + 255;
        self.signal_point_vector.push(SignalPoint { pos: [x, y], id });
        self.add_way_point(MarkType::WayPoint, id, x, y, String::new(), 0);
    }

    pub fn clear_all_signal_point(&mut self) {
        let ids_to_remove: Vec<u32> = self.signal_point_vector.iter().map(|sp| sp.id).collect();
        for id in ids_to_remove {
            self.remove_way_point(id);
        }
        self.signal_point_vector.clear();
    }

    pub fn clear_guild_area(&mut self) {
        self.guild_area_info_vector.clear();
    }

    pub fn register_guild_area(&mut self, _id: u32, guild_id: u32, x: i32, y: i32, width: i32, height: i32) {
        self.guild_area_info_vector.push(GuildAreaInfo {
            guild_id,
            x,
            y,
            width,
            height,
            sx_render: 0.0,
            sy_render: 0.0,
            ex_render: 0.0,
            ey_render: 0.0,
        });
    }

    pub fn get_guild_area_id(&self, x: u32, y: u32) -> u32 {
        let x = x as i32;
        let y = y as i32;
        for info in &self.guild_area_info_vector {
            if x >= info.x && y >= info.y && x <= info.x + info.width && y <= info.y + info.height {
                return info.guild_id;
            }
        }
        0
    }

    pub fn clear_atlas_mark_info(&mut self) {
        self.atlas_npc_info_vector.clear();
        self.atlas_warp_info_vector.clear();
    }

    pub fn register_atlas_mark(&mut self, mark_type: MarkType, text: String, x: i32, y: i32) {
        let info = AtlasMarkInfo {
            mark_type,
            id: 0,
            x: x as f32,
            y: y as f32,
            screen_x: 0.0,
            screen_y: 0.0,
            minimap_x: 0.0,
            minimap_y: 0.0,
            chr_vid: 0,
            text,
        };
        match mark_type {
            MarkType::Npc => self.atlas_npc_info_vector.push(info),
            MarkType::Warp => self.atlas_warp_info_vector.push(info),
            _ => {}
        }
    }

    pub fn create_target(&mut self, id: u32, text: String) {
        self.add_way_point(MarkType::Target, id, 0.0, 0.0, text, 0);
    }

    pub fn create_target_with_vid(&mut self, id: u32, text: String, vid: u32) {
        self.add_way_point(MarkType::Target, id, 0.0, 0.0, text, vid);
    }

    pub fn update_target(&mut self, id: u32, x: i32, y: i32) {
        if let Some(info) = self.atlas_waypoint_info_vector.iter_mut().find(|info| info.id == id) {
            info.x = x as f32;
            info.y = y as f32;
        }
    }

    pub fn delete_target(&mut self, id: u32) {
        self.remove_way_point(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_minimap() {
        let instance = UiMinimap::new();
        assert!(instance.is_ready());
    }
}
