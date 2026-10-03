#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputCoordinateScaler {
    pub physical: Resolution,
    pub logical: Resolution,
}

impl InputCoordinateScaler {
    pub fn new(physical: Resolution, logical: Resolution) -> Self {
        Self { physical, logical }
    }

    pub fn set_physical_resolution(&mut self, width: u32, height: u32) {
        self.physical = Resolution { width, height };
    }

    pub fn set_logical_resolution(&mut self, width: u32, height: u32) {
        self.logical = Resolution { width, height };
    }

    pub fn scale_to_logical(&self, physical_x: i32, physical_y: i32) -> (i32, i32) {
        if self.physical.width == 0 || self.physical.height == 0 {
            return (physical_x, physical_y);
        }

        let scale_x = self.logical.width as f64 / self.physical.width as f64;
        let scale_y = self.logical.height as f64 / self.physical.height as f64;

        let logical_x = (physical_x as f64 * scale_x).round() as i32;
        let logical_y = (physical_y as f64 * scale_y).round() as i32;

        (logical_x, logical_y)
    }

    pub fn scale_to_physical(&self, logical_x: i32, logical_y: i32) -> (i32, i32) {
        if self.logical.width == 0 || self.logical.height == 0 {
            return (logical_x, logical_y);
        }

        let scale_x = self.physical.width as f64 / self.logical.width as f64;
        let scale_y = self.physical.height as f64 / self.logical.height as f64;

        let physical_x = (logical_x as f64 * scale_x).round() as i32;
        let physical_y = (logical_y as f64 * scale_y).round() as i32;

        (physical_x, physical_y)
    }
}
