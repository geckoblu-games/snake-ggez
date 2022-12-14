use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
// use ggez::graphics::DrawMode;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
// use ggez::graphics::Mesh;
// use ggez::graphics::Rect;
use ggez::glam::Vec2;
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
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let head_image = Image::from_path(ctx, "/snakehead.png")?;
        let g = MyGame { head_image };
        Ok(g)
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Just a color for the background
        let background_color = Color::new(0.1, 0.2, 0.3, 1.0);

        // Create a new Canvas that renders directly to the window surface.
        let mut canvas = Canvas::from_frame(ctx, background_color);

        /*
        // Create the snake head
        let head_color = Color::new(0.0, 1.0, 0.0, 1.0);
        let rect = Rect::new(10.0, 20.0, 32.0, 32.0);
        let head = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, head_color)?;

        // Draw the snake head
        canvas.draw(&head, DrawParam::default());
        */

		// Draw the snake head
        let head_dest = Vec2::new(10.0, 20.0);
        canvas.draw(&self.head_image, DrawParam::default().dest(head_dest));

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }
}
