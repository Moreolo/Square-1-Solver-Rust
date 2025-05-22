// Controller takes commands

// Controller starts detection, solver and execution
// Controller reacts to emergency stop
// Controller indicates status through lights
// The Controller is started by a seperate file that also takes numpad inputs

use std::sync::{Arc, RwLock};

use crate::{solver::{load_table, solve, Solution}, square1::Square1};

use super::{builder::detect_square1, cameras::{Cameras, Show}};

pub struct Controller {
    solution: Option<Solution>
}

impl Controller {
    pub fn init() {
        println!("Starting Robot");
        Cameras::show(&Show::Init);
        load_table();
        Cameras::start();
        // TODO : setup serial for motors
        Cameras::show(&Show::Normal);
    }

    pub fn detect(&mut self) {
        if let Some((square1, (thumb_to_cam, bar_solved, _))) = detect_square1() {
            // TODO : setup motors for thumb to cam and for slice direction
            match solve(square1, bar_solved) {
                Ok(solution) => self.solution = Some(solution),
                Err(_) => {
                    println!("Square-1 invalid");
                    self.solution = None
                },
            }
        } else {
            println!("Detection failed");
            Cameras::blink(&Show::Error);
            self.solution = None
            // TODO : return to normal state
            // TODO : blink outside of if else statement
        }
    }

    pub fn execute(&mut self, stop: Arc<RwLock<bool>>) {
        match &self.solution {
            None => {
                self.detect();
                if let None = self.solution {
                    return
                }
            },
            _ => {}
        }
        match &self.solution {
            Some(solution) => {
                // TODO : execute solution
                if {
                    !*stop.read().unwrap()
                } {

                } else {
                    // TODO : stop execution
                    *stop.write().unwrap() = false;
                }
                self.solution = None;
            }
            None => {
                // TODO : blink red
            }
        }
        
    }

    pub fn quit() {
        println!("Stopping Robot");
        Cameras::stop();
        // TODO : stop motors
    }
}