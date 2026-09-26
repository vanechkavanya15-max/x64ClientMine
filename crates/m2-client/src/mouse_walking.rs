use glam::{Mat4, Vec2, Vec3};

pub struct Camera {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub fn unproject_screen_to_ray(mouse_x: f32, mouse_y: f32, camera: &Camera) -> Ray {
    // Convert screen coordinates to NDC (Normalized Device Coordinates) [-1, 1]
    let ndc_x = (mouse_x / camera.viewport_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (mouse_y / camera.viewport_height) * 2.0;

    let inverse_vp = (camera.projection_matrix * camera.view_matrix).inverse();
    let near_point = inverse_vp.project_point3(Vec3::new(ndc_x, ndc_y, 0.0));
    let far_point = inverse_vp.project_point3(Vec3::new(ndc_x, ndc_y, 1.0));

    let direction = (far_point - near_point).normalize();

    Ray {
        origin: near_point,
        direction,
    }
}

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub grid_size: f32,
    pub heights: Vec<f32>,
    pub walkable: Vec<bool>,
}

impl Terrain {
    pub fn new(width: usize, height: usize, grid_size: f32) -> Self {
        Self {
            width,
            height,
            grid_size,
            heights: vec![0.0; width * height],
            walkable: vec![true; width * height],
        }
    }

    pub fn set_height(&mut self, x: usize, y: usize, h: f32) {
        if x < self.width && y < self.height {
            self.heights[y * self.width + x] = h;
        }
    }

    pub fn set_walkable(&mut self, x: usize, y: usize, is_walkable: bool) {
        if x < self.width && y < self.height {
            self.walkable[y * self.width + x] = is_walkable;
        }
    }

    pub fn get_height(&self, x: f32, y: f32) -> f32 {
        let grid_x = (x / self.grid_size).floor() as i32;
        let grid_y = (y / self.grid_size).floor() as i32;

        if grid_x >= 0 && grid_x < self.width as i32 && grid_y >= 0 && grid_y < self.height as i32 {
            self.heights[(grid_y as usize) * self.width + (grid_x as usize)]
        } else {
            0.0
        }
    }

    pub fn is_walkable(&self, x: f32, y: f32) -> bool {
        let grid_x = (x / self.grid_size).floor() as i32;
        let grid_y = (y / self.grid_size).floor() as i32;

        if grid_x >= 0 && grid_x < self.width as i32 && grid_y >= 0 && grid_y < self.height as i32 {
            self.walkable[(grid_y as usize) * self.width + (grid_x as usize)]
        } else {
            false
        }
    }

    pub fn raycast(&self, ray: &Ray, max_distance: f32, step_size: f32) -> Option<Vec2> {
        let mut t = 0.0;

        while t <= max_distance {
            let p = ray.origin + ray.direction * t;
            let terrain_z = self.get_height(p.x, p.y);

            // Assuming Z is up and ray points downwards towards terrain
            if p.z <= terrain_z {
                return Some(Vec2::new(p.x, p.y));
            }

            t += step_size;
        }

        None
    }
}

pub struct PlayerMovement {
    pub position: Vec2,
    pub target_position: Option<Vec2>,
    pub is_moving: bool,
}

impl PlayerMovement {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            target_position: None,
            is_moving: false,
        }
    }

    pub fn handle_mouse_click(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        camera: &Camera,
        terrain: &Terrain,
    ) -> bool {
        let ray = unproject_screen_to_ray(mouse_x, mouse_y, camera);

        if let Some(target_point) = terrain.raycast(&ray, 10000.0, 1.0) {
            if terrain.is_walkable(target_point.x, target_point.y) {
                self.target_position = Some(target_point);
                self.is_moving = true;
                return true;
            }
        }

        false
    }
}
