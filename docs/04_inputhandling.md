Time to get some input from the user.

We already seen the [`EventHandler`](https://docs.rs/ggez/0.8.1/ggez/event/trait.EventHandler.html) trait
and implemented the two required methods [`update(...)`](https://docs.rs/ggez/0.8.1/ggez/event/trait.EventHandler.html#tymethod.update) and [`draw(...)`](https://docs.rs/ggez/0.8.1/ggez/event/trait.EventHandler.html#tymethod.draw)
but [`EventHandler`](https://docs.rs/ggez/0.8.1/ggez/event/trait.EventHandler.html)
provide a lot more methods for handling user inputs, like key pressed, mouse events, window resize and so on.

First try with [`key_down_event`](https://docs.rs/ggez/0.8.1/ggez/event/trait.EventHandler.html#method.key_down_event)
note that `EventHandler` already provide a default implementation for this method

From the docs

"The default implementation of this will call [`ctx.request_quit()`](https://docs.rs/ggez/0.8.1/ggez/context/struct.Context.html#method.request_quit)
when the escape key is pressed. 
If you override this with your own event handler you have to re-implement that functionality yourself."""

So just print out which key is pressed

```rust
fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
	print!("{:?} {}\n", input, repeated);
	if input.keycode == Some(KeyCode::Escape) {
		ctx.request_quit();
	}
	Ok(())
}
```

run and see the output, do you note something strange?

Try to use this method to move the snake head.

Add the position of the head

```rust
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
```

and update the position when a key is pressed

```rust
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
```

Putting all together

```rust
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
use ggez::input::keyboard::KeyInput;
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
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
}
``` 

Run the code and watch the snake head moving around when an arrow key is pressed.

This time do you note something strange in the head movement?

There is a small lapse between the first key press event which cause a lapse on the head movement.

The [`key_down_event`](https://docs.rs/ggez/0.8.1/ggez/event/trait.EventHandler.html#method.key_down_event) 
is not the right function for a game.

ggez provides a method [`ctx.keyboard.is_key_pressed`](https://docs.rs/ggez/latest/ggez/input/keyboard/struct.KeyboardContext.html#method.is_key_pressed)
which solves our problem.

Just put it in the [`update(...)`](https://docs.rs/ggez/latest/ggez/event/trait.EventHandler.html#tymethod.update) method

```rust
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
```

that's right! Now the head movement is smooth.

You could find the complete source of this lesson [here](https://github.com/geckoblu-games/snake-ggez/blob/main/examples/04_inputhandling.rs)




