use square_1_solver_rust::robot::cameras;


fn main() {
    let pictures = cameras::capture_with_lights();
    for left in [true, false] {
        (0..2).for_each(| id | {pictures.get_partpiece(left, id);});
    }
    let _ = pictures.get_slice_config();
    pictures.save_overlay_config();
}