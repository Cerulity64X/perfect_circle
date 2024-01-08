use std::{time::Duration, thread, f64::consts::TAU};

use enigo::{Enigo, MouseControllable, MouseButton};
use mouse_position::mouse_position::Mouse;

fn main() {
    let mut eni = Enigo::new();
    // wait 1 second to position cursor
    let wait_dur = Duration::from_secs(1);
    thread::sleep(wait_dur);
    let (sx, sy) = match Mouse::get_mouse_position() {
        Mouse::Error => panic!("Could not get mouse position!"),
        Mouse::Position { x, y } => (x, y)
    };
    let steps = 100;
    let interval = Duration::from_millis(10);
    let radius = 400.0;
    eni.mouse_move_to(sx, sy + radius as i32);
    eni.mouse_down(MouseButton::Left);
    for i in 0..steps {
        let period = i as f64 / steps as f64 * TAU;
        let x = -period.sin() * radius;
        let y = period.cos() * radius;
        eni.mouse_move_to(sx + x as i32, sy + y as i32);
        thread::sleep(interval);
    }
    eni.mouse_up(MouseButton::Left);
}
