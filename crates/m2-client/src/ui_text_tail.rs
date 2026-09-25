//! Modul `ui_text_tail` — System napisow 3D nad postaciami i przedmiotami CPythonTextTail
//! Wzorzec referencyjny C++: `source/UserInterface/PythonTextTail.cpp`
//! Opis: Odwzorowanie CPythonTextTail: rzutowanie pozycji 3D glowy postaci na ekran 2D, wyswietlanie poziomu (Lv.), rangi, nazwy gildii, nazwy gracza/potwora oraz dymkow czatu.

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct GraphicTextInstance {
    pub text: String,
    pub color: Color,
    pub outline: bool,
    pub horizontal_align: i32,
    pub vertical_align: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: i32,
    pub height: i32,
}

impl GraphicTextInstance {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_value(&mut self, text: &str) {
        self.text = text.to_string();
    }
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color.r = r;
        self.color.g = g;
        self.color.b = b;
    }
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn get_text_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphicMarkInstance {
    pub image_file_name: String,
    pub index: u32,
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
}

impl GraphicMarkInstance {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn get_width(&self) -> i32 {
        self.width
    }
}

#[derive(Debug, Clone)]
pub struct TextTail {
    pub virtual_id: u32,

    pub text_instance: Option<GraphicTextInstance>,
    pub owner_text_instance: Option<GraphicTextInstance>,

    pub mark_instance: Option<GraphicMarkInstance>,
    pub guild_name_text_instance: Option<GraphicTextInstance>,

    pub title_text_instance: Option<GraphicTextInstance>,
    pub level_text_instance: Option<GraphicTextInstance>,

    pub owner_is_show: bool,
    pub owner_position: Vector3,

    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub distance_from_player: f32,
    pub color: Color,
    pub name_flag: bool,

    pub x_start: f32,
    pub y_start: f32,
    pub x_end: f32,
    pub y_end: f32,

    pub living_time: u32,
    pub height: f32,
}

