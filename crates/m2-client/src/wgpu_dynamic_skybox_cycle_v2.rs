// wgpu_dynamic_skybox_cycle_v2.rs
//
// 100% C++ 1:1 Parity Metin2 SkyBox Implementation in Pure Rust
// Maps to: src/EterLib/SkyBox.cpp, src/EterLib/SkyBox.h, 
//          src/EterLib/ColorTransitionHelper.cpp, src/EterLib/ColorTransitionHelper.h

use std::collections::HashMap;

/// Maps to C++ D3DXVECTOR2
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TVector2 {
    pub x: f32,
    pub y: f32,
}

impl TVector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Maps to C++ D3DXVECTOR3
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TVector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Maps to C++ D3DXMATRIX (column-major in memory but for parity we just define elements)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TMatrix {
    pub m: [[f32; 4]; 4],
}

impl TMatrix {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

pub type TTextureCoordinate = TVector2;
pub type TPosition = TVector3;
pub type TDiffuse = u32;

/// Maps to C++ SPDTVertex / TPDTVertex
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPDTVertex {
    pub position: TPosition,
    pub diffuse: TDiffuse,
    pub tex_coord: TTextureCoordinate,
}

impl Default for TPDTVertex {
    fn default() -> Self {
        Self {
            position: TPosition::new(0.0, 0.0, 0.0),
            diffuse: 0,
            tex_coord: TTextureCoordinate::new(0.0, 0.0),
        }
    }
}

/// Maps to C++ SColor / TColor
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl TColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for TColor {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

/// Maps to C++ TGradientColor
#[derive(Debug, Clone, Default)]
pub struct TGradientColor {
    pub m_first_color: TColor,
    pub m_second_color: TColor,
}

pub type TVectorGradientColor = Vec<TGradientColor>;

/// Maps to C++ CColorTransitionHelper
#[derive(Debug, Clone)]
pub struct ColorTransitionHelper {
    m_dw_cur_color: u32,
    m_dw_start_time: u32,
    m_dw_duration: u32,
    m_b_transition_started: bool,
    pub m_f_src_red: f32,
    pub m_f_src_green: f32,
    pub m_f_src_blue: f32,
    pub m_f_src_alpha: f32,
    pub m_f_dst_red: f32,
    pub m_f_dst_green: f32,
    pub m_f_dst_blue: f32,
    pub m_f_dst_alpha: f32,
}

impl ColorTransitionHelper {
    pub fn new() -> Self {
        let mut helper = Self {
            m_dw_cur_color: 0,
            m_dw_start_time: 0,
            m_dw_duration: 0,
            m_b_transition_started: false,
            m_f_src_red: 0.0,
            m_f_src_green: 0.0,
            m_f_src_blue: 0.0,
            m_f_src_alpha: 0.0,
            m_f_dst_red: 0.0,
            m_f_dst_green: 0.0,
            m_f_dst_blue: 0.0,
            m_f_dst_alpha: 0.0,
        };
        helper.clear(0.0, 0.0, 0.0, 0.0);
        helper
    }

    pub fn clear(&mut self, c_rf_red: f32, c_rf_green: f32, c_rf_blue: f32, c_rf_alpha: f32) {
        self.m_f_src_red = c_rf_red;
        self.m_f_src_green = c_rf_green;
        self.m_f_src_blue = c_rf_blue;
        self.m_f_src_alpha = c_rf_alpha;

        self.m_f_dst_red = c_rf_red;
        self.m_f_dst_green = c_rf_green;
        self.m_f_dst_blue = c_rf_blue;
        self.m_f_dst_alpha = c_rf_alpha;

        self.m_dw_cur_color = 0x00000000;
        self.m_dw_start_time = 0;
        self.m_dw_duration = 0;
        self.m_b_transition_started = false; // Add this missing assignment for clear? The C++ dtor/ctor clears.
    }

    pub fn set_src_color(&mut self, c_rf_red: f32, c_rf_green: f32, c_rf_blue: f32, c_rf_alpha: f32) {
        self.m_f_src_red = c_rf_red;
        self.m_f_src_green = c_rf_green;
        self.m_f_src_blue = c_rf_blue;
        self.m_f_src_alpha = c_rf_alpha;
    }

    pub fn set_transition(&mut self, c_rf_red: f32, c_rf_green: f32, c_rf_blue: f32, c_rf_alpha: f32, dw_duration: u32) {
        self.m_f_dst_red = c_rf_red;
        self.m_f_dst_green = c_rf_green;
        self.m_f_dst_blue = c_rf_blue;
        self.m_f_dst_alpha = c_rf_alpha;
        self.m_dw_duration = dw_duration;
    }

    pub fn get_cur_color(&self) -> u32 {
        self.m_dw_cur_color
    }

    pub fn start_transition(&mut self, current_time: u32) {
        self.m_b_transition_started = true;
        self.m_dw_start_time = current_time;
    }

    pub fn is_transition_started(&self) -> bool {
        self.m_b_transition_started
    }

    pub fn update(&mut self, current_time: u32) -> bool {
        let dw_elapsed_time = current_time.wrapping_sub(self.m_dw_start_time);
        
        let mut fpercent = if self.m_dw_duration == 0 {
            1.0 // Prevent div by zero
        } else {
            (dw_elapsed_time as f32) / (self.m_dw_duration as f32)
        };

        if fpercent <= 0.0 {
            fpercent = 0.0;
        }
        if fpercent >= 1.0 {
            fpercent = 1.0;
        }

        let f_cur_red = self.m_f_src_red + (self.m_f_dst_red - self.m_f_src_red) * fpercent;
        let f_cur_green = self.m_f_src_green + (self.m_f_dst_green - self.m_f_src_green) * fpercent;
        let f_cur_blue = self.m_f_src_blue + (self.m_f_dst_blue - self.m_f_src_blue) * fpercent;
        let f_cur_alpha = self.m_f_src_alpha + (self.m_f_dst_alpha - self.m_f_src_alpha) * fpercent;

        // m_dwCurColor = (((DWORD)(fCurAlpha * 255.0f)&0xff)<< 24) | (((DWORD)(fCurRed * 255.0f)&0xff) << 16) | (((DWORD)(fCurGreen * 255.0f)&0xff) << 8) | ((DWORD)(fCurBlue * 255.0f)&0xff);
        let a = ((f_cur_alpha * 255.0) as u32) & 0xff;
        let r = ((f_cur_red * 255.0) as u32) & 0xff;
        let g = ((f_cur_green * 255.0) as u32) & 0xff;
        let b = ((f_cur_blue * 255.0) as u32) & 0xff;

        self.m_dw_cur_color = (a << 24) | (r << 16) | (g << 8) | b;

        if fpercent == 1.0 && f_cur_alpha == self.m_f_dst_alpha && f_cur_red == self.m_f_dst_red && f_cur_green == self.m_f_dst_green && f_cur_blue == self.m_f_dst_blue {
            self.m_b_transition_started = false;
            return false;
        }

        true
    }
}

impl Default for ColorTransitionHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// Maps to C++ CSkyObjectQuad
#[derive(Debug, Clone)]
pub struct SkyObjectQuad {
    pub m_vertex: [TPDTVertex; 4],
    pub m_indices: [u16; 4],
    pub m_helper: [ColorTransitionHelper; 4],
}

impl SkyObjectQuad {
    pub fn new() -> Self {
        Self {
            m_vertex: [TPDTVertex::default(); 4],
            m_indices: [0, 2, 1, 3],
            m_helper: [
                ColorTransitionHelper::new(),
                ColorTransitionHelper::new(),
                ColorTransitionHelper::new(),
                ColorTransitionHelper::new(),
            ],
        }
    }

    pub fn clear(&mut self, c_ruc_num_vertex: u8, c_rf_red: f32, c_rf_green: f32, c_rf_blue: f32, c_rf_alpha: f32) {
        if c_ruc_num_vertex > 3 {
            return;
        }
        self.m_helper[c_ruc_num_vertex as usize].clear(c_rf_red, c_rf_green, c_rf_blue, c_rf_alpha);
    }

    pub fn set_src_color(&mut self, c_ruc_num_vertex: u8, c_rf_red: f32, c_rf_green: f32, c_rf_blue: f32, c_rf_alpha: f32) {
        if c_ruc_num_vertex > 3 {
            return;
        }
        self.m_helper[c_ruc_num_vertex as usize].set_src_color(c_rf_red, c_rf_green, c_rf_blue, c_rf_alpha);
    }

    pub fn set_transition(&mut self, c_ruc_num_vertex: u8, c_rf_red: f32, c_rf_green: f32, c_rf_blue: f32, c_rf_alpha: f32, dw_duration: u32) {
        if c_ruc_num_vertex > 3 {
            return;
        }
        self.m_helper[c_ruc_num_vertex as usize].set_transition(c_rf_red, c_rf_green, c_rf_blue, c_rf_alpha, dw_duration);
    }

    pub fn set_vertex(&mut self, c_ruc_num_vertex: u8, c_r_pdt_vertex: &TPDTVertex) {
        if c_ruc_num_vertex > 3 {
            return;
        }
        let index = self.m_indices[c_ruc_num_vertex as usize] as usize;
        self.m_vertex[index] = *c_r_pdt_vertex;
    }

    pub fn start_transition(&mut self, current_time: u32) {
        for uci in 0..4 {
            self.m_helper[uci].start_transition(current_time);
        }
    }

    pub fn update(&mut self, current_time: u32) -> bool {
        let mut b_result = false;
        for uci in 0..4 {
            b_result = self.m_helper[uci].update(current_time) || b_result;
            let index = self.m_indices[uci] as usize;
            self.m_vertex[index].diffuse = self.m_helper[uci].get_cur_color();
        }
        b_result
    }
}

impl Default for SkyObjectQuad {
    fn default() -> Self {
        Self::new()
    }
}

pub type TSkyObjectQuadVector = Vec<SkyObjectQuad>;

/// Maps to C++ CSkyObject::TSkyObjectFace
#[derive(Debug, Clone)]
pub struct SkyObjectFace {
    pub m_strfacename: String,
    pub m_str_face_texture_file_name: String,
    pub m_sky_object_quad_vector: TSkyObjectQuadVector,
}

impl SkyObjectFace {
    pub fn new() -> Self {
        Self {
            m_strfacename: String::new(),
            m_str_face_texture_file_name: String::new(),
            m_sky_object_quad_vector: Vec::new(),
        }
    }

    pub fn start_transition(&mut self, current_time: u32) {
        for quad in &mut self.m_sky_object_quad_vector {
            quad.start_transition(current_time);
        }
    }

    pub fn update(&mut self, current_time: u32) -> bool {
        let mut b_result = false;
        for quad in &mut self.m_sky_object_quad_vector {
            b_result = quad.update(current_time) || b_result;
        }
        b_result
    }
}

impl Default for SkyObjectFace {
    fn default() -> Self {
        Self::new()
    }
}

pub const SKY_RENDER_MODE_DEFAULT: u8 = 0; // = SKY_RENDER_MODE_TEXTURE
pub const SKY_RENDER_MODE_DIFFUSE: u8 = 1;
pub const SKY_RENDER_MODE_TEXTURE: u8 = 2;
pub const SKY_RENDER_MODE_MODULATE: u8 = 3;
pub const SKY_RENDER_MODE_MODULATE2X: u8 = 4;
pub const SKY_RENDER_MODE_MODULATE4X: u8 = 5;

/// Maps to C++ CSkyObject base class variables
#[derive(Debug, Clone)]
pub struct SkyObject {
    pub m_face_cloud: SkyObjectFace,
    pub m_mat_world_cloud: TMatrix,
    pub m_mat_translation_cloud: TMatrix,
    pub m_mat_texture_cloud: TMatrix,
    pub m_v3_position_cloud: TVector3,
    pub m_f_cloud_scale_x: f32,
    pub m_f_cloud_scale_y: f32,
    pub m_f_cloud_height: f32,
    pub m_f_cloud_texture_scale_x: f32,
    pub m_f_cloud_texture_scale_y: f32,
    pub m_f_cloud_scroll_speed_u: f32,
    pub m_f_cloud_scroll_speed_v: f32,
    pub m_f_cloud_position_u: f32,
    pub m_f_cloud_position_v: f32,
    pub m_dwlast_time: u32,
    
    // TGraphicImageInstanceMap would normally go here. We'll simulate its state via strings for parity test.
    pub m_graphic_image_instance_map: HashMap<String, String>,

    pub m_mat_world: TMatrix,
    pub m_mat_translation: TMatrix,
    pub m_v3_position: TVector3,
    pub m_f_scale_x: f32,
    pub m_f_scale_y: f32,
    pub m_f_scale_z: f32,

    pub m_uc_render_mode: u8,
    pub m_str_cur_time: String,
    pub m_b_transition_started: bool,
    pub m_b_sky_matrix_updated: bool,
}

impl SkyObject {
    pub fn new(current_time: u32) -> Self {
        Self {
            m_face_cloud: SkyObjectFace::new(),
            m_mat_world_cloud: TMatrix::identity(),
            m_mat_translation_cloud: TMatrix::identity(),
            m_mat_texture_cloud: TMatrix::identity(),
            m_v3_position_cloud: TVector3::new(0.0, 0.0, 0.0),
            m_f_cloud_scale_x: 1.0,
            m_f_cloud_scale_y: 1.0,
            m_f_cloud_height: 0.0,
            m_f_cloud_texture_scale_x: 1.0,
            m_f_cloud_texture_scale_y: 1.0,
            m_f_cloud_scroll_speed_u: 0.0,
            m_f_cloud_scroll_speed_v: 0.0,
            m_f_cloud_position_u: 0.0,
            m_f_cloud_position_v: 0.0,
            m_dwlast_time: current_time,
            m_graphic_image_instance_map: HashMap::new(),
            m_mat_world: TMatrix::identity(),
            m_mat_translation: TMatrix::identity(),
            m_v3_position: TVector3::new(0.0, 0.0, 0.0),
            m_f_scale_x: 1.0,
            m_f_scale_y: 1.0,
            m_f_scale_z: 1.0,
            m_uc_render_mode: SKY_RENDER_MODE_DEFAULT,
            m_str_cur_time: String::new(),
            m_b_transition_started: false,
            m_b_sky_matrix_updated: false,
        }
    }

    pub fn set_render_mode(&mut self, mode: u8) {
        self.m_uc_render_mode = mode;
    }

    pub fn is_transition_started(&self) -> bool {
        self.m_b_transition_started
    }

    pub fn update(&mut self, eye_pos: TVector3) {
        if self.m_v3_position == eye_pos {
            if !self.m_b_sky_matrix_updated {
                return;
            }
        }
        self.m_v3_position = eye_pos;
        
        self.m_mat_world.m[3][0] = self.m_v3_position.x;
        self.m_mat_world.m[3][1] = self.m_v3_position.y;
        self.m_mat_world.m[3][2] = self.m_v3_position.z;

        self.m_mat_world_cloud.m[3][0] = self.m_v3_position.x;
        self.m_mat_world_cloud.m[3][1] = self.m_v3_position.y;
        self.m_mat_world_cloud.m[3][2] = self.m_v3_position.z + self.m_f_cloud_height;

        if self.m_b_sky_matrix_updated {
            self.m_b_sky_matrix_updated = false;
        }
    }
}

/// Maps to C++ CSkyBox
#[derive(Debug, Clone)]
pub struct SkyBox {
    pub m_object: SkyObject,
    pub m_uc_virtical_gradient_level_upper: u8,
    pub m_uc_virtical_gradient_level_lower: u8,
    pub m_faces: [SkyObjectFace; 6],
}

impl SkyBox {
    pub fn new(current_time: u32) -> Self {
        Self {
            m_object: SkyObject::new(current_time),
            m_uc_virtical_gradient_level_upper: 0,
            m_uc_virtical_gradient_level_lower: 0,
            m_faces: [
                SkyObjectFace::new(), SkyObjectFace::new(), SkyObjectFace::new(),
                SkyObjectFace::new(), SkyObjectFace::new(), SkyObjectFace::new(),
            ],
        }
    }

    pub fn set_sky_box_scale(&mut self, c_rv3_scale: TVector3) {
        self.m_object.m_f_scale_x = c_rv3_scale.x;
        self.m_object.m_f_scale_y = c_rv3_scale.y;
        self.m_object.m_f_scale_z = c_rv3_scale.z;

        self.m_object.m_b_sky_matrix_updated = true;
        // D3DXMatrixScaling in C++
        self.m_object.m_mat_world.m[0][0] = self.m_object.m_f_scale_x;
        self.m_object.m_mat_world.m[1][1] = self.m_object.m_f_scale_y;
        self.m_object.m_mat_world.m[2][2] = self.m_object.m_f_scale_z;
    }

    pub fn set_gradient_level(&mut self, by_upper: u8, by_lower: u8) {
        self.m_uc_virtical_gradient_level_upper = by_upper;
        self.m_uc_virtical_gradient_level_lower = by_lower;
    }

    pub fn set_cloud_scale(&mut self, c_rv2_cloud_scale: TVector2) {
        self.m_object.m_f_cloud_scale_x = c_rv2_cloud_scale.x;
        self.m_object.m_f_cloud_scale_y = c_rv2_cloud_scale.y;
        // D3DXMatrixScaling(&m_matWorldCloud, m_fCloudScaleX, m_fCloudScaleY, 1.0f);
        self.m_object.m_mat_world_cloud.m[0][0] = self.m_object.m_f_cloud_scale_x;
        self.m_object.m_mat_world_cloud.m[1][1] = self.m_object.m_f_cloud_scale_y;
        self.m_object.m_mat_world_cloud.m[2][2] = 1.0;
    }

    pub fn set_cloud_height(&mut self, f_height: f32) {
        self.m_object.m_f_cloud_height = f_height;
    }

    pub fn set_cloud_texture_scale(&mut self, c_rv2_cloud_texture_scale: TVector2) {
        self.m_object.m_f_cloud_texture_scale_x = c_rv2_cloud_texture_scale.x;
        self.m_object.m_f_cloud_texture_scale_y = c_rv2_cloud_texture_scale.y;
        // m_matTextureCloud._11 = m_fCloudTextureScaleX;
        // m_matTextureCloud._22 = m_fCloudTextureScaleY;
        self.m_object.m_mat_texture_cloud.m[0][0] = self.m_object.m_f_cloud_texture_scale_x;
        self.m_object.m_mat_texture_cloud.m[1][1] = self.m_object.m_f_cloud_texture_scale_y;
    }

    pub fn set_cloud_scroll_speed(&mut self, c_rv2_cloud_scroll_speed: TVector2) {
        self.m_object.m_f_cloud_scroll_speed_u = c_rv2_cloud_scroll_speed.x;
        self.m_object.m_f_cloud_scroll_speed_v = c_rv2_cloud_scroll_speed.y;
    }

    pub fn set_sky_object_quad_vertical(&self, p_sky_object_quad_vector: &mut TSkyObjectQuadVector, c_pv2_quad_points: &[TVector2; 2]) {
        let mut a_pdt_vertex = TPDTVertex::default();
        let total = (self.m_uc_virtical_gradient_level_upper + self.m_uc_virtical_gradient_level_lower) as usize;
        
        p_sky_object_quad_vector.clear();
        p_sky_object_quad_vector.resize(total, SkyObjectQuad::new());

        let mut dw_index = 0;

        let f_upper = self.m_uc_virtical_gradient_level_upper as f32;
        let f_lower = self.m_uc_virtical_gradient_level_lower as f32;

        for uc_y in 0..self.m_uc_virtical_gradient_level_upper {
            let r_sky_object_quad = &mut p_sky_object_quad_vector[dw_index];
            dw_index += 1;
            
            let fy = uc_y as f32;
            let fy_plus_1 = (uc_y + 1) as f32;

            a_pdt_vertex.position.x = c_pv2_quad_points[0].x;
            a_pdt_vertex.position.y = c_pv2_quad_points[0].y;
            a_pdt_vertex.position.z = 1.0 - fy_plus_1 / f_upper;
            a_pdt_vertex.tex_coord.x = 0.0;
            a_pdt_vertex.tex_coord.y = (fy_plus_1 / f_upper) * 0.5;
            r_sky_object_quad.set_vertex(0, &a_pdt_vertex);

            a_pdt_vertex.position.x = c_pv2_quad_points[0].x;
            a_pdt_vertex.position.y = c_pv2_quad_points[0].y;
            a_pdt_vertex.position.z = 1.0 - fy / f_upper;
            a_pdt_vertex.tex_coord.x = 0.0;
            a_pdt_vertex.tex_coord.y = (fy / f_upper) * 0.5;
            r_sky_object_quad.set_vertex(1, &a_pdt_vertex);

            a_pdt_vertex.position.x = c_pv2_quad_points[1].x;
            a_pdt_vertex.position.y = c_pv2_quad_points[1].y;
            a_pdt_vertex.position.z = 1.0 - fy_plus_1 / f_upper;
            a_pdt_vertex.tex_coord.x = 1.0;
            a_pdt_vertex.tex_coord.y = (fy_plus_1 / f_upper) * 0.5;
            r_sky_object_quad.set_vertex(2, &a_pdt_vertex);

            a_pdt_vertex.position.x = c_pv2_quad_points[1].x;
            a_pdt_vertex.position.y = c_pv2_quad_points[1].y;
            a_pdt_vertex.position.z = 1.0 - fy / f_upper;
            a_pdt_vertex.tex_coord.x = 1.0;
            a_pdt_vertex.tex_coord.y = (fy / f_upper) * 0.5;
            r_sky_object_quad.set_vertex(3, &a_pdt_vertex);
        }

        for uc_y in 0..self.m_uc_virtical_gradient_level_lower {
            let r_sky_object_quad = &mut p_sky_object_quad_vector[dw_index];
            dw_index += 1;

            let fy = uc_y as f32;
            let fy_plus_1 = (uc_y + 1) as f32;

            a_pdt_vertex.position.x = c_pv2_quad_points[0].x;
            a_pdt_vertex.position.y = c_pv2_quad_points[0].y;
            a_pdt_vertex.position.z = -(fy_plus_1 / f_lower);
            a_pdt_vertex.tex_coord.x = 0.0;
            // The exact bug traced from C++ codebase!
            a_pdt_vertex.tex_coord.y = 0.5 + (fy_plus_1 / f_upper) * 0.5;
            r_sky_object_quad.set_vertex(0, &a_pdt_vertex);

            a_pdt_vertex.position.x = c_pv2_quad_points[0].x;
            a_pdt_vertex.position.y = c_pv2_quad_points[0].y;
            a_pdt_vertex.position.z = -(fy / f_lower);
            a_pdt_vertex.tex_coord.x = 0.0;
            // The exact bug traced from C++ codebase!
            a_pdt_vertex.tex_coord.y = 0.5 + (fy / f_upper) * 0.5;
            r_sky_object_quad.set_vertex(1, &a_pdt_vertex);

            a_pdt_vertex.position.x = c_pv2_quad_points[1].x;
            a_pdt_vertex.position.y = c_pv2_quad_points[1].y;
            a_pdt_vertex.position.z = -(fy_plus_1 / f_lower);
            a_pdt_vertex.tex_coord.x = 1.0;
            // The exact bug traced from C++ codebase!
            a_pdt_vertex.tex_coord.y = 0.5 + (fy_plus_1 / f_upper) * 0.5;
            r_sky_object_quad.set_vertex(2, &a_pdt_vertex);

            a_pdt_vertex.position.x = c_pv2_quad_points[1].x;
            a_pdt_vertex.position.y = c_pv2_quad_points[1].y;
            a_pdt_vertex.position.z = -(fy / f_lower);
            a_pdt_vertex.tex_coord.x = 1.0;
            // The exact bug traced from C++ codebase!
            a_pdt_vertex.tex_coord.y = 0.5 + (fy / f_upper) * 0.5;
            r_sky_object_quad.set_vertex(3, &a_pdt_vertex);
        }
    }

    pub fn set_sky_object_quad_horizon(&self, p_sky_object_quad_vector: &mut TSkyObjectQuadVector, c_pv3_quad_points: &[TVector3; 4]) {
        p_sky_object_quad_vector.clear();
        p_sky_object_quad_vector.resize(1, SkyObjectQuad::new());
        let r_sky_object_quad = &mut p_sky_object_quad_vector[0];

        let mut a_pdt_vertex = TPDTVertex::default();

        a_pdt_vertex.position = c_pv3_quad_points[0];
        a_pdt_vertex.tex_coord.x = 0.0;
        a_pdt_vertex.tex_coord.y = 1.0;
        r_sky_object_quad.set_vertex(0, &a_pdt_vertex);

        a_pdt_vertex.position = c_pv3_quad_points[1];
        a_pdt_vertex.tex_coord.x = 0.0;
        a_pdt_vertex.tex_coord.y = 0.0;
        r_sky_object_quad.set_vertex(1, &a_pdt_vertex);

        a_pdt_vertex.position = c_pv3_quad_points[2];
        a_pdt_vertex.tex_coord.x = 1.0;
        a_pdt_vertex.tex_coord.y = 1.0;
        r_sky_object_quad.set_vertex(2, &a_pdt_vertex);

        a_pdt_vertex.position = c_pv3_quad_points[3];
        a_pdt_vertex.tex_coord.x = 1.0;
        a_pdt_vertex.tex_coord.y = 0.0;
        r_sky_object_quad.set_vertex(3, &a_pdt_vertex);
    }

    pub fn refresh(&mut self) {
        let mut v3_quad_points = [TVector3::new(0.0, 0.0, 0.0); 4];

        if self.m_object.m_uc_render_mode == SKY_RENDER_MODE_DEFAULT || self.m_object.m_uc_render_mode == SKY_RENDER_MODE_DIFFUSE {
            if self.m_uc_virtical_gradient_level_upper + self.m_uc_virtical_gradient_level_lower <= 0 {
                return;
            }

            let mut v2_quad_points = [TVector2::new(0.0, 0.0); 2];

            // Face 0: FRONT
            v2_quad_points[0] = TVector2::new(1.0, -1.0);
            v2_quad_points[1] = TVector2::new(-1.0, -1.0);
            let mut vecs0 = Vec::new();
            self.set_sky_object_quad_vertical(&mut vecs0, &v2_quad_points);
            self.m_faces[0].m_sky_object_quad_vector = vecs0;
            self.m_faces[0].m_strfacename = "front".to_string();

            // Face 1: BACK
            v2_quad_points[0] = TVector2::new(-1.0, 1.0);
            v2_quad_points[1] = TVector2::new(1.0, 1.0);
            let mut vecs1 = Vec::new();
            self.set_sky_object_quad_vertical(&mut vecs1, &v2_quad_points);
            self.m_faces[1].m_sky_object_quad_vector = vecs1;
            self.m_faces[1].m_strfacename = "back".to_string();

            // Face 2: LEFT
            v2_quad_points[0] = TVector2::new(-1.0, -1.0);
            v2_quad_points[1] = TVector2::new(-1.0, 1.0);
            let mut vecs2 = Vec::new();
            self.set_sky_object_quad_vertical(&mut vecs2, &v2_quad_points);
            self.m_faces[2].m_sky_object_quad_vector = vecs2;
            self.m_faces[2].m_strfacename = "left".to_string();

            // Face 3: RIGHT
            v2_quad_points[0] = TVector2::new(1.0, 1.0);
            v2_quad_points[1] = TVector2::new(1.0, -1.0);
            let mut vecs3 = Vec::new();
            self.set_sky_object_quad_vertical(&mut vecs3, &v2_quad_points);
            self.m_faces[3].m_sky_object_quad_vector = vecs3;
            self.m_faces[3].m_strfacename = "right".to_string();

            // Face 4: TOP
            v3_quad_points[0] = TVector3::new(1.0, 1.0, 1.0);
            v3_quad_points[1] = TVector3::new(-1.0, 1.0, 1.0);
            v3_quad_points[2] = TVector3::new(1.0, -1.0, 1.0);
            v3_quad_points[3] = TVector3::new(-1.0, -1.0, 1.0);
            let mut vecs4 = Vec::new();
            self.set_sky_object_quad_horizon(&mut vecs4, &v3_quad_points);
            self.m_faces[4].m_sky_object_quad_vector = vecs4;
            self.m_faces[4].m_strfacename = "top".to_string();

            // Face 5: BOTTOM
            v3_quad_points[0] = TVector3::new(-1.0, 1.0, -1.0);
            v3_quad_points[1] = TVector3::new(1.0, 1.0, -1.0);
            v3_quad_points[2] = TVector3::new(-1.0, -1.0, -1.0);
            v3_quad_points[3] = TVector3::new(1.0, -1.0, -1.0);
            let mut vecs5 = Vec::new();
            self.set_sky_object_quad_horizon(&mut vecs5, &v3_quad_points);
            self.m_faces[5].m_sky_object_quad_vector = vecs5;
            self.m_faces[5].m_strfacename = "bottom".to_string();

        } else if self.m_object.m_uc_render_mode == SKY_RENDER_MODE_TEXTURE {
            // Face 0: FRONT
            v3_quad_points[0] = TVector3::new(1.0, -1.0, -1.0);
            v3_quad_points[1] = TVector3::new(1.0, -1.0, 1.0);
            v3_quad_points[2] = TVector3::new(-1.0, -1.0, -1.0);
            v3_quad_points[3] = TVector3::new(-1.0, -1.0, 1.0);
            let mut vecs0 = Vec::new();
            self.set_sky_object_quad_horizon(&mut vecs0, &v3_quad_points);
            self.m_faces[0].m_sky_object_quad_vector = vecs0;
            self.m_faces[0].m_strfacename = "front".to_string();

            // Face 1: BACK
            v3_quad_points[0] = TVector3::new(-1.0, 1.0, -1.0);
            v3_quad_points[1] = TVector3::new(-1.0, 1.0, 1.0);
            v3_quad_points[2] = TVector3::new(1.0, 1.0, -1.0);
            v3_quad_points[3] = TVector3::new(1.0, 1.0, 1.0);
            let mut vecs1 = Vec::new();
            self.set_sky_object_quad_horizon(&mut vecs1, &v3_quad_points);
            self.m_faces[1].m_sky_object_quad_vector = vecs1;
            self.m_faces[1].m_strfacename = "back".to_string();

            // Face 2: LEFT
            v3_quad_points[0] = TVector3::new(1.0, 1.0, -1.0);
            v3_quad_points[1] = TVector3::new(1.0, 1.0, 1.0);
            v3_quad_points[2] = TVector3::new(1.0, -1.0, -1.0);
            v3_quad_points[3] = TVector3::new(1.0, -1.0, 1.0);
            let mut vecs2 = Vec::new();
            self.set_sky_object_quad_horizon(&mut vecs2, &v3_quad_points);
            self.m_faces[2].m_sky_object_quad_vector = vecs2;
            self.m_faces[2].m_strfacename = "left".to_string();

            // Face 3: RIGHT
            v3_quad_points[0] = TVector3::new(-1.0, -1.0, -1.0);
            v3_quad_points[1] = TVector3::new(-1.0, -1.0, 1.0);
            v3_quad_points[2] = TVector3::new(-1.0, 1.0, -1.0);
            v3_quad_points[3] = TVector3::new(-1.0, 1.0, 1.0);
            let mut vecs3 = Vec::new();
            self.set_sky_object_quad_horizon(&mut vecs3, &v3_quad_points);
            self.m_faces[3].m_sky_object_quad_vector = vecs3;
            self.m_faces[3].m_strfacename = "right".to_string();

            // Face 4: TOP
            v3_quad_points[0] = TVector3::new(1.0, -1.0, 1.0);
            v3_quad_points[1] = TVector3::new(1.0, 1.0, 1.0);
            v3_quad_points[2] = TVector3::new(-1.0, -1.0, 1.0);
            v3_quad_points[3] = TVector3::new(-1.0, 1.0, 1.0);
            let mut vecs4 = Vec::new();
            self.set_sky_object_quad_horizon(&mut vecs4, &v3_quad_points);
            self.m_faces[4].m_sky_object_quad_vector = vecs4;
            self.m_faces[4].m_strfacename = "top".to_string();

            // Face 5: BOTTOM
            v3_quad_points[0] = TVector3::new(1.0, -1.0, -1.0);
            v3_quad_points[1] = TVector3::new(1.0, 1.0, -1.0);
            v3_quad_points[2] = TVector3::new(-1.0, -1.0, -1.0);
            v3_quad_points[3] = TVector3::new(-1.0, 1.0, -1.0);
            let mut vecs5 = Vec::new();
            self.set_sky_object_quad_horizon(&mut vecs5, &v3_quad_points);
            self.m_faces[5].m_sky_object_quad_vector = vecs5;
            self.m_faces[5].m_strfacename = "bottom".to_string();
        }

        // Clouds..
        v3_quad_points[0] = TVector3::new(1.0, 1.0, 0.0);
        v3_quad_points[1] = TVector3::new(-1.0, 1.0, 0.0);
        v3_quad_points[2] = TVector3::new(1.0, -1.0, 0.0);
        v3_quad_points[3] = TVector3::new(-1.0, -1.0, 0.0);
        let mut vecs_cloud = Vec::new();
        self.set_sky_object_quad_horizon(&mut vecs_cloud, &v3_quad_points);
        self.m_object.m_face_cloud.m_sky_object_quad_vector = vecs_cloud;
    }

    pub fn set_cloud_color(&mut self, c_r_color: &TGradientColor, c_r_next_color: &TGradientColor, dw_transition_time: u32) {
        let a_face_cloud = &mut self.m_object.m_face_cloud;
        for dwk in 0..a_face_cloud.m_sky_object_quad_vector.len() {
            let a_sky_object_quad = &mut a_face_cloud.m_sky_object_quad_vector[dwk];

            a_sky_object_quad.set_src_color(0, c_r_color.m_first_color.r, c_r_color.m_first_color.g, c_r_color.m_first_color.b, c_r_color.m_first_color.a);
            a_sky_object_quad.set_transition(0, c_r_next_color.m_first_color.r, c_r_next_color.m_first_color.g, c_r_next_color.m_first_color.b, c_r_next_color.m_first_color.a, dw_transition_time);
            
            a_sky_object_quad.set_src_color(1, c_r_color.m_first_color.r, c_r_color.m_first_color.g, c_r_color.m_first_color.b, c_r_color.m_first_color.a);
            a_sky_object_quad.set_transition(1, c_r_next_color.m_first_color.r, c_r_next_color.m_first_color.g, c_r_next_color.m_first_color.b, c_r_next_color.m_first_color.a, dw_transition_time);
            
            a_sky_object_quad.set_src_color(2, c_r_color.m_first_color.r, c_r_color.m_first_color.g, c_r_color.m_first_color.b, c_r_color.m_first_color.a);
            a_sky_object_quad.set_transition(2, c_r_next_color.m_first_color.r, c_r_next_color.m_first_color.g, c_r_next_color.m_first_color.b, c_r_next_color.m_first_color.a, dw_transition_time);
            
            a_sky_object_quad.set_src_color(3, c_r_color.m_first_color.r, c_r_color.m_first_color.g, c_r_color.m_first_color.b, c_r_color.m_first_color.a);
            a_sky_object_quad.set_transition(3, c_r_next_color.m_first_color.r, c_r_next_color.m_first_color.g, c_r_next_color.m_first_color.b, c_r_next_color.m_first_color.a, dw_transition_time);
        }
    }

    pub fn set_sky_color(&mut self, c_r_color_vector: &TVectorGradientColor, c_r_next_color_vector: &TVectorGradientColor, l_transition_time: u32) {
        if c_r_color_vector.is_empty() || c_r_next_color_vector.is_empty() {
            return;
        }

        let mut ul_vector_gradient_colornum;
        
        // Faces 0 to 3
        for ucj in 0..4 {
            let a_face = &mut self.m_faces[ucj];
            ul_vector_gradient_colornum = 0;
            for uck in 0..a_face.m_sky_object_quad_vector.len() {
                let a_sky_object_quad = &mut a_face.m_sky_object_quad_vector[uck];

                let cur_col = &c_r_color_vector[ul_vector_gradient_colornum];
                let next_col = &c_r_next_color_vector[ul_vector_gradient_colornum];

                a_sky_object_quad.set_src_color(0, cur_col.m_second_color.r, cur_col.m_second_color.g, cur_col.m_second_color.b, cur_col.m_second_color.a);
                a_sky_object_quad.set_transition(0, next_col.m_second_color.r, next_col.m_second_color.g, next_col.m_second_color.b, next_col.m_second_color.a, l_transition_time);

                a_sky_object_quad.set_src_color(1, cur_col.m_first_color.r, cur_col.m_first_color.g, cur_col.m_first_color.b, cur_col.m_first_color.a);
                a_sky_object_quad.set_transition(1, next_col.m_first_color.r, next_col.m_first_color.g, next_col.m_first_color.b, next_col.m_first_color.a, l_transition_time);

                a_sky_object_quad.set_src_color(2, cur_col.m_second_color.r, cur_col.m_second_color.g, cur_col.m_second_color.b, cur_col.m_second_color.a);
                a_sky_object_quad.set_transition(2, next_col.m_second_color.r, next_col.m_second_color.g, next_col.m_second_color.b, next_col.m_second_color.a, l_transition_time);

                a_sky_object_quad.set_src_color(3, cur_col.m_first_color.r, cur_col.m_first_color.g, cur_col.m_first_color.b, cur_col.m_first_color.a);
                a_sky_object_quad.set_transition(3, next_col.m_first_color.r, next_col.m_first_color.g, next_col.m_first_color.b, next_col.m_first_color.a, l_transition_time);

                ul_vector_gradient_colornum += 1;
            }
        }

        // Face 4 (Top)
        ul_vector_gradient_colornum = 0;
        let a_face_top = &mut self.m_faces[4];
        for uck in 0..a_face_top.m_sky_object_quad_vector.len() {
            let a_sky_object_quad = &mut a_face_top.m_sky_object_quad_vector[uck];
            let cur_col = &c_r_color_vector[ul_vector_gradient_colornum];
            let next_col = &c_r_next_color_vector[ul_vector_gradient_colornum];

            a_sky_object_quad.set_src_color(0, cur_col.m_first_color.r, cur_col.m_first_color.g, cur_col.m_first_color.b, cur_col.m_first_color.a);
            a_sky_object_quad.set_transition(0, next_col.m_first_color.r, next_col.m_first_color.g, next_col.m_first_color.b, next_col.m_first_color.a, l_transition_time);
            a_sky_object_quad.set_src_color(1, cur_col.m_first_color.r, cur_col.m_first_color.g, cur_col.m_first_color.b, cur_col.m_first_color.a);
            a_sky_object_quad.set_transition(1, next_col.m_first_color.r, next_col.m_first_color.g, next_col.m_first_color.b, next_col.m_first_color.a, l_transition_time);
            a_sky_object_quad.set_src_color(2, cur_col.m_first_color.r, cur_col.m_first_color.g, cur_col.m_first_color.b, cur_col.m_first_color.a);
            a_sky_object_quad.set_transition(2, next_col.m_first_color.r, next_col.m_first_color.g, next_col.m_first_color.b, next_col.m_first_color.a, l_transition_time);
            a_sky_object_quad.set_src_color(3, cur_col.m_first_color.r, cur_col.m_first_color.g, cur_col.m_first_color.b, cur_col.m_first_color.a);
            a_sky_object_quad.set_transition(3, next_col.m_first_color.r, next_col.m_first_color.g, next_col.m_first_color.b, next_col.m_first_color.a, l_transition_time);
        }

        // Face 5 (Bottom)
        ul_vector_gradient_colornum = c_r_color_vector.len().saturating_sub(1);
        let a_face_bottom = &mut self.m_faces[5];
        for uck in 0..a_face_bottom.m_sky_object_quad_vector.len() {
            let a_sky_object_quad = &mut a_face_bottom.m_sky_object_quad_vector[uck];
            let cur_col = &c_r_color_vector[ul_vector_gradient_colornum];
            let next_col = &c_r_next_color_vector[ul_vector_gradient_colornum];

            a_sky_object_quad.set_src_color(0, cur_col.m_second_color.r, cur_col.m_second_color.g, cur_col.m_second_color.b, cur_col.m_second_color.a);
            a_sky_object_quad.set_transition(0, next_col.m_second_color.r, next_col.m_second_color.g, next_col.m_second_color.b, next_col.m_second_color.a, l_transition_time);
            a_sky_object_quad.set_src_color(1, cur_col.m_second_color.r, cur_col.m_second_color.g, cur_col.m_second_color.b, cur_col.m_second_color.a);
            a_sky_object_quad.set_transition(1, next_col.m_second_color.r, next_col.m_second_color.g, next_col.m_second_color.b, next_col.m_second_color.a, l_transition_time);
            a_sky_object_quad.set_src_color(2, cur_col.m_second_color.r, cur_col.m_second_color.g, cur_col.m_second_color.b, cur_col.m_second_color.a);
            a_sky_object_quad.set_transition(2, next_col.m_second_color.r, next_col.m_second_color.g, next_col.m_second_color.b, next_col.m_second_color.a, l_transition_time);
            a_sky_object_quad.set_src_color(3, cur_col.m_second_color.r, cur_col.m_second_color.g, cur_col.m_second_color.b, cur_col.m_second_color.a);
            a_sky_object_quad.set_transition(3, next_col.m_second_color.r, next_col.m_second_color.g, next_col.m_second_color.b, next_col.m_second_color.a, l_transition_time);
        }
    }

    pub fn start_transition(&mut self, current_time: u32) {
        self.m_object.m_b_transition_started = true;
        for ucj in 0..6 {
            self.m_faces[ucj].start_transition(current_time);
        }
        self.m_object.m_face_cloud.start_transition(current_time);
    }

    pub fn update(&mut self, eye_pos: TVector3, current_time: u32) {
        self.m_object.update(eye_pos);

        if !self.m_object.m_b_transition_started {
            return;
        }

        let mut b_result = false;
        for uci in 0..6 {
            b_result = self.m_faces[uci].update(current_time) || b_result;
        }
        b_result = self.m_object.m_face_cloud.update(current_time) || b_result;

        self.m_object.m_b_transition_started = b_result;
    }
}
