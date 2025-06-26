
use std::{fs, sync::LazyLock};

use serde_derive::Deserialize;

use crate::robot::partpiece::Shape;

pub(crate) static PICCONFIG: LazyLock<PicConfig> = LazyLock::new(|| PicConfig::from_file());

#[derive(Deserialize)]
pub(crate) struct PicConfig {
    spi: u8,
    usb_port: u8,
    line_classes: [i32; 6],
    spots_left_ud: [(u32, u32); 2],
    spots_left_side_e: [(u32, u32); 2],
    spots_left_side_cs: [(u32, u32); 2],
    spots_left_side_ce: [(u32, u32); 2],
    spots_right_ud: [(u32, u32); 2],
    spots_right_side_e: [(u32, u32); 2],
    spots_right_side_cs: [(u32, u32); 2],
    spots_right_side_ce: [(u32, u32); 2],
    spot_extra: (u32, u32),
    areas_left: [(u32, u32, u32, u32); 2],
    areas_right: [(u32, u32, u32, u32); 2],
    area_upper_slice: (u32, u32, u32, u32),
    area_lower_slice: (u32, u32, u32, u32)
}

impl PicConfig {
    pub(crate) fn from_file() -> Self {
        let contents = fs::read_to_string("picconfig.toml").expect("Failed to load picture config file");
        toml::from_str(&contents).expect("Failed to parse picture config file")
    }

    pub(crate) fn get_spi_path(&self) -> String {
        format!("/dev/spidev{}.0", self.spi)
    }

    pub(crate) fn get_usb_path(&self) -> String {
        format!("/dev/ttyACM{}", self.usb_port)
    }

    pub(crate) fn get_line_classes(&self) -> [i32; 6] {
        self.line_classes
    }

    pub(crate) fn get_spot(&self, id: usize, shape: Shape) -> (u32, u32) {
        let field = id / 2;
        let index = id % 2;
        match field {
            0 => self.spots_left_ud[index],
            1 => match shape {
                Shape::CornerStart => self.spots_left_side_cs[index],
                Shape::Edge => self.spots_left_side_e[index],
                Shape::CornerEnd => self.spots_left_side_ce[index],
            }
            2 => self.spots_right_ud[index],
            3 => match shape {
                Shape::CornerStart => self.spots_right_side_cs[index],
                Shape::Edge => self.spots_right_side_e[index],
                Shape::CornerEnd => self.spots_right_side_ce[index],
            }
            4 => if index == 0 {
                self.spot_extra
            } else {
                panic!("Spot id out of range")
            }
            _ => panic!("Spot id out of range")
        }
    }

    pub(crate) fn get_area(&self, id: usize) -> (u32, u32, u32, u32) {
        let field = id / 2;
        let index = id % 2;
        match field {
            0 => self.areas_left[index],
            1 => self.areas_right[index],
            2 => if index == 0 {
                self.area_upper_slice
            } else if index == 1 {
                self.area_lower_slice
            } else {
                panic!("Area id out of range")
            }
            _ => panic!("Area id out of range")
        }
    }
}