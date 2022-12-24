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
use ggez::graphics::PxScale;
use ggez::graphics::Text;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyInput;
use ggez::mint::Point2;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use oorandom::Rand32;
use std::collections::LinkedList;
use std::time::Duration;

const GRID_WIDTH: u32 = 25;
const GRID_HEIGHT: u32 = 20;
const CELL_SIZE: u32 = 32;
const WINDOW_WIDTH: f32 = (GRID_WIDTH * CELL_SIZE) as f32;
const WINDOW_HEIGHT: f32 = (GRID_HEIGHT * CELL_SIZE) as f32;
const FRUIT_POINTS: u32 = 10;

const COLOR_BACKGROUND: Color = Color::new(0.075, 0.098, 0.149, 1.0);
const COLOR_GRAY: Color = Color::new(0.2, 0.2, 0.2, 1.0);

const MOVE_TIME: Duration = Duration::from_millis(300);

/// The direction the snake moves in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// A struct that will hold an entity's position on our game board
/// or grid which we defined above. We'll use signed integers because we only want
/// to store whole numbers, and we need them to be signed so that they work properly
/// with our modulus arithmetic later.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: u32,
    y: u32,
}

impl GridPosition {
    /// We make a standard helper function so that we can create a new `GridPosition`
    /// more easily.
    pub fn new(x: u32, y: u32) -> Self {
        GridPosition { x, y }
    }

    pub fn random(rng: &mut Rand32) -> Self {
        let x = rng.rand_range(0..GRID_WIDTH);
        let y = rng.rand_range(0..GRID_HEIGHT);
        GridPosition { x, y }
    }

