// The motors sends commands to the Arduino board through Serial

// The motors need to startup in the correct slice state
// The motors need to be able to go to a slice state
// This means it needs to slice, grab, release
// The motors need to turn the layers
// The motors turn on and off

use std::{io::{Read, Write}, thread::sleep, time::Duration};

use serialport::{self, SerialPort, TTYPort};

use super::picconfig::PICCONFIG;

pub struct Motors {
    port: TTYPort,
    running: bool,
    slice_pos: i8
}

impl Motors {
    pub fn new() -> Result<Self, ()> {
        if let Ok(port) = serialport::new(PICCONFIG.get_usb_path(), 115200).timeout(Duration::from_secs(100)).open_native() {
            sleep(Duration::from_secs(1));
            if port.clear(serialport::ClearBuffer::Input).is_err() {
                println!("Failed to clear motor input buffer");
            }
            Ok(Self {
                port,
                running: false,
                slice_pos: 0
            })
        } else {
            println!("Failed to connect to motors");
            Err(())
        }
    }

    pub fn start(&mut self, slice_pos: Option<i8>) {
        if let Some(new_pos) = slice_pos {
            self.slice_pos = new_pos;
        }
        let cmd = ser_slice_pos(self.slice_pos);
        self.send_cmd(0b11110000 + 0b0100 + cmd);
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.send_cmd(0b11110000);
        self.running = false;
    }

    pub(crate) fn slow_mode(&mut self) {
        self.send_cmd(0b11110000 + 0b1100);
    }

    pub fn fast_mode(&mut self) {
        self.send_cmd(0b11110000 + 0b1101);
    }

    pub fn turn_slice(&mut self) {
        if self.slice_pos != 0 {
            self.slice_pos *= -1;
        }
        let cmd = ser_slice_pos(self.slice_pos);
        self.send_cmd(0b11110000 + 0b1000 + cmd);
    }

    pub(crate) fn grab(&mut self) {
        if self.slice_pos < 0 {
            self.slice_pos = -2
        } else {
            self.slice_pos = 2
        }
        let cmd = ser_slice_pos(self.slice_pos);
        self.send_cmd(0b11110000 + 0b1000 + cmd);
    }

    pub(crate) fn _release(&mut self) {
        if self.slice_pos.abs() > 1 {
            self.slice_pos /= 2;
            let cmd = ser_slice_pos(self.slice_pos);
            self.send_cmd(0b11110000 + 0b1000 + cmd);
        } else {
            println!("Already released")
        }
    }

    /// Turns layers of Square-1
    /// Set thumb_to_cam to true for (left, right) usage
    pub fn turn_layers(&mut self, up: i8, down: i8, thumb_to_cam: bool) {
        if up < -6 || up > 11 || down < -6 || down > 11 {
            self.stop();
            panic!("Layer Turn invalid")
        }
        if self.running {
            let left = if up < 0 {up + 12} else {up} as u8;
            let right = if down < 0 {down + 12} else {down} as u8;
            let bytes = if thumb_to_cam {
                (left << 4) + right
            } else {
                (right << 4) + left
            };
            self.send_cmd(bytes);
        } else {
            println!("Motors aren't running")
        }
    }

    fn send_cmd(&mut self, cmd: u8) -> bool {
        sleep(Duration::from_millis(1));
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

fn ser_slice_pos(slice_pos: i8) -> u8 {
    match slice_pos {
        -2 => 0,
        -1 => 1,
        1 => 2,
        2 => 3,
        x => panic!("Slice position {} not valid", x)
    }
}