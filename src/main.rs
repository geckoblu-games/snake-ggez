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

/// An enum with all the possible game states
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameState {
    Starting,
    Running,
    Paused,
    GameOver,
    Quitting,
}

/// An enum that will represent all the possible
/// directions that our snake could move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// An helper function that will allow us to easily check the opposite
    /// of a `Direction` which we can use later to check if the player should be
    /// able to move the snake in a certain direction.
    pub fn opposite(&self, op: Direction) -> bool {
        match *self {
            Direction::Up => op == Direction::Down,
            Direction::Down => op == Direction::Up,
            Direction::Left => op == Direction::Right,
            Direction::Right => op == Direction::Left,
        }
    }
}

/// A struct that will hold an entity's position on our game board
/// or grid which we defined above.
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

    /*
    /// An helper function that will give us a random `GridPosition` in the grid
    pub fn random(rng: &mut Rand32) -> Self {
        let x = rng.rand_range(0..GRID_WIDTH);
        let y = rng.rand_range(0..GRID_HEIGHT);
        GridPosition { x, y }
    }
    */

    /// Move the position in the given direction.
    /// The grid is toroidal
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

    /// Convert the position to a Vec2 used by ggez.
    /// Note that a position is in grid coordinates, the Vec2 is in pixels
    fn as_vec2(&self) -> Vec2 {
        Vec2::new((CELL_SIZE * self.x) as f32, (CELL_SIZE * self.y) as f32)
    }
}

/// A struct that contains all the information needed to describe the state of the game.
struct MyGame {
    /// The current head position on the grid
    head_pos: GridPosition,
    /// The head velocity (gap between single head movement)
    head_timer: Duration,
    /// The current head moving direction
    dir: Direction,
    /// The new direction the snake will turn
    dir_new: Option<Direction>,

    /// The current fruit position on the grid
    fruit_pos: GridPosition,

    /// The list of body segments
    body: LinkedList<GridPosition>,

    /// Our RNG state
    rng: Rand32,

    /// The score
    score: u32,
    /// Hide / show the grid
    show_grid: bool,
    /// Hide / show the FPS
    show_fps: bool,
    /// Game state
    state: GameState,

    /// Counter used by the choose_random_direction function
    choose_random_direction_counter: u32,

    /// Head image
    head_image: Image,
    /// Body image
    body_image: Image,
    /// Fruit image
    fruit_image: Image,
}

impl MyGame {
    /// Create a new game
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        // Load images from filesystem
        let head_image = Image::from_path(ctx, "/snakehead.png")?;
        let body_image = Image::from_path(ctx, "/snakebody.png")?;
        let fruit_image = Image::from_path(ctx, "/apple.png")?;

        // We seed our RNG with the system RNG.
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let rng = Rand32::new(u64::from_ne_bytes(seed));

        let head_pos = GridPosition::new(4, 4);
        let fruit_pos = GridPosition::new(4, 4);
        let mut body = LinkedList::new();
        body.push_back(GridPosition::new(3, 4));
        body.push_back(GridPosition::new(2, 4));
        body.push_back(GridPosition::new(1, 4));
        body.push_back(GridPosition::new(0, 4));

        let mut g = MyGame {
            head_image,
            body_image,
            fruit_image,
            head_pos,
            fruit_pos,
            body,
            rng,
            dir: Direction::Right,
            dir_new: None,
            score: 0,
            show_grid: true,
            show_fps: true,
            state: GameState::Starting,
            head_timer: Duration::from_millis(0),
            choose_random_direction_counter: 0,
        };

        g.fruit_pos = g.random_free_pos();

