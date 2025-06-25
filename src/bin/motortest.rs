use std::{thread::sleep, time::Duration};

use square_1_solver_rust::robot::{cameras, motors::Motors};


fn main() {
    let mut motors = Motors::new().expect("Failed to connect to motors");

    let pictures = cameras::capture();
    let (thumb_to_cam, bar_solved, _) = pictures.get_slice_config();
    let slice_pos = if thumb_to_cam == bar_solved {-2} else {2};
    //motors.fast_mode();
    motors.start(Some(slice_pos));
    // motors.turn_slice();
    // sleep(Duration::from_millis(2000));
    // motors.turn_slice();
    sleep(Duration::from_millis(10));
    // motors.turn_slice();
    // sleep(Duration::from_millis(10));
    let step_size = 2;
    let total = 12 / step_size;
    for _ in 0..total {
        motors.turn_layers(-step_size, step_size, true);
        sleep(Duration::from_millis(500));
    }
    // motors.turn_layers(6, 6, true);
    // sleep(Duration::from_millis(10));
    // motors.turn_slice();
    motors.stop();
}