impl TextTail {
    pub fn new(virtual_id: u32) -> Self {
        Self {
            virtual_id,
            text_instance: None,
            owner_text_instance: None,
            mark_instance: None,
            guild_name_text_instance: None,
            title_text_instance: None,
            level_text_instance: None,
            owner_is_show: true,
            owner_position: Vector3::default(),
            x: -100.0,
            y: -100.0,
            z: 0.0,
            distance_from_player: 0.0,
            color: Color::default(),
            name_flag: false,
            x_start: 0.0,
            y_start: -2.0,
            x_end: 0.0,
            y_end: 0.0,
            living_time: 0,
            height: 0.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UiTextTail {
    pub initialized: bool,
    pub pk_title_enable: bool,
    pub text_tail_living_time: u32,

    pub character_text_tail_map: HashMap<u32, TextTail>,
    pub item_text_tail_map: HashMap<u32, TextTail>,
    pub chat_tail_map: HashMap<u32, TextTail>,

    pub character_text_tail_list: Vec<u32>,
    pub item_text_tail_list: Vec<u32>,
}

impl UiTextTail {
    pub fn new() -> Self {
        Self {
            initialized: true,
            pk_title_enable: true,
            text_tail_living_time: 5000,
            character_text_tail_map: HashMap::new(),
            item_text_tail_map: HashMap::new(),
            chat_tail_map: HashMap::new(),
            character_text_tail_list: Vec::new(),
            item_text_tail_list: Vec::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn enable_pk_title(&mut self, flag: bool) {
        self.pk_title_enable = flag;
    }

    pub fn text_tail_get_living_time(&self) -> u32 {
        self.text_tail_living_time
    }

    pub fn text_tail_set_living_time(&mut self, time: u32) {
        self.text_tail_living_time = time;
    }

    fn register_text_tail(&mut self, virtual_id: u32, text: &str, height: f32, color: Color) -> TextTail {
        let mut text_tail = TextTail::new(virtual_id);
        
        let mut text_instance = GraphicTextInstance::new();
        text_instance.set_value(text);
        text_instance.set_color(color.r, color.g, color.b);
        text_tail.text_instance = Some(text_instance);
        
        text_tail.height = height;
        text_tail.color = color.clone();
        
        let x_size = (text.len() * 8) as i32; 
        let y_size = 15;
        
        text_tail.x_start = (-x_size / 2 - 2) as f32;
        text_tail.y_start = -2.0;
        text_tail.x_end = (x_size / 2 + 2) as f32;
        text_tail.y_end = y_size as f32;
        
        text_tail
    }

    pub fn register_character_text_tail(&mut self, guild_id: u32, virtual_id: u32, color: Color, add_height: f32, name: &str, guild_name_str: &str) {
        let mut text_tail = self.register_text_tail(virtual_id, name, 100.0 + add_height, color);
        
        if let Some(ref mut ti) = text_tail.text_instance {
            ti.outline = true;
            ti.vertical_align = 2;
        }

        if guild_id != 0 {
            text_tail.mark_instance = Some(GraphicMarkInstance::new());
            
            let mut guild_name = GraphicTextInstance::new();
            guild_name.set_value(guild_name_str);
            guild_name.outline = true;
            guild_name.horizontal_align = 1; 
            guild_name.vertical_align = 2; 
            guild_name.set_color(1.0, 0.937, 0.827);
            
            text_tail.guild_name_text_instance = Some(guild_name);
        }

        self.character_text_tail_map.insert(virtual_id, text_tail);
    }

    pub fn register_item_text_tail(&mut self, virtual_id: u32, text: &str) {
        let text_tail = self.register_text_tail(virtual_id, text, -10.0, Color::new(1.0, 1.0, 1.0, 1.0));
        self.item_text_tail_map.insert(virtual_id, text_tail);
    }

    pub fn register_chat_tail(&mut self, virtual_id: u32, chat: &str, current_time: u32) {
        if let Some(text_tail) = self.chat_tail_map.get_mut(&virtual_id) {
            if let Some(ref mut ti) = text_tail.text_instance {
                ti.set_value(chat);
                ti.set_color(1.0, 1.0, 1.0);
            }
            text_tail.color = Color::new(1.0, 1.0, 1.0, 1.0);
            text_tail.living_time = current_time + self.text_tail_living_time;
            text_tail.name_flag = true;
            return;
        }

        let mut text_tail = self.register_text_tail(virtual_id, chat, 110.0, Color::new(1.0, 1.0, 1.0, 1.0));
        text_tail.living_time = current_time + self.text_tail_living_time;
        text_tail.name_flag = true;
        
        if let Some(ref mut ti) = text_tail.text_instance {
            ti.outline = true;
            ti.vertical_align = 2;
        }
        
        self.chat_tail_map.insert(virtual_id, text_tail);
    }

    pub fn register_info_tail(&mut self, virtual_id: u32, chat: &str, current_time: u32) {
        let color = Color::new(1.0, 0.785, 0.785, 1.0);
        
        if let Some(text_tail) = self.chat_tail_map.get_mut(&virtual_id) {
            if let Some(ref mut ti) = text_tail.text_instance {
                ti.set_value(chat);
                ti.set_color(color.r, color.g, color.b);
            }
            text_tail.color = color;
            text_tail.living_time = current_time + self.text_tail_living_time;
            text_tail.name_flag = false;
            return;
        }

        let mut text_tail = self.register_text_tail(virtual_id, chat, 110.0, color.clone());
        text_tail.living_time = current_time + self.text_tail_living_time;
        text_tail.name_flag = false;
        
        if let Some(ref mut ti) = text_tail.text_instance {
            ti.outline = true;
            ti.vertical_align = 2;
        }
        
        self.chat_tail_map.insert(virtual_id, text_tail);
    }

    fn update_distance(center_position: &PixelPosition, text_tail: &mut TextTail) {
        let dx = text_tail.owner_position.x - center_position.x;
        let dy = -text_tail.owner_position.y - center_position.y;
        text_tail.distance_from_player = (dx * dx + dy * dy).sqrt();
    }

    pub fn update_all_text_tail(&mut self, player_position: &PixelPosition) {
        for text_tail in self.character_text_tail_map.values_mut() {
            Self::update_distance(player_position, text_tail);
        }

        for text_tail in self.item_text_tail_map.values_mut() {
            Self::update_distance(player_position, text_tail);
        }

        let mut to_show = Vec::new();
        for (vid, chat_tail) in self.chat_tail_map.iter_mut() {
            Self::update_distance(player_position, chat_tail);
            if chat_tail.name_flag {
                to_show.push(*vid);
            }
        }
        
        for vid in to_show {
            self.show_character_text_tail(vid);
        }
    }

    #[allow(dead_code)]
    fn update_text_tail(&self, text_tail: &mut TextTail) {
        if !text_tail.owner_is_show {
            return;
        }

        text_tail.x = text_tail.owner_position.x;
        text_tail.y = text_tail.owner_position.y;
        text_tail.z = text_tail.owner_position.z + text_tail.height;
        
        text_tail.x = text_tail.x.floor();
        text_tail.y = text_tail.y.floor();

        if text_tail.distance_from_player < 1300.0 {
            text_tail.z = 0.0;
        } else {
            text_tail.z = text_tail.z * -1.0 + 10.0;
        }
    }

    pub fn update_showing_text_tail(&mut self) {
        for vid in &self.item_text_tail_list {
            if let Some(text_tail) = self.item_text_tail_map.get_mut(vid) {
                if !text_tail.owner_is_show { continue; }
                text_tail.x = text_tail.owner_position.x.floor();
                text_tail.y = text_tail.owner_position.y.floor();
                text_tail.z = text_tail.owner_position.z + text_tail.height;
                if text_tail.distance_from_player < 1300.0 {
                    text_tail.z = 0.0;
                } else {
                    text_tail.z = text_tail.z * -1.0 + 10.0;
                }
            }
        }

        for chat_tail in self.chat_tail_map.values_mut() {
            if !chat_tail.owner_is_show { continue; }
            chat_tail.x = chat_tail.owner_position.x.floor();
            chat_tail.y = chat_tail.owner_position.y.floor();
            chat_tail.z = chat_tail.owner_position.z + chat_tail.height;
            if chat_tail.distance_from_player < 1300.0 {
                chat_tail.z = 0.0;
            } else {
                chat_tail.z = chat_tail.z * -1.0 + 10.0;
            }
        }

        for vid in &self.character_text_tail_list {
            if let Some(mut text_tail) = self.character_text_tail_map.remove(vid) {
                if text_tail.owner_is_show {
                    text_tail.x = text_tail.owner_position.x.floor();
                    text_tail.y = text_tail.owner_position.y.floor();
                    text_tail.z = text_tail.owner_position.z + text_tail.height;
                    if text_tail.distance_from_player < 1300.0 {
                        text_tail.z = 0.0;
                    } else {
                        text_tail.z = text_tail.z * -1.0 + 10.0;
                    }

                    if let Some(chat_tail) = self.chat_tail_map.get(vid) {
                        if chat_tail.name_flag {
                            text_tail.y = chat_tail.y - 17.0;
                        }
                    }
                }
                self.character_text_tail_map.insert(*vid, text_tail);
            }
        }
    }

    fn is_in(source: &TextTail, target: &TextTail) -> bool {
        let x1_s = source.x + source.x_start;
        let y1_s = source.y + source.y_start;
        let x2_s = source.x + source.x_end;
        let y2_s = source.y + source.y_end;
        
        let x1_t = target.x + target.x_start;
        let y1_t = target.y + target.y_start;
        let x2_t = target.x + target.x_end;
        let y2_t = target.y + target.y_end;

        x1_s <= x2_t && x2_s >= x1_t && y1_s <= y2_t && y2_s >= y1_t
    }

    pub fn arrange_text_tail(&mut self, current_time: u32) {
        let sorted_items = self.item_text_tail_list.clone();
        
        for i in 0..sorted_items.len() {
            let vid_insert = sorted_items[i];
            
            let mut insert_text_tail = if let Some(t) = self.item_text_tail_map.remove(&vid_insert) { t } else { continue; };
            
            let y_temp = 5.0;
            let mut limit_count = 0;
            
            let mut j = 0;
            while j < sorted_items.len() {
                if i == j {
                    j += 1;
                    continue;
                }
                
                if limit_count >= 20 {
                    break;
                }
                
                let vid_compare = sorted_items[j];
                if let Some(compare_text_tail) = self.item_text_tail_map.get(&vid_compare) {
                    if Self::is_in(&insert_text_tail, compare_text_tail) {
                        insert_text_tail.y = compare_text_tail.y + compare_text_tail.y_end + y_temp;
                        j = 0;
                        limit_count += 1;
                        continue;
                    }
                }
                j += 1;
            }
            
            if let Some(ref mut owner_ti) = insert_text_tail.owner_text_instance {
                owner_ti.set_position(insert_text_tail.x, insert_text_tail.y, insert_text_tail.z);
                if let Some(ref mut ti) = insert_text_tail.text_instance {
                    ti.set_position(insert_text_tail.x, insert_text_tail.y + 15.0, insert_text_tail.z);
                }
            } else {
                if let Some(ref mut ti) = insert_text_tail.text_instance {
                    ti.set_position(insert_text_tail.x, insert_text_tail.y, insert_text_tail.z);
                }
            }
            
            self.item_text_tail_map.insert(vid_insert, insert_text_tail);
        }

        let sorted_chars = self.character_text_tail_list.clone();
        for vid in &sorted_chars {
            if let Some(mut p_text_tail) = self.character_text_tail_map.remove(vid) {
                let fx_add;
                
                let c_fx_mark_position = 1.5;
                let c_fy_mark_position = 15.0 + 11.0;
                let c_fy_guild_name_position = 15.0;

                let has_mark_and_guild = p_text_tail.mark_instance.is_some() && p_text_tail.guild_name_text_instance.is_some();
                if has_mark_and_guild {
                    let mark_width = p_text_tail.mark_instance.as_ref().unwrap().get_width();
                    let i_image_half_size = (mark_width as f32 / 2.0 + c_fx_mark_position) as i32;
                    
                    if let Some(ref mut mark) = p_text_tail.mark_instance {
                        let i_width = p_text_tail.guild_name_text_instance.as_ref().unwrap().get_text_size().0;
                        mark.set_position(p_text_tail.x - (i_width as f32 / 2.0) - i_image_half_size as f32, p_text_tail.y - c_fy_mark_position);
                    }
                    if let Some(ref mut guild) = p_text_tail.guild_name_text_instance {
                        guild.set_position(p_text_tail.x + i_image_half_size as f32, p_text_tail.y - c_fy_guild_name_position, p_text_tail.z);
                    }
                }

                let i_name_width = if let Some(ref ti) = p_text_tail.text_instance { ti.get_text_size().0 } else { 0 };

                let has_title = p_text_tail.title_text_instance.is_some();
                if has_title {
                    let i_title_width = p_text_tail.title_text_instance.as_ref().unwrap().get_text_size().0;
                    fx_add = 8.0;
                    
                    if let Some(ref mut title) = p_text_tail.title_text_instance {
                        title.set_position(p_text_tail.x - (i_name_width as f32 / 2.0) - fx_add, p_text_tail.y, p_text_tail.z);
                    }

                    if let Some(ref mut level) = p_text_tail.level_text_instance {
                        level.set_position(p_text_tail.x - (i_name_width as f32 / 2.0) - fx_add - i_title_width as f32, p_text_tail.y, p_text_tail.z);
                    }
                } else {
                    fx_add = 4.0;
                    if let Some(ref mut level) = p_text_tail.level_text_instance {
                        level.set_position(p_text_tail.x - (i_name_width as f32 / 2.0) - fx_add, p_text_tail.y, p_text_tail.z);
                    }
                }
                
                if let Some(ref mut ti) = p_text_tail.text_instance {
                    ti.set_position(p_text_tail.x + fx_add, p_text_tail.y, p_text_tail.z);
                }
                
                self.character_text_tail_map.insert(*vid, p_text_tail);
            }
        }

        let mut to_remove = Vec::new();
        for (vid, p_text_tail) in self.chat_tail_map.iter_mut() {
            if p_text_tail.living_time < current_time {
                to_remove.push(*vid);
                continue;
            }
            if let Some(ref mut ti) = p_text_tail.text_instance {
                ti.set_position(p_text_tail.x, p_text_tail.y, p_text_tail.z);
            }
        }
        for vid in to_remove {
            self.chat_tail_map.remove(&vid);
        }
    }

    pub fn hide_all_text_tail(&mut self) {
        self.character_text_tail_list.clear();
        self.item_text_tail_list.clear();
    }

    pub fn show_all_text_tail(&mut self) {
        let mut to_show_char = Vec::new();
        for (vid, text_tail) in &self.character_text_tail_map {
            if text_tail.distance_from_player < 3500.0 {
                to_show_char.push(*vid);
            }
        }
        for vid in to_show_char {
            self.show_character_text_tail(vid);
        }

        let mut to_show_item = Vec::new();
        for (vid, text_tail) in &self.item_text_tail_map {
            if text_tail.distance_from_player < 3500.0 {
                to_show_item.push(*vid);
            }
        }
        for vid in to_show_item {
            self.show_item_text_tail(vid);
        }
    }

    pub fn show_character_text_tail(&mut self, virtual_id: u32) {
        if let Some(text_tail) = self.character_text_tail_map.get(&virtual_id) {
            if !self.character_text_tail_list.contains(&virtual_id) {
                if text_tail.owner_is_show {
                    self.character_text_tail_list.push(virtual_id);
                }
            }
        }
    }

    pub fn show_item_text_tail(&mut self, virtual_id: u32) {
        if self.item_text_tail_map.contains_key(&virtual_id) {
            if !self.item_text_tail_list.contains(&virtual_id) {
                self.item_text_tail_list.push(virtual_id);
            }
        }
    }

    pub fn get_text_tail_position(&self, vid: u32) -> Option<(f32, f32, f32)> {
        self.character_text_tail_map.get(&vid).map(|t| (t.x, t.y, t.z))
    }

    pub fn is_chat_text_tail(&self, vid: u32) -> bool {
        self.chat_tail_map.contains_key(&vid)
    }

    pub fn set_character_text_tail_color(&mut self, virtual_id: u32, color: Color) {
        if let Some(text_tail) = self.character_text_tail_map.get_mut(&virtual_id) {
            if let Some(ref mut ti) = text_tail.text_instance {
                ti.set_color(color.r, color.g, color.b);
            }
            text_tail.color = color;
        }
    }

    pub fn set_item_text_tail_owner(&mut self, virtual_id: u32, name: &str) {
        if let Some(text_tail) = self.item_text_tail_map.get_mut(&virtual_id) {
            if !name.is_empty() {
                if text_tail.owner_text_instance.is_none() {
                    text_tail.owner_text_instance = Some(GraphicTextInstance::new());
                }
                
                let mut owner_name = name.to_string();
                owner_name.push_str("'s");
                
                if let Some(ref mut owner_ti) = text_tail.owner_text_instance {
                    owner_ti.set_value(&owner_name);
                    owner_ti.set_color(1.0, 1.0, 0.0);
                    owner_ti.horizontal_align = 1;
                    
                    let x_owner_size = owner_ti.get_text_size().0 as f32;
                    let y_owner_size = owner_ti.get_text_size().1 as f32;
                    
                    text_tail.y_start = -2.0;
                    text_tail.y_end += y_owner_size + 4.0;
                    text_tail.x_start = text_tail.x_start.min(-x_owner_size / 2.0 - 1.0);
                    text_tail.x_end = text_tail.x_end.max(x_owner_size / 2.0 + 1.0);
                }
            } else {
                text_tail.owner_text_instance = None;
                if let Some(ref ti) = text_tail.text_instance {
                    let x_size = ti.get_text_size().0 as f32;
                    let y_size = ti.get_text_size().1 as f32;
                    text_tail.x_start = -x_size / 2.0 - 2.0;
                    text_tail.y_start = -2.0;
                    text_tail.x_end = x_size / 2.0 + 2.0;
                    text_tail.y_end = y_size;
                }
            }
        }
    }

    pub fn delete_character_text_tail(&mut self, virtual_id: u32) {
        self.character_text_tail_map.remove(&virtual_id);
        self.character_text_tail_list.retain(|&x| x != virtual_id);
        self.chat_tail_map.remove(&virtual_id);
    }

    pub fn delete_item_text_tail(&mut self, virtual_id: u32) {
        self.item_text_tail_map.remove(&virtual_id);
        self.item_text_tail_list.retain(|&x| x != virtual_id);
    }

    pub fn pick(&mut self, ix_mouse: f32, iy_mouse: f32) -> Option<u32> {
        let mut picked_vid = None;
        for (vid, text_tail) in &self.item_text_tail_map {
            if ix_mouse >= text_tail.x + text_tail.x_start && ix_mouse <= text_tail.x + text_tail.x_end &&
               iy_mouse >= text_tail.y + text_tail.y_start && iy_mouse <= text_tail.y + text_tail.y_end {
                picked_vid = Some(*vid);
                break;
            }
        }
        if let Some(vid) = picked_vid {
            self.select_item_name(vid);
        }
        picked_vid
    }

    pub fn select_item_name(&mut self, virtual_id: u32) {
        if let Some(text_tail) = self.item_text_tail_map.get_mut(&virtual_id) {
            if let Some(ref mut ti) = text_tail.text_instance {
                ti.set_color(0.1, 0.9, 0.1);
            }
        }
    }

    pub fn attach_title(&mut self, virtual_id: u32, name: &str, color: Color) {
        if !self.pk_title_enable { return; }
        if let Some(text_tail) = self.character_text_tail_map.get_mut(&virtual_id) {
            if text_tail.title_text_instance.is_none() {
                let mut title = GraphicTextInstance::new();
                title.outline = true;
                title.horizontal_align = 1;
                title.vertical_align = 2;
                text_tail.title_text_instance = Some(title);
            }
            if let Some(ref mut title) = text_tail.title_text_instance {
                title.set_value(name);
                title.set_color(color.r, color.g, color.b);
            }
        }
    }

    pub fn detach_title(&mut self, virtual_id: u32) {
        if !self.pk_title_enable { return; }
        if let Some(text_tail) = self.character_text_tail_map.get_mut(&virtual_id) {
            text_tail.title_text_instance = None;
        }
    }

    pub fn attach_level(&mut self, virtual_id: u32, text: &str, color: Color) {
        if !self.pk_title_enable { return; }
        if let Some(text_tail) = self.character_text_tail_map.get_mut(&virtual_id) {
            if text_tail.level_text_instance.is_none() {
                let mut level = GraphicTextInstance::new();
                level.outline = true;
                level.horizontal_align = 2;
                level.vertical_align = 2;
                text_tail.level_text_instance = Some(level);
            }
            if let Some(ref mut level) = text_tail.level_text_instance {
                level.set_value(text);
                level.set_color(color.r, color.g, color.b);
            }
        }
    }

    pub fn detach_level(&mut self, virtual_id: u32) {
        if !self.pk_title_enable { return; }
        if let Some(text_tail) = self.character_text_tail_map.get_mut(&virtual_id) {
            text_tail.level_text_instance = None;
        }
    }

    pub fn clear(&mut self) {
        self.character_text_tail_map.clear();
        self.character_text_tail_list.clear();
        self.item_text_tail_map.clear();
        self.item_text_tail_list.clear();
        self.chat_tail_map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_text_tail() {
        let instance = UiTextTail::new();
        assert!(instance.is_ready());
    }
}