        Ok(g)
    }

    fn restart(&mut self) {
        self.head_pos = GridPosition::new(4, 4);
        self.body.clear();
        self.fruit_pos = self.random_free_pos();
        self.dir = Direction::Right;
        self.dir_new = None;
        self.score = 0;
        self.state = GameState::Running;
        self.head_timer = Duration::from_millis(0);
    }

    /// Draw the grid
    fn draw_grid(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // First and last point of an horizzontal line
        let mut point1 = Point2::from_slice(&[0.0, 0.0]);
        let mut point2 = Point2::from_slice(&[WINDOW_WIDTH, 0.0]);

        // Draw the horizzontal lines
        let mut y: f32 = 0.0;
        while y < WINDOW_HEIGHT {
            point1.y = y;
            point2.y = y;
            let hline = Mesh::new_line(ctx, &[point1, point2], 1.0, COLOR_GRAY)?;
            // Draw an horizzontal line
            canvas.draw(&hline, DrawParam::default());
            y += CELL_SIZE as f32;
        }

        // First and last point of a vertical line
        let mut point1 = Point2::from_slice(&[0.0, 0.0]);
        let mut point2 = Point2::from_slice(&[0.0, WINDOW_HEIGHT]);

        // Draw the vertical lines
        let mut x: f32 = 0.0;
        while x < WINDOW_WIDTH {
            point1.x = x;
            point2.x = x;
            let vline = Mesh::new_line(ctx, &[point1, point2], 1.0, COLOR_GRAY).unwrap();
            // Draw a vertical line
            canvas.draw(&vline, DrawParam::default());
            x += CELL_SIZE as f32;
        }

        Ok(())
    }

    /// Draw the score
    fn draw_score(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Create a new text
        let s = format!("Points: {}", self.score);
        let mut text = Text::new(s);

        // Set font size
        text.set_scale(PxScale::from(20.0));

        // Set text position to the bottom-right of the screen
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

    /// Draw the FPS
    fn draw_fps(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // read FPS
        let fps = ctx.time.fps() as u32;

        // Create a new text
        let s = format!("FPS: {}", fps);
        let mut text = Text::new(s);

        // Set font size
        text.set_scale(PxScale::from(20.0));

        // Set text position to the bottom-left of the screen
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

    /// Draw the arena of a running game
    fn draw_running(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Draw the grid
        if self.show_grid {
            self.draw_grid(ctx, canvas)?;
        }

        // Draw the fruit
        canvas.draw(
            &self.fruit_image,
            DrawParam::default().dest(self.fruit_pos.as_vec2()),
        );

        // Draw the body
        for seg in self.body.iter() {
            canvas.draw(&self.body_image, DrawParam::default().dest(seg.as_vec2()));
        }

        // Draw the snake head
        canvas.draw(
            &self.head_image,
            DrawParam::default().dest(self.head_pos.as_vec2()),
        );

        // Draw FPS
        if self.show_fps {
            self.draw_fps(ctx, canvas)?;
        }

        // Draw the score
        self.draw_score(ctx, canvas)?;

        Ok(())
    }

    fn draw_starting(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Draw the grid
        if self.show_grid {
            self.draw_grid(ctx, canvas)?;
        }

        // Draw the body
        for seg in self.body.iter() {
            canvas.draw(&self.body_image, DrawParam::default().dest(seg.as_vec2()));
        }

        // Draw the snake head
        canvas.draw(
            &self.head_image,
            DrawParam::default().dest(self.head_pos.as_vec2()),
        );

        // Create a new text
        let mut text1 = Text::new("SNAKE");
        let mut text2 = Text::new("Press any key to start");

        // Set font size
        text1.set_scale(PxScale::from(200.0));
        text2.set_scale(PxScale::from(20.0));

        // Set text position to the center of the screen
        let m1 = text1.measure(ctx)?;
        let m2 = text2.measure(ctx)?;

        let y1 = (WINDOW_HEIGHT - m1.y) / 2.0;
        let y2 = (WINDOW_HEIGHT - m2.y + m1.y) / 2.0;
        let coords1 = [(WINDOW_WIDTH - m1.x) / 2.0, y1];
        let coords2 = [(WINDOW_WIDTH - m2.x) / 2.0, y2];

        // Set params
        let params = DrawParam::default().dest(coords1).color(Color::GREEN);
        // Draw the text
        canvas.draw(&text1, params);

        // Set params
        let params = DrawParam::default().dest(coords2).color(Color::WHITE);
        // Draw the text
        canvas.draw(&text2, params);

        Ok(())
    }

    fn draw_gameover(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        self.draw_running(ctx, canvas)?;

        // Create a new text
        let mut text1 = Text::new("GAME OVER");
        let mut text2 = Text::new("Do you want to play again? (Y / N)");

        // Set font size
        text1.set_scale(PxScale::from(100.0));
        text2.set_scale(PxScale::from(20.0));

        // Set text position to the center of the screen
        let m1 = text1.measure(ctx)?;
        let m2 = text2.measure(ctx)?;

        let y1 = (WINDOW_HEIGHT - m1.y) / 2.0;
        let y2 = (WINDOW_HEIGHT - m2.y + m1.y) / 2.0;
        let coords1 = [(WINDOW_WIDTH - m1.x) / 2.0, y1];
        let coords2 = [(WINDOW_WIDTH - m2.x) / 2.0, y2];

        // Set params
        let params = DrawParam::default().dest(coords1).color(Color::GREEN);
        // Draw the text
        canvas.draw(&text1, params);

        // Set params
        let params = DrawParam::default().dest(coords2).color(Color::WHITE);
        // Draw the text
        canvas.draw(&text2, params);

        Ok(())
    }

    fn draw_quitting(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        self.draw_running(ctx, canvas)?;

        // Create a new text
        let mut text2 = Text::new("Do you really want to quit? (Y / N)");

        // Set font size
        text2.set_scale(PxScale::from(20.0));

        // Set text position to the center of the screen
        let m2 = text2.measure(ctx)?;

        let y2 = (WINDOW_HEIGHT - m2.y) / 2.0;
        let coords2 = [(WINDOW_WIDTH - m2.x) / 2.0, y2];

        // Set params
        let params = DrawParam::default().dest(coords2).color(Color::WHITE);
        // Draw the text
        canvas.draw(&text2, params);

        Ok(())
    }

    /// Change the direction when an arrow key is pressed
    fn process_input(&mut self, ctx: &Context) {
        // Turn the direction based on the key pressed,
        // but avoid the opposite direction if the snake has a body
        if ctx.keyboard.is_key_pressed(KeyCode::Down)
            && (self.body.is_empty() || !self.dir.opposite(Direction::Down))
        {
            self.dir_new = Some(Direction::Down);
        } else if ctx.keyboard.is_key_pressed(KeyCode::Up)
            && (self.body.is_empty() || !self.dir.opposite(Direction::Up))
        {
            self.dir_new = Some(Direction::Up);
        } else if ctx.keyboard.is_key_pressed(KeyCode::Right)
            && (self.body.is_empty() || !self.dir.opposite(Direction::Right))
        {
            self.dir_new = Some(Direction::Right);
        } else if ctx.keyboard.is_key_pressed(KeyCode::Left)
            && (self.body.is_empty() || !self.dir.opposite(Direction::Left))
        {
            self.dir_new = Some(Direction::Left);
        }
    }

    /// Return a random free position on the grid
    fn random_free_pos(&mut self) -> GridPosition {
        let mut freepos: Vec<GridPosition> = Vec::new();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let p = GridPosition::new(x, y);
                if p != self.head_pos && !self.body.contains(&p) {
                    freepos.push(p)
                }
            }
        }

        // This should happens only when the snake body fills all the grid
        if freepos.is_empty() {
            self.body.clear();
            return self.random_free_pos();
        }

        let i = self.rng.rand_range(0..freepos.len() as u32) as usize;

        freepos[i]
    }

    /// Choose a new random direction for the snake
    fn choose_random_direction(&mut self) {
        let d = self.rng.rand_range(0..4);
        let somedir = match d {
            0 => Some(Direction::Down),
            1 => Some(Direction::Left),
            2 => Some(Direction::Right),
            4 => Some(Direction::Up),
            _ => None,
        };
        if let Some(dir_new) = somedir {
            if !dir_new.opposite(self.dir) {
                self.dir_new = somedir;
            }
        }
    }
}

