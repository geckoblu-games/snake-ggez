use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("snake-ggez", "author")
        .window_setup(WindowSetup::default().title("Snake ggez"))
        .window_mode(WindowMode::default().dimensions(800.0, 640.0))
        .build()
        .expect("Could not create ggez context!");

    let my_game = MyGame {};

    event::run(ctx, event_loop, my_game)
}

struct MyGame {}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Just a color for the background
        let background_color = Color::new(0.1, 0.2, 0.3, 1.0);

        // Create a new Canvas that renders directly to the window surface.
        let canvas = Canvas::from_frame(ctx, background_color);

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }
}
