## Food

Time to eat some food

We need to place a fruit ramdomly on the grid

```rust
struct MyGame {
 
    fruit_pos: GridPosition,
    fruit_image: Image,

    /// Our RNG state
    rng: Rand32,
    
    ...
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let fruit_image = Image::from_path(ctx, "/apple.png")?;

        // We seed our RNG with the system RNG.
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));

        let fruit_pos = GridPosition::random(&mut rng);

        let g = MyGame {
			...
        };
        Ok(g)
    }
}

impl GridPosition {
    pub fn random(rng: &mut Rand32) -> Self {
        let x = rng.rand_range(0..GRID_WIDTH);
        let y = rng.rand_range(0..GRID_HEIGHT);
        GridPosition { x, y }
    }
    
    ...
}
 
impl EventHandler for MyGame {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a new Canvas that renders directly to the window surface.
        let mut canvas = Canvas::from_frame(ctx, COLOR_BACKGROUND);

		...

        // Draw the fruit
        canvas.draw(
            &self.fruit_image,
            DrawParam::default().dest(self.fruit_pos.to_vec2()),
        );

		...

        // Finish drawing with this canvas and submit all the draw calls.
        canvas.finish(ctx)
    }
}

```

and then we check for collision

```rust
impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.process_input(ctx);

        self.head_timer += ctx.time.delta();

        if self.head_timer >= MOVE_TIME {
            self.head_pos.move_to_direction(self.dir);

            if self.head_pos == self.fruit_pos {
                self.score += 10;
                self.fruit_pos = GridPosition::random(&mut self.rng);
            }

            self.head_timer = Duration::from_millis(0);
        }

        Ok(())
    }
}
```

## Body growth

Every time we eat a fruit the body grows

```rust
struct MyGame {
    body: LinkedList<GridPosition>,

	...
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.process_input(ctx);

        self.head_timer += ctx.time.delta();

        if self.head_timer >= MOVE_TIME {
            self.body.push_front(self.head_pos);
            self.head_pos.move_to_direction(self.dir);

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
		...

        for seg in self.body.iter() {
            canvas.draw(&self.body_image, DrawParam::default().dest(seg.to_vec2()));
        }

       ...
    }    
}
```

every time the head moves we put a body segment in front and remove one
from the tail this generates the illusion the body is moving.
We don't remove the tail if the snake eats a fruit, so the body grows.

We then need to check if the head collide with its body, in that case the game is over.

```rust
impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
		
		...
		
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
}
```

You could find the complete source of this lesson [here](https://github.com/geckoblu-games/snake-ggez/blob/main/examples/06_snake.rs)

## End

The game is complete now, well playable, it needs to fix some details to be a complete game.
A start menu and a game over menu,
a way to avoid to turn the head in the opposite direction (this would cause an immediate game over)
and a way to avoid to place a new fruit over the snake body.

This is left as an exercise but you can find my solution [here](https://github.com/geckoblu-games/snake-ggez).