    pub fn move_to_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Down => {
                self.y += 1;
                if self.y >= GRID_HEIGHT {
                    self.y = 0;
                }
            }
            Direction::Up => {
                if self.y == 0 {
                    self.y = GRID_HEIGHT;
                }
                self.y -= 1;
            }
            Direction::Right => {
                self.x += 1;
                if self.x >= GRID_WIDTH {
                    self.x = 0;
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    self.x = GRID_WIDTH;
                }
                self.x -= 1;
            }
        }
    }

    fn to_vec2(&self) -> Vec2 {
        Vec2::new((CELL_SIZE * self.x) as f32, (CELL_SIZE * self.y) as f32)
    }
}

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
    dir: Direction,

    head_pos: GridPosition,
    head_timer: Duration,
    fruit_pos: GridPosition,
    body: LinkedList<GridPosition>,

    /// Our RNG state
    rng: Rand32,

    score: u32,
    show_grid: bool,
    show_fps: bool,

    head_image: Image,
    body_image: Image,
    fruit_image: Image,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let head_image = Image::from_path(ctx, "/snakehead.png")?;
        let body_image = Image::from_path(ctx, "/snakebody.png")?;
        let fruit_image = Image::from_path(ctx, "/apple.png")?;

        // We seed our RNG with the system RNG.
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));

        let head_pos = GridPosition::new(4, 4);
        let fruit_pos = GridPosition::random(&mut rng);

        let body = LinkedList::new();

        let g = MyGame {
            head_image,
            body_image,
            fruit_image,
            head_pos,
            fruit_pos,
            dir: Direction::Right,
            score: 0,
            show_grid: true,
            show_fps: true,
            head_timer: Duration::from_millis(0),
            rng,
            body,
        };
        Ok(g)
    }

    fn draw_grid(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let mut point1 = Point2::from_slice(&[0.0, 0.0]);
        let mut point2 = Point2::from_slice(&[WINDOW_WIDTH, 0.0]);

        let mut y: f32 = 0.0;
        while y < WINDOW_HEIGHT {
            point1.y = y;
            point2.y = y;
            let hline = Mesh::new_line(ctx, &[point1, point2], 1.0, COLOR_GRAY)?;
            // Draw an horizzontal line
            canvas.draw(&hline, DrawParam::default());
            y += CELL_SIZE as f32;
        }

        let mut point1 = Point2::from_slice(&[0.0, 0.0]);
        let mut point2 = Point2::from_slice(&[0.0, WINDOW_HEIGHT]);

        let mut x: f32 = 0.0;
        while x < WINDOW_WIDTH {
            point1.x = x;
            point2.x = x;
            let hline = Mesh::new_line(ctx, &[point1, point2], 1.0, COLOR_GRAY).unwrap();
            // Draw an horizzontal line
            canvas.draw(&hline, DrawParam::default());
            x += CELL_SIZE as f32;
        }

        Ok(())
    }

    fn draw_score(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Create a new text
        let s = format!("Points: {}", self.score);
        let mut text = Text::new(s);

        // Set font size
        text.set_scale(PxScale::from(20.0));

        // Set text position to the center of the screen
        let m = text.measure(ctx)?;
        let coords = [
            WINDOW_WIDTH - m.x - CELL_SIZE as f32 / 2.0,
            WINDOW_HEIGHT - m.y - CELL_SIZE as f32 / 3.0,
        ];

        // Set params
        let params = DrawParam::default().dest(coords).color(Color::YELLOW);

        // Draw the text
        canvas.draw(&text, params);

        Ok(())
    }

    fn draw_fps(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // read FPS
        let fps = ctx.time.fps() as u32;

        // Create a new text
        let s = format!("FPS: {}", fps);
        let mut text = Text::new(s);

        // Set font size
        text.set_scale(PxScale::from(20.0));

        // Set text position to the center of the screen
        let m = text.measure(ctx)?;
        let coords = [
            CELL_SIZE as f32 / 2.0,
            WINDOW_HEIGHT - m.y - CELL_SIZE as f32 / 3.0,
        ];

        // Set params
        let params = DrawParam::default().dest(coords).color(COLOR_GRAY);

        // Draw the text
        canvas.draw(&text, params);

        Ok(())
    }

    fn process_input(&mut self, ctx: &Context) {
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.dir = Direction::Down;
        } else if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.dir = Direction::Up;
        } else if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.dir = Direction::Right;
        } else if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.dir = Direction::Left;
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.process_input(ctx);

        self.head_timer += ctx.time.delta();

        if self.head_timer >= MOVE_TIME {
            self.body.push_front(self.head_pos);
            self.head_pos.move_to_direction(self.dir);

            for seg in self.body.iter() {
                if self.head_pos == *seg {
                    panic!("Game over");
                }
            }

            if self.head_pos == self.fruit_pos {
                self.score += FRUIT_POINTS;
                self.fruit_pos = GridPosition::random(&mut self.rng);
            } else {
                self.body.pop_back();
            }

            self.head_timer = Duration::from_millis(0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a new Canvas that renders directly to the window surface.
        let mut canvas = Canvas::from_frame(ctx, COLOR_BACKGROUND);

        // Draw the grid
        if self.show_grid {
            self.draw_grid(ctx, &mut canvas)?;
        }

        // Draw FPS
        if self.show_fps {
            self.draw_fps(ctx, &mut canvas)?;
        }

        // Draw the score
        self.draw_score(ctx, &mut canvas)?;

        // Draw the fruit
        canvas.draw(
            &self.fruit_image,
            DrawParam::default().dest(self.fruit_pos.to_vec2()),
        );

        for seg in self.body.iter() {
            canvas.draw(&self.body_image, DrawParam::default().dest(seg.to_vec2()));
        }

        // Draw the snake head
        canvas.draw(
            &self.head_image,
            DrawParam::default().dest(self.head_pos.to_vec2()),
        );

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repetd: bool) -> GameResult {
        match input.keycode.unwrap() {
            KeyCode::G => self.show_grid = !self.show_grid,
            KeyCode::F => self.show_fps = !self.show_fps,
            _ => {}
        }

        Ok(())
    }
}
