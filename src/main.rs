use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 640.0;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("snake-ggez", "author")
        .window_setup(WindowSetup::default().title("Snake ggez"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .expect("Could not create ggez context!");

    let my_game = MyGame {};

    event::run(ctx, event_loop, my_game)
}

struct MyGame {}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
