use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::PxScale;
use ggez::graphics::Text;
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
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Just a color for the background
        let background_color = Color::new(0.075, 0.098, 0.149, 1.0);

        // Create a new Canvas that renders directly to the window surface.
        let mut canvas = Canvas::from_frame(ctx, background_color);

        // Create a new text
        let mut text = Text::new("Hello Snake!");

        // Set font size
        text.set_scale(PxScale::from(50.0));

        // Set text position to the center of the screen
        let m = text.measure(ctx)?;
        let coords = [(WINDOW_WIDTH - m.x) / 2.0, (WINDOW_HEIGHT - m.y) / 2.0];

        // Text color
        let color = Color::new(0.0, 1.0, 0.0, 1.0);

        // Set params
        let params = DrawParam::default().dest(coords).color(color);

        // Draw the text
        canvas.draw(&text, params);

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }
}
