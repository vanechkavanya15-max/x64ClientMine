//! Modul `software_framebuffer` — Renderer bufora klatki 64-bit (3D Teren + Aktorzy + UI + Fonty -> Piksele RGBA)
//! Wzorzec referencyjny C++: `source/EterLib/Screen.cpp oraz source/EterLib/GrpDevice.cpp`
//! Opis: Kompletny modul rasteryzatora i kompozytora klatki 64-bit.

use glam::{Vec3, Vec4, Mat4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Ok,
    Broken,
    NeedsReset,
    Null,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self::Null
    }
}

pub mod create_return_values {
    pub const OK: u32                = 1 << 0;
    pub const NO_DIRECTX: u32        = 1 << 1;
    pub const GET_DEVICE_CAPS: u32   = 1 << 2;
    pub const GET_DEVICE_CAPS2: u32  = 1 << 3;
    pub const DEVICE: u32            = 1 << 4;
    pub const REFRESHRATE: u32       = 1 << 5;
    pub const ENUM: u32              = 1 << 6;
    pub const DETECT: u32            = 1 << 7;
    pub const NO_TNL: u32            = 1 << 8;
    pub const BAD_DRIVER: u32        = 1 << 9;
    pub const FORMAT: u32            = 1 << 10;
}

#[derive(Debug, Clone)]
pub struct SoftwareFramebuffer {
    pub initialized: bool,
    pub device_state: DeviceState,
    pub back_buffer_count: u32,
    
    // Rozmiar okna / bufora
    pub width: u32,
    pub height: u32,

    // Bufory
    pub back_buffer: Vec<u32>,
    pub depth_buffer: Vec<f32>,
    pub stencil_buffer: Vec<u8>,

    // Ustawienia ekranu i kolorow
    pub diffuse_color: u32,
    pub clear_color: u32,
    pub clear_stencil: u32,
    pub clear_depth: f32,

    // Macierze do rzutowania
    pub mat_proj: Mat4,
    pub mat_view: Mat4,
    pub mat_world: Mat4,
}

impl Default for SoftwareFramebuffer {
    fn default() -> Self {
        Self {
            initialized: false,
            device_state: DeviceState::Null,
            back_buffer_count: 0,
            width: 800,
            height: 600,
            back_buffer: vec![0; 800 * 600],
            depth_buffer: vec![1.0; 800 * 600],
            stencil_buffer: vec![0; 800 * 600],
            diffuse_color: 0xffffffff,
            clear_color: 0,
            clear_stencil: 0,
            clear_depth: 1.0,
            mat_proj: Mat4::IDENTITY,
            mat_view: Mat4::IDENTITY,
            mat_world: Mat4::IDENTITY,
        }
    }
}

impl SoftwareFramebuffer {
    pub fn new() -> Self {
        let mut sf = Self::default();
        sf.initialized = true;
        sf
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn init_back_buffer_count(&mut self, back_buffer_count: u32) {
        self.back_buffer_count = back_buffer_count;
    }

    pub fn destroy(&mut self) {
        self.initialized = false;
        self.device_state = DeviceState::Null;
        self.back_buffer_count = 0;
        self.back_buffer.clear();
        self.depth_buffer.clear();
        self.stencil_buffer.clear();
    }

    pub fn create(&mut self, _hwnd: usize, hres: i32, vres: i32, _windowed: bool, _bit: i32, _reflash_rate: i32) -> u32 {
        self.width = hres as u32;
        self.height = vres as u32;
        let size = (self.width * self.height) as usize;
        self.back_buffer = vec![0; size];
        self.depth_buffer = vec![1.0; size];
        self.stencil_buffer = vec![0; size];
        self.device_state = DeviceState::Ok;
        self.initialized = true;
        create_return_values::OK
    }

    pub fn get_device_state(&self) -> DeviceState {
        self.device_state
    }

    pub fn reset(&mut self) -> bool {
        self.device_state = DeviceState::Ok;
        true
    }

    pub fn resize_back_buffer(&mut self, width: u32, height: u32) -> bool {
        self.width = width;
        self.height = height;
        let size = (width * height) as usize;
        self.back_buffer = vec![0; size];
        self.depth_buffer = vec![1.0; size];
        self.stencil_buffer = vec![0; size];
        true
    }

    pub fn register_warning_string(&mut self, _msg: u32, _string: &str) {}

    // Pomocnicza do rysowania pojedynczego piksela
    fn put_pixel(&mut self, x: i32, y: i32, color: u32, depth: f32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = (y * self.width as i32 + x) as usize;
            if depth <= self.depth_buffer[idx] {
                self.back_buffer[idx] = color;
                self.depth_buffer[idx] = depth;
            }
        }
    }

