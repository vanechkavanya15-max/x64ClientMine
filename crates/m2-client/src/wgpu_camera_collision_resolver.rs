use glam::Vec3;

const EPSILON: f32 = 1e-5;

/// Funkcja obliczajaca wysokosc terenu za pomoca Raycastingu w dol ze skokami LOD, 
/// dokladnie tak jak robi to CMapOutdoor::__PickTerrainHeight w oryginalnym C++.
/// Promien ray'a `fRayRange` i dlugosc `fLimitRange` sprawdzana na kazdym kroku fStep.
fn pick_terrain_height<F>(
    start: Vec3,
    end: Vec3,
    step: f32,
    ray_range: f32,
    limit_range: f32,
    get_terrain_height: &F,
) -> Option<Vec3>
where
    F: Fn(f32, f32) -> f32,
{
    if ray_range <= EPSILON {
        return None;
    }
    
    let mut f_pos = 0.0;
    let ray_range_inv = 1.0 / ray_range;

    while f_pos < ray_range && f_pos < limit_range {
        let cur_pos = start.lerp(end, f_pos * ray_range_inv);
        let map_height = get_terrain_height(cur_pos.x, cur_pos.y);

        if map_height >= cur_pos.z {
            return Some(cur_pos);
        } else {
            let diff = cur_pos.z - map_height;
            let mut multiplier = 0.01 * diff;
            if multiplier < 1.0 {
                multiplier = 1.0;
            }
            f_pos += step * multiplier;
        }
    }

    None
}

/// Rzuca promien z startu w kierunku dir (znormalizowanym) na dlugosc distance.
/// Jesli przetnie teren (jego wysokosc > Z na promieniu), zwraca wektor kolizji.
fn get_picking_point_with_ray_only_terrain<F>(
    start: Vec3,
    dir: Vec3,
    distance: f32,
    get_terrain_height: &F,
) -> Option<Vec3>
where
    F: Fn(f32, f32) -> f32,
{
    if distance <= EPSILON {
        return None;
    }

    let end = start + dir * distance;
    
    // 3 poziomy LOD: step 5 (zasieg 5000), step 10 (zasieg 10000), step 100 (zasieg 100000)
    if let Some(hit) = pick_terrain_height(start, end, 5.0, distance, 5000.0, get_terrain_height) {
        return Some(hit);
    }
    if let Some(hit) = pick_terrain_height(start, end, 10.0, distance, 10000.0, get_terrain_height) {
        return Some(hit);
    }
    if let Some(hit) = pick_terrain_height(start, end, 100.0, distance, 100000.0, get_terrain_height) {
        return Some(hit);
    }

    None
}

pub struct CameraCollisionResolver {
    pub target_distance: f32,
    pub current_distance: f32,
    pub zoom_speed: f32,
}

impl CameraCollisionResolver {
    pub fn new(initial_distance: f32, zoom_speed: f32) -> Self {
        Self {
            target_distance: initial_distance,
            current_distance: initial_distance,
            zoom_speed,
        }
    }

    #[allow(dead_code)]
    pub fn set_target_distance(&mut self, distance: f32) {
        self.target_distance = distance;
    }

    pub fn get_current_distance(&self) -> f32 {
        self.current_distance
    }

    pub fn update<F, O>(
        &mut self,
        target: Vec3,
        camera_dir: Vec3,
        delta_time: f32,
        get_terrain_height: &F,
        get_object_collision: &O,
    ) where
        F: Fn(f32, f32) -> f32,
        O: Fn(Vec3, Vec3) -> Option<Vec3>,
    {
        // Dir points from target to camera eye
        let dir = if camera_dir.length_squared() > EPSILON {
            camera_dir.normalize()
        } else {
            Vec3::new(0.0, -1.0, 0.0) // default fallback
        };

        let mut closest_distance = self.target_distance;

        // Check Terrain
        if let Some(terrain_hit) = get_picking_point_with_ray_only_terrain(target, dir, self.target_distance, get_terrain_height) {
            let hit_dist = target.distance(terrain_hit);
            if hit_dist < closest_distance {
                closest_distance = hit_dist;
            }
        }

        // Check Object
        let end_point = target + dir * self.target_distance;
        if let Some(obj_hit) = get_object_collision(target, end_point) {
            let hit_dist = target.distance(obj_hit);
            if hit_dist < closest_distance {
                closest_distance = hit_dist;
            }
        }

        if closest_distance < self.target_distance {
            // Collision occurred - snap to collision distance immediately (zoom-in)
            // Leave a small buffer (e.g. 1.0 unit) so it doesn't clip into the geometry
            let safe_distance = (closest_distance - 1.0).max(10.0); // min 10.0 distance
            self.current_distance = safe_distance;
        } else {
            // No collision - smoothly zoom out to target_distance
            if self.current_distance < self.target_distance {
                self.current_distance += self.zoom_speed * delta_time;
                if self.current_distance > self.target_distance {
                    self.current_distance = self.target_distance;
                }
            } else if self.current_distance > self.target_distance {
                // If current is larger than target (e.g. user scrolled to zoom in)
                self.current_distance -= self.zoom_speed * delta_time;
                if self.current_distance < self.target_distance {
                    self.current_distance = self.target_distance;
                }
            }
        }
    }
}
