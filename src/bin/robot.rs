use std::{process::exit, sync::{Arc, Mutex}};

use square_1_solver_rust::robot::controller::Controller;

fn main() {
    let contr_base: Arc<Mutex<Controller>> = Arc::new(Mutex::new(Controller::new().expect("Failed to create controller")));
    let stop: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    {
        contr_base.lock().unwrap().init();
    }

    let contr_7 = Arc::clone(&contr_base);
    inputbot::KeybdKey::HomeKey.bind(move || {
        println!("Numpad 7 / Home pressed");
        if let Ok(mut contr) = contr_7.try_lock() {
            contr.quit();
            exit(0)
        }
    });

    let contr_d = Arc::clone(&contr_base);
    inputbot::KeybdKey::DeleteKey.bind(move || {
        println!(". / Delete pressed");
        if let Ok(mut contr) = contr_d.try_lock() {
            contr.detect(true);
        }
    });

    let contr_0 = Arc::clone(&contr_base);
    let stop_0 = Arc::clone(&stop);
    inputbot::KeybdKey::InsertKey.bind(move || {
        println!("Numpad 0 / Insert pressed");
        if let Ok(mut contr) = contr_0.try_lock() {
            contr.execute(&stop_0);
        }
    });

    let stop_backspace = Arc::clone(&stop);
    inputbot::KeybdKey::BackspaceKey.bind(move || {
        println!("Backspace pressed");
        *stop_backspace.lock().unwrap() = true;
    });

    let contr_numlock = Arc::clone(&contr_base);
    inputbot::KeybdKey::NumLockKey.bind(move || {
        println!("Numlock pressed");
        if let Ok(mut contr) = contr_numlock.try_lock() {
            contr.toggle_fast_mode();
        }
    });

    inputbot::handle_input_events();
}