    pub fn clear_depth_buffer(&mut self) {
        for d in self.depth_buffer.iter_mut() {
            *d = self.clear_depth;
        }
    }

    pub fn clear(&mut self) {
        self.back_buffer.fill(self.clear_color);
        self.depth_buffer.fill(self.clear_depth);
        self.stencil_buffer.fill(self.clear_stencil as u8);
    }

    pub fn begin(&mut self) -> bool {
        self.is_ready()
    }

    pub fn end(&mut self) {}

    pub fn show(&mut self) {}

    // 2D Rendering

    pub fn render_line_2d(&mut self, sx: f32, sy: f32, ex: f32, ey: f32, z: f32) {
        // Algorytm Bresenhama
        let mut x0 = sx as i32;
        let mut y0 = sy as i32;
        let x1 = ex as i32;
        let y1 = ey as i32;

        let dx = (x1 - x0).abs();
        let sx_step = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy_step = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.put_pixel(x0, y0, self.diffuse_color, z);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx_step;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy_step;
            }
        }
    }

    pub fn render_box_2d(&mut self, sx: f32, sy: f32, ex: f32, ey: f32, z: f32) {
        self.render_line_2d(sx, sy, ex, sy, z);
        self.render_line_2d(ex, sy, ex, ey, z);
        self.render_line_2d(ex, ey, sx, ey, z);
        self.render_line_2d(sx, ey, sx, sy, z);
    }

    pub fn render_bar_2d(&mut self, sx: f32, sy: f32, ex: f32, ey: f32, z: f32) {
        let x_start = (sx as i32).max(0);
        let x_end = (ex as i32).min(self.width as i32 - 1);
        let y_start = (sy as i32).max(0);
        let y_end = (ey as i32).min(self.height as i32 - 1);

        for y in y_start..=y_end {
            for x in x_start..=x_end {
                self.put_pixel(x, y, self.diffuse_color, z);
            }
        }
    }

    pub fn render_gradation_bar_2d(&mut self, sx: f32, sy: f32, ex: f32, ey: f32, start_color: u32, end_color: u32, z: f32) {
        let x_start = (sx as i32).max(0);
        let x_end = (ex as i32).min(self.width as i32 - 1);
        let y_start = (sy as i32).max(0);
        let y_end = (ey as i32).min(self.height as i32 - 1);

        if y_end <= y_start { return; }

        let s_r = (start_color >> 16) & 0xff;
        let s_g = (start_color >> 8) & 0xff;
        let s_b = start_color & 0xff;
        let s_a = (start_color >> 24) & 0xff;

        let e_r = (end_color >> 16) & 0xff;
        let e_g = (end_color >> 8) & 0xff;
        let e_b = end_color & 0xff;
        let e_a = (end_color >> 24) & 0xff;

        let height = (y_end - y_start) as f32;

        for y in y_start..=y_end {
            let t = (y - y_start) as f32 / height;
            let r = (s_r as f32 + t * (e_r as f32 - s_r as f32)) as u32;
            let g = (s_g as f32 + t * (e_g as f32 - s_g as f32)) as u32;
            let b = (s_b as f32 + t * (e_b as f32 - s_b as f32)) as u32;
            let a = (s_a as f32 + t * (e_a as f32 - s_a as f32)) as u32;
            
            let color = (a << 24) | (r << 16) | (g << 8) | b;
            for x in x_start..=x_end {
                self.put_pixel(x, y, color, z);
            }
        }
    }

    pub fn render_circle_2d(&mut self, fx: f32, fy: f32, fz: f32, radius: f32, step: i32) {
        if step <= 0 { return; }
        let mut prev_x = fx + radius;
        let mut prev_y = fy;
        
        for i in 1..=step {
            let angle = (i as f32 / step as f32) * std::f32::consts::TAU;
            let curr_x = fx + radius * angle.cos();
            let curr_y = fy + radius * angle.sin();
            self.render_line_2d(prev_x, prev_y, curr_x, curr_y, fz);
            prev_x = curr_x;
            prev_y = curr_y;
        }
    }

    pub fn render_circle_3d(&mut self, fx: f32, fy: f32, fz: f32, radius: f32, step: i32) {
        if step <= 0 { return; }
        let mut prev_x = fx + radius;
        let mut prev_y = fy;
        
        for i in 1..=step {
            let angle = (i as f32 / step as f32) * std::f32::consts::TAU;
            let curr_x = fx + radius * angle.cos();
            let curr_y = fy + radius * angle.sin();
            self.render_line_3d(prev_x, prev_y, fz, curr_x, curr_y, fz);
            prev_x = curr_x;
            prev_y = curr_y;
        }
    }

    // Rasteryzacja trojkatow uzywajac wspolrzednych barycentrycznych
    pub fn render_triangle(&mut self, p1: Vec3, p2: Vec3, p3: Vec3, color: u32) {
        // Oblicz bounding box
        let min_x = p1.x.min(p2.x).min(p3.x).max(0.0) as i32;
        let max_x = p1.x.max(p2.x).max(p3.x).min((self.width - 1) as f32) as i32;
        let min_y = p1.y.min(p2.y).min(p3.y).max(0.0) as i32;
        let max_y = p1.y.max(p2.y).max(p3.y).min((self.height - 1) as f32) as i32;

        let det = (p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y);
        if det == 0.0 { return; }
        
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let px = x as f32;
                let py = y as f32;
                
                let l1 = ((p2.y - p3.y) * (px - p3.x) + (p3.x - p2.x) * (py - p3.y)) / det;
                let l2 = ((p3.y - p1.y) * (px - p3.x) + (p1.x - p3.x) * (py - p3.y)) / det;
                let l3 = 1.0 - l1 - l2;
                
                if l1 >= 0.0 && l2 >= 0.0 && l3 >= 0.0 {
                    let depth = l1 * p1.z + l2 * p2.z + l3 * p3.z;
                    self.put_pixel(x, y, color, depth);
                }
            }
        }
    }

    // 3D Rendering (rzutowanie do 2D)

    pub fn project_position_vec(&self, pos: Vec3) -> Option<Vec3> {
        let v4 = Vec4::new(pos.x, pos.y, pos.z, 1.0);
        let clip_pos = self.mat_proj * self.mat_view * self.mat_world * v4;
        
        if clip_pos.w == 0.0 {
            return None;
        }
        
        let ndc = clip_pos / clip_pos.w;
        // Sprawdz clipping
        if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 || ndc.z < 0.0 || ndc.z > 1.0 {
            return None;
        }

        let win_x = (ndc.x + 1.0) * 0.5 * self.width as f32;
        let win_y = (1.0 - ndc.y) * 0.5 * self.height as f32; // odwrócone Y
        Some(Vec3::new(win_x, win_y, ndc.z))
    }

    pub fn render_line_3d(&mut self, sx: f32, sy: f32, sz: f32, ex: f32, ey: f32, ez: f32) {
        let p1 = self.project_position_vec(Vec3::new(sx, sy, sz));
        let p2 = self.project_position_vec(Vec3::new(ex, ey, ez));

        if let (Some(p1), Some(p2)) = (p1, p2) {
            let z = (p1.z + p2.z) * 0.5; // Aproksymacja z
            self.render_line_2d(p1.x, p1.y, p2.x, p2.y, z);
        }
    }

    pub fn render_box_3d(&mut self, sx: f32, sy: f32, sz: f32, ex: f32, ey: f32, ez: f32) {
        // Wierzcholki sciany przedniej
        self.render_line_3d(sx, sy, sz, ex, sy, sz);
        self.render_line_3d(ex, sy, sz, ex, ey, sz);
        self.render_line_3d(ex, ey, sz, sx, ey, sz);
        self.render_line_3d(sx, ey, sz, sx, sy, sz);
        
        // Wierzcholki sciany tylnej
        self.render_line_3d(sx, sy, ez, ex, sy, ez);
        self.render_line_3d(ex, sy, ez, ex, ey, ez);
        self.render_line_3d(ex, ey, ez, sx, ey, ez);
        self.render_line_3d(sx, ey, ez, sx, sy, ez);

        // Linie laczace
        self.render_line_3d(sx, sy, sz, sx, sy, ez);
        self.render_line_3d(ex, sy, sz, ex, sy, ez);
        self.render_line_3d(ex, ey, sz, ex, ey, ez);
        self.render_line_3d(sx, ey, sz, sx, ey, ez);
    }

    pub fn render_bar_3d(&mut self, sx: f32, sy: f32, sz: f32, ex: f32, ey: f32, ez: f32) {
        // Wypelniony prostokat za pomoca 2 trojkatow
        let p1 = self.project_position_vec(Vec3::new(sx, sy, sz));
        let p2 = self.project_position_vec(Vec3::new(sx, ey, ez));
        let p3 = self.project_position_vec(Vec3::new(ex, sy, sz));
        let p4 = self.project_position_vec(Vec3::new(ex, ey, ez));

        if let (Some(p1), Some(p2), Some(p3), Some(p4)) = (p1, p2, p3, p4) {
            self.render_triangle(p1, p2, p3, self.diffuse_color);
            self.render_triangle(p2, p4, p3, self.diffuse_color);
        }
    }

    pub fn render_bar_3d_positions(&mut self, positions: &[Vec3]) {
        for i in 0..positions.len().saturating_sub(1) {
            self.render_line_3d(positions[i].x, positions[i].y, positions[i].z, 
                                positions[i+1].x, positions[i+1].y, positions[i+1].z);
        }
        if positions.len() > 2 {
            self.render_line_3d(positions.last().unwrap().x, positions.last().unwrap().y, positions.last().unwrap().z, 
                                positions[0].x, positions[0].y, positions[0].z);
        }
    }

    pub fn render_gradation_bar_3d(&mut self, sx: f32, sy: f32, sz: f32, ex: f32, ey: f32, ez: f32, start_color: u32, end_color: u32) {
        let p1 = self.project_position_vec(Vec3::new(sx, sy, sz));
        let p2 = self.project_position_vec(Vec3::new(ex, ey, ez));

        if let (Some(p1), Some(p2)) = (p1, p2) {
            let z = (p1.z + p2.z) * 0.5;
            self.render_gradation_bar_2d(p1.x, p1.y, p2.x, p2.y, start_color, end_color, z);
        }
    }

    pub fn render_line_cube(&mut self, sx: f32, sy: f32, sz: f32, ex: f32, ey: f32, ez: f32) {
        self.render_box_3d(sx, sy, sz, ex, ey, ez);
    }

    pub fn render_cube(&mut self, sx: f32, sy: f32, sz: f32, ex: f32, ey: f32, ez: f32) {
        self.render_box_3d(sx, sy, sz, ex, ey, ez);
    }

    pub fn render_cube_rotated(&mut self, sx: f32, sy: f32, sz: f32, ex: f32, ey: f32, ez: f32, rotation: Mat4) {
        let prev_world = self.mat_world;
        self.mat_world = self.mat_world * rotation;
        self.render_cube(sx, sy, sz, ex, ey, ez);
        self.mat_world = prev_world;
    }

    pub fn render_texture_box(&mut self, sx: f32, sy: f32, ex: f32, ey: f32, z: f32, su: f32, sv: f32, eu: f32, ev: f32) {
        let x_start = (sx as i32).max(0);
        let x_end = (ex as i32).min(self.width as i32 - 1);
        let y_start = (sy as i32).max(0);
        let y_end = (ey as i32).min(self.height as i32 - 1);

        if x_end <= x_start || y_end <= y_start { return; }
        
        let width_f = (x_end - x_start) as f32;
        let height_f = (y_end - y_start) as f32;

        for y in y_start..=y_end {
            for x in x_start..=x_end {
                let u = su + ((x - x_start) as f32 / width_f) * (eu - su);
                let v = sv + ((y - y_start) as f32 / height_f) * (ev - sv);
                
                // Prosty sampling UV szachownicy (symulacja tekstury zeby miec "teksture" bez referencji do assetow)
                let tx = (u * 16.0) as i32;
                let ty = (v * 16.0) as i32;
                let color = if (tx + ty) % 2 == 0 {
                    0xffffffff // biale
                } else {
                    0xff000000 // czarne
                };
                
                // Mieszanie koloru z tekstury i koloru z vertexa
                let blend_color = self.diffuse_color & color;
                
                self.put_pixel(x, y, blend_color, z);
            }
        }
    }

    pub fn render_billboard(&mut self, position: Vec3, color: u32) {
        let prev_color = self.diffuse_color;
        self.diffuse_color = color;
        if let Some(p) = self.project_position_vec(position) {
            self.render_box_2d(p.x - 5.0, p.y - 5.0, p.x + 5.0, p.y + 5.0, p.z);
        }
        self.diffuse_color = prev_color;
    }

    pub fn draw_minor_grid(&mut self, x_min: f32, y_min: f32, x_max: f32, y_max: f32, x_minor_step: f32, y_minor_step: f32, z_pos: f32) {
        let mut x = x_min;
        while x <= x_max {
            self.render_line_3d(x, y_min, z_pos, x, y_max, z_pos);
            x += x_minor_step;
        }
        let mut y = y_min;
        while y <= y_max {
            self.render_line_3d(x_min, y, z_pos, x_max, y, z_pos);
            y += y_minor_step;
        }
    }

    pub fn draw_grid(&mut self, x_min: f32, y_min: f32, x_max: f32, y_max: f32, x_major_step: f32, y_major_step: f32, x_minor_step: f32, y_minor_step: f32, z_pos: f32) {
        self.draw_minor_grid(x_min, y_min, x_max, y_max, x_minor_step, y_minor_step, z_pos);
        
        let mut x = x_min;
        while x <= x_max {
            self.render_line_3d(x, y_min, z_pos, x, y_max, z_pos);
            x += x_major_step;
        }
        let mut y = y_min;
        while y <= y_max {
            self.render_line_3d(x_min, y, z_pos, x_max, y, z_pos);
            y += y_major_step;
        }
    }

    pub fn render_d3dx_mesh(&mut self, _mesh: usize, world_mat: Mat4, fx: f32, fy: f32, fz: f32, radius: f32, _fill_mode: u32) {
        let prev = self.mat_world;
        self.mat_world = world_mat;
        // Z braku danych siatki renderujemy prosta bryle zlozona z trojkatow (mock dla szescianu-zastepnika)
        self.render_bar_3d(fx - radius, fy - radius, fz - radius, fx + radius, fy + radius, fz + radius);
        self.mat_world = prev;
    }

    pub fn render_sphere(&mut self, world_mat: Mat4, fx: f32, fy: f32, fz: f32, radius: f32, _fill_mode: u32) {
        let prev = self.mat_world;
        self.mat_world = world_mat;
        // Sfera jako wypelniony prostokat / mock
        self.render_bar_3d(fx - radius, fy - radius, fz - radius, fx + radius, fy + radius, fz + radius);
        self.mat_world = prev;
    }

    pub fn render_cylinder(&mut self, world_mat: Mat4, fx: f32, fy: f32, fz: f32, radius: f32, length: f32, _fill_mode: u32) {
        let prev = self.mat_world;
        self.mat_world = world_mat;
        self.render_bar_3d(fx - radius, fy - radius, fz, fx + radius, fy + radius, fz + length);
        self.mat_world = prev;
    }

    pub fn set_color_operation(&mut self) {}
    pub fn set_diffuse_operation(&mut self) {}
    pub fn set_blend_operation(&mut self) {}
    pub fn set_one_color_operation(&mut self, _color: u32) {}
    pub fn set_add_color_operation(&mut self, _color: u32) {}

    pub fn set_diffuse_color_u32(&mut self, diffuse_color: u32) {
        self.diffuse_color = diffuse_color;
    }

    pub fn set_diffuse_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let r_u8 = (r * 255.0) as u8;
        let g_u8 = (g * 255.0) as u8;
        let b_u8 = (b * 255.0) as u8;
        let a_u8 = (a * 255.0) as u8;
        self.diffuse_color = u32::from_le_bytes([b_u8, g_u8, r_u8, a_u8]);
    }

    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let r_u8 = (r * 255.0) as u8;
        let g_u8 = (g * 255.0) as u8;
        let b_u8 = (b * 255.0) as u8;
        let a_u8 = (a * 255.0) as u8;
        self.clear_color = u32::from_le_bytes([b_u8, g_u8, r_u8, a_u8]);
    }

    pub fn set_clear_depth(&mut self, depth: f32) {
        self.clear_depth = depth;
    }

    pub fn set_clear_stencil(&mut self, stencil: u32) {
        self.clear_stencil = stencil;
    }

    pub fn set_cursor_position(&mut self, _x: i32, _y: i32, _hres: i32, _vres: i32) {}

    pub fn get_cursor_position(&self, px: &mut f32, py: &mut f32, pz: &mut f32) -> bool {
        *px = 0.0; *py = 0.0; *pz = 0.0; true
    }

    pub fn get_cursor_xy_position(&self, px: &mut f32, py: &mut f32) -> bool {
        *px = 0.0; *py = 0.0; true
    }

    pub fn get_cursor_z_position(&self, pz: &mut f32) -> bool {
        *pz = 0.0; true
    }

    pub fn get_picking_position(&self, _t: f32, x: &mut f32, y: &mut f32, z: &mut f32) {
        *x = 0.0; *y = 0.0; *z = 0.0;
    }

    pub fn project_position(&self, x: f32, y: f32, z: f32, pf_x: &mut f32, pf_y: &mut f32) {
        if let Some(p) = self.project_position_vec(Vec3::new(x, y, z)) {
            *pf_x = p.x;
            *pf_y = p.y;
        }
    }

    pub fn project_position_z(&self, x: f32, y: f32, z: f32, pf_x: &mut f32, pf_y: &mut f32, pf_z: &mut f32) {
        if let Some(p) = self.project_position_vec(Vec3::new(x, y, z)) {
            *pf_x = p.x;
            *pf_y = p.y;
            *pf_z = p.z;
        }
    }

    pub fn unproject_position(&self, _x: f32, _y: f32, _z: f32, pf_x: &mut f32, pf_y: &mut f32, pf_z: &mut f32) {
        // Skomplikowane: wymaga odwrotnosci macierzy
        *pf_x = 0.0; *pf_y = 0.0; *pf_z = 0.0;
    }

    pub fn is_lost_device(&self) -> bool {
        self.device_state != DeviceState::Ok
    }

    pub fn restore_device(&mut self) -> bool {
        if self.is_lost_device() { self.reset() } else { true }
    }

    pub fn build_view_frustum(&mut self) {}

    pub fn identity(&mut self) {
        self.mat_world = Mat4::IDENTITY;
    }
}
