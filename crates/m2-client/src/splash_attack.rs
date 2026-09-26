use glam::Vec3;

#[derive(Debug, Clone)]
pub struct HitGeometry {
    pub origin: Vec3,
    pub direction: Vec3, // normalized
    pub range: f32,
    pub cone_angle_cos: f32, // Cosine of half the cone angle
}

#[derive(Debug, Clone)]
pub struct TargetInfo {
    pub vid: u32,
    pub position: Vec3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitTargetInfo {
    pub vid: u32,
}

/// Zwraca listę VID trafionych ofiar, filtrując cele w zasięgu stozka, sortując je po najkrótszym dystansie,
/// oraz ucinając wynik do zadanego limitu `max_hits`.
pub fn process_splash_attack(
    geometry: &HitGeometry,
    max_hits: usize,
    targets: &[TargetInfo],
) -> Vec<HitTargetInfo> {
    if max_hits == 0 {
        return Vec::new();
    }

    let mut hit_targets: Vec<(u32, f32)> = Vec::new();

    let range_sq = geometry.range * geometry.range;

    for target in targets {
        let diff = target.position - geometry.origin;
        let distance_sq = diff.length_squared();

        if distance_sq <= range_sq {
            if distance_sq > 0.0 {
                let distance = distance_sq.sqrt();
                let dir_to_target = diff / distance;
                let dot_product = geometry.direction.dot(dir_to_target);

                if dot_product >= geometry.cone_angle_cos {
                    hit_targets.push((target.vid, distance_sq));
                }
            } else {
                // Jesli cel znajduje sie idealnie na srodku uderzenia, trafia zawsze
                hit_targets.push((target.vid, 0.0));
            }
        }
    }

    // Sortowanie by distance rosnaco. Uzywamy `partial_cmp`, bo `f32` nie implementuje `Ord`.
    hit_targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    hit_targets
        .into_iter()
        .take(max_hits)
        .map(|(vid, _)| HitTargetInfo { vid })
        .collect()
}
