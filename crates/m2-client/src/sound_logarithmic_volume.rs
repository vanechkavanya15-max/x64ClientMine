pub fn ratio_to_apply_volume(ratio: f32) -> f32 {
    if ratio <= 0.0 {
        0.0
    } else if ratio < 0.1 {
        ratio
    } else {
        10.0f32.powf(-1.0 + ratio)
    }
}

pub fn grade_to_apply_volume(grade: i32) -> f32 {
    let ratio = (grade as f32) / 5.0;
    ratio_to_apply_volume(ratio)
}
