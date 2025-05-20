// The builder builds the Square-1 from camera information

// It is supplied with information about pieces color and shape at specific spots
// It will turn the layers of the Square-1 to see all pieces
// It will reconstruct the Square-1 completly and adjust for config

use crate::square1::Square1;

use super::{cameras::Cameras, partpiece::{PartPiece, Shape}, pictureset::PictureSet};

pub fn detect_square1() -> Option<Square1> {
    let (left_layer, right_layer) = match build_partpiece_layers() {
        Some(x) => x,
        None => return None
    };
    print!("Left Layer");
    for partpiece in &left_layer {
        match partpiece {
            Some(pp) => print!(" - {pp}"),
            None => print!("NONE")
        }
    }
    print!("\nRight Layer");
    for partpiece in &right_layer {
        match partpiece {
            Some(pp) => print!(" - {pp}"),
            None => print!("NONE")
        }
    }
    println!();
    convert_partpieces(left_layer, right_layer)
}

pub fn build_partpiece_layers() -> Option<([Option<PartPiece>; 12], [Option<PartPiece>; 12])> {
    let mut left_layer = [const { None }; 12];
    let mut right_layer = [const { None }; 12];
    // Take multiple pictures
    for pic_num in 0..3 {
        if pic_num != 0 {
            // TODO : move layers
        }
        let pictures = Cameras::capture();

        // Process every spot
        if let Err(spot) = fill_layer(&mut left_layer, &pictures, pic_num, true) {
            println!("Piece overlap on left-{} at spot {}", pic_num, spot);
            return None
        }
        if let Err(spot) = fill_layer(&mut right_layer, &pictures, pic_num, false) {
            println!("Piece overlap on right-{} at spot {}", pic_num, spot);
            return None
        }
    };
    // Correct for offset of layers relative to real turn
    left_layer.rotate_right(2);
    right_layer.rotate_left(4);
    Some((left_layer, right_layer))
}

fn fill_layer(layer: &mut [Option<PartPiece>; 12], pictures: &PictureSet, pic_num: usize, left: bool) -> Result<(), usize> {
    // Checks all spots
    for spot in [1, 0, 2, 3] {
        match layer[pic_num * 4 + spot] {
            Some(_) => {},
            None => match pictures.get_partpiece(left, spot) {
                // If partpiece is detected
                Some(partpiece) => {
                    match partpiece.shape {
                        // Also fill spot next to corner
                        Shape::CornerStart => {
                            let adj_index = (pic_num * 4 + spot + 1) % 12;
                            match layer[adj_index] {
                                // In case of corner edge mixup, return error
                                Some(_) => return Err(spot),
                                None => layer[adj_index] = partpiece.get_adj(left)
                            }
                        }
                        Shape::Edge => {}
                        Shape::CornerEnd => {
                            let adj_index = (12 + pic_num * 4 + spot - 1) % 12;
                            match layer[adj_index] {
                                Some(_) => return Err(spot),
                                None => layer[adj_index] = partpiece.get_adj(left)
                            }
                        }
                    }
                    // Fill this spot
                    layer[pic_num * 4 + spot] = Some(partpiece)
                }
                None => {}
            }
        }
    };
    Ok(())
}

fn convert_partpieces(left_layer: [Option<PartPiece>; 12], right_layer: [Option<PartPiece>; 12]) -> Option<Square1> {
    // Check for undetected partpieces
    let mut cleared_left_layer: Vec<PartPiece> = left_layer.into_iter().filter_map(|piece| piece).collect();
    let mut cleared_right_layer: Vec<PartPiece> = right_layer.into_iter().filter_map(|piece| piece).collect();
    if cleared_left_layer.len() != 12 || cleared_right_layer.len() != 12 {
        println!("Some pieces undetected");
        return None
    }
    // Get the turn to correct the Layers for sliceable
    let left_turn = get_turn_to_valid(&cleared_left_layer);
    let right_turn = get_turn_to_valid(&cleared_right_layer);

    // TODO : turn motors to correct layers
    // make sure to flip the sign of one layer
    if left_turn < 4 {
        cleared_left_layer.rotate_left(left_turn);
    } else {
        cleared_left_layer.rotate_left(left_turn + 6);
    }
    if right_turn < 4 {
        cleared_right_layer.rotate_left(right_turn);
    } else {
        cleared_right_layer.rotate_left(right_turn + 6);
    }

    // Converts partpieces to ids
    let left_pieces: Vec<u8> = cleared_left_layer.into_iter().filter_map(|partpiece| partpiece.get_id(true)).collect();
    let right_pieces: Vec<u8> = cleared_right_layer.into_iter().filter_map(|part_piece| part_piece.get_id(false)).collect();

    // Creates Square-1 from ids
    let mut pieces = [0; 16];
    let split = left_pieces.len();
    pieces[..split].clone_from_slice(&left_pieces);
    pieces[split..].clone_from_slice(&right_pieces);
    let square1 = Square1::from_arr(pieces);
    if square1.is_valid() {
        println!("{square1:?}");
        Some(square1)
    } else {
        println!("Detected Square-1 is invalid");
        None
    }
}

fn get_turn_to_valid(layer: &Vec<PartPiece>) -> usize {
    // Iterates over possible turns
    (0..6).filter_map(|turn| {
        // Checks partpieces at 6 offset for Corner End
        // If Corner End, then the turn is not possible
        match &layer[turn].shape {
            Shape::CornerEnd => None,
            _ => match &layer[turn + 6].shape {
                Shape::CornerEnd => None,
                _ => Some(turn)
            }
        }
    }).fold(None, |acc, pos_turn| {
        match acc {
            Some(turn) => {
                // Decides on the best turn
                let best_diff = (turn as i32 - 3).abs();
                let new_diff = (pos_turn as i32 - 3).abs();
                if new_diff < best_diff {
                    Some(pos_turn)
                } else {
                    Some(turn)
                }
            },
            None => Some(pos_turn)
        }
    }).unwrap()
}