use std::{process::exit, sync::{Arc, RwLock}, thread::sleep, time::Duration};


fn main() {
    // TODO : put Controller behind lock
    // In case of lock, don't do function
    // don't put emergency stop behind lock, give execution function seperate lock, that checks for emergency stop
    let counter: Arc<RwLock<u8>> = Arc::new(RwLock::new(0));
    inputbot::KeybdKey::QKey.bind(move || {
        println!("q pressed");
        {
            let num = counter.read().unwrap();
            println!("{num}");
        }
        sleep(Duration::from_secs(5));
        {
            let mut num = counter.write().unwrap();
            *num += 1;
        }
    });
    inputbot::KeybdKey::BackspaceKey.bind(|| {
        println!("Backspace pressed");
        exit(0)
    });
    inputbot::KeybdKey::WKey.block_bind(|| {
        println!("w pressed");
        sleep(Duration::from_secs(5));
    });
    inputbot::handle_input_events();
}