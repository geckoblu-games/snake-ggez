use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
// use ggez::input::keyboard::KeyInput;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("snake-ggez", "author")
        .window_setup(WindowSetup::default().title("Snake ggez"))
        .window_mode(WindowMode::default().dimensions(800.0, 640.0))
        .build()
        .expect("Could not create ggez context!");

    let my_game = MyGame::new(&mut ctx)?;

    event::run(ctx, event_loop, my_game)
}

struct MyGame {
    head_image: Image,
    head_pos: Vec2,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let head_image = Image::from_path(ctx, "/snakehead.png")?;
        let head_pos = Vec2::new(64.0, 64.0);
        let g = MyGame {
            head_image,
            head_pos,
        };
        Ok(g)
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.head_pos.y += 1.0;
        } else if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.head_pos.y -= 1.0;
        } else if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.head_pos.x += 1.0;
        } else if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.head_pos.x -= 1.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Just a color for the background
        let background_color = Color::new(0.1, 0.2, 0.3, 1.0);

        // Create a new Canvas that renders directly to the window surface.
        let mut canvas = Canvas::from_frame(ctx, background_color);

        // Draw the snake head
        canvas.draw(&self.head_image, DrawParam::default().dest(self.head_pos));

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }

    /*
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        match input.keycode.unwrap() {
            KeyCode::Down => self.head_pos.y += 1.0,
            KeyCode::Up => self.head_pos.y -= 1.0,
            KeyCode::Right => self.head_pos.x += 1.0,
            KeyCode::Left => self.head_pos.x -= 1.0,
            KeyCode::Escape => ctx.request_quit(),
            _ => {}
        }

        Ok(())
    }
    */
}
