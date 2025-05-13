use square_1_solver_rust::robot::cameras::Cameras;

fn main() {
    let pictures = Cameras::capture();
    for i in [0,1,2,3] {
        match pictures.get_partpiece(false, i) {
            None => println!("{}: None", i),
            Some(t) => println!("{}: {}", i, t)
        }
    }
    pictures.save_overlay_config();
}