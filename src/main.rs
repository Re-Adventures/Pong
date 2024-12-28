use raylib::prelude::*;

const SCREEN_WIDTH  : f32 = 1280.0;
const SCREEN_HEIGHT : f32 = 720.0;

/// Represents the velocity of an object
struct Velocity {
    dx: f32,
    dy: f32,
}

/// The paddle used for hitting the ball
struct Paddle {
    position : Rectangle,
    speed    : f32,
    score    : usize,
}

impl Paddle {
    /// Method for updating the position of the paddle
    fn move_paddle(&mut self, window: &RaylibHandle) {
        if window.is_key_down(KeyboardKey::KEY_S) {
            // Move the player down
            self.position.y += self.speed;

            // If the paddle moves out of bottom of the screen, then move it
            // back into the window
            if self.position.y + self.position.height > SCREEN_HEIGHT {
                self.position.y = SCREEN_HEIGHT - self.position.height;
            }
        }
        else if window.is_key_down(KeyboardKey::KEY_W) {
            // Move the player up
            self.position.y -= self.speed;

            // If the paddle moves out of top of the screen, then move it
            // back into the window
            if self.position.y < 0.0 {
                self.position.y = 0.0;
            }
        }
    }

    /// Method for automatically moving the computer paddle
    fn move_paddle_ai(&mut self, ball: &Ball) {
        // Only move the paddle when the ball is approaching it
        if ball.velocity.dx < 0.0 {
            return;
        }

        if ball.position.y < self.position.y {
            self.position.y -= self.speed;
        }
        else if ball.position.y > self.position.y {
            self.position.y += self.speed;
        }

        // Out of bounds check

        // If the paddle moves out of bottom of the screen, then move it
        // back into the window
        if self.position.y + self.position.height > SCREEN_HEIGHT {
            self.position.y = SCREEN_HEIGHT - self.position.height;
        }

        // If the paddle moves out of top of the screen, then move it
        // back into the window
        if self.position.y < 0.0 {
            self.position.y = 0.0;
        }
    }


    // Draws the paddle onto the window
    fn draw(&self, ctx: &mut RaylibDrawHandle) {
        ctx.draw_rectangle_rec(&self.position, Color::WHITE);
    }
}

/// The ball for playing the game
struct Ball {
    position : Vector2,
    velocity : Velocity,
    radius   : f32,
    color    : Color,
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
        // Draw the circle onto the screen
        ctx.draw_circle_v(self.position, self.radius, self.color);
    }

    fn check_collision(&mut self, paddle: &Rectangle) {
        let closest_x = Ball::clamp(
            self.position.x, paddle.x, paddle.x + paddle.width);

        let closest_y = Ball::clamp(
            self.position.y, paddle.y, paddle.y + paddle.height);

        let distance_x = self.position.x - closest_x;
        let distance_y = self.position.y - closest_y;

        let distance_squared = distance_x * distance_x
            + distance_y * distance_y;

        if distance_squared >= self.radius * self.radius {
            return;
        }

        // Determine the side of the collision
        let paddle_center_x = paddle.x + paddle.width  / 2.0;
        let paddle_center_y = paddle.y + paddle.height / 2.0;

        let dx = self.position.x - paddle_center_x;
        let dy = self.position.y - paddle_center_y;

        if dx.abs() > dy.abs() {
            // Collision on the x-axis
            if dx > 0.0 {
                // Collision on the right side
                self.position.x = paddle.x + paddle.width + self.radius;
                self.velocity.dx = self.velocity.dx.abs();
            } else {
                // Collision on the left side
                self.position.x = paddle.x - self.radius;
                self.velocity.dx = -self.velocity.dx.abs();
            }
        } else {
            // Collision on the y-axis
            if dy > 0.0 {
                // Collision on the bottom side
                self.position.y = paddle.y + paddle.height + self.radius;
                self.velocity.dy = self.velocity.dy.abs();
            } else {
                // Collision on the top side
                self.position.y = paddle.y - self.radius;
                self.velocity.dy = -self.velocity.dy.abs();
            }
        }
    }

    fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
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
        position : Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        velocity : Velocity { dx: 15.0, dy: 13.0 },
        radius   : 30.0,
        color    : Color::GREEN,
    };

    // The space to be left from the window edge to paddle
    let paddle_padding = 30.0;

    let paddle_width   = 30.0;
    let paddle_height  = 150.0;

    let paddle_pos_y  = (SCREEN_HEIGHT / 2.0) - (paddle_height / 2.0);

    // The player paddle
    let mut player = Paddle {
        position: Rectangle::new(
            paddle_width + paddle_padding, paddle_pos_y, paddle_width, 150.0),
        score: 0,
        speed: 15.0,
    };

    // The computer paddle
    let mut computer = Paddle {
        position: Rectangle::new(
            // * 2 is because the rectangle x starts at top left of the
            // rectangle & the rectangle will expand to its right(width pixels)
            SCREEN_WIDTH - (paddle_width * 2.0) - paddle_padding,
            paddle_pos_y,
            paddle_width,
            150.0),
        score: 0,
        speed: 15.0,
    };

    // Game loop
    // Run this loop forever
    while window.window_should_close() == false {
        // On ESC key press, stop the game
        if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }

        // Move the paddle's y according to the ball's y axis
        computer.move_paddle_ai(&ball);


        // Move the player paddle's position according to keyboard input
        player.move_paddle(&window);

        // Get the context for drawing on the window
        let mut ctx = window.begin_drawing(&raylib_thread);

        ctx.clear_background(Color::BLACK);

        ball.move_ball();
        ball.draw(&mut ctx);

        ball.check_collision(&player.position);
        ball.check_collision(&computer.position);

        player.draw(&mut ctx);

        computer.draw(&mut ctx);
    }
}
