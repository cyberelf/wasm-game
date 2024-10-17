mod game;
// mod game3d;
mod traits;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window,  KeyboardEvent};
use std::cell::RefCell;
use std::rc::Rc;

use game::Game2D;
use traits::{Game, DIRECTION};

// Rc<RefCell<Game>> is used to manage the game instance globally
thread_local! {
    static GAME_INSTANCE: Rc<RefCell<dyn Game>> = Rc::new(RefCell::new(Game2D::new()));
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
                "ArrowUp" => DIRECTION::UP,
                "ArrowRight" => DIRECTION::RIGHT,
                "ArrowDown" => DIRECTION::DOWN,
                "ArrowLeft" => DIRECTION::LEFT,
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
