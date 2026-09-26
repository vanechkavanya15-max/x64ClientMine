pub const CELL_SIZE: f32 = 200.0;

pub struct TerrainHeightMap {
    width: usize,
    height: usize,
    heights: Vec<u16>,
}

impl TerrainHeightMap {
    pub fn new(width: usize, height: usize, heights: Vec<u16>) -> Self {
        assert_eq!(width * height, heights.len(), "Wymiary siatki wysokosci nie zgadzaja sie z dlugoscia wektora.");
        Self { width, height, heights }
    }

    pub fn get_height(&self, x: f32, y: f32) -> f32 {
        let max_x = ((self.width - 1) as f32) * CELL_SIZE;
        let max_y = ((self.height - 1) as f32) * CELL_SIZE;

        let x = x.clamp(0.0, max_x);
        let y = y.clamp(0.0, max_y);

        let gx = (x / CELL_SIZE).floor();
        let gy = (y / CELL_SIZE).floor();

        let rx = (x - gx * CELL_SIZE) / CELL_SIZE;
        let ry = (y - gy * CELL_SIZE) / CELL_SIZE;

        let gx = gx as usize;
        let gy = gy as usize;

        let gx1 = (gx + 1).min(self.width - 1);
        let gy1 = (gy + 1).min(self.height - 1);

        let h00 = self.get_node_height(gx, gy);
        let h10 = self.get_node_height(gx1, gy);
        let h01 = self.get_node_height(gx, gy1);
        let h11 = self.get_node_height(gx1, gy1);

        let h0 = h00 * (1.0 - rx) + h10 * rx;
        let h1 = h01 * (1.0 - rx) + h11 * rx;
        let z = h0 * (1.0 - ry) + h1 * ry;

        z
    }

    fn get_node_height(&self, x: usize, y: usize) -> f32 {
        self.heights[y * self.width + x] as f32
    }
}
