We now have all the bricks to build our Snake game.

Time to implement some logic.

Start again from an empty template

```rust
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
```

## The grid

First of all draw a 32px grid (did you do your homeworks?). 
The grid could be hide \ show with the `g` key.

```rust
struct MyGame {
    show_grid: bool,
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
        if self.show_grid {
            self.draw_grid(ctx, &mut canvas)?;
        }

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repetd: bool) -> GameResult {
        match input.keycode.unwrap() {
            KeyCode::G => self.show_grid = !self.show_grid,
            _ => {}
        }

        Ok(())
    }
}

```

## The score

Now a game needs a score, right?

```rust
struct MyGame {
    score: u32,
    ...,
}

impl MyGame {
	
	...
	
	fn draw_score(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Create a new text
        let s = format!("Points: {}", self.score);
        let mut text = Text::new(s);

        // Set font size
        text.set_scale(PxScale::from(20.0));

        // Set text position to the center of the screen
        let m = text.measure(ctx)?;
        let coords = [
            WINDOW_WIDTH - m.x - CELL_SIZE / 2.0,
            WINDOW_HEIGHT - m.y - CELL_SIZE / 3.0,
        ];

        // Set params
        let params = DrawParam::default().dest(coords).color(Color::YELLOW);

        // Draw the text
        canvas.draw(&text, params);

        Ok(())
    }
}

impl EventHandler for MyGame {

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
		
		...
		
        // Draw the score
        self.draw_score(ctx, &mut canvas)?;

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }
}
```

## FPS

Not really necessary for the game, but will be usefull to have a way 
to see how fast we are rendering. Ggez will give us FPS (Frame Per Second)
and we should try to don't go below 60 FPS.

```rust
let fps = ctx.time.fps();

```

draw it like the score for exercise.

## Draw the head

Nothing new here, do you remember?, but we will use grid coords instead of pixel for the head position

```rust
struct MyGame {
    head_image: Image,
    head_pos: Vec2, // This are the coords for the head as grid coords, not pixels	
	...
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let head_image = Image::from_path(ctx, "/snakehead.png")?;
        let head_pos = Vec2::new(2.0, 2.0);

        let g = MyGame {
            head_image,
            head_pos,
            ...
        };
        Ok(g)
    }
}

impl EventHandler for MyGame {

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
		
		...
		
        // Draw the snake head
        // convert to pixel
        let head_dest = Vec2::new(CELL_SIZE * self.head_pos.x, CELL_SIZE * self.head_pos.y);
        canvas.draw(&self.head_image, DrawParam::default().dest(head_dest));

		..
    }

}
```

## Move the head

We already implemented something like

```rust
impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.head_pos.y += 1.0;
            if self.head_pos.y >= GRID_HEIGHT {
                self.head_pos.y = 0.0;
            }
        } else if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.head_pos.y -= 1.0;
            if self.head_pos.y < 0.0 {
                self.head_pos.y = GRID_HEIGHT - 1.0;
            }
        } else if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.head_pos.x += 1.0;
            if self.head_pos.x >= GRID_WIDTH {
                self.head_pos.x = 0.0;
            }
        } else if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.head_pos.x -= 1.0;
            if self.head_pos.x < 0.0 {
                self.head_pos.x = GRID_WIDTH - 1.0;
            }
        }
        Ok(())
    }
}
```

but doing this the head moves only when we press a key,
instead we need the head to keep moving, so let's introduce a direction (and some auxilary structs)


```rust

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    pub fn to_direction(self: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Down => {
                let mut y = self.y + 1;
                if y >= GRID_HEIGHT {
                    y = 0;
                }
                GridPosition { x: self.x, y }
            }
            Direction::Up => {
                let mut y = self.y - 1;
                if y < 0 {
                    y = GRID_HEIGHT - 1;
                }
                GridPosition { x: self.x, y }
            }
            Direction::Right => {
                let mut x = self.x + 1;
                if x >= GRID_WIDTH {
                    x = 0;
                }
                GridPosition { x, y: self.y }
            }
            Direction::Left => {
                let mut x = self.x - 1;
                if x < 0 {
                    x = GRID_WIDTH - 1;
                }
                GridPosition { x, y: self.y }
            }
        }
    }
}

struct MyGame {
    dir: Direction,
    head_pos: GridPosition,
    ...
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.process_input(ctx);

        self.head_pos = self.head_pos.to_direction(self.dir);

        Ok(())
    }
    
    ...
}

impl MyGame {
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
```

now the head is moving but too fast!

We need a way to decouple the head movement speed from the FPS.

Let's add a timer and and check the [`delta`](https://docs.rs/ggez/0.8.1/ggez/timer/struct.TimeContext.html#method.delta)

```rust
const MOVE_TIME: Duration = Duration::from_millis(300);

struct MyGame {
    head_timer: Duration,
    ...
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.process_input(ctx);

        self.head_timer += ctx.time.delta();

        if self.head_timer >= MOVE_TIME {
            self.head_pos.to_direction(self.dir);
            self.head_timer = Duration::from_millis(0);
        }

        Ok(())
    }
    
    ...
}
```
