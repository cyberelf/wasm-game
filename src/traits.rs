// The game trait

pub enum DIRECTION {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub trait Game {
    fn update(&mut self);
    fn draw(&self);
    fn reset(&mut self);
    fn change_direction(&mut self, dir: DIRECTION);
}
