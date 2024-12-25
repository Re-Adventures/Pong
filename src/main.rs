use raylib::prelude::*;

const SCREEN_WIDTH  : i32 = 640;
const SCREEN_HEIGHT : i32 = 480;

#[derive(Eq, PartialEq)]
enum Moves {
    Up,
    Down,
    Left,
    Right,
    Stopped,
}

/// Represents the position of an object
struct Position {
    x: i32,
    y: i32,
}

/// Represents the velocity of an object
struct Velocity {
    dx: i32,
    dy: i32,
}

/// The paddle object used for hitting the ball
struct Paddle {
    width    : usize,
    height   : usize,
    position : Position,
    speed    : usize,
}

/// The ball for playing the game
struct Ball {
    position       : Position,
    velocity       : Velocity,
    radius         : f32,
    speed          : i32,
    move_direction : Moves
}

impl Ball {
    fn move_ball(&mut self) {
        // Check if the ball has gone out of bounds
        // X-Axis check
        if self.position.x - (self.radius as i32) <= 0
                || self.position.x + (self.radius as i32) >= SCREEN_WIDTH {
            // Reverse the x axis velocity
            self.velocity.dx = -self.velocity.dx;
        }

        // Y-Axis check
        if self.position.y - (self.radius as i32) <= 0
                || self.position.y + (self.radius as i32) >= SCREEN_HEIGHT {
            // Reverse the y axis velocity
            self.velocity.dy = -self.velocity.dy;
        }

        // Now update the position of the ball
        self.position.x += self.velocity.dx;
        self.position.y += self.velocity.dy;


    }

    fn draw(&self, ctx: &mut RaylibDrawHandle) {
        ctx.draw_circle(
            self.position.x as i32, self.position.y as i32,
            self.radius, Color::GREEN);
    }
}

fn main() {
    // Initializing the raylib library
    let (mut window, raylib_thread) = raylib::init()
        // Set the size of the window
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        // Set the title of the window
        .title("Pong")
        // Enable vsync
        .vsync()
        // Create the window
        .build();

    let mut ball = Ball {
        position       : Position { x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT / 2 },
        velocity       : Velocity { dx: 5, dy: 3 }, 
        radius         : 30.0,
        speed          : 10,
        move_direction : Moves::Right,
    };

    // Game loop
    // Run this loop forever
    while window.window_should_close() == false {
        // On ESC key press, stop the game
        if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }

        // ball.move_ball(&window);

        // Get the context for drawing on the window
        let mut ctx = window.begin_drawing(&raylib_thread);

        ctx.clear_background(Color::BLACK);

        ball.move_ball();

        ball.draw(&mut ctx);

    }
}
