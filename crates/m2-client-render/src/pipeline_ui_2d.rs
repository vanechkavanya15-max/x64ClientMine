//! Modul pipeline_ui_2d dla skrzynki m2-client-render
//! Potok wsadowego renderowania interfejsu 2D (UI Batcher)
//! Rzutowanie ortogonalne 2D, wsadowe rysowanie kafelkow tekstur i ikon okienkowych

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct UiVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiBatch {
    pub texture_id: Option<u64>,
    pub index_start: u32,
    pub index_count: u32,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PipelineUi2d {
    pub initialized: bool,
    pub screen_width: f32,
    pub screen_height: f32,
    pub vertices: Vec<UiVertex>,
    pub indices: Vec<u32>,
    pub batches: Vec<UiBatch>,
    
    current_texture: Option<u64>,
    current_index_start: u32,
}

impl PipelineUi2d {
    pub fn new() -> Self {
        Self {
            initialized: true,
            screen_width: 800.0,
            screen_height: 600.0,
            vertices: Vec::new(),
            indices: Vec::new(),
            batches: Vec::new(),
            current_texture: None,
            current_index_start: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_screen_size(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
    }

    pub fn begin(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.batches.clear();
        self.current_texture = None;
        self.current_index_start = 0;
    }

    pub fn end(&mut self) {
        self.flush_batch();
    }

    pub fn set_texture(&mut self, texture_id: Option<u64>) {
        if self.current_texture != texture_id {
            self.flush_batch();
            self.current_texture = texture_id;
        }
    }

    fn flush_batch(&mut self) {
        let current_index_count = self.indices.len() as u32 - self.current_index_start;
        if current_index_count > 0 {
            self.batches.push(UiBatch {
                texture_id: self.current_texture,
                index_start: self.current_index_start,
                index_count: current_index_count,
            });
            self.current_index_start = self.indices.len() as u32;
        }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        self.draw_textured_rect(x, y, w, h, [0.0, 0.0, 1.0, 1.0], color);
    }

    pub fn draw_textured_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        uv_rect: [f32; 4], // [u_min, v_min, u_max, v_max]
        color: [f32; 4],
    ) {
        let start_vertex = self.vertices.len() as u32;

        // Normowanie wspolrzednych ekranu (opcjonalne, mozna zostawic orto w shaderze)
        // W tym przypadku zakladamy rzutowanie ortogonalne w shaderze, wiec podajemy
        // pozycje w pikselach (lub ulamkach, jesli tak jest zdefiniowane, tu uzywamy pikseli)
        
        let u_min = uv_rect[0];
        let v_min = uv_rect[1];
        let u_max = uv_rect[2];
        let v_max = uv_rect[3];

        self.vertices.push(UiVertex {
            position: [x, y],
            uv: [u_min, v_min],
            color,
        });
        self.vertices.push(UiVertex {
            position: [x + w, y],
            uv: [u_max, v_min],
            color,
        });
        self.vertices.push(UiVertex {
            position: [x + w, y + h],
            uv: [u_max, v_max],
            color,
        });
        self.vertices.push(UiVertex {
            position: [x, y + h],
            uv: [u_min, v_max],
            color,
        });

        self.indices.push(start_vertex);
        self.indices.push(start_vertex + 1);
        self.indices.push(start_vertex + 2);
        
        self.indices.push(start_vertex);
        self.indices.push(start_vertex + 2);
        self.indices.push(start_vertex + 3);
    }

    pub fn vertices(&self) -> &[UiVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn batches(&self) -> &[UiBatch] {
        &self.batches
    }
}
