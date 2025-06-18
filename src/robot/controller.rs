// Controller takes commands

// Controller starts detection, solver and execution
// Controller reacts to emergency stop
// Controller indicates status through lights
// The Controller is started by a seperate file that also takes numpad inputs

use std::sync::{Arc, Mutex};

use crate::solver::{load_table, solve, Solution};

use super::{builder::detect_square1, cameras::{self, Info}, motors::Motors};

pub struct Controller {
    motors: Motors,
    fast_mode: bool,
    solution: Option<Solution>,
    thumb_to_cam: bool
}

impl Controller {
    pub fn new() -> Result<Self, ()> {
        if cameras::is_working() {
            if let Ok(motors) = Motors::new() {
                Ok(Controller {
                    motors,
                    fast_mode: false,
                    solution: None,
                    thumb_to_cam: true
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
        
    }

    pub fn init(&mut self) {
        println!("Starting Robot");
        cameras::show(&Info::Init);
        load_table();
        self.show_idle();
    }

    pub fn toggle_fast_mode(&mut self) {
        self.fast_mode = !self.fast_mode;
        if self.fast_mode {
            self.motors.fast_mode();
        } else {
            self.motors.slow_mode();
        }
        self.show_idle();
    }

    pub fn detect(&mut self, wait: bool) -> bool {
        cameras::show(&Info::Off);

        // Take pictures to get slice config information
        let pictures = cameras::capture();
        let (thumb_to_cam, bar_solved, red_top) = pictures.get_slice_config();
        self.thumb_to_cam = thumb_to_cam;
        let slice_pos = if thumb_to_cam == bar_solved {-2} else {2};
        self.motors.start(Some(slice_pos));
        self.motors.grab();

        self.solution = if let Some(square1) = detect_square1(&mut self.motors, thumb_to_cam, red_top) {
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
        };
        match self.solution {
            Some(_) => {
                if wait {
                    self.show_idle();
                    self.motors.stop();
                }
                true
            },
            None => {
                cameras::blink(&Info::Error);
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
                cameras::show(&Info::Off);
                self.motors.start(None);
            }
        }

        // Execute solution
        let mut first = true;
        for (up,down) in self.solution.as_ref().unwrap().notation.clone() {
            // Emergency stop
            if {
                *stop.lock().unwrap()
            } {
                cameras::blink(&Info::Error);
                *stop.lock().unwrap() = false;
                break
            }
            if !first {
                // Slice
                self.motors.turn_slice();
            } else {
                first = false;
            }
            // Layerturn
            self.motors.turn_layers(up, down, self.thumb_to_cam);
        }

        self.motors.release();
        // Reset state
        self.solution = None;
        self.show_idle();
        self.motors.stop();
        
    }

    pub fn quit(&mut self) {
        println!("Stopping Robot");
        cameras::show(&Info::Off);
        self.motors.stop();
    }

    fn show_idle(&mut self) {
        if self.fast_mode {
            cameras::show(&Info::Fast);
        } else {
            cameras::show(&Info::Normal);
        }
    }
}