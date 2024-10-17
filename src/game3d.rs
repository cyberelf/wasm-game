use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};
use std::cell::RefCell;
use std::rc::Rc;
use three::prelude::*;

use crate::traits::{Game, DIRECTION};

#[wasm_bindgen]
pub fn start() {
    // Get the canvas element
    let document = window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("snake_canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();

    // Set up the 3D scene
    let context = Context::new(&canvas).unwrap();
    let mut scene = Scene::new(&context);
    let camera = PerspectiveCamera::new(45.0, canvas.width() as f32 / canvas.height() as f32, 0.1, 1000.0);
    camera.set_position(&Vector3::new(0.0, 5.0, 5.0));
    camera.look_at(&Vector3::new(0.0, 0.0, 0.0));

    // Add a grid to represent the snake's area
    let plane = Plane::new(10.0, 10.0, 10, 10);
    let plane_material = MeshBasicMaterial::new(&BasicMaterialOptions {
        color: 0x00ff00,
        ..Default::default()
    });
    let plane_mesh = Mesh::new(plane, &plane_material);
    scene.add(&plane_mesh);

    // Set up the render loop
    let game_state = Rc::new(RefCell::new(Game::new(&scene)));
    let game_state_clone = Rc::clone(&game_state);
    let closure = Closure::wrap(Box::new(move || {
        game_state_clone.borrow_mut().update();
        game_state_clone.borrow().draw(&context, &camera);
        context.render(&scene, &camera);
        window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref()).unwrap();
    }) as Box<dyn FnMut()>);

    window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

// Game struct to manage the state
struct Game {
    snake: Snake,
    scene: Scene,
}

impl Game {
    fn new(scene: &Scene) -> Self {
        let snake = Snake::new();
        Self {
            snake,
            scene: scene.clone(),
        }
    }

    fn update(&mut self) {
        // Update snake position
        self.snake.update();
    }

    fn draw(&self, context: &Context, camera: &PerspectiveCamera) {
        // Clear previous frame
        context.clear();

        // Draw snake
        self.snake.draw(&context);
    }
}

// Snake struct for managing snake state
struct Snake {
    segments: Vec<Mesh>,
}

impl Snake {
    fn new() -> Self {
        // Initialize snake with one segment
        let segment = Box::new(Cube::new(1.0, 1.0, 1.0));
        let material = MeshBasicMaterial::new(&BasicMaterialOptions {
            color: 0xff0000,
            ..Default::default()
        });
        let segment_mesh = Mesh::new(segment, &material);
        Self {
            segments: vec![segment_mesh],
        }
    }

    fn update(&mut self) {
        // Logic for moving the snake
        // You may want to update the positions of the segments based on direction
    }

    fn draw(&self, context: &Context) {
        // Draw each segment of the snake
        for segment in &self.segments {
            context.render_mesh(segment);
        }
    }
}

// Function to reset the game state
#[wasm_bindgen]
pub fn reset_game() {
    // Reset logic here
    // For example, you might want to reset the snake position and size
}
