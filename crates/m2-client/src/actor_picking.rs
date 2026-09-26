pub struct Ray {
    pub origin: [f32; 3],
    pub dir: [f32; 3],
}

pub trait Actor {
    fn vid(&self) -> u32;
    fn sphere_center(&self) -> [f32; 3];
    fn radius(&self) -> f32;
    fn on_selected(&mut self);
    fn on_unselected(&mut self);
}

pub fn intersect_defending_sphere(ray: &Ray, sphere_center: [f32; 3], radius: f32) -> bool {
    let oc = [
        ray.origin[0] - sphere_center[0],
        ray.origin[1] - sphere_center[1],
        ray.origin[2] - sphere_center[2],
    ];

    let a = ray.dir[0] * ray.dir[0] + ray.dir[1] * ray.dir[1] + ray.dir[2] * ray.dir[2];
    let b = 2.0 * (oc[0] * ray.dir[0] + oc[1] * ray.dir[1] + oc[2] * ray.dir[2]);
    let c = oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return false;
    }

    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b - sqrt_discriminant) / (2.0 * a);
    let t2 = (-b + sqrt_discriminant) / (2.0 * a);

    t1 >= 0.0 || t2 >= 0.0
}

fn calculate_distance_sq(p1: [f32; 3], p2: [f32; 3]) -> f32 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    let dz = p1[2] - p2[2];
    dx * dx + dy * dy + dz * dz
}

pub struct ActorPicker {
    picked_vid: Option<u32>,
}

impl ActorPicker {
    pub fn new() -> Self {
        Self { picked_vid: None }
    }

    pub fn pick<'a, I>(
        &mut self,
        ray: &Ray,
        main_vid: u32,
        actors: I,
    ) -> Option<u32>
    where
        I: IntoIterator<Item = &'a mut dyn Actor>,
    {
        let mut closest_vid: Option<u32> = None;
        let mut min_distance_sq = f32::MAX;

        // Instead of doing multiple passes over the iterator, we can collect the actors as mutable references in a Vec, 
        // find the best one, and then iterate through the Vec again to call on_selected/on_unselected.
        let mut actors_vec: Vec<&mut dyn Actor> = actors.into_iter().collect();

        for actor in &actors_vec {
            if actor.vid() == main_vid {
                continue;
            }

            if intersect_defending_sphere(ray, actor.sphere_center(), actor.radius()) {
                let dist_sq = calculate_distance_sq(ray.origin, actor.sphere_center());
                if dist_sq < min_distance_sq {
                    min_distance_sq = dist_sq;
                    closest_vid = Some(actor.vid());
                }
            }
        }

        if self.picked_vid == closest_vid {
            return closest_vid;
        }

        for actor in &mut actors_vec {
            if Some(actor.vid()) == self.picked_vid {
                actor.on_unselected();
            }
            if Some(actor.vid()) == closest_vid {
                actor.on_selected();
            }
        }

        self.picked_vid = closest_vid;
        closest_vid
    }
}
