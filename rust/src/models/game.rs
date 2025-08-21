use std::rc::Rc;

use crate::models::bullet::Bullet;
use crate::models::map::Map;
use crate::models::player::Player;
use crate::models::traits::CanvasObject;
use web_sys::{CanvasRenderingContext2d, ImageData};

// move bullets and all other new objects to objects array
pub struct Game {
    pub map: Map,
    pub players: Vec<Player>,
    pub objects: Vec<Box<dyn CanvasObject>>,
    pub canvas: Rc<CanvasRenderingContext2d>,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub bullets: Vec<Bullet>,
}

impl Game {
    pub fn new(
        canvas_width: u32,
        canvas_height: u32,
        canvas: Rc<CanvasRenderingContext2d>,
        data: ImageData,
    ) -> Self {
        let map = Map::new(canvas_width, canvas_height, Rc::clone(&canvas), data);
        let players = Vec::new();
        let objects: Vec<Box<dyn CanvasObject>> = Vec::new();

        Game {
            map,
            players,
            objects,
            canvas,
            canvas_width,
            canvas_height,
            bullets: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn draw(&mut self) {
        self.canvas.clear_rect(
            0.0,
            0.0,
            self.canvas_width as f64,
            self.canvas_height as f64,
        );

        if let Some(player) = self.players.get(0) {
            self.map.update_camera(player);
        }
        self.map.draw();

        self.players.iter().for_each(|player| {
            player.draw(&self.canvas, &self.map);
        });

        for bullet in &self.bullets {
            bullet.draw(&self.canvas, &self.map);
        }
    }

    pub fn get_current_player_mut(&mut self) -> Option<&mut Player> {
        self.players.first_mut()
    }

    pub fn update(&mut self) {
        let canvas_height = self.map.image_data.height() as f64;

        for player in &mut self.players {
            player.update(0.016, &mut self.map, canvas_height);
        }

        for bullet in &mut self.bullets {
            bullet.update(0.016, &mut self.map, canvas_height);
        }

        self.bullets.retain(|b| b.is_active);
    }
}
