use inputbot::MouseCursor;
use rand::Rng;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let x: i32 = rng.gen_range(1, 7);
        let y: i32 = rng.gen_range(1, 7);
        let s: u64 = rng.gen_range(1, 6);
        MouseCursor.move_rel(x, y);
        sleep(Duration::from_secs(s));
    }
}
