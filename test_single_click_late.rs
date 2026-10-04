use m2_client::actor_combo_window_timing::{ComboInputData, RaceMotionData};
use m2_client::engine_combo_input_controller::EngineComboInputController;

fn main() {
    let mut controller = EngineComboInputController::new();
    
    controller.trigger_single_attack();
    
    let motion_data = RaceMotionData {
        combo_input_data: Some(ComboInputData {
            input_start_time: 0.2,
            next_combo_time: 0.5,
            input_end_time: 0.8,
        }),
        motion_duration: 1.0,
    };
    controller.set_motion_data(motion_data);
    
    controller.update_frame(0.6);
    
    let triggered = controller.trigger_single_attack();
    println!("Triggered: {}, elapsed time: {}", triggered, controller.combo_state.attacking_elapsed_time);
}
