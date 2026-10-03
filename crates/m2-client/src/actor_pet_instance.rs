// actor_pet_instance.rs - Instancja zwierzaka w silniku Metin2 Rust
// Logika podazania za wlascicielem, strefa zatrzymania, teleportacja oraz summon/unsummon.

pub const PET_STOP_DISTANCE: f32 = 200.0;
pub const PET_TELEPORT_DISTANCE: f32 = 2500.0;
pub const PET_SPAWN_OFFSET_DISTANCE: f32 = 150.0;

#[derive(Debug, Clone, PartialEq)]
pub struct PetInstance {
    pub vid: u32,
    pub owner_vid: u32,
    pub race_id: u32,
    pub name: String,
    pub pos: (f32, f32, f32),
    pub rotation: f32,
    pub move_speed: f32,
    pub is_summoned: bool,
    pub is_moving: bool,
}

impl PetInstance {
    pub fn new(vid: u32, owner_vid: u32, race_id: u32, name: impl Into<String>) -> Self {
        Self {
            vid,
            owner_vid,
            race_id,
            name: name.into(),
            pos: (0.0, 0.0, 0.0),
            rotation: 0.0,
            move_speed: 0.0,
            is_summoned: false,
            is_moving: false,
        }
    }

    /// Przywolanie zwierzaka na swiat gry w poblizu gracza.
    pub fn summon(&mut self, owner_pos: (f32, f32, f32), owner_rot_deg: f32) {
        self.is_summoned = true;
        self.is_moving = false;
        self.move_speed = 0.0;

        // Ustawienie pozycji peta lekko za graczem w oparciu o kat rotacji gracza
        let rot_rad = owner_rot_deg.to_radians();
        let spawn_x = owner_pos.0 - rot_rad.sin() * PET_SPAWN_OFFSET_DISTANCE;
        let spawn_y = owner_pos.1 - rot_rad.cos() * PET_SPAWN_OFFSET_DISTANCE;
        let spawn_z = owner_pos.2;

        self.pos = (spawn_x, spawn_y, spawn_z);
        self.rotation = owner_rot_deg;
    }

    /// Odwolanie zwierzaka ze swiata gry.
    pub fn unsummon(&mut self) {
        self.is_summoned = false;
        self.is_moving = false;
        self.move_speed = 0.0;
    }

    /// Oblicza odleglosc dwuwymiarowa (w plaszczyznie XY) do zadanej pozycji wlasciciela.
    pub fn distance_to_owner_2d(&self, owner_pos: (f32, f32, f32)) -> f32 {
        let dx = owner_pos.0 - self.pos.0;
        let dy = owner_pos.1 - self.pos.1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Aktualizuje podazanie zwierzaka za wlascicielem:
    /// - Jesli nie jest przywolany: brak akcji, zwraca false.
    /// - Jesli dystans > 2500 jednostek: natychmiastowa teleportacja do gracza.
    /// - Jesli dystans <= 200 jednostek: strefa zatrzymania, pet stoi w miejscu.
    /// - Jesli dystans miedzy 200 a 2500: ruch w strone gracza z predkoscia gracza.
    pub fn update_follow(
        &mut self,
        owner_pos: (f32, f32, f32),
        owner_speed: f32,
        delta_time: f32,
    ) -> bool {
        if !self.is_summoned {
            return false;
        }

        let dx = owner_pos.0 - self.pos.0;
        let dy = owner_pos.1 - self.pos.1;
        let dist = (dx * dx + dy * dy).sqrt();

        // 1. Przypadek teleportacji: dystans > 2500
        if dist > PET_TELEPORT_DISTANCE {
            self.pos = owner_pos;
            self.move_speed = 0.0;
            self.is_moving = false;
            return true;
        }

        // 2. Przypadek strefy zatrzymania: dystans <= 200
        if dist <= PET_STOP_DISTANCE {
            self.move_speed = 0.0;
            self.is_moving = false;
            return true;
        }

        // 3. Przypadek podazania: 200 < dystans <= 2500
        self.move_speed = owner_speed;
        self.is_moving = true;

        // Obliczenie rotacji w stopniach (kierunek do wlasciciela)
        let angle_rad = dy.atan2(dx);
        let mut deg = angle_rad.to_degrees();
        if deg < 0.0 {
            deg += 360.0;
        }
        self.rotation = deg;

        // Krok przemieszczenia w danej ramce czasowej
        let desired_step = self.move_speed * delta_time;
        let max_allowed_step = dist - PET_STOP_DISTANCE;
        let step = desired_step.min(max_allowed_step);

        if dist > 0.0001 {
            let nx = dx / dist;
            let ny = dy / dist;
            self.pos.0 += nx * step;
            self.pos.1 += ny * step;
        }
        self.pos.2 = owner_pos.2;

        true
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.pos = pos;
    }

    pub fn set_rotation(&mut self, rot_deg: f32) {
        self.rotation = rot_deg;
    }
}

/// Menedzer instancji zwierzakow powiazanych z graczami
#[derive(Default, Debug)]
pub struct PetInstanceManager {
    pets: std::collections::HashMap<u32, PetInstance>,
}

impl PetInstanceManager {
    pub fn new() -> Self {
        Self {
            pets: std::collections::HashMap::new(),
        }
    }

