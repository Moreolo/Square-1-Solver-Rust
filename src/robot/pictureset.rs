use image::{imageops::crop_imm, Luma, Rgb};
use imageproc::{definitions::Image, drawing::{draw_filled_circle_mut, draw_hollow_rect_mut}, edges::canny, hough::{detect_lines, LineDetectionOptions}, map::{blue_channel, map_colors, red_channel}, rect::Rect};

use super::partpiece::{PartPiece, Shape, SideColor, UDColor};

const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const YELLOW: Rgb<u8> = Rgb([255, 255, 0]);
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const RADIUS: u32 = 6;

const LINE_DIFF: [i32; 6] = [-15, 0, 16, 24, 40, 55];

const AREAS: [(u32, u32, u32, u32); 10] = [
    // left
    (100, 200, 20, 20),
    (90, 170, 20, 20),
    (80, 140, 20, 20),
    (100, 110, 20, 20),
    // right
    (180, 510, 100, 80),
    (150, 320, 80, 100),
    (80, 140, 80, 150),
    (20, 50, 50, 50),
    // extra
    (380, 0, 80, 150),
    (520, 470, 160, 100)
];

const SPOTS: [(u32, u32); 17] = [
    // left ud
    (100, 200),
    (90, 170),
    (80, 140),
    (100, 110),
    // left side
    (100, 200),
    (90, 170),
    (80, 140),
    (70, 110),
    // right ud
    (170, 500),
    (150, 390),
    (100, 250),
    (40, 150),
    // right side
    (320, 530),
    (280, 360),
    (200, 180),
    (100, 60),
    // extra
    (470, 350)
];

pub struct PictureSet {
    image_rgb_left: Image<Rgb<u8>>,
    image_rgb_right: Image<Rgb<u8>>,
    image_hsv_left: Image<Rgb<u8>>,
    image_hsv_right: Image<Rgb<u8>>,
    image_edges_left: Image<Luma<u8>>,
    image_edges_right: Image<Luma<u8>>
}

impl PictureSet {
    pub(super) fn new(image_rgb_left: Image<Rgb<u8>>, image_rgb_right: Image<Rgb<u8>>) -> Self {
        let low_threshold = 50.;
        let high_threshold = 80.;
        let image_hsv_left = rgb2hsv(&image_rgb_left);
        let image_hsv_right = rgb2hsv(&image_rgb_right);
        let image_value_left = val_channel(&image_hsv_left);
        let image_value_right = val_channel(&image_hsv_right);
        let image_edges_left = canny(&image_value_left, low_threshold, high_threshold);
        let image_edges_right = canny(&image_value_right, low_threshold, high_threshold);

        Self {
            image_rgb_left,
            image_rgb_right,
            image_hsv_left,
            image_hsv_right,
            image_edges_left,
            image_edges_right
        }
    }

    pub fn get_partpiece(&self, left: bool, id: usize) -> Option<PartPiece> {
        if let Some(shape) = self.get_shape(left, id) {
            Some(PartPiece {
                shape,
                udcolor: self.get_udcolor(left, id),
                sidecolor: self.get_sidecolor(left, id)
            })
        } else {
            None
        }
    }

    fn get_shape(&self, left: bool, id: usize) -> Option<Shape> {
        if id > 3 {
            panic!("id too large")
        }
        // process area
        let (edge_image, (x, y, width, height)) = if left {
            (&self.image_edges_right, AREAS[id])
        } else {
            (&self.image_edges_right, AREAS[id+4])
        };
        // crop image
        let cropped_image = crop_imm(edge_image, x, y, width, height).to_image();
        // detect lines
        let options = LineDetectionOptions {
            vote_threshold: 40,
            suppression_radius: 6
        };
        let lines = detect_lines(&cropped_image, options);
        for line in lines {
            let deg = if line.angle_in_degrees > 90 {
                if left {
                line.angle_in_degrees as i32 - 180
                } else {
                    180 - (line.angle_in_degrees as i32)
                }
            } else {
                if left {
                line.angle_in_degrees as i32
                } else {
                    -(line.angle_in_degrees as i32)
                }
            };
            println!("{}", deg);
            let deg_class = arg_min(LINE_DIFF.iter().map(|diff| (deg - diff).abs()).collect());
            match deg_class as i32 - id as i32 {
                0 => if id != 0 {return Some(Shape::CornerStart)}
                1 => return Some(Shape::Edge),
                2 => if id != 3 {return Some(Shape::CornerEnd)}
                _ => {}
            }
        }
        None
    }

    fn get_udcolor(&self, left: bool, id: usize) -> UDColor {
        if id > 3 {
            panic!("id too large")
        }
        // process spot
        let (val_image, (x, y)) = if left {
            (val_channel(&self.image_hsv_left), SPOTS[id])
        } else {
            (val_channel(&self.image_hsv_right), SPOTS[id+8])
        };
        // crop image
        let cropped_image = crop_imm(&val_image,
            x-RADIUS,
            y-RADIUS,
            RADIUS * 2 + 1,
            RADIUS * 2 + 1).to_image().to_vec();
        // get median of cropped image
        let median_val = median(cropped_image);
        // classify median value
        if median_val < 70 {
            UDColor::Black
        } else {
            UDColor::White
        }
    }

