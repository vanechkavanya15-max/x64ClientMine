#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    Inside,
    Outside,
    Partial,
}

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }

    pub fn distance(&self, center: (f32, f32, f32)) -> f32 {
        self.a * center.0 + self.b * center.1 + self.c * center.2 + self.d
    }
}

#[derive(Debug, Clone)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn new(planes: [Plane; 6]) -> Self {
        Self { planes }
    }

    pub fn cull_sphere(&self, center: (f32, f32, f32), radius: f32) -> ViewState {
        let mut has_intersected = false;

        for plane in &self.planes {
            let dist = plane.distance(center);
            if dist < -radius {
                return ViewState::Outside;
            }
            if dist < radius {
                has_intersected = true;
            }
        }

        if has_intersected {
            ViewState::Partial
        } else {
            ViewState::Inside
        }
    }
}
