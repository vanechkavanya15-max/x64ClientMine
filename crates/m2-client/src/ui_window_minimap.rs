use crate::minimap_compass_mapper_v2::{m2_minimap_world_to_screen, MinimapCompassMapperV2, PinType};
use std::result::Result;
use crate::minimap_navigation::MinimapNavigation;
use crate::ui_window_core::{Window, WindowType, HORIZONTAL_ALIGN_CENTER, VERTICAL_ALIGN_CENTER};

#[derive(Debug, Clone)]
pub struct MinimapPin {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub pin_type: PinType,
}

#[derive(Debug, Clone)]
pub struct MinimapStateData {
    pub zoom_scale: f32,
    pub current_map_name: String,
    pub current_channel: String,
    pub player_x: f32,
    pub player_y: f32,
    pub camera_rotation: f32,
    pub player_rotation: f32,
    pub pins: Vec<MinimapPin>,
}

impl Default for MinimapStateData {
    fn default() -> Self {
        Self {
            zoom_scale: 1.0,
            current_map_name: "Unknown".to_string(),
            current_channel: "CH1".to_string(),
            player_x: 0.0,
            player_y: 0.0,
            camera_rotation: 0.0,
            player_rotation: 0.0,
            pins: Vec::new(),
        }
    }
}

pub struct UiWindowMinimap {
    pub root_handle: u32,
    pub base_window: Window,
    pub bg_image_box: Window,
    pub pos_text_line: Window,
    pub map_name_text_line: Window,
    pub zoom_in_btn: Window,
    pub zoom_out_btn: Window,
    pub player_arrow_img: Window,
    pub compass_img: Window,

    pub state: MinimapStateData,
}

impl UiWindowMinimap {
    pub fn new(root_handle: u32) -> Self {
        let mut base_window = Window::new(root_handle, WindowType::Box);
        base_window.name = "MinimapBoard".to_string();
        base_window.width = 136;
        base_window.height = 136;
        
        let mut bg_image_box = Window::new(root_handle + 1, WindowType::ImageBox);
        bg_image_box.name = "MinimapBackground".to_string();
        bg_image_box.image_path = "d:/ymir work/ui/minimap/minimap.dds".to_string();
        bg_image_box.width = 136;
        bg_image_box.height = 136;
        
        let mut pos_text_line = Window::new(root_handle + 2, WindowType::TextLine);
        pos_text_line.name = "PositionText".to_string();
        pos_text_line.text = "X: 0, Y: 0".to_string();
        pos_text_line.local_y = 140;

        let mut map_name_text_line = Window::new(root_handle + 3, WindowType::TextLine);
        map_name_text_line.name = "MapNameText".to_string();
        map_name_text_line.text = "Unknown - CH1".to_string();
        map_name_text_line.local_y = -20;

        let mut zoom_in_btn = Window::new(root_handle + 4, WindowType::Button);
        zoom_in_btn.name = "ZoomInButton".to_string();
        zoom_in_btn.up_visual = "d:/ymir work/ui/minimap/minimap_zoom_in_default.sub".to_string();
        zoom_in_btn.local_x = 110;
        zoom_in_btn.local_y = 10;
        zoom_in_btn.width = 16;
        zoom_in_btn.height = 16;

        let mut zoom_out_btn = Window::new(root_handle + 5, WindowType::Button);
        zoom_out_btn.name = "ZoomOutButton".to_string();
        zoom_out_btn.up_visual = "d:/ymir work/ui/minimap/minimap_zoom_out_default.sub".to_string();
        zoom_out_btn.local_x = 110;
        zoom_out_btn.local_y = 30;
        zoom_out_btn.width = 16;
        zoom_out_btn.height = 16;

        let mut player_arrow_img = Window::new(root_handle + 6, WindowType::ImageBox);
        player_arrow_img.name = "PlayerArrow".to_string();
        player_arrow_img.image_path = "d:/ymir work/ui/minimap/minimap_arrow.dds".to_string();
        player_arrow_img.horizontal_align = HORIZONTAL_ALIGN_CENTER;
        player_arrow_img.vertical_align = VERTICAL_ALIGN_CENTER;

        let mut compass_img = Window::new(root_handle + 7, WindowType::ImageBox);
        compass_img.name = "Compass".to_string();
        compass_img.image_path = "d:/ymir work/ui/minimap/compass.dds".to_string();
        compass_img.horizontal_align = HORIZONTAL_ALIGN_CENTER;
        compass_img.vertical_align = VERTICAL_ALIGN_CENTER;

        base_window.children = vec![
            bg_image_box.handle,
            pos_text_line.handle,
            map_name_text_line.handle,
            zoom_in_btn.handle,
            zoom_out_btn.handle,
            player_arrow_img.handle,
            compass_img.handle,
        ];

        Self {
            root_handle,
            base_window,
            bg_image_box,
            pos_text_line,
            map_name_text_line,
            zoom_in_btn,
            zoom_out_btn,
            player_arrow_img,
            compass_img,
            state: MinimapStateData::default(),
        }
    }

