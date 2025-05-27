// Controller takes commands

// Controller starts detection, solver and execution
// Controller reacts to emergency stop
// Controller indicates status through lights
// The Controller is started by a seperate file that also takes numpad inputs

use std::sync::{Arc, Mutex};

use crate::{solver::{load_table, solve, Solution}, square1::Square1};

use super::{builder::detect_square1, cameras::{Cameras, Show}, motors::Motors};

pub struct Controller {
    motors: Motors,
    fast_mode: bool,
    solution: Option<Solution>
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            motors: Motors::new(),
            fast_mode: false,
            solution: None
        }
    }

    pub fn init(&self) {
        println!("Starting Robot");
        Cameras::show(&Show::Init);
        load_table();
        Cameras::start();
        self.show_idle();
    }

    pub fn toggle_fast_mode(&mut self) {
        self.fast_mode = !self.fast_mode;
        self.motors.toggle_fast_mode();
    }

    pub fn detect(&mut self, wait: bool) -> bool {
        Cameras::show(&Show::Off);
        // TODO : turn on motors
        // TODO : take config information first and parse into detection function
        self.motors.start(unimplemented!());
        self.solution = if let Some((square1, (thumb_to_cam, bar_solved, _))) = detect_square1() {
            // TODO : setup motors for thumb to cam and for slice direction
            match solve(square1, bar_solved) {
                Ok(solution) => Some(solution),
                Err(_) => {
                    println!("Square-1 invalid");
                    None
                },
            }
        } else {
            println!("Detection failed");
            None
            // TODO : return to normal state
            // TODO : blink outside of if else statement
        };
        match self.solution {
            Some(_) => {
                if wait {
                    self.motors.stop();
                }
                true
            },
            None => {
                Cameras::blink(&Show::Error);
                self.motors.stop();
                false
            }
        }
    }

    pub fn execute(&mut self, stop: &Arc<Mutex<bool>>) {
        {
            *stop.lock().unwrap() = false;
        }
        match &self.solution {
            None => {
                if !self.detect(false) {
                    return
                }
            },
            _ => {
                Cameras::show(&Show::Off);
                // TODO : turn on motors
                self.motors.start(unimplemented!());
            }
        }
        // TODO : execute solution
        
        if {
            !*stop.lock().unwrap()
        } {

        } else {
            // TODO : stop execution
            *stop.lock().unwrap() = false;
        }
        self.solution = None;
        self.show_idle();
        self.motors.stop();
        
    }

    pub fn quit(&mut self) {
        println!("Stopping Robot");
        Cameras::show(&Show::Off);
        Cameras::stop();
        self.motors.stop();
    }

    fn show_idle(&self) {
        if self.fast_mode {
            Cameras::show(&Show::Fast);
        } else {
            Cameras::show(&Show::Normal);
        }
    }
}