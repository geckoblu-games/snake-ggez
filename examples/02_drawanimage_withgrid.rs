use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::graphics::Mesh;
use ggez::mint::Point2;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 640.0;
const CELL_SIZE: f32 = 32.0;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("snake-ggez", "author")
        .window_setup(WindowSetup::default().title("Snake ggez"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
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

    fn draw_grid(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let color = Color::new(0.2, 0.2, 0.2, 1.0);

        let mut point1 = Point2::from_slice(&[0.0, 0.0]);
        let mut point2 = Point2::from_slice(&[WINDOW_WIDTH, 0.0]);

        let mut y: f32 = 0.0;
        while y < WINDOW_HEIGHT {
            point1.y = y;
            point2.y = y;
            let hline = Mesh::new_line(ctx, &[point1, point2], 1.0, color)?;
            // Draw an horizzontal line
            canvas.draw(&hline, DrawParam::default());
            y += CELL_SIZE;
        }

        let mut point1 = Point2::from_slice(&[0.0, 0.0]);
        let mut point2 = Point2::from_slice(&[0.0, WINDOW_HEIGHT]);

        let mut x: f32 = 0.0;
        while x < WINDOW_WIDTH {
            point1.x = x;
            point2.x = x;
            let hline = Mesh::new_line(ctx, &[point1, point2], 1.0, color).unwrap();
            // Draw an horizzontal line
            canvas.draw(&hline, DrawParam::default());
            x += CELL_SIZE;
        }

        Ok(())
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Just a color for the background
        let background_color = Color::new(0.075, 0.098, 0.149, 1.0);

        // Create a new Canvas that renders directly to the window surface.
        let mut canvas = Canvas::from_frame(ctx, background_color);

        // Draw the grid
        self.draw_grid(ctx, &mut canvas)?;

		// Draw the snake head
        let head_dest = Vec2::new(32.0, 32.0);
        canvas.draw(&self.head_image, DrawParam::default().dest(head_dest));

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }
}
