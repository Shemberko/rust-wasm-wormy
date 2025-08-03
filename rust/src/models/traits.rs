use crate::models::map::Map;
use web_sys::CanvasRenderingContext2d;

pub trait CanvasObject {
    fn draw(&self, ctx: &CanvasRenderingContext2d);
    fn update(&mut self, delta_time: f64, map: &Map, canvas_height: f64);
}

pub trait GravityObject {
    fn apply_physics(&mut self, map: &Map, canvas_height: f64);
    fn apply_gravity(&mut self);
    fn apply_vertical_movement(&mut self, map: &Map, canvas_height: f64);
    fn is_on_ground(&self, map: &Map) -> bool;
    fn handle_ground_collision(&mut self, map: &Map) -> bool;
    fn handle_ceiling_collision(&mut self, map: &Map) -> bool;
    fn check_vertical_bounds(&mut self, canvas_height: f64) -> bool;
}

pub trait MovableObject: CanvasObject {
    fn change_position(&mut self, dx: f64, dy: f64, map: &Map, canvas_height: f64);
    fn try_move_y(&mut self, dy: f64, map: &Map, canvas_height: f64) -> bool;
    fn can_move_horizontally(&self, x: f64, map: &Map) -> bool;
}

pub trait AnimatedObject {
    fn set_animation_row(&mut self, row: u32);
    fn update_animation_state(&mut self, is_moving: bool, is_on_ground: bool);
    fn draw_animation(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64);
}

pub trait InputControlledObject {
    fn is_moving_horizontally(&self) -> bool;
    fn is_left_pressed(&self) -> bool;
    fn is_right_pressed(&self) -> bool;
    fn is_jump_pressed(&self) -> bool;
    fn handle_horizontal_movement(&mut self, map: &Map, canvas_height: f64);
    fn handle_jump(&mut self, map: &Map, canvas_height: f64, is_on_ground: bool);
}
