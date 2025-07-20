use js_sys::Promise;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, CanvasRenderingContext2d, HtmlImageElement};

use crate::animation::Animation;
use crate::models::position::Position;

use crate::models::map::Map;
use crate::models::traits::CanvasObject;
use rand::Rng;
use std::collections::HashSet;
pub struct Player {
    pub position: Position,
    pub velocity_y: f64,
    pub width: f64,
    pub height: f64,
    pub horizontal_offset: f64,
    pub animation: Option<Animation>,
    pub pressed_keys: HashSet<String>,
    pub facing_left: bool,
}
impl CanvasObject for Player {
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        let draw_x = self.position.x - self.horizontal_offset;

        if self.facing_left {
            let _ = ctx.translate(
                self.position.x + self.width - self.horizontal_offset,
                self.position.y,
            );
            let _ = ctx.scale(-1.0, 1.0);
            if let Some(anim) = &self.animation {
                anim.draw(
                    ctx,
                    0.0,
                    0.0,
                    self.width + self.horizontal_offset * 2.0,
                    self.height,
                );
            } else {
                ctx.set_fill_style(&JsValue::from_str("blue").into());
                ctx.fill_rect(0.0, 0.0, self.width, self.height);
            }
        } else {
            if let Some(anim) = &self.animation {
                anim.draw(
                    ctx,
                    draw_x,
                    self.position.y,
                    self.width + self.horizontal_offset * 2.0,
                    self.height,
                );
            } else {
                ctx.set_fill_style(&JsValue::from_str("blue").into());
                ctx.fill_rect(self.position.x, self.position.y, self.width, self.height);
            }
        }

        ctx.restore();
    }

    fn update(&mut self, delta_time: f64, map: &Map, canvas_height: f64) {
        let is_on_ground = self.is_on_ground(map) || self.position.y + self.height >= canvas_height;
        let is_moving = self.pressed_keys.contains("ArrowLeft")
            || self.pressed_keys.contains("ArrowRight")
            || self.pressed_keys.contains("KeyA")
            || self.pressed_keys.contains("KeyD");

        if self.pressed_keys.contains("ArrowLeft") || self.pressed_keys.contains("KeyA") {
            self.facing_left = true;
            self.move_left(map);
        }

        if self.pressed_keys.contains("ArrowRight") || self.pressed_keys.contains("KeyD") {
            self.facing_left = false;
            self.move_right(map);
        }

        if (self.pressed_keys.contains("Space")
            || self.pressed_keys.contains("KeyW")
            || self.pressed_keys.contains("ArrowUp"))
            && is_on_ground
        {
            self.jump(map, canvas_height);
        }

        self.update_animation_state(is_moving, is_on_ground);
        self.apply_physics(map, canvas_height);
        if let Some(anim) = &mut self.animation {
            anim.update(delta_time, self.velocity_y);
        }
    }
}

// MovableObject for
impl Player {
    fn change_position(&mut self, dx: f64, dy: f64, map: Map) {
        self.position.x += dx;
        self.position.y += dy;
    }

    fn move_left(&mut self, map: &Map) {
        let new_x = self.position.x - 5.0;
        if map.can_move_to(new_x, self.position.y, self.width, self.height) {
            self.position.x = new_x;
        }
    }

    fn move_right(&mut self, map: &Map) {
        let new_x = self.position.x + 5.0;
        if map.can_move_to(new_x, self.position.y, self.width, self.height) {
            self.position.x = new_x;
        }
    }
}

// GravityObject for
impl Player {
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
        let feet_y = self.position.y + self.height + 1.0;
        let check_points = [
            self.position.x,
            self.position.x + self.width / 2.0,
            self.position.x + self.width - 1.0,
        ];
        for &x in &check_points {
            if map.is_solid_at(x, feet_y) {
                return true;
            }
        }
        false
    }

    pub fn try_move_y(&mut self, dy: f64, map: &Map, canvas_height: f64) -> bool {
        self.position.y += dy;

        if dy > 0.0 && self.is_on_ground(map) {
            let feet_y = self.position.y + self.height + 1.0;
            let tile_row = (feet_y / map.tile_size).floor();
            self.position.y = tile_row * map.tile_size - self.height;
            return false;
        }

        if dy < 0.0 {
            let head_y = self.position.y;
            let check_points = [
                self.position.x + 1.0,
                self.position.x + self.width / 2.0,
                self.position.x + self.width - 1.0,
            ];
            for &px in &check_points {
                if map.is_solid_at(px, head_y) {
                    let tile_row = (head_y / map.tile_size).floor();
                    self.position.y = (tile_row + 1.0) * map.tile_size;
                    return false;
                }
            }
        }

        if self.position.y + self.height >= canvas_height {
            self.position.y = canvas_height - self.height;
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

        // Available colors for the player
        let colors = [
            "black", "blue", "brown", "cyan", "green", "lime", "orange", "pink", "purple", "red",
            "white", "yellow",
        ];
        // Pick a random color
        let mut rng = rand::thread_rng();
        let color = colors[rng.gen_range(0..colors.len())];
        let src = format!(
            "animations/NuclearLeak_CharacterAnim_1.2/character_20x20_{}.png",
            color
        );
        img.set_src(&src);

        let animation = Animation::new(
            img,
            20.0,
            20.0,
            vec![4, 4, 6, 3, 2, 6], // кількість кадрів у рядку
            0.1,
            1,
        );

        Self {
            position: Position { x: 100.0, y: 50.0 },
            velocity_y: 0.0,
            width: 64.0,
            height: 64.0,
            horizontal_offset: 22.0,
            animation: Some(animation),
            pressed_keys: HashSet::new(),
            facing_left: false,
        }
    }

    pub fn jump(&mut self, map: &Map, canvas_height: f64) {
        let is_on_ground = self.position.y + self.height >= canvas_height;
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

    let colors = [
        "black", "blue", "brown", "cyan", "green", "lime", "orange", "pink", "purple", "red",
        "white", "yellow",
    ];
    let mut rng = rand::thread_rng();
    let color = colors[rng.gen_range(0..colors.len())];
    let src = format!(
        "animations/NuclearLeak_CharacterAnim_1.2/character_20x20_{}.png",
        color
    );
    img.set_src(&src);

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
        position: Position { x: 50.0, y: 50.0 },
        velocity_y: 0.0,
        width: 64.0,
        height: 64.0,
        horizontal_offset: 22.0,
        animation: Some(animation),
        pressed_keys: HashSet::new(),
        facing_left: false,
    })
}