    pub fn update_state(&mut self) -> Result<(), &'static str> {
        // Skalowanie współrzędnych do formatu z klienta / 100 (centymetry na metry lub komórki)
        let display_x = (self.state.player_x / 100.0).round() as i32;
        let display_y = (self.state.player_y / 100.0).round() as i32;
        self.pos_text_line.text = format!("X: {}, Y: {}", display_x, display_y);

        self.map_name_text_line.text = format!("{} - {}", self.state.current_map_name, self.state.current_channel);

        // Obrot kompasu oparty na rotacji kamery
        let compass_rot = MinimapCompassMapperV2::calculate_compass_rotation(self.state.camera_rotation);
        self.compass_img.color = [compass_rot, compass_rot, compass_rot, compass_rot]; // Zapisanie rotacji w polu color jako fallback

        // Obrot strzalki gracza wzgledem rotacji gracza
        let arrow_rot = MinimapNavigation::calculate_arrow_rotation(self.state.player_rotation);
        self.player_arrow_img.color = [arrow_rot, arrow_rot, arrow_rot, arrow_rot]; // Zapisanie rotacji w polu color jako fallback
        
        Ok(())
    }

    pub fn handle_zoom_in(&mut self) -> Result<(), &'static str> {
        if self.state.zoom_scale >= 2.0 {
            return Err("Max zoom scale reached");
        }
        if self.state.zoom_scale < 1.0 {
            self.state.zoom_scale = 1.0;
        } else {
            self.state.zoom_scale = 2.0;
        }
        Ok(())
    }

    pub fn handle_zoom_out(&mut self) -> Result<(), &'static str> {
        if self.state.zoom_scale <= 0.5 {
            return Err("Min zoom scale reached");
        }
        if self.state.zoom_scale > 1.0 {
            self.state.zoom_scale = 1.0;
        } else {
            self.state.zoom_scale = 0.5;
        }
        Ok(())
    }

    pub fn add_pin(&mut self, id: u32, x: f32, y: f32, pin_type: PinType) -> Result<(), &'static str> {
        self.state.pins.push(MinimapPin { id, x, y, pin_type });
        Ok(())
    }

    pub fn clear_pins(&mut self) -> Result<(), &'static str> {
        self.state.pins.clear();
        Ok(())
    }

    pub fn calculate_pin_screen_positions(&self, radius: f32) -> Result<Vec<(u32, f32, f32, u32)>, &'static str> {
        let mut results = Vec::new();
        // Zastosowanie zoomu do promienia pozwala pokazac wiekszy obszar
        let effective_radius = radius * self.state.zoom_scale;

        for pin in &self.state.pins {
            let screen_pos = m2_minimap_world_to_screen(
                self.state.player_x,
                self.state.player_y,
                pin.x,
                pin.y,
                effective_radius,
            );
            
            // Kolor ze struktury pinu
            let color = pin.pin_type.get_color();
            
            results.push((pin.id, screen_pos.x, screen_pos.y, color));
        }

        Ok(results)
    }
}
