//! Game Combat Sphere Collision System Module
//!
//! Precyzyjny system kolizji sferycznych walki wrecz:
//! Oblicza przeciecia miedzy sferami atakujacymi broni (Attacking Sphere) a sferami ciala celow (Defending Sphere)
//! w kazdej klatce aktywnego ataku (TestCollisionWithActor).

use std::collections::HashSet;

/// Prosty wektor 3D do obliczen fizyki i kolizji.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_squared(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn distance(&self, other: &Vec3) -> f32 {
        self.distance_squared(other).sqrt()
    }
}

/// Sfera kolizyjna w przestrzeni swiata gry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollisionSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl CollisionSphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Sprawdza czy dwie sfery zachodza na siebie (odleglosc <= r1 + r2).
    pub fn intersects(&self, other: &CollisionSphere) -> bool {
        let sum_r = self.radius + other.radius;
        self.center.distance_squared(&other.center) <= (sum_r * sum_r)
    }
}

/// Dane instancji aktora na scenie sprawdzanego pod katem trafienia.
#[derive(Debug, Clone)]
pub struct CombatTargetActor {
    pub vid: u32,
    pub is_dead: bool,
    pub is_friendly: bool,
    pub defending_spheres: Vec<CollisionSphere>,
}

impl CombatTargetActor {
    pub fn new(vid: u32, is_friendly: bool) -> Self {
        Self {
            vid,
            is_dead: false,
            is_friendly,
            defending_spheres: Vec::new(),
        }
    }

    pub fn add_defending_sphere(&mut self, center: Vec3, radius: f32) {
        self.defending_spheres.push(CollisionSphere::new(center, radius));
    }
}

/// Wynik detekcji trafienia pojedynczego celu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitResult {
    pub target_vid: u32,
    pub hit_position: Vec3,
}

/// Zarzadca testow kolizji walki wrecz.
#[derive(Debug, Default, Clone)]
pub struct GameCombatSphereCollisionSystem {
    hit_history_current_swing: HashSet<u32>, // zapobiega wielokrotnemu trafieniu w jednym machnieciu miecza
}

impl GameCombatSphereCollisionSystem {
    pub fn new() -> Self {
        Self {
            hit_history_current_swing: HashSet::new(),
        }
    }

    /// Resetuje historie trafien przed nowym uderzeniem/kombinacja.
    pub fn reset_swing(&mut self) {
        self.hit_history_current_swing.clear();
    }

    /// Sprawdza kolizje zestawu sfer broni z dostepnymi celami w kadrze.
    pub fn test_attack_collision(
        &mut self,
        weapon_spheres: &[CollisionSphere],
        targets: &[CombatTargetActor],
    ) -> Vec<HitResult> {
        let mut results = Vec::new();

        for target in targets {
            if target.is_dead || target.is_friendly {
                continue;
            }

            // Jesli juz trafilismy ten cel w tym ciosie, pomijamy
            if self.hit_history_current_swing.contains(&target.vid) {
                continue;
            }

            let mut hit_detected = false;
            let mut hit_pos = Vec3::new(0.0, 0.0, 0.0);

            for w_sphere in weapon_spheres {
                for d_sphere in &target.defending_spheres {
                    if w_sphere.intersects(d_sphere) {
                        hit_detected = true;
                        // Przyblizony punkt kontaktu
                        hit_pos = Vec3::new(
                            (w_sphere.center.x + d_sphere.center.x) * 0.5,
                            (w_sphere.center.y + d_sphere.center.y) * 0.5,
                            (w_sphere.center.z + d_sphere.center.z) * 0.5,
                        );
                        break;
                    }
                }
                if hit_detected {
                    break;
                }
            }

            if hit_detected {
                self.hit_history_current_swing.insert(target.vid);
                results.push(HitResult {
                    target_vid: target.vid,
                    hit_position: hit_pos,
                });
            }
        }

        results
    }
}
