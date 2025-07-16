    use js_sys::Promise;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, CanvasRenderingContext2d, HtmlImageElement};

use crate::animation::Animation;

use crate::models::traits::{CanvasObject};
use crate::models::map::Map;
use std::collections::HashSet;
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub velocity_y: f64,
    pub width: f64,
    pub height: f64,
    pub animation: Option<Animation>,
    pub pressed_keys:  HashSet<String>,

}
impl CanvasObject for Player {
    
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        if let Some(anim) = &self.animation {
            anim.draw(ctx, self.x, self.y, self.width, self.height);
        } else {
            ctx.set_fill_style(&JsValue::from_str("blue"));
            ctx.fill_rect(self.x, self.y, self.width, self.height);
        }
    }

    fn update(&mut self, delta_time: f64) {
        if let Some(anim) = &mut self.animation {
            anim.update(delta_time);
        }
    }
}

// MovableObject for
impl  Player{
    fn change_position(&mut self, dx: f64, dy: f64, map: Map) {
        self.x += dx;
        self.y += dy;
    }

    fn move_left(&mut self, map: &Map) {
        let new_x = self.x - 5.0;
        if map.can_move_to(new_x, self.y, self.width, self.height) {
            self.x = new_x;
        }
    }

    fn move_right(&mut self, map: &Map) {
        let new_x = self.x + 5.0;
        if map.can_move_to(new_x, self.y, self.width, self.height) {
            self.x = new_x;
        }
    }


}

// GravityObject for
impl  Player{
    pub fn apply_gravity(&mut self, gravity: f64) {
        self.velocity_y += gravity;
    }

    pub fn apply_physics(&mut self, map: &Map, canvas_height: f64) {
        const GRAVITY: f64 = 0.5;
        const MAX_STEP: f64 = 1.0; // субкрок — не більше 1px за раз

        self.velocity_y += GRAVITY;

        let mut remaining = self.velocity_y;
        let step = MAX_STEP.copysign(self.velocity_y); // +1 або -1

        while remaining.abs() >= MAX_STEP {
            if !self.try_move_y(step, map, canvas_height) {
                self.velocity_y = 0.0;
                return;
            }
            remaining -= step;
        }

        // останній малий крок
        if remaining.abs() > 0.0 {
            self.try_move_y(remaining, map, canvas_height);
        }
    }
    
    pub fn is_on_ground(&self, map: &Map) -> bool {
        let feet_y = self.y + self.height + 1.0;
        let check_points = [self.x, self.x + self.width / 2.0, self.x + self.width - 1.0];
        for &x in &check_points {
            if map.is_solid_at(x, feet_y) {
                return true;
            }
        }
        false
    }

    
    pub fn try_move_y(&mut self, dy: f64, map: &Map, canvas_height: f64) -> bool {
        self.y += dy;

        if dy > 0.0 && self.is_on_ground(map) {
            let feet_y = self.y + self.height + 1.0;
            let tile_row = (feet_y / map.tile_size).floor();
            self.y = tile_row * map.tile_size - self.height;
            return false;
        }

        if dy < 0.0 {
            let head_y = self.y;
            let check_points = [
                self.x + 1.0,
                self.x + self.width / 2.0,
                self.x + self.width - 1.0,
            ];
            for &px in &check_points {
                if map.is_solid_at(px, head_y) {
                    let tile_row = (head_y / map.tile_size).floor();
                    self.y = (tile_row + 1.0) * map.tile_size;
                    return false;
                }
            }
        }

        if self.y + self.height >= canvas_height {
            self.y = canvas_height - self.height;
            return false;
        }

        true
    }

}


impl Player {
    pub fn new() -> Self {
        let document = window().unwrap().document().unwrap();
        let img = document
            .create_element("img")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        img.set_src("animations/NuclearLeak_CharacterAnim_1.2/character_20x20_red.png");

        let animation = Animation::new(
            img,
            20.0,
            20.0,
            vec![4, 4, 6, 3, 2, 6], // кількість кадрів у рядку
            0.1,
            1,
        );

        Self {
            x: 100.0,
            y: 50.0,
            velocity_y: 0.0,
            width: 64.0,
            height: 64.0,
            animation: Some(animation),
            pressed_keys: HashSet::new(),
        }
    }


    pub fn jump(&mut self, map: &Map, canvas_height: f64) {
        let is_on_ground = self.y + self.height >= canvas_height;
        let is_on_platform = self.is_on_ground(map);
        if is_on_ground || is_on_platform {
            self.velocity_y = -10.0;
        }
    }

    pub fn set_animation_row(&mut self, row: u32) {
        if let Some(anim) = &mut self.animation {
            anim.set_animation_row(row as usize);
        }
    }

    pub fn update_animation_state(&mut self, is_moving: bool, is_on_ground: bool) {
        if !is_on_ground {
            self.set_animation_row(3); // jump / falling
        } else if is_moving {
            self.set_animation_row(2); // walking
        } else {
            self.set_animation_row(1); // idle
        }
    }

    pub fn set_pressed_keys(&mut self, keys: HashSet<String>) {
        self.pressed_keys = keys;
    }
}

pub async fn create_player() -> Result<Player, JsValue> {
    let document = window().unwrap().document().unwrap();
    let img = document
        .create_element("img")?
        .dyn_into::<HtmlImageElement>()?;

    let promise = Promise::new(&mut |resolve, reject| {
        let onload = Closure::once_into_js(move || {
            resolve.call0(&JsValue::NULL).unwrap();
        });

        let onerror = Closure::once_into_js(move || {
            reject
                .call1(&JsValue::NULL, &JsValue::from_str("Image failed to load"))
                .unwrap();
        });

        img.set_onload(Some(onload.unchecked_ref()));
        img.set_onerror(Some(onerror.unchecked_ref()));
    });

    img.set_src("/animations/NuclearLeak_CharacterAnim_1.2/character_20x20_red.png");

    JsFuture::from(promise).await?;

    let animation = Animation::new(
        img,
        20.0,
        20.0,
        vec![4, 4, 6, 3, 2, 6], // кількість кадрів у рядку
        0.1,
        1,
    );

    Ok(Player {
        x: 100.0,
        y: 50.0,
        velocity_y: 0.0,
        width: 64.0,
        height: 64.0,
        animation: Some(animation),
        pressed_keys: HashSet::new(),
    })
}

