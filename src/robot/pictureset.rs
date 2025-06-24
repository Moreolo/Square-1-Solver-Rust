use image::{imageops::crop_imm, Luma, Rgb};
use imageproc::{definitions::Image, drawing::{draw_filled_circle_mut, draw_hollow_rect_mut}, edges::canny, hough::{detect_lines, LineDetectionOptions}, map::{blue_channel, green_channel, map_colors, red_channel}, rect::Rect};

use super::{partpiece::{PartPiece, Shape, SideColor, UDColor}, picconfig::PICCONFIG};

const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const YELLOW: Rgb<u8> = Rgb([255, 255, 0]);
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const RADIUS: u32 = 6;

pub struct PictureSet {
    image_rgb_left: Image<Rgb<u8>>,
    image_rgb_right: Image<Rgb<u8>>,
    image_hsv_left: Image<Rgb<u8>>,
    image_hsv_right: Image<Rgb<u8>>,
    image_sat_edges_left: Image<Luma<u8>>,
    image_sat_edges_right: Image<Luma<u8>>,
    image_val_edges_left: Image<Luma<u8>>,
    image_val_edges_right: Image<Luma<u8>>
}

impl PictureSet {
    pub(super) fn new(image_rgb_left: Image<Rgb<u8>>, image_rgb_right: Image<Rgb<u8>>) -> Self {
        let low_threshold = 90.;
        let high_threshold = 140.;
        let image_hsv_left = rgb2hsv(&image_rgb_left);
        let image_hsv_right = rgb2hsv(&image_rgb_right);
        let image_sat_left = sat_channel(&image_hsv_left);
        let image_sat_right = sat_channel(&image_hsv_right);
        let image_val_left = val_channel(&image_hsv_left);
        let image_val_right = val_channel(&image_hsv_right);
        let image_sat_edges_left = canny(&image_sat_left, low_threshold, high_threshold);
        let image_sat_edges_right = canny(&image_sat_right, low_threshold, high_threshold);
        let image_val_edges_left = canny(&image_val_left, low_threshold, high_threshold);
        let image_val_edges_right = canny(&image_val_right, low_threshold, high_threshold);

        Self {
            image_rgb_left,
            image_rgb_right,
            image_hsv_left,
            image_hsv_right,
            image_sat_edges_left,
            image_sat_edges_right,
            image_val_edges_left,
            image_val_edges_right
        }
    }

    pub fn get_partpiece(&self, left: bool, id: usize) -> Option<PartPiece> {
        if id > 3 {
            panic!("id too large")
        }
        let left_text = if left {"left"} else {"right"};
        let udcolor = self.get_udcolor(left, id);
        let black = match udcolor {
            UDColor::Black => true,
            UDColor::White => false
        };
        if let Some(shape) = self.get_shape(left, id, black) {
            let alt = match shape {
                Shape::CornerStart => id == 1,
                Shape::Edge => false,
                Shape::CornerEnd => id == 2,
            };
            let partpiece = PartPiece {
                shape,
                udcolor,
                sidecolor: self.get_sidecolor(left, id, alt)
            };
            println!("Piece at {}-{} : {}", left_text, id, partpiece);
            Some(partpiece)
        } else {
            println!("Piece at {}-{} : None", left_text, id);
            None
        }
    }

    // returns the configuration of the slice
    // format: (thumb towards cam, slice solved, small red top)
    pub fn get_slice_config(&self) -> (bool, bool, bool) {
        let thumb_to_cam = self.get_lines(false, 4, false).iter().fold(false, | acc, deg | {
            if acc {
                true
            } else {
                *deg > -20 && *deg < 0
            }
        });
        let slice_solved = thumb_to_cam == self.get_lines(false, 5, false).iter().fold(true, | acc, deg | {
            if acc {
                *deg < 0 || *deg > 20
            } else {
                false
            }
        });
        let red_top = match self.get_sidecolor(false, 4, false) {
            SideColor::Red => thumb_to_cam,
            _ => !thumb_to_cam
        };
        println!("Thumb to cam: {}", thumb_to_cam);
        println!("Slice solved: {}", slice_solved);
        println!("Red top: {}", red_top);
        (thumb_to_cam, slice_solved, red_top)
    }

    // pub fn get_slice_turn(&self, thumb_to_cam: bool, bar_solved: bool) -> i8 {
    //     let bot_thumb_to_cam = thumb_to_cam != bar_solved;
    //     // TODO : output correct direction and figure out if cube is grabbed
    //     unimplemented!()
    // }

    fn get_lines(&self, left: bool, id: usize, black: bool) -> Vec<i32> {
        let left_text = if left {"left"} else {"right"};
        print!("Lines off {}-{} ", left_text, id);
        // process area
        let (edge_image, (x, y, width, height)) = if left {
            (if black {&self.image_val_edges_left} else {&self.image_sat_edges_left}, PICCONFIG.get_area(id))
        } else {
            (if black {&self.image_val_edges_right} else {&self.image_sat_edges_right}, PICCONFIG.get_area(id + 4))
        };
        // crop image
        let cropped_image = crop_imm(edge_image, x, y, width, height).to_image();
        // detect lines
        let options = LineDetectionOptions {
            vote_threshold: 40,
            suppression_radius: 6
        };
        let lines = detect_lines(&cropped_image, options).iter().map(| line | {
            let deg = if line.angle_in_degrees > 90 {
                line.angle_in_degrees as i32 - 180
            } else {
                line.angle_in_degrees as i32
            };
            print!(": {} ", deg);
            deg
        }).collect();
        println!();
        lines
    }

