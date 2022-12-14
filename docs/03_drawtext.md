As we are drawing, sooner or later we should draw some text.

So just start with a simple "Hello World".

```rust
let text = Text::new("Hello World");
canvas.draw(&text, DrawParam::default());
```

In general, you shouldn’t call [`Text::new()`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.Text.html#method.new) in your draw loop.
Text rendering is fairly expensive, so you should cache it and call [`Text::new()`](https://docs.rs/ggez/0.8.1/ggez/graphics/struct.Text.html#method.new) only when the text changes.
But for the moment we can live with it.

Easy?

Let's tray to set some attributes

```rust
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
```

comments should be self explanatory, it's all for this lesson.

You could find the complete source of this lesson [here](https://github.com/geckoblu-games/snake-ggez/blob/main/examples/03_drawtext.rs)



