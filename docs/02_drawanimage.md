To write a game we need graphics, right?

So let's start drawing something.

## Draw a shape

The `Canvas` has a method [`draw(...)`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.Canvas.html#method.draw) which we can use:

```rust
pub fn draw(&mut self, drawable: &impl Drawable, param: impl Into<DrawParam>)
```

So first we need a [`Drawable`](https://docs.rs/ggez/0.8.1/ggez/graphics/trait.Drawable.html).
We start with a simple sqare for the head of our snake:

```rust
let head_color = Color::new(0.0, 1.0, 0.0, 1.0);
let rect = Rect::new(10.0, 20.0, 32.0, 32.0);
let head = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, head_color)?;

canvas.draw(&head, DrawParam::default());
```

So we first define the `Color` of our head, then the `Rect`:

```rust
Rect::new(pos_x, pos_y, width, height)
```

and finally we could create a [`Mesh`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.Mesh.html) which implemens the Drawable method.

Note the [`DrawMode`](https://docs.rs/ggez/0.8.1/ggez/graphics/enum.DrawMode.html) for the [`Mesh`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.Mesh.html) which is [`fill`](https://docs.rs/ggez/0.8.1/ggez/graphics/enum.DrawMode.html#variant.Fill) in our case (a filled shape)
and the [`DrawParam`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.DrawParam.html) for the [`draw(...)`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.Canvas.html#method.draw) method which we will ignore for the moment setting it at its default values.

Putting all together

```rust
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::DrawParam;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
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
        let mut canvas = Canvas::from_frame(ctx, background_color);

        // Create the snake head
        let head_color = Color::new(0.0, 1.0, 0.0, 1.0);
        let rect = Rect::new(10.0, 20.0, 32.0, 32.0);
        let head = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, head_color)?;

        // Draw the snake head
        canvas.draw(&head, DrawParam::default());

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }
}
```

Is possible to write a game combining various shapes but I prefer to use images, so ...

## Draw an image

Let's start with a minor problem.

In order to display an image in the game, we need to have it stored somewhere.
At runtime, `ggez` will look for resources in the `resources/` directory next to the executable (not in `$PWD`),
or in `resources.zip` in the same place or in `~/.local/share/<gameid>/`.
See also the documentation of the [`filesystem`](https://docs.rs/ggez/0.8.1/ggez/filesystem/) module and the [`FAQ entry about resource paths`](https://github.com/ggez/ggez/blob/master/docs/FAQ.md#errors_resource).
You'll probably want to have a `resources` directory in the project's root and create a symbolic link to it in `target/debug`.

```
.
├── Cargo.toml
├── resources
│   └── my_image.png
├── src
│   └── main.rs
└── target
    └── debug
        └── resources -> ../../resources/
```

on [`github`](https://github.com/geckoblu-games/snake-ggez) you can find the images for this lesson.

To load an image resource we just need a `Context` and a path

```
let image1 = graphics::Image::new(ctx, "/my_image.png")?;
```

The image might be needed for the entire duration of the game,
so a good owner for it might be a field of the game state struct.
Also, we want to load the image as soon as the game starts,
so we could do it in the `new()` function of the game state.
Since `Image::new()` takes a `&mut Context`, so will the `new()` function.

```rust
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
```

and then just 

```rust
let my_dest = Vec2::new(500.0, 200.0);
canvas.draw(&self.image1, DrawParam::default().dest(my_dest));
```

so we display the head PNG instead of the simple green sqauare we have drawn before.

Note that an `Image` hasn't a destination position so we need to set the destination
using the [`DrawParam`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.DrawParam.html).

You could find the complete source of this lesson [here](https://github.com/geckoblu-games/snake-ggez/blob/main/examples/02_drawanimage.rs)

