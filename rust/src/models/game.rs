use std::rc::Rc;

use crate::models::map::Map;
use crate::models::player::Player;
use crate::models::traits::CanvasObject;
use web_sys::CanvasRenderingContext2d;

pub struct Game {
    pub map: Map,
    pub players: Vec<Player>,
    pub objects: Vec<Box<dyn CanvasObject>>,
    pub canvas: Rc<CanvasRenderingContext2d>,
    pub canvas_width: f64,
    pub canvas_height: f64,
}

impl Game {
    pub fn new(
        canvas_width: f64,
        canvas_height: f64,
        canvas: Rc<CanvasRenderingContext2d>,
    ) -> Self {
        let map = Map::new(canvas_width, canvas_height, Rc::clone(&canvas));
        let players = Vec::new();
        let objects: Vec<Box<dyn CanvasObject>> = Vec::new();

        Game {
            map,
            players,
            objects,
            canvas,
            canvas_width,
            canvas_height,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn add_object(&mut self, object: Box<dyn CanvasObject>) {
        self.objects.push(object);
    }

    pub fn draw(&self) {
        self.canvas
            .clear_rect(0.0, 0.0, self.canvas_width, self.canvas_height);

        self.map.draw();

        self.players.iter().for_each(|player| {
            player.draw(&self.canvas);
        });
    }

    pub fn get_current_player_mut(&mut self) -> Option<&mut Player> {
        self.players.first_mut()
    }

    pub fn update(&mut self) {
        let map = &self.map;
        let canvas_height = self.canvas_height;

        self.players.iter_mut().for_each(|player| {
            player.update(0.016, map, canvas_height);
        });
    }
}
