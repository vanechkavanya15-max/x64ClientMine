use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AmbienceZone {
    pub zone_id: u32,
    pub sound_name: String,
    pub center: (f32, f32, f32),
    pub radius: f32,
    pub max_vol: f32,
}

pub struct SoundAmbienceManager {
    zones: HashMap<u32, AmbienceZone>,
    listener_pos: (f32, f32, f32),
    // Simulate time/variation for global ambience
    time: f32,
}

impl SoundAmbienceManager {
    pub fn new() -> Self {
        Self {
            zones: HashMap::new(),
            listener_pos: (0.0, 0.0, 0.0),
            time: 0.0,
        }
    }

    pub fn add_ambience_zone(&mut self, zone_id: u32, sound_name: &str, center: (f32, f32, f32), radius: f32, max_vol: f32) {
        self.zones.insert(zone_id, AmbienceZone {
            zone_id,
            sound_name: sound_name.to_string(),
            center,
            radius,
            max_vol,
        });
    }

    pub fn update_listener_position(&mut self, listener_pos: (f32, f32, f32)) {
        self.listener_pos = listener_pos;
        self.time += 0.1; // Simple time advance for global ambience variation
    }

    pub fn get_zone_volume(&self, zone_id: u32) -> f32 {
        if let Some(zone) = self.zones.get(&zone_id) {
            if zone.radius <= 0.0 {
                // Global ambience (e.g., wind): base volume with slight variation over time
                let variation = (self.time.sin() * 0.1) * zone.max_vol;
                return (zone.max_vol + variation).clamp(0.0, zone.max_vol * 1.1);
            }

            let dx = self.listener_pos.0 - zone.center.0;
            let dy = self.listener_pos.1 - zone.center.1;
            let dz = self.listener_pos.2 - zone.center.2;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            if distance <= 0.0 {
                return zone.max_vol;
            }

            if distance >= zone.radius {
                return 0.0;
            }

            let attenuation = 1.0 - (distance / zone.radius);
            return zone.max_vol * attenuation;
        }

        0.0
    }
}
