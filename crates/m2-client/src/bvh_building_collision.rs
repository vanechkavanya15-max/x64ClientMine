// bvh_building_collision.rs - Drzewo BVH dla kolizji z budynkami, scianami i przeszkodami
// Zgodnosc z CActorInstance::TestActorCollision oraz CMapOutdoor::ObjectCollision w Metin2 C++.
// Oblicza wektory slizgania (wall slide) zapobiegajac przenikaniu postaci przez mury i domy.
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

use glam::Vec3;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb3D {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb3D {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Sprawdza czy punkt znajduje sie wewnatrz AABB z marginesem promienia postaci
    pub fn contains_point_with_radius(&self, pt: Vec3, radius: f32) -> bool {
        pt.x >= self.min.x - radius && pt.x <= self.max.x + radius
            && pt.y >= self.min.y - radius && pt.y <= self.max.y + radius
            && pt.z >= self.min.z && pt.z <= self.max.z
    }

    /// Oblicza najblizszy punkt na powierzchni AABB do podanego punktu
    pub fn closest_point(&self, pt: Vec3) -> Vec3 {
        Vec3::new(
            pt.x.clamp(self.min.x, self.max.x),
            pt.y.clamp(self.min.y, self.max.y),
            pt.z.clamp(self.min.z, self.max.z),
        )
    }
}

#[derive(Debug, Clone)]
pub struct BuildingObstacle {
    pub id: u32,
    pub aabb: Aabb3D,
}

pub struct BuildingCollisionSystem {
    pub buildings: Vec<BuildingObstacle>,
}

impl Default for BuildingCollisionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildingCollisionSystem {
    pub fn new() -> Self {
        Self {
            buildings: Vec::new(),
        }
    }

    pub fn add_building(&mut self, id: u32, min: Vec3, max: Vec3) {
        self.buildings.push(BuildingObstacle {
            id,
            aabb: Aabb3D::new(min, max),
        });
    }

    pub fn clear(&mut self) {
        self.buildings.clear();
    }

    /// Testuje kolizje i koryguje nowa pozycje gracza.
    /// Jesli gracz wejdzie w kolizje z budynkiem, wektor ruchu zostaje zrzutowany
    /// wzdluz sciany (wall slide), a pozycja przesunieta na krawedz sciany.
    pub fn resolve_movement(&self, old_pos: Vec3, mut new_pos: Vec3, actor_radius: f32) -> Vec3 {
        for b in &self.buildings {
            let closest = b.aabb.closest_point(new_pos);
            let diff = new_pos - closest;
            // Pomijamy os Z (wysokosc gracza reguluje teren)
            let diff_2d = Vec3::new(diff.x, diff.y, 0.0);
            let dist_2d = diff_2d.length();

            if dist_2d < actor_radius && dist_2d > 1e-4 {
                // Kolizja - normalna sciany
                let normal = diff_2d / dist_2d;
                // Odsun postac poza promien kolizji
                let push_out = normal * (actor_radius - dist_2d);
                new_pos.x += push_out.x;
                new_pos.y += push_out.y;

                // Slizganie: usuwamy skladowa wektora predkosci wchodzaca w sciane
                let move_vec = new_pos - old_pos;
                let dot = move_vec.dot(normal);
                if dot < 0.0 {
                    let slide_vec = move_vec - normal * dot;
                    new_pos.x = old_pos.x + slide_vec.x;
                    new_pos.y = old_pos.y + slide_vec.y;
                }
            } else if dist_2d <= 1e-4 && b.aabb.contains_point_with_radius(new_pos, 0.0) {
                // Postac znalazla sie calkowicie wewnatrz budynku - cofnij do bezpiecznej starej pozycji
                new_pos.x = old_pos.x;
                new_pos.y = old_pos.y;
            }
        }

        new_pos
    }
}

static COLLISION_SYSTEM: OnceLock<Mutex<BuildingCollisionSystem>> = OnceLock::new();

pub fn get_collision_system() -> &'static Mutex<BuildingCollisionSystem> {
    COLLISION_SYSTEM.get_or_init(|| Mutex::new(BuildingCollisionSystem::new()))
}

// ============================================================================
// Eksporty FFI dla Silnika Gry
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_collision_add_building_aabb(
    id: u32,
    min_x: f32, min_y: f32, min_z: f32,
    max_x: f32, max_y: f32, max_z: f32,
) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if let Ok(mut sys) = get_collision_system().lock() {
            sys.add_building(id, Vec3::new(min_x, min_y, min_z), Vec3::new(max_x, max_y, max_z));
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_collision_resolve_movement(
    old_x: f32, old_y: f32, old_z: f32,
    new_x: f32, new_y: f32, new_z: f32,
    radius: f32,
    out_x: *mut f32, out_y: *mut f32, out_z: *mut f32,
) -> bool {
    m2_client_bridge::ffi_guard::safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() {
            return false;
        }
        if let Ok(sys) = get_collision_system().lock() {
            let old_pos = Vec3::new(old_x, old_y, old_z);
            let new_pos = Vec3::new(new_x, new_y, new_z);
            let corrected = sys.resolve_movement(old_pos, new_pos, radius);
            unsafe {
                *out_x = corrected.x;
                *out_y = corrected.y;
                *out_z = corrected.z;
            }
            return true;
        }
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_collision_slide() {
        let mut sys = BuildingCollisionSystem::new();
        // Budynek w punkcie (100, 100) o wymiarach 50x50
        sys.add_building(1, Vec3::new(100.0, 100.0, 0.0), Vec3::new(150.0, 150.0, 100.0));

        let old_pos = Vec3::new(80.0, 120.0, 0.0);
        // Gracz probuje wejsc w sciane do punktu (110, 120)
        let intended_pos = Vec3::new(110.0, 120.0, 0.0);
        let radius = 10.0;

        let corrected = sys.resolve_movement(old_pos, intended_pos, radius);
        // Pozycja powinna zostac zatrzymana przed sciana x=100 (z uwzglednieniem promienia 10)
        assert!(corrected.x <= 90.001);
    }
}
