// The builder builds the Square-1 from camera information

// It is supplied with information about pieces color and shape at specific spots
// It will turn the layers of the Square-1 to see all pieces
// It will reconstruct the Square-1 completly and adjust for config

use crate::square1::Square1;

use super::{cameras::Cameras, partpiece::{PartPiece, Shape}};

pub(crate) fn build_partpiece_layers() -> ([Option<PartPiece>; 12], [Option<PartPiece>; 12]) {
    let mut left_layer = [const { None }; 12];
    let mut right_layer = [const { None }; 12];
    // take multiple pictures
    for pic_num in 0..3 {
        if pic_num != 0 {
            // TODO : move layers
        }
        let pictures = Cameras::capture();

        // process every spot
        for id in [1, 0, 2, 3] {
            // check if spot not yet detected
            match left_layer[pic_num * 4 + id] {
                Some(_) => {},
                // detect partpiece at spot
                None => match pictures.get_partpiece(true, id) {
                    // fill adjacent spot in case of corner
                    Some(partpiece) => {
                        match partpiece.shape {
                            Shape::CornerStart => left_layer[(pic_num * 4 + id + 1) % 12] = partpiece.get_adj(true),
                            Shape::Edge => {},
                            Shape::CornerEnd => left_layer[(12 + pic_num * 4 + id - 1) % 12] = partpiece.get_adj(true)
                        };
                        // fill current spot
                        left_layer[pic_num * 4 + id] = Some(partpiece);
                    },
                    None => {}
                }
            }
            // same as left
            match right_layer[pic_num * 4 + id] {
                Some(_) => {},
                None => match pictures.get_partpiece(false, id) {
                    Some(partpiece) => {
                        match partpiece.shape {
                            Shape::CornerStart => right_layer[(pic_num * 4 + id + 1) % 12] = partpiece.get_adj(false),
                            Shape::Edge => {},
                            Shape::CornerEnd => right_layer[(12 + pic_num * 4 + id - 1) % 12] = partpiece.get_adj(false)
                        };
                        right_layer[pic_num * 4 + id] = Some(partpiece);
                    },
                    None => {}
                }
            }
        }
    };
    left_layer.rotate_right(2);
    right_layer.rotate_left(4);
    (left_layer, right_layer)
}

fn convert_partpieces(left_layer: [Option<PartPiece>; 12], right_layer: [Option<PartPiece>; 12]) -> Result<Square1, ()> {
    unimplemented!()
}