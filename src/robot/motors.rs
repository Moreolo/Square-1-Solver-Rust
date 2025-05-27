// The motors sends commands to the Arduino board through Serial

// The motors need to startup in the correct slice state
// The motors need to be able to go to a slice state
// This means it needs to slice, grab, release
// The motors need to turn the layers
// The motors turn on and off

use std::{io::{Read, Write}, time::Duration};

use serialport::{self, TTYPort};

pub(crate) struct Motors {
    port: TTYPort,
    running: bool,
    slice_pos: i8,
}

impl Motors {
    pub(crate) fn new() -> Self {
        let port = serialport::new("/dev/ttyUSB0", 9600).timeout(Duration::from_secs(2)).open_native().expect("Failed to open USB port");
        // TODO : send init to board
        Self {
            port,
            running: false,
            slice_pos: 0
        }
    }

    pub(crate) fn start(&mut self, slice_pos: Option<i8>) {
        if let Some(new_pos) = slice_pos {
            self.slice_pos = new_pos;
            // TODO : send new slice pos to board
            unimplemented!()
        } else {
            // TODO : send empty startup to board
            unimplemented!()
        }
        self.running = true;
    }

    pub(crate) fn stop(&mut self) {
        // TODO : send stop to board
        unimplemented!();
        self.running = false;
    }

    pub(crate) fn toggle_fast_mode(&mut self) {
        // TODO : send toggle fast mode to board
        unimplemented!()
    }

    pub(crate) fn turn_slice(&mut self) {
        if self.slice_pos != 0 {
            self.slice_pos *= -1;
        }
        // TODO : send slice turn to board
        unimplemented!()
    }

    pub(crate) fn grab(&mut self) {
        if self.slice_pos.abs() < 2 {
            if self.slice_pos < 0 {
                self.slice_pos = -2
            } else {
                self.slice_pos = 2
            }
            // TODO : send slice turn to board
            unimplemented!()
        } else {
            print!("Already grabbed")
        }
    }

    pub(crate) fn release(&mut self) {
        if self.slice_pos.abs() > 1 {
            self.slice_pos /= 2;
            // TODO : send slice turn to board
            unimplemented!()
        } else {
            println!("Already released")
        }
    }

    /// Turns layers of Square-1
    /// Set thumb_to_cam to true for (left, right) usage
    pub(crate) fn turn_layers(&mut self, up: i8, down: i8, thumb_to_cam: bool) {
        if up < -6 || up > 11 || down < -6 || down > 11 {
            self.stop();
            panic!("Layer Turn invalid")
        }
        if self.running {
            self.grab();
            let left = if up < 0 {up + 12} else {up} as u8;
            let right = if down < 0 {down + 12} else {down} as u8;
            let bytes = if thumb_to_cam {
                left << 4 + right
            } else {
                right << 4 + left
            };
            self.send_cmd(bytes);
        } else {
            println!("Motors aren't running")
        }
    }

    fn send_cmd(&mut self, cmd: u8) -> bool {
        self.port.write(&[cmd]).expect("Failed to write command");
        let mut buf: Vec<u8> = vec![0];
        if let Ok(n) = self.port.read(buf.as_mut_slice()) {
            if n > 0 {
                buf[0] == 0
            } else {
                false
            }
        } else {
            false
        }
    }
}