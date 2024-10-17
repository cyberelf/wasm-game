use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d, KeyboardEvent};
use std::cell::RefCell;
use std::rc::Rc;

// Snake direction constants
const UP: i32 = 0;
const RIGHT: i32 = 1;
const DOWN: i32 = 2;
const LEFT: i32 = 3;

// Define the Snake structure
struct Snake {
    body: Vec<(i32, i32)>,
    direction: i32,
}

impl Snake {
    fn new(body_size: i32) -> Snake {
        Snake {
            body: vec![(body_size, body_size)], // Initial position
            direction: RIGHT,     // Initial direction
        }
    }

    fn update(&mut self) {
        let (x, y) = self.body[0];

        let new_head = match self.direction {
            UP => (x, y - 1),
            RIGHT => (x + 1, y),
            DOWN => (x, y + 1),
            LEFT => (x - 1, y),
            _ => (x, y),
        };

        self.body.insert(0, new_head);
        self.body.pop(); // Remove the tail
    }

    fn change_direction(&mut self, dir: i32) {
        self.direction = dir;
    }

    fn head_position(&self) -> (i32, i32) {
        self.body[0]
    }
}

// Game state
struct Game {
    snake: Snake,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    width: i32,
    height: i32,
    cell_size: i32,
    game_over: bool,
}

impl Game {
    fn new() -> Game {
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

        Game {
            snake: Snake::new(10),
            canvas,
            ctx,
            width,
            height,
            cell_size: 10,  // Size of each grid cell
            game_over: false,
        }
    }

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

    fn change_direction(&mut self, dir: i32) {
        self.snake.change_direction(dir);
    }

    fn reset(&mut self) {
        *self = Game::new(); // Reinitialize the game state
    }
}

// Rc<RefCell<Game>> is used to manage the game instance globally
thread_local! {
    static GAME_INSTANCE: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new()));
}

// Entry point
#[wasm_bindgen(start)]
pub fn start() {
    GAME_INSTANCE.with(|game| {
        let game_clone = Rc::clone(game);

        // Game loop closure
        let closure = Closure::wrap(Box::new(move || {
            game_clone.borrow_mut().update();
            game_clone.borrow().draw();
        }) as Box<dyn FnMut()>);

        // Set up the game loop to be called every 100ms
        window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 100)
            .unwrap();
        closure.forget();
    });

    // Set up event listener for key presses
    GAME_INSTANCE.with(|game| {
        let game_for_keydown = Rc::clone(game);
        let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = event.key();
            let direction = match key.as_str() {
                "ArrowUp" => UP,
                "ArrowRight" => RIGHT,
                "ArrowDown" => DOWN,
                "ArrowLeft" => LEFT,
                _ => return,
            };
            game_for_keydown.borrow_mut().change_direction(direction);
        }) as Box<dyn FnMut(_)>);

        window()
            .unwrap()
            .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
            .unwrap();
        keydown_closure.forget();
    });
}

// Function to reset the game state
#[wasm_bindgen]
pub fn reset_game() {
    GAME_INSTANCE.with(|game| {
        game.borrow_mut().reset();
    });
}
