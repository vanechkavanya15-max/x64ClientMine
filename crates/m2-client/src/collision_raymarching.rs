#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: (f32, f32, f32),
    pub direction: (f32, f32, f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxCollider {
    pub min: (f32, f32, f32),
    pub max: (f32, f32, f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaycastHit {
    pub distance: f32,
    pub point: (f32, f32, f32),
    pub normal: (f32, f32, f32),
}

impl BoxCollider {
    pub fn intersect(&self, ray: &Ray) -> Option<RaycastHit> {
        let mut t0 = f32::NEG_INFINITY;
        let mut t1 = f32::INFINITY;
        let mut n0 = (0.0, 0.0, 0.0);
        let mut n1 = (0.0, 0.0, 0.0);

        let o = [ray.origin.0, ray.origin.1, ray.origin.2];
        let d = [ray.direction.0, ray.direction.1, ray.direction.2];
        let min_b = [self.min.0, self.min.1, self.min.2];
        let max_b = [self.max.0, self.max.1, self.max.2];

        for i in 0..3 {
            if d[i].abs() < 1e-8 {
                if o[i] < min_b[i] || o[i] > max_b[i] {
                    return None;
                }
            } else {
                let inv_d = 1.0 / d[i];
                let mut t_near = (min_b[i] - o[i]) * inv_d;
                let mut t_far = (max_b[i] - o[i]) * inv_d;

                let mut n_axis = (0.0, 0.0, 0.0);
                match i {
                    0 => n_axis.0 = 1.0,
                    1 => n_axis.1 = 1.0,
                    2 => n_axis.2 = 1.0,
                    _ => {}
                }

                let near_n;
                let far_n;
                if t_near > t_far {
                    std::mem::swap(&mut t_near, &mut t_far);
                    near_n = (n_axis.0, n_axis.1, n_axis.2);
                    far_n = (-n_axis.0, -n_axis.1, -n_axis.2);
                } else {
                    near_n = (-n_axis.0, -n_axis.1, -n_axis.2);
                    far_n = (n_axis.0, n_axis.1, n_axis.2);
                }

                if t_near > t0 {
                    t0 = t_near;
                    n0 = near_n;
                }
                if t_far < t1 {
                    t1 = t_far;
                    n1 = far_n;
                }

                if t0 > t1 {
                    return None;
                }
            }
        }

        if t1 < 0.0 {
            return None;
        }

        let t;
        let hit_normal;

        if t0 < 0.0 {
            t = t1;
            hit_normal = n1;
        } else {
            t = t0;
            hit_normal = n0;
        }

        let point = (
            o[0] + d[0] * t,
            o[1] + d[1] * t,
            o[2] + d[2] * t,
        );

        Some(RaycastHit {
            distance: t,
            point,
            normal: hit_normal,
        })
    }
}
