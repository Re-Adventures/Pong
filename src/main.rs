use raylib::prelude::*;

fn main() {
    // Initializing the raylib library
    let (mut window, raylib_thread) = raylib::init()
        // Set the size of the window
        .size(640, 480)
        // Set the title of the window
        .title("Pong")
        // Enable vsync
        .vsync()
        // Create the window
        .build();
}
