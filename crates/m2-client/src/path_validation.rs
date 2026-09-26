pub const STEP: f32 = 50.0;

pub struct PathResult {
    pub end_x: f32,
    pub end_y: f32,
    pub has_obstacle: bool,
}

/// Funkcja sprawdzajaca, czy prosta linia pomiedzy punktem poczatkowym a koncowym nie przecina sciany.
/// Parametr `is_walkable_fn` pozwala na wstrzykniecie funkcji (np. z backgroundu) sprawdzajacej punkt na mapie.
pub fn validate_path<F>(
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    mut is_walkable_fn: F,
) -> PathResult
where
    F: FnMut(f32, f32) -> bool,
{
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance == 0.0 {
        return PathResult {
            end_x: start_x,
            end_y: start_y,
            has_obstacle: false,
        };
    }

    let dir_x = dx / distance;
    let dir_y = dy / distance;

    let mut current_distance = 0.0;
    let mut last_valid_x = start_x;
    let mut last_valid_y = start_y;

    while current_distance < distance {
        let sample_x = start_x + dir_x * current_distance;
        let sample_y = start_y + dir_y * current_distance;

        if !is_walkable_fn(sample_x, sample_y) {
            return PathResult {
                end_x: last_valid_x,
                end_y: last_valid_y,
                has_obstacle: true,
            };
        }

        last_valid_x = sample_x;
        last_valid_y = sample_y;
        current_distance += STEP;
    }

    // Sprawdzenie ostatniego punktu (celu), jezeli nie pokrywa sie on z probkami krok po kroku
    if !is_walkable_fn(end_x, end_y) {
        return PathResult {
            end_x: last_valid_x,
            end_y: last_valid_y,
            has_obstacle: true,
        };
    }

    PathResult {
        end_x,
        end_y,
        has_obstacle: false,
    }
}