    fn get_sidecolor(&self, left: bool, id: usize) -> SideColor {
        if id > 3 {
            panic!("id too large")
        }
        // process spot
        let (hue_image, (x, y)) = if left {
            (hue_channel(&self.image_hsv_left), SPOTS[id+4])
        } else {
            (hue_channel(&self.image_hsv_right), SPOTS[id+12])
        };
        // crop image
        let cropped_image = crop_imm(&hue_image,
            (x-RADIUS) as u32,
            (y-RADIUS) as u32,
            (RADIUS*2+1) as u32,
            (RADIUS*2+1) as u32).to_image().to_vec();
        // get median of cropped image
        let median_hue = median(cropped_image);
        // classify median hue
        if median_hue < 25 {
            SideColor::Red
        } else if median_hue < 60 {
            SideColor::Orange
        } else if median_hue < 100 {
            SideColor::Green
        } else {
            SideColor::Blue
        }
    }

    pub fn save_overlay_config(&self) {
        let mut config_image_rgb_left = self.image_rgb_left.clone();
        let mut config_image_rgb_right = self.image_rgb_right.clone();
        let mut config_image_edges_left = edg2rgb(&self.image_edges_left);
        let mut config_image_edges_right = edg2rgb(&self.image_edges_right);

        for (x, y, width, height) in &AREAS[..4] {
            draw_hollow_rect_mut(&mut config_image_edges_left, Rect::at(*x as i32, *y as i32).of_size(*width, *height), GREEN);
        }
        for (x, y, width, height) in &AREAS[4..] {
            draw_hollow_rect_mut(&mut config_image_edges_right, Rect::at(*x as i32, *y as i32).of_size(*width, *height), GREEN);
        }
        for (x, y) in &SPOTS[..4] {
            draw_filled_circle_mut(&mut config_image_rgb_left, (*x as i32, *y as i32), RADIUS as i32, BLACK);
            draw_filled_circle_mut(&mut config_image_rgb_left, (*x as i32, *y as i32), (RADIUS-2) as i32, WHITE);
        }
        for (x, y) in &SPOTS[4..8] {
            draw_filled_circle_mut(&mut config_image_rgb_left, (*x as i32, *y as i32), RADIUS as i32, BLACK);
            draw_filled_circle_mut(&mut config_image_rgb_left, (*x as i32, *y as i32), (RADIUS-2) as i32, YELLOW);
        }
        for (x, y) in &SPOTS[8..12] {
            draw_filled_circle_mut(&mut config_image_rgb_right, (*x as i32, *y as i32), RADIUS as i32, BLACK);
            draw_filled_circle_mut(&mut config_image_rgb_right, (*x as i32, *y as i32), (RADIUS-2) as i32, WHITE);
        }
        for (x, y) in &SPOTS[12..] {
            draw_filled_circle_mut(&mut config_image_rgb_right, (*x as i32, *y as i32), RADIUS as i32, BLACK);
            draw_filled_circle_mut(&mut config_image_rgb_right, (*x as i32, *y as i32), (RADIUS-2) as i32, YELLOW);
        }
        config_image_rgb_left.save("left_spots.jpg").expect("failed to save image");
        config_image_rgb_right.save("right_spots.jpg").expect("failed to save image");
        config_image_edges_left.save("left_edges.jpg").expect("failed to save image");
        config_image_edges_right.save("right_edges.jpg").expect("failed to save image");
    }
}

fn rgb2hsv(img: &Image<Rgb<u8>>) -> Image<Rgb<u8>> {
    map_colors(img, |rgb| {
        let max = rgb.0.iter().max().unwrap();
        let min = rgb.0.iter().min().unwrap();
        let diff = max - min;
        let pre_hue = if max == min {
            0.
        } else if max == &rgb[0] {
            (rgb[1] as f32 - rgb[2] as f32) / diff as f32
        } else if max == &rgb[1] {
            2. + (rgb[2] as f32 - rgb[0] as f32) / diff as f32
        } else {
            4. + (rgb[0] as f32 - rgb[1] as f32) / diff as f32
        };
        let pre_sat = diff as f32 / *max as f32;
        let pre_value = *max as f32 / 256.;
        Rgb([
            (if pre_hue < 0. {pre_hue + 6.} else {pre_hue} * 256. / 6.) as u8,
            (pre_sat * 256.) as u8,
            (pre_value * 256.) as u8
        ])
    })
}

fn hue_channel(img: &Image<Rgb<u8>>) -> Image<Luma<u8>> {
    red_channel(img)
}

fn val_channel(img: &Image<Rgb<u8>>) -> Image<Luma<u8>> {
    blue_channel(img)
}

fn edg2rgb(img: &Image<Luma<u8>>) -> Image<Rgb<u8>> {
    map_colors(img, | p | {
        let v = 255 - p[0];
        Rgb([v, v, v])
    })
}

fn median<A: Copy + Ord>(array: Vec<A>) -> A {
    let mut array_clone = array.clone();
    array_clone.sort();
    array_clone[array_clone.len() / 2]
}

fn arg_min<A: Ord>(array: Vec<A>) -> usize {
    array.iter().enumerate().min_by_key(|(_, elem)| *elem).unwrap().0
}