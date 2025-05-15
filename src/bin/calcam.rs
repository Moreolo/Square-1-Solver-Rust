use square_1_solver_rust::robot::cameras::Cameras;

fn main() {
    let pictures = Cameras::capture();
    for left in [true, false] {
        (0..4).for_each(| id | {pictures.get_partpiece(left, id);});
    }
    let _ = pictures.get_slice_config();
    pictures.save_overlay_config();
}