impl EventHandler for MyGame {
    /// The main update function for our snake which gets called every time
    /// we want to update the game state.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.state == GameState::Running {
            // Check input
            self.process_input(ctx);
        }

        if self.state == GameState::Running || self.state == GameState::Starting {
            // Time from the last snake movement
            self.head_timer += ctx.time.delta();

            // If it's time move the snake
            if self.head_timer >= MOVE_TIME {
                // Choose a random direction
                if self.state == GameState::Starting {
                    self.choose_random_direction_counter += 1;
                    if self.choose_random_direction_counter > 3 {
                        self.choose_random_direction();
                        self.choose_random_direction_counter = 0;
                    }
                }

                // Eventually change the direction
                if let Some(dir_new) = self.dir_new {
                    self.dir = dir_new;
                    self.dir_new = None;
                }

                // Move the head
                self.body.push_front(self.head_pos);
                self.head_pos.move_to_direction(self.dir);

                // If the snake eats itself is game over
                for seg in self.body.iter() {
                    if self.head_pos == *seg {
                        self.state = GameState::GameOver;
                    }
                }

                // If the snake eats a fruit increment the score and the body lenght
                if self.head_pos == self.fruit_pos {
                    self.score += FRUIT_POINTS;
                    self.fruit_pos = self.random_free_pos();
                } else {
                    self.body.pop_back();
                }

                // Reset the timer
                self.head_timer = Duration::from_millis(0);
            }
        }

        Ok(())
    }

    /// The main drawing function
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a new Canvas that renders directly to the window surface.
        let mut canvas = Canvas::from_frame(ctx, COLOR_BACKGROUND);

        match self.state {
            GameState::Starting => self.draw_starting(ctx, &mut canvas)?,
            GameState::GameOver => self.draw_gameover(ctx, &mut canvas)?,
            GameState::Quitting => self.draw_quitting(ctx, &mut canvas)?,
            _ => self.draw_running(ctx, &mut canvas)?,
        };

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }

    /// A keyboard button was pressed.
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repetd: bool) -> GameResult {
        match self.state {
            GameState::Starting => match input.keycode.unwrap() {
                KeyCode::Q | KeyCode::Escape => ctx.request_quit(),
                _ => self.restart(),
            },
            GameState::GameOver => match input.keycode.unwrap() {
                KeyCode::Q | KeyCode::Escape | KeyCode::N => ctx.request_quit(),
                KeyCode::Y => {
                    self.restart();
                }
                _ => {}
            },
            GameState::Quitting => match input.keycode.unwrap() {
                KeyCode::Q | KeyCode::Escape | KeyCode::Y => ctx.request_quit(),
                KeyCode::N => {
                    self.state = GameState::Running;
                }
                _ => {}
            },
            _ => match input.keycode.unwrap() {
                KeyCode::G => self.show_grid = !self.show_grid,
                KeyCode::F => self.show_fps = !self.show_fps,
                KeyCode::P | KeyCode::Space => {
                    self.state = match self.state {
                        GameState::Running => GameState::Paused,
                        GameState::Paused => GameState::Running,
                        _ => self.state,
                    }
                }
                KeyCode::Q | KeyCode::Escape => self.state = GameState::Quitting,
                _ => {}
            },
        };

        Ok(())
    }
}

/// The main function
fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("snake-ggez", "author")
        .window_setup(WindowSetup::default().title("Snake ggez"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .expect("Could not create ggez context!");

    let my_game = MyGame::new(&mut ctx)?;

    event::run(ctx, event_loop, my_game)
}
