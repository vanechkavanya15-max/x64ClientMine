use glam::Vec3;

#[derive(Debug, Clone)]
pub struct MassPoint {
    pub position: Vec3,
    pub old_position: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    pub is_pinned: bool,
}

impl MassPoint {
    pub fn new(position: Vec3, mass: f32, is_pinned: bool) -> Self {
        Self {
            position,
            old_position: position,
            acceleration: Vec3::ZERO,
            mass,
            is_pinned,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DistanceConstraint {
    pub p1_idx: usize,
    pub p2_idx: usize,
    pub rest_length: f32,
    pub stiffness: f32,
}

impl DistanceConstraint {
    pub fn new(p1_idx: usize, p2_idx: usize, rest_length: f32, stiffness: f32) -> Self {
        Self {
            p1_idx,
            p2_idx,
            rest_length,
            stiffness,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoneClothSimulation {
    pub points: Vec<MassPoint>,
    pub constraints: Vec<DistanceConstraint>,
    pub gravity: Vec3,
    pub wind_time: f32,
    pub drag: f32,
}

impl BoneClothSimulation {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            constraints: Vec::new(),
            gravity: Vec3::new(0.0, 0.0, -9.81),
            wind_time: 0.0,
            drag: 0.98,
        }
    }

    pub fn add_point(&mut self, position: Vec3, mass: f32, is_pinned: bool) -> usize {
        let idx = self.points.len();
        self.points.push(MassPoint::new(position, mass, is_pinned));
        idx
    }

    pub fn add_constraint(&mut self, p1_idx: usize, p2_idx: usize, stiffness: f32) {
        if p1_idx >= self.points.len() || p2_idx >= self.points.len() {
            return;
        }
        let p1 = self.points[p1_idx].position;
        let p2 = self.points[p2_idx].position;
        let rest_length = p1.distance(p2);
        self.constraints.push(DistanceConstraint::new(
            p1_idx,
            p2_idx,
            rest_length,
            stiffness,
        ));
    }

    pub fn apply_external_forces(&mut self, base_velocity: Vec3, wind_direction: Vec3, dt: f32) {
        self.wind_time += dt;

        // Inertia (opposite of base velocity)
        let inertia = -base_velocity * 0.5;

        // Wind flutter using sine waves
        let flutter = (self.wind_time * 5.0).sin() * 0.5 + (self.wind_time * 3.14).cos() * 0.25;
        let wind_force = wind_direction * flutter;

        for point in &mut self.points {
            if !point.is_pinned {
                point.acceleration += inertia + wind_force;
            }
        }
    }

    pub fn update(&mut self, dt: f32, iterations: usize) {
        // Verlet Integration
        for point in &mut self.points {
            if point.is_pinned {
                continue;
            }

            let temp = point.position;
            // pos = pos + (pos - old_pos) * drag + accel * dt * dt
            let velocity = (point.position - point.old_position) * self.drag;
            point.position = point.position + velocity + point.acceleration * dt * dt;
            point.old_position = temp;
            
            // Reset acceleration
            point.acceleration = self.gravity;
        }

        // Constraint satisfaction
        for _ in 0..iterations {
            for constraint in &self.constraints {
                let p1 = self.points[constraint.p1_idx].position;
                let p2 = self.points[constraint.p2_idx].position;

                let delta = p2 - p1;
                let dist = delta.length();
                if dist < 0.0001 {
                    continue;
                }

                let diff = (dist - constraint.rest_length) / dist;
                let correction = delta * diff * constraint.stiffness * 0.5;

                let w1 = if self.points[constraint.p1_idx].is_pinned { 0.0 } else { 1.0 };
                let w2 = if self.points[constraint.p2_idx].is_pinned { 0.0 } else { 1.0 };
                let total_w = w1 + w2;

                if total_w > 0.0 {
                    if !self.points[constraint.p1_idx].is_pinned {
                        self.points[constraint.p1_idx].position += correction * (w1 / total_w);
                    }
                    if !self.points[constraint.p2_idx].is_pinned {
                        self.points[constraint.p2_idx].position -= correction * (w2 / total_w);
                    }
                }
            }
        }
    }
}

impl Default for BoneClothSimulation {
    fn default() -> Self {
        Self::new()
    }
}
