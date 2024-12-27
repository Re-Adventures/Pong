use raylib::prelude::*;

const SCREEN_WIDTH  : f32 = 1280.0;
const SCREEN_HEIGHT : f32 = 720.0;

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
    dx: f32,
    dy: f32,
}

/// The paddle object used for hitting the ball
struct Paddle {
    width    : usize,
    height   : usize,
    position : Position,
    speed    : usize,
    score    : usize,
}

/// The ball for playing the game
struct Ball {
    position       : Vector2,
    velocity       : Velocity,
    radius         : f32,
    move_direction : Moves
}

impl Ball {
    /// Updates the position of the ball
    fn move_ball(&mut self) {
        // Check if the ball has gone out of bounds
        // X-Axis check
        if self.position.x - self.radius <= 0.0
                || self.position.x + self.radius >= SCREEN_WIDTH {
            // Reverse the x axis velocity
            self.velocity.dx = -self.velocity.dx;
        }

        // Y-Axis check
        if self.position.y - self.radius <= 0.0
                || self.position.y + self.radius >= SCREEN_HEIGHT {
            // Reverse the y axis velocity
            self.velocity.dy = -self.velocity.dy;
        }

        // Now update the position of the ball
        self.position.x += self.velocity.dx;
        self.position.y += self.velocity.dy;
    }

    /// Function for drawing the ball
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
        position       : Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        velocity       : Velocity { dx: 15.0, dy: 13.0 }, 
        radius         : 30.0,
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
