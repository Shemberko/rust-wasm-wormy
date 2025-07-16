use web_sys::CanvasRenderingContext2d;
use crate::animation::Animation;
use crate::models::map::Map;

pub trait CanvasObject {
    fn draw(&self, ctx: &CanvasRenderingContext2d);
    fn update(&mut self, delta_time: f64);
}

pub trait GravityObject: CanvasObject {
    fn apply_gravity(&mut self, gravity: f64, map: Map);
    fn is_on_ground(&self, map: &Map) -> bool;
}

pub trait MovableObject: CanvasObject {
    fn change_position(&mut self, dx: f64, dy: f64);
    fn move_left(&mut self, map: Map);
    fn move_right(&mut self, map: Map); 
    fn move_up(&mut self, map: Map);
    fn move_down(&mut self, map: Map);
    fn try_move_y(&mut self, dy: f64, map: &Map);
    fn try_move_x(&mut self, dx: f64, map: &Map);
}

pub trait AnimatedObject: CanvasObject {
    fn set_animation(&mut self, animation: Animation);
    fn update_animation_state(&mut self, is_moving: bool, is_on_ground: bool);
}
