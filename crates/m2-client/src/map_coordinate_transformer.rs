pub const CM_PER_METER: f32 = 100.0;
pub const CM_PER_CELL: f32 = 200.0;
pub const METERS_PER_CELL: f32 = 2.0;

/// Converts centimeters (server units) to meters (3D world space).
pub fn cm_to_meters(cm: f32) -> f32 {
    cm / CM_PER_METER
}

/// Converts meters (3D world space) to centimeters (server units).
pub fn meters_to_cm(meters: f32) -> f32 {
    meters * CM_PER_METER
}

/// Converts centimeters (server units) to cell indices.
pub fn cm_to_cell(cm: f32) -> i32 {
    (cm / CM_PER_CELL).floor() as i32
}

/// Converts cell indices to centimeters (server units).
pub fn cell_to_cm(cell: i32) -> f32 {
    (cell as f32) * CM_PER_CELL
}

/// Converts meters (3D world space) to cell indices.
pub fn meters_to_cell(meters: f32) -> i32 {
    (meters / METERS_PER_CELL).floor() as i32
}

/// Converts cell indices to meters (3D world space).
pub fn cell_to_meters(cell: i32) -> f32 {
    (cell as f32) * METERS_PER_CELL
}
