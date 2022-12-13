Time to start using ggez. We will try just to open a Window.

From ggez's side, we'll need a [`Context`](https://docs.rs/ggez/0.8.1/ggez/context/struct.Context.html), which is an object that provides access to the hardware:
video for displaying the game, audio for playing sounds, keyboard for input and so on.
A Context can be obtained from a [`ContextBuilder`](https://docs.rs/ggez/0.8.1/ggez/context/struct.ContextBuilder.html).
To make a [`ContextBuilder`](https://docs.rs/ggez/0.8.1/ggez/context/struct.ContextBuilder.html) you'll need a game ID.
This will be used for the name of a directory in `~/.local/share` on the player's computer,
where game resources can be stored (see the [`filesystem`](https://docs.rs/ggez/0.8.1/ggez/filesystem/index.html	) module).
You also need to provide a name for the game's author, but that's not used for GNU/Linux. 
To create them just call:

```rust
let (mut ctx, event_loop) = ContextBuilder::new("hello-ggez", "author").build()
```
We could now start the event loop, but we need first an [`EventHandler`](https://docs.rs/ggez/0.8.1/ggez/event/trait.EventHandler.html).
Were the logic of our game will reside.

Just define a struct:

```rust
struct MyGame {}
```

and then implement the minimal [`EventHandler`](https://docs.rs/ggez/0.8.1/ggez/event/trait.EventHandler.html) trait

```rust
impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        todo!() // todo!()  is a macro which just exits the program with an error.
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        todo!()
    }
}
```
Putting all together:

```rust
use ggez::event::EventHandler;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("hello-ggez", "author")
        .build()
        .expect("Could not create ggez context!");

    let my_game = MyGame {};

    event::run(ctx, event_loop, my_game)
}

struct MyGame {}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        todo!() // todo!()  is a macro which just exits the program with an error.
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        todo!()
    }
}
```



Now try to `run` it and ... nothing! The application just panic.

```
thread 'main' panicked at 'not yet implemented', src/main.rs:21:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Nothing special, the `event_loop` is just calling the `update()` method with has just a `todo!()`,
so try a nicer empty implementation.

```rust
impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
```

Try to `run` again and this time a Window appears (`Esc` to close).

We are drawing nothing so the Window background could display some garbage,
better to color it:

```rust
impl EventHandler for MyGame {
    ...

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Just a color for the background
        let color = Color::from([0.1, 0.2, 0.3, 1.0]);

        // Create a new Canvas that renders directly to the window surface.
        let canvas = Canvas::from_frame(ctx, color);

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }
}
```

[`Canvas`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.Canvas.html) are the main places for drawing in ggez.
Drawing are not made directly to the screen but instead on an internal buffer, so remeber to call [`finish()`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.Canvas.html#method.finish)
at the end to flush drawings or you will not see them.

`run` another time and we finally have our fancy Window.

As we are working with windows let set some attributes like title and size:

```rust
let (ctx, event_loop) = ContextBuilder::new("snake-ggez", "author")
	.window_setup(WindowSetup::default().title("Snake ggez"))
	.window_mode(WindowMode::default().dimensions(800.0, 640.0))
	.build()
```

Read the documentation of [`WindowSetup`](https://docs.rs/ggez/0.8.1/ggez/conf/struct.WindowSetup.html) and [`WindowMode`](https://docs.rs/ggez/0.8.1/ggez/conf/struct.WindowMode.html) for a complete list of attributes.

You could find the complete source of this lesson [here](https://github.com/geckoblu-games/snake-ggez/blob/main/examples/01_justawindow.rs)
