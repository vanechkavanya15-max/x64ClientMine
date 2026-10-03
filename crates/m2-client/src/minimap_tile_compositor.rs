use crate::minimap_navigation::MinimapNavigation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarkerType {
    Player,
    Npc,
    Waypoint,
}

pub struct MinimapCompositor {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl MinimapCompositor {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn draw_tile(&mut self, x: i32, y: i32, tile_width: u32, tile_height: u32, tile_data: &[u8]) {
        for ty in 0..tile_height as i32 {
            for tx in 0..tile_width as i32 {
                let dest_x = x + tx;
                let dest_y = y + ty;

                if dest_x >= 0 && dest_x < self.width as i32 && dest_y >= 0 && dest_y < self.height as i32 {
                    let dest_idx = ((dest_y * self.width as i32 + dest_x) * 4) as usize;
                    let src_idx = ((ty * tile_width as i32 + tx) * 4) as usize;

                    let src_r = tile_data[src_idx] as f32;
                    let src_g = tile_data[src_idx + 1] as f32;
                    let src_b = tile_data[src_idx + 2] as f32;
                    let src_a = tile_data[src_idx + 3] as f32 / 255.0;

                    let dest_r = self.buffer[dest_idx] as f32;
                    let dest_g = self.buffer[dest_idx + 1] as f32;
                    let dest_b = self.buffer[dest_idx + 2] as f32;
                    let dest_a = self.buffer[dest_idx + 3] as f32 / 255.0;

                    let out_a = src_a + dest_a * (1.0 - src_a);
                    
                    if out_a > 0.0 {
                        let out_r = (src_r * src_a + dest_r * dest_a * (1.0 - src_a)) / out_a;
                        let out_g = (src_g * src_a + dest_g * dest_a * (1.0 - src_a)) / out_a;
                        let out_b = (src_b * src_a + dest_b * dest_a * (1.0 - src_a)) / out_a;

                        self.buffer[dest_idx] = out_r as u8;
                        self.buffer[dest_idx + 1] = out_g as u8;
                        self.buffer[dest_idx + 2] = out_b as u8;
                        self.buffer[dest_idx + 3] = (out_a * 255.0) as u8;
                    }
                }
            }
        }
    }

    pub fn draw_marker(&mut self, x: i32, y: i32, marker_type: MarkerType) {
        let color = match marker_type {
            MarkerType::Player => [255, 0, 0, 255], // Red
            MarkerType::Npc => [0, 255, 0, 255],    // Green
            MarkerType::Waypoint => [0, 0, 255, 255], // Blue
        };

        // Draw a simple 3x3 square marker
        for my in -1..=1 {
            for mx in -1..=1 {
                let dest_x = x + mx;
                let dest_y = y + my;

                if dest_x >= 0 && dest_x < self.width as i32 && dest_y >= 0 && dest_y < self.height as i32 {
                    let dest_idx = ((dest_y * self.width as i32 + dest_x) * 4) as usize;
                    self.buffer[dest_idx] = color[0];
                    self.buffer[dest_idx + 1] = color[1];
                    self.buffer[dest_idx + 2] = color[2];
                    self.buffer[dest_idx + 3] = color[3];
                }
            }
        }
    }

    pub fn draw_marker_world(
        &mut self,
        world_x: f32,
        world_y: f32,
        marker_type: MarkerType,
        nav: &MinimapNavigation,
    ) {
        let (minimap_x, minimap_y) = nav.world_to_minimap(world_x, world_y);
        // We assume the minimap mapping returned by world_to_minimap matches the compositor dimensions in some way,
        // or we just cast to i32 for the compositor's coordinate space. 
        self.draw_marker(minimap_x as i32, minimap_y as i32, marker_type);
    }
}
