trait CanvasObject {
    fn draw(&self, ctx: &CanvasRenderingContext2d);
    fn set_state(&mut self, delta_time: f64);
    fn has_collision(&self, other: &Self) -> bool;
}

trait GravityObject: CanvasObject {
    fn apply_gravity(&mut self, gravity: f64);
    fn is_on_ground(&self, map: &Map) -> bool;
}

trait MovableObject: CanvasObject {
    fn change_position(&mut self, dx: f64, dy: f64);
}
