// Controller takes commands

// Controller starts detection, solver and execution
// Controller reacts to emergency stop
// Controller indicates status through lights
// The Controller is started by a seperate file that also takes numpad inputs

use std::sync::{Arc, Mutex};

use crate::{solver::{load_table, solve, Solution}, square1::Square1};

use super::{builder::detect_square1, cameras::{Cameras, Show}};

pub struct Controller {
    solution: Option<Solution>
}

impl Controller {
    pub fn new() -> Self {
        Controller { solution: None }
    }

    pub fn init(&self) {
        println!("Starting Robot");
        Cameras::show(&Show::Init);
        load_table();
        Cameras::start();
        // TODO : setup serial for motors
        Cameras::show(&Show::Normal);
    }

    pub fn switch_fast_mode(&mut self) {
        unimplemented!()
        // TODO : send command to motors
    }

    pub fn detect(&mut self) -> bool {
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
            Some(_) => true,
            None => {
                Cameras::blink(&Show::Error);
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
                if !self.detect() {
                    return
                }
            },
            _ => {}
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
        
    }

    pub fn quit(&self) {
        println!("Stopping Robot");
        Cameras::stop();
        // TODO : stop motors
    }
}