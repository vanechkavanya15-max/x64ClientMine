#[derive(Debug, Clone, PartialEq)]
pub struct ParticleInstance {
    pub position: (f32, f32, f32),
    pub velocity: (f32, f32, f32),
    pub age: f32,
    pub lifetime: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub base_scale: (f32, f32),
}

impl ParticleInstance {
    pub fn new(lifetime: f32, base_scale_x: f32, base_scale_y: f32) -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            velocity: (0.0, 0.0, 0.0),
            age: 0.0,
            lifetime,
            rotation: 0.0,
            rotation_speed: 0.0,
            base_scale: (base_scale_x, base_scale_y),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.age < self.lifetime
    }

    pub fn evaluate_scale(&self, time_events: &[(f32, f32)]) -> (f32, f32) {
        if self.lifetime <= 0.0 {
            return self.base_scale;
        }

        let progress = self.age / self.lifetime;
        let scale_factor = get_time_event_blend_value_f32(progress, time_events);
        
        (self.base_scale.0 * scale_factor, self.base_scale.1 * scale_factor)
    }

    pub fn evaluate_color(&self, time_events: &[(f32, [f32; 4])]) -> [f32; 4] {
        if self.lifetime <= 0.0 {
            return [1.0, 1.0, 1.0, 1.0]; // Default white with 1.0 alpha
        }

        let progress = self.age / self.lifetime;
        get_time_event_blend_value_color(progress, time_events)
    }
}

fn get_time_event_blend_value_f32(time: f32, events: &[(f32, f32)]) -> f32 {
    if events.is_empty() {
        return 1.0; // Default scale multiplier is 1.0
    }

    if events.len() == 1 {
        return events[0].1;
    }

    if time <= events[0].0 {
        return events[0].1;
    }

    if time >= events.last().unwrap().0 {
        return events.last().unwrap().1;
    }

    // Binary search for the right interval
    let idx = match events.binary_search_by(|e| e.0.partial_cmp(&time).unwrap()) {
        Ok(index) => return events[index].1,
        Err(index) => index,
    };

    let low = &events[idx - 1];
    let high = &events[idx];

    let time_diff = high.0 - low.0;
    let perc = (time - low.0) / time_diff;
    let val_diff = high.1 - low.1;

    low.1 + perc * val_diff
}

fn get_time_event_blend_value_color(time: f32, events: &[(f32, [f32; 4])]) -> [f32; 4] {
    if events.is_empty() {
        return [1.0, 1.0, 1.0, 1.0]; // Default color
    }

    if events.len() == 1 {
        return events[0].1;
    }

    if time <= events[0].0 {
        return events[0].1;
    }

    if time >= events.last().unwrap().0 {
        return events.last().unwrap().1;
    }

    let idx = match events.binary_search_by(|e| e.0.partial_cmp(&time).unwrap()) {
        Ok(index) => return events[index].1,
        Err(index) => index,
    };

    let low = &events[idx - 1];
    let high = &events[idx];

    let time_diff = high.0 - low.0;
    let perc = (time - low.0) / time_diff;

    let mut result = [0.0; 4];
    for i in 0..4 {
        let val_diff = high.1[i] - low.1[i];
        result[i] = low.1[i] + perc * val_diff;
    }

    result
}
