//! Modul `instance_transform` — Transformacje przestrzenne i orientacja aktora
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBaseTransform.cpp`
//! Opis: Odwzorowanie NEW_GetPixelPosition, NEW_SetPixelPosition, LookAt, obliczanie odleglosci 2D/3D pomiedzy dwoma postaciami.

const DIR_MAX_NUM: usize = 8;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PixelPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_2d(&self, other: &PixelPosition) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn distance_3d(&self, other: &PixelPosition) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Default)]
pub struct InstanceTransform {
    pub initialized: bool,
    pub pixel_position: PixelPosition,
    pub rotation: f32,
    pub advancing_rotation: f32,
}

impl InstanceTransform {
    pub fn new() -> Self {
        Self {
            initialized: true,
            pixel_position: PixelPosition::default(),
            rotation: 0.0,
            advancing_rotation: 0.0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn script_set_pixel_position(&mut self, height_resolver: &impl Fn(f32, f32) -> f32, fx: f32, fy: f32) {
        let fz = height_resolver(fx, fy);
        self.new_set_pixel_position(&PixelPosition::new(fx, fy, fz));
    }

    pub fn new_set_pixel_position(&mut self, pos: &PixelPosition) {
        self.pixel_position = *pos;
    }

    pub fn new_get_pixel_position(&self) -> PixelPosition {
        self.pixel_position
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn blend_rotation(&mut self, rotation: f32, _blend_time: f32) {
        self.set_rotation(rotation);
    }

    pub fn new_look_at_fly_target(&mut self) {
    }

    pub fn new_look_at_dest_pixel_position(&mut self, dst: &PixelPosition) {
        let dx = dst.x - self.pixel_position.x;
        let dy = -(dst.y - self.pixel_position.y);
        let mut f_degree = dx.atan2(dy).to_degrees();
        if f_degree < 0.0 {
            f_degree += 360.0;
        }
        self.set_rotation(f_degree);
    }

    pub fn new_look_at_dest_instance(&mut self, other: &InstanceTransform) {
        self.new_look_at_dest_pixel_position(&other.new_get_pixel_position());
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_advancing_rotation(&self) -> f32 {
        self.advancing_rotation
    }

    pub fn set_advancing_rotation(&mut self, rotation: f32) {
        self.advancing_rotation = rotation;
    }

    pub fn set_direction(&mut self, dir: i32) {
        let f_degree = Self::get_degree_from_direction(dir);
        self.set_rotation(f_degree);
        self.set_advancing_rotation(f_degree);
    }

    pub fn blend_direction(&mut self, dir: i32, blend_time: f32) {
        self.blend_rotation(Self::get_degree_from_direction(dir), blend_time);
    }

    pub fn get_degree_from_direction(dir: i32) -> f32 {
        if dir < 0 {
            return 0.0;
        }
        if dir >= DIR_MAX_NUM as i32 {
            return 0.0;
        }
        
        let s_dir_rot: [f32; DIR_MAX_NUM] = [
            45.0 * 4.0,
            45.0 * 3.0,
            45.0 * 2.0,
            45.0,
            0.0,
            360.0 - 45.0,
            360.0 - 45.0 * 2.0,
            360.0 - 45.0 * 3.0,
        ];
        
        s_dir_rot[dir as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_instance_transform() {
        let instance = InstanceTransform::new();
        assert!(instance.is_ready());
    }
}
