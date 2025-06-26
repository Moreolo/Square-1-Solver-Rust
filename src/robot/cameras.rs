// The cameras see the cube and supply the lights

// The cameras supply a function to take pictures of the Square-1
// The cameras supply functions to get shape and color information about specified spots
// The cameras supply a function to control the lights

use std::{process::Command, thread::sleep, time::Duration};

use image::ImageReader;
use ws2818_rgb_led_spi_driver::{adapter_gen::WS28xxAdapter, adapter_spi::WS28xxSpiAdapter, encoding::encode_rgb_slice};

use crate::robot::picconfig::PICCONFIG;

use super::pictureset::PictureSet;

pub(crate) enum Info {
    Off,
    Init,
    Normal,
    Fast,
    Error,
    Capture
}

pub(crate) fn is_working() -> bool {
    get_spi_device().is_ok() && raw_capture().is_ok()
}

pub(crate) fn show(info: &Info) {
    let mut spi_adapter = get_spi_device().unwrap();
    raw_show(&mut spi_adapter, info);
}

fn raw_show(spi_adapter: &mut WS28xxSpiAdapter, info: &Info) {
    let rgb = match info {
        Info::Off => (0, 0, 0),
        Info::Init => (64, 32, 0),
        Info::Normal => (0, 64, 0),
        Info::Fast => (0, 0, 64),
        Info::Error => (64, 0, 0),
        Info::Capture => (255, 255, 255)
    };
    let encoded_data = encode_rgb_slice(&[rgb; 24]);
    spi_adapter.write_encoded_rgb(&encoded_data).expect("Failed to change leds");
}

fn get_spi_device() -> Result<WS28xxSpiAdapter, String> {
    WS28xxSpiAdapter::new(&PICCONFIG.get_spi_path())
}

pub(crate) fn blink(info: &Info) {
    let mut spi_adapter = get_spi_device().unwrap();
    for _ in 0..3 {
        raw_show(&mut spi_adapter, &info);
        sleep(Duration::from_millis(500));
        raw_show(&mut spi_adapter, &Info::Off);
        sleep(Duration::from_millis(500));
    }
}

pub fn capture_with_lights() -> PictureSet {
    let mut spi_adapter = get_spi_device().unwrap();
    raw_show(&mut spi_adapter, &Info::Capture);
    let pictures = capture();
    raw_show(&mut spi_adapter, &Info::Off);
    pictures
}

pub fn capture() -> PictureSet {
    println!("Capturing Pictures");
    raw_capture().unwrap();

    let image_left = ImageReader::open("left.jpg")
        .expect("failed to load image")
        .decode()
        .expect("failed to decode image")
        .into();
    let image_right = ImageReader::open("right.jpg")
        .expect("failed to load image")
        .decode()
        .expect("failed to decode image")
        .into();
    PictureSet::new(image_left, image_right)
}

pub(crate) fn raw_capture() -> Result<(), ()> {
    // TODO : Check if capture actually returns Error in case capture fails
    for config in ["config_cam_left.txt", "config_cam_right.txt"] {
        if Command::new("rpicam-still")
            .arg("--config").arg(config)
            .output().is_err() {return Err(())};
    }
    Ok(())
}