    pub fn register_pet(&mut self, pet: PetInstance) {
        self.pets.insert(pet.owner_vid, pet);
    }

    pub fn remove_pet_by_owner(&mut self, owner_vid: u32) -> Option<PetInstance> {
        self.pets.remove(&owner_vid)
    }

    pub fn get_pet_by_owner(&self, owner_vid: u32) -> Option<&PetInstance> {
        self.pets.get(&owner_vid)
    }

    pub fn get_pet_by_owner_mut(&mut self, owner_vid: u32) -> Option<&mut PetInstance> {
        self.pets.get_mut(&owner_vid)
    }

    pub fn update_all_follows(
        &mut self,
        owners_info: &[(u32, (f32, f32, f32), f32)],
        delta_time: f32,
    ) {
        for (owner_vid, owner_pos, owner_speed) in owners_info {
            if let Some(pet) = self.pets.get_mut(owner_vid) {
                pet.update_follow(*owner_pos, *owner_speed, delta_time);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_summon_and_unsummon() {
        let mut pet = PetInstance::new(1001, 50, 34001, "TestowyPiesek");
        assert!(!pet.is_summoned);
        assert!(!pet.is_moving);

        pet.summon((1000.0, 1000.0, 0.0), 0.0);
        assert!(pet.is_summoned);
        assert_eq!(pet.pos.2, 0.0);

        pet.unsummon();
        assert!(!pet.is_summoned);
        assert!(!pet.is_moving);
        assert_eq!(pet.move_speed, 0.0);
    }

    #[test]
    fn test_pet_stop_within_200_units() {
        let mut pet = PetInstance::new(1001, 50, 34001, "Boczek");
        pet.summon((1000.0, 1000.0, 0.0), 0.0);
        pet.set_position((1150.0, 1000.0, 0.0)); // Dystans = 150 (<= 200)

        let updated = pet.update_follow((1000.0, 1000.0, 0.0), 300.0, 0.1);
        assert!(updated);
        assert!(!pet.is_moving);
        assert_eq!(pet.move_speed, 0.0);
        assert_eq!(pet.pos.0, 1150.0); // Pozycja bez zmian
    }

    #[test]
    fn test_pet_follow_movement() {
        let mut pet = PetInstance::new(1001, 50, 34001, "Boczek");
        pet.summon((1000.0, 1000.0, 0.0), 0.0);
        pet.set_position((500.0, 1000.0, 0.0)); // Dystans = 500 (> 200)

        // Predkosc 400.0, dt = 0.5s -> krok 200.0
        let updated = pet.update_follow((1000.0, 1000.0, 0.0), 400.0, 0.5);
        assert!(updated);
        assert!(pet.is_moving);
        assert_eq!(pet.move_speed, 400.0);
        assert_eq!(pet.pos.0, 700.0); // 500 + 200 = 700
    }

    #[test]
    fn test_pet_teleport_when_dist_greater_than_2500() {
        let mut pet = PetInstance::new(1001, 50, 34001, "Boczek");
        pet.summon((0.0, 0.0, 0.0), 0.0);
        pet.set_position((0.0, 0.0, 0.0));

        let owner_pos = (5000.0, 0.0, 10.0); // Dystans = 5000 (> 2500)
        let updated = pet.update_follow(owner_pos, 400.0, 0.1);
        assert!(updated);
        assert_eq!(pet.pos, owner_pos); // Teleportacja do gracza
        assert!(!pet.is_moving);
        assert_eq!(pet.move_speed, 0.0);
    }
}
