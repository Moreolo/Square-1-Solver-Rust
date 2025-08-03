// Controller takes commands

// Controller starts detection, solver and execution
// Controller reacts to emergency stop
// Controller indicates status through lights
// The Controller is started by a seperate file that also takes numpad inputs

use std::{sync::{Arc, Mutex}, thread::sleep, time::{Duration, Instant}};

use crate::{solver::{load_table, solve, Solution}, square1};

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
        println!("Ready");
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

    pub fn scramble(&mut self, stop: &Arc<Mutex<bool>>) {
        cameras::show(&Info::Capture);

        // Take pictures to get slice config information

        let pictures = cameras::capture();
        let (thumb_to_cam, bar_solved, _) = pictures.get_slice_config();

        if !bar_solved {
            println!("Bar not solved, cannot be solved");
        } else {
            self.thumb_to_cam = thumb_to_cam;
            let slice_pos = if thumb_to_cam == bar_solved {-2} else {2};

            cameras::show(&Info::Off);

            let (square1, new_bar_solved) = square1::Square1::scrambled();
            let solution = match solve(square1.clone(), new_bar_solved) {
                Ok(solution) => solution,
                Err(()) => {
                    println!("Failed to scramble");
                    return
                }
            };
            let scramble = solution.inverse();
            println!("{square1:?}, {}", if new_bar_solved {"bar solved"} else {"bar not solved"});
            println!("Scramble: {scramble}");
            println!("Soltion: {solution}");
            self.solution = Some(scramble);

            self.motors.start(Some(slice_pos));
            if self.execute(stop) {
                self.solution = Some(solution);
            }
        } 

        self.show_idle();
    }

    pub fn detect(&mut self, wait: bool) -> bool {
        println!("Detecting...");
        cameras::show(&Info::Capture);

        // Take pictures to get slice config information

        let pictures = cameras::capture();
        let (thumb_to_cam, bar_solved, red_top) = pictures.get_slice_config();
        self.thumb_to_cam = thumb_to_cam;
        let slice_pos = if thumb_to_cam == bar_solved {-2} else {2};
        // if self.fast_mode {
        //     self.motors.slow_mode();
        // }
        self.motors.start(Some(slice_pos));
        self.motors.grab();

        self.solution = if let Some(square1) = detect_square1(&mut self.motors, thumb_to_cam, red_top) {
            match solve(square1, bar_solved) {
                Ok(solution) => {
                    println!("Solution: {}", solution);
                    Some(solution)
                },
                Err(_) => {
                    println!("Square-1 invalid");
                    None
                },
            }
        } else {
            println!("Detection failed");
            None
        };

        let success = match self.solution {
            Some(_) => {
                if wait {
                    self.show_idle();
                    self.motors.stop();
                }
                true
            },
            None => {
                self.motors.stop();
                cameras::blink(&Info::Error);
                self.show_idle();
                false
            }
        };
        // if self.fast_mode {
        //     self.motors.fast_mode();
        // }
        success
    }

    pub fn execute(&mut self, stop: &Arc<Mutex<bool>>) -> bool {
        println!("Executing...");
        {
            *stop.lock().unwrap() = false;
        }
        match &self.solution {
            None => {
                if !self.detect(false) {
                    return false
                }
            },
            _ => {
                cameras::show(&Info::Off);
                self.motors.start(None);
            }
        }
        let now = Instant::now();
        // Execute solution
        let mut first = true;
        for (up,down) in self.solution.as_ref().unwrap().notation.clone() {
            // Emergency stop
            if {
                *stop.lock().unwrap()
            } {
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

        let elapsed = now.elapsed();
        //self.motors.release();
        sleep(Duration::from_millis(100));
        self.motors.stop();
        self.solution = None;
        if {
            *stop.lock().unwrap()
        } {
            println!("Fail safe activated");
            cameras::blink(&Info::Error);
            *stop.lock().unwrap() = false;
            self.show_idle();
            false
        } else {
            println!("Finished in {:.2?}", elapsed);
            self.show_idle();
            true
        }
        
    }

    pub fn start_motors(&mut self) {
        self.motors.start(Some(1));
    }

    pub fn stop_motors(&mut self) {
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