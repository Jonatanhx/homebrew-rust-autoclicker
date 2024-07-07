use enigo::{Button, Direction::Press, Enigo, Mouse, Settings};
use inputbot::KeybdKey::*;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread, time,
};

fn main() {
    let clicker_is_active = Arc::new(AtomicBool::new(false));
    let clicker_is_active_clone = Arc::clone(&clicker_is_active);

    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        loop {
            if clicker_is_active_clone.load(Ordering::Relaxed) {
                // Handle the Result returned by enigo.button
                let _ = enigo.button(Button::Left, Press);
                println!("Mouse click!");
                thread::sleep(time::Duration::from_millis(1));
            }
        }
    });

    F6Key.bind(move || {
        let current_state = clicker_is_active.load(Ordering::Relaxed);
        clicker_is_active.store(!current_state, Ordering::Relaxed);
        println!(
            "F6 pressed. Clicker is now {}",
            if !current_state { "active" } else { "inactive" }
        );
    });

    // Confirm the event loop is running
    println!("Starting event loop...");
    inputbot::handle_input_events();
    println!("Event loop has started.");
}
