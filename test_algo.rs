fn main() {
    let current_time = 5.0;
    let start_time = 0.0;
    let total_time = 10.0;
    let time = (current_time - start_time) / total_time;
    let box_pos = [
        (-1.0, -1.0),
        (-1.0, 0.0),
        (-1.0, 1.0),
        (0.0, 1.0),
        (1.0, 1.0),
        (1.0, 0.0),
        (1.0, -1.0),
        (0.0, -1.0),
    ];
    let tri_count = (8.0 - 8.0 * time) as i32;
    let last_percentage = (8.0 - 8.0 * time) - (tri_count as f32);
    println!("time: {}, tri_count: {}, last_percent: {}", time, tri_count, last_percentage);
}
