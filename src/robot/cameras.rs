// The cameras see the cube and supply the lights

// The cameras supply a function to take pictures of the Square-1
// The cameras supply functions to get shape and color information about specified spots
// The cameras supply a function to control the lights

use std::{thread::sleep, time::Duration};

use image::ImageReader;

use super::pictureset::PictureSet;

pub(crate) enum Show {
    Off,
    Init,
    Normal,
    Fast,
    Error
}

pub struct Cameras {

}

impl Cameras {
    pub(crate) fn new() -> Self {
        unimplemented!()
    }

    pub(crate) fn start() {
        println!("Starting Cameras");
        unimplemented!()
    }

    pub(crate) fn stop() {
        println!("Stopping Cameras");
        unimplemented!()
    }

    pub(crate) fn show(show: &Show) {
        // TODO: leds
        unimplemented!()
    }

    pub(crate) fn blink(show: &Show) {
        for _ in 0..3 {
            Cameras::show(&show);
            sleep(Duration::from_secs(1));
            Cameras::show(&Show::Off);
            sleep(Duration::from_secs(1));
        }
    }

    pub fn capture() -> PictureSet {
        println!("Capturing Pictures");
        // TODO: replace fake with real
        let image_left = ImageReader::open("right_0.jpg")
            .expect("failed to load image")
            .decode()
            .expect("failed to decode image")
            .into();
        let image_right = ImageReader::open("right_1.jpg")
            .expect("failed to load image")
            .decode()
            .expect("failed to decode image")
            .into();
        PictureSet::new(image_left, image_right)
    }
}