use std::{process::exit, sync::{Arc, Mutex}};

use square_1_solver_rust::robot::controller::Controller;

fn main() {
    let contr_base: Arc<Mutex<Controller>> = Arc::new(Mutex::new(Controller::new().expect("Failed to create controller")));
    let stop: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    {
        contr_base.lock().unwrap().init();
    }

    let contr_esc = Arc::clone(&contr_base);
    inputbot::KeybdKey::EscapeKey.bind(move || {
        println!("Escape pressed");
        if let Ok(mut contr) = contr_esc.try_lock() {
            contr.quit();
            exit(0)
        }
    });

    let contr_e = Arc::clone(&contr_base);
    inputbot::KeybdKey::EKey.bind(move || {
        println!("e pressed");
        if let Ok(mut contr) = contr_e.try_lock() {
            contr.detect(true);
        }
    });

    let contr_enter = Arc::clone(&contr_base);
    let stop_enter = Arc::clone(&stop);
    inputbot::KeybdKey::EnterKey.bind(move || {
        println!("enter pressed");
        if let Ok(mut contr) = contr_enter.try_lock() {
            contr.execute(&stop_enter);
        }
    });

    let contr_backspace = Arc::clone(&contr_base);
    let stop_backspace = Arc::clone(&stop);
    inputbot::KeybdKey::BackspaceKey.block_bind(move || {
        println!("Backspace pressed");
        match contr_backspace.try_lock() {
            Ok(mut contr) => contr.toggle_fast_mode(),
            Err(_) => *stop_backspace.lock().unwrap() = true
        };
    });

    inputbot::handle_input_events();
}