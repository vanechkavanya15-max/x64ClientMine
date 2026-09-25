//! Modul collision_detector dla skrzynki m2-client-terrain
//! Detektor kolizji promienia z terenem i atrybutami
//! Raycast z kamery lub postaci, znajdowanie dokladnej wysokosci Z pod graczem

/// Reprezentacja wektora 3D do obliczen geometrycznych
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length == 0.0 {
            *self
        } else {
            Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        }
    }
    
    pub fn add(&self, other: &Vector3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    
    pub fn multiply_scalar(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

/// Reprezentacja promienia (Ray) uzywana do detekcji kolizji
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction: direction.normalize() }
    }
    
    pub fn point_at(&self, distance: f32) -> Vector3 {
        self.origin.add(&self.direction.multiply_scalar(distance))
    }
}

/// Trait dostarczajacy dane terenu dla detektora kolizji
pub trait TerrainProvider {
    /// Zwraca interpolowana wysokosc terenu dla zadanych wspolrzednych (x, y)
    fn get_height(&self, x: f32, y: f32) -> Option<f32>;
    
    /// Sprawdza czy dany punkt jest poprawny do chodzenia (brak atrybutu block/water)
    fn is_walkable(&self, x: f32, y: f32) -> bool;
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone)]
pub struct CollisionDetector<T: TerrainProvider> {
    pub initialized: bool,
    provider: T,
}

impl<T: TerrainProvider> CollisionDetector<T> {
    pub fn new(provider: T) -> Self {
        Self { 
            initialized: true,
            provider,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    /// Znajduje wysokosc terenu pod graczem dla zadanej pozycji X, Y
    pub fn get_height_under_player(&self, x: f32, y: f32) -> Option<f32> {
        self.provider.get_height(x, y)
    }
    
    /// Weryfikuje czy dany punkt (X, Y) pozwala na poruszanie sie
    pub fn is_position_walkable(&self, x: f32, y: f32) -> bool {
        self.provider.is_walkable(x, y)
    }
    
    /// Przeprowadza raycast uzywajac techniki raymarching
    /// Zwraca punkt przeciecia z powierzchnia terenu jesli istnieje
    pub fn raycast(&self, ray: &Ray, max_distance: f32, step_size: f32) -> Option<Vector3> {
        if max_distance <= 0.0 || step_size <= 0.0 {
            return None;
        }
        
        let mut current_distance = 0.0;
        
        while current_distance <= max_distance {
            let point = ray.point_at(current_distance);
            
            if let Some(terrain_z) = self.provider.get_height(point.x, point.y) {
                // Jesli promien wszedl pod teren (wczesniej byl nad)
                // Wymaga to aby kierunek promienia mial skladowa Z w dol, albo teren sie wznosil
                // W uproszczeniu: sprawdzamy czy obecny punkt jest ponizej lub na rowni z terenem
                if point.z <= terrain_z {
                    // Precyzyjniejsze znalezienie punktu z uzyciem bisekcji lub powrotu o pol kroku
                    // W tym prostym przykladzie zwracamy punkt na powierzchni terenu dla X, Y z point
                    return Some(Vector3::new(point.x, point.y, terrain_z));
                }
            }
            
            current_distance += step_size;
        }
        
        None
    }
}
