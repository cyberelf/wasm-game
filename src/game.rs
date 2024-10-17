use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

use crate::traits::{Game, DIRECTION};

// Define the Snake structure
struct Snake {
    body: Vec<(i32, i32)>,
    direction: DIRECTION,
}

impl Snake {
    fn new(body_size: i32) -> Snake {
        Snake {
            body: vec![(body_size, body_size)], // Initial position
            direction: DIRECTION::RIGHT,     // Initial direction
        }
    }

    fn update(&mut self) {
        let (x, y) = self.body[0];

        let new_head = match self.direction {
            DIRECTION::UP => (x, y - 1),
            DIRECTION::RIGHT => (x + 1, y),
            DIRECTION::DOWN => (x, y + 1),
            DIRECTION::LEFT => (x - 1, y),
            _ => (x, y),
        };

        self.body.insert(0, new_head);
        self.body.pop(); // Remove the tail
    }

    fn change_direction(&mut self, dir: DIRECTION) {
        self.direction = dir;
    }

    fn head_position(&self) -> (i32, i32) {
        self.body[0]
    }
}

// Game state
pub(crate) struct Game2D {
    snake: Snake,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    width: i32,
    height: i32,
    cell_size: i32,
    game_over: bool,
}

impl Game2D {
    pub(crate) fn new() -> Game2D {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("snake_canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let width = canvas.width() as i32;
        let height = canvas.height() as i32;

        Self {
            snake: Snake::new(10),
            canvas,
            ctx,
            width,
            height,
            cell_size: 10,  // Size of each grid cell
            game_over: false,
        }
    }
}

// implement the Game trait for Game2D
impl Game for Game2D {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.snake.update();

        // Check for collision with the border
        let (head_x, head_y) = self.snake.head_position();
        if head_x < 0 || head_x >= self.width / self.cell_size || head_y < 0 || head_y >= self.height / self.cell_size {
            self.game_over = true;
        }
    }

    fn draw(&self) {
        // Clear the canvas
        self.ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);

        // Draw the snake
        for (x, y) in &self.snake.body {
            self.ctx.set_fill_style_str("green");
            self.ctx.fill_rect(
                (*x * self.cell_size) as f64,
                (*y * self.cell_size) as f64,
                self.cell_size as f64,
                self.cell_size as f64,
            );
        }

        // Draw the border
        self.ctx.set_stroke_style_str("black");
        self.ctx.set_line_width(2.0);
        self.ctx.stroke_rect(
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
        );

        // Display "Game Over" text if the game has ended
        if self.game_over {
            self.ctx.set_fill_style_str("red");
            self.ctx.set_font("30px Arial");
            self.ctx.fill_text("Game Over", (self.width / 2 - 80) as f64, (self.height / 2) as f64).unwrap();
        }
    }

    fn change_direction(&mut self, dir: DIRECTION) {
        self.snake.change_direction(dir);
    }

    fn reset(&mut self) {
        *self = Self::new(); // Reinitialize the game state
    }
}