    fn get_shape(&self, left: bool, id: usize, black: bool) -> Option<Shape> {
        self.get_lines(left, id, black).iter().fold( None, | acc, deg | {
            match acc {
                Some(shape) => Some(shape),
                None => {
                    let deg_class = arg_min(PICCONFIG.get_line_classes().iter().map(|diff| (deg - diff).abs()).collect());
                    match deg_class as i32 - id as i32 {
                        0 => if id != 0 {return Some(Shape::CornerStart)} else {None},
                        1 => return Some(Shape::Edge),
                        2 => if id != 3 {return Some(Shape::CornerEnd)} else {None},
                        _ => None
                    }
                }
            }
        })
    }

    fn get_udcolor(&self, left: bool, id: usize) -> UDColor {
        // process spot
        let (val_image, (x, y)) = if left {
            (val_channel(&self.image_hsv_left), PICCONFIG.get_spot(id, false))
        } else {
            (val_channel(&self.image_hsv_right), PICCONFIG.get_spot(id + 8, false))
        };
        // crop image
        let cropped_image = crop_imm(&val_image,
            x-RADIUS,
            y-RADIUS,
            RADIUS * 2 + 1,
            RADIUS * 2 + 1).to_image().to_vec();
        // get median of cropped image
        let median_val = median(cropped_image);
        let left_text = if left {"left"} else {"right"};
        println!("Value of {}-{} : {}", left_text, id, median_val);
        // classify median value
        if median_val < 70 {
            UDColor::Black
        } else {
            UDColor::White
        }
    }

    fn get_sidecolor(&self, left: bool, id: usize, alt: bool) -> SideColor {
        // process spot
        let (hue_image, (x, y)) = if left {
            (hue_channel(&self.image_hsv_left), PICCONFIG.get_spot(id + 4, alt))
        } else {
            (hue_channel(&self.image_hsv_right), PICCONFIG.get_spot(id + 12, alt))
        };
        // crop image
        let cropped_image = crop_imm(&hue_image,
            (x-RADIUS) as u32,
            (y-RADIUS) as u32,
            (RADIUS*2+1) as u32,
            (RADIUS*2+1) as u32).to_image().to_vec();
        // get median of cropped image
        let median_hue = median(cropped_image);
        let left_text = if left {"left"} else {"right"};
        println!("Hue of {}-{} : {}", left_text, id, median_hue);
        // classify median hue
        if median_hue < 11 && (id == 0 || id == 3) {
            SideColor::Red
        } else if median_hue < 20 && (id == 1 || id == 2 || id == 4) {
            SideColor::Red
        } else if median_hue < 60 {
            SideColor::Orange
        } else if median_hue < 100 {
            SideColor::Green
        } else {
            SideColor::Blue
        }
    }

    // TODO : Calibration points and point adjustment for new robot
    pub fn save_overlay_config(&self) {
        let mut config_image_rgb_left = self.image_rgb_left.clone();
        let mut config_image_rgb_right = self.image_rgb_right.clone();
        let mut config_image_edges_left = edg2rgb(&self.image_sat_edges_left);
        let mut config_image_edges_right = edg2rgb(&self.image_sat_edges_right);

        for i in 0..10 {
            let (x, y, width, height) = PICCONFIG.get_area(i);
            if i < 4 {
                draw_hollow_rect_mut(&mut config_image_edges_left, Rect::at(x as i32, y as i32).of_size(width, height), GREEN);
            } else {
                draw_hollow_rect_mut(&mut config_image_edges_right, Rect::at(x as i32, y as i32).of_size(width, height), GREEN);
            }
        }
        for i in 0..17 {
            for alt in [true, false] {
                let (x, y) = PICCONFIG.get_spot(i, alt);
                if i < 8 {
                    draw_filled_circle_mut(&mut config_image_rgb_left, (x as i32, y as i32), RADIUS as i32, BLACK);
                    if i < 4 {
                        draw_filled_circle_mut(&mut config_image_rgb_left, (x as i32, y as i32), (RADIUS-2) as i32, WHITE);
                    } else {
                        draw_filled_circle_mut(&mut config_image_rgb_left, (x as i32, y as i32), (RADIUS-2) as i32, YELLOW);
                    }
                } else {
                    draw_filled_circle_mut(&mut config_image_rgb_right, (x as i32, y as i32), RADIUS as i32, BLACK);
                    if i < 12 {
                        draw_filled_circle_mut(&mut config_image_rgb_right, (x as i32, y as i32), (RADIUS-2) as i32, WHITE);
                    } else {
                        draw_filled_circle_mut(&mut config_image_rgb_right, (x as i32, y as i32), (RADIUS-2) as i32, YELLOW);
                    }
                }
            }
            
        }
        config_image_rgb_left.save("left_spots.jpg").expect("Failed to save image");
        config_image_rgb_right.save("right_spots.jpg").expect("Failed to save image");
        config_image_edges_left.save("left_edges.jpg").expect("Failed to save image");
        config_image_edges_right.save("right_edges.jpg").expect("Failed to save image");
        edg2rgb(&self.image_val_edges_left).save("left_val_edges.jpg").expect("Failed to save image");
        edg2rgb(&self.image_val_edges_right).save("right_val_edges.jpg").expect("Failed to save image");
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

fn sat_channel(img: &Image<Rgb<u8>>) -> Image<Luma<u8>> {
    green_channel(img)
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