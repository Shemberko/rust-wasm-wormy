use js_sys::Promise;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, CanvasRenderingContext2d, HtmlImageElement};

use crate::animation::Animation;
use crate::models::position::Position;
use crate::models::weapon::Weapon;

use crate::models::map::Map;
use crate::models::traits::{
    AnimatedObject, CanvasObject, GravityObject, InputControlledObject, MovableObject,
};
use rand::Rng;
use std::collections::HashSet;
pub struct Player {
    pub position: Position,
    pub velocity_y: f64,
    pub width: f64,
    pub height: f64,
    pub animation: Option<Animation>,
    pub pressed_keys: HashSet<String>,
    pub facing_left: bool,
    pub weapon: Option<Weapon>,
}

impl CanvasObject for Player {
    fn draw(&self, ctx: &CanvasRenderingContext2d, map: &Map) {
        let draw_x = self.position.x - map.camera_x as f64;
        let draw_y = self.position.y - map.camera_y as f64;

        ctx.save();
        if self.facing_left {
            let _ = ctx.translate(draw_x + self.width, draw_y);
            let _ = ctx.scale(-1.0, 1.0);
            self.draw_animation(ctx, 0.0, 0.0);
        } else {
            self.draw_animation(ctx, draw_x, draw_y);
        }
        ctx.restore();

        if let Some(weapon) = &self.weapon {
            weapon.draw(ctx, map); // weapon теж повинен враховувати зсув
        }
    }

    fn update(&mut self, delta_time: f64, map: &Map, canvas_height: f64) {
        let is_on_ground = self.check_if_on_ground(map, canvas_height);
        let is_moving = self.is_moving_horizontally();

        self.handle_horizontal_movement(map, canvas_height);
        self.handle_jump(map, canvas_height, is_on_ground);

        self.update_animation_state(is_moving, is_on_ground);
        self.apply_physics(map, canvas_height);

        if let Some(anim) = &mut self.animation {
            anim.update(delta_time, is_moving, is_on_ground, self.velocity_y);
        }

        if let Some(weapon) = &mut self.weapon {
            weapon.update_position(self.position.x, self.position.y, self.facing_left);
            weapon.update(delta_time, map, canvas_height);
            weapon.update_animation_state(is_moving, is_on_ground);
        }
    }
}

impl MovableObject for Player {
    fn change_position(&mut self, dx: f64, dy: f64, map: &Map, canvas_height: f64) {
        if dx != 0.0 {
            let new_x = self.position.x + dx;
            if self.can_move_horizontally(new_x, map) {
                self.position.x = new_x;
            }
        }

        if dy != 0.0 {
            let new_y = self.position.y + dy;

            if new_y + self.height >= canvas_height {
                self.position.y = canvas_height - self.height;
            } else if new_y < 0.0 {
                self.position.y = 0.0;
            } else {
                self.position.y = new_y;
            }
        }
    }

    fn try_move_y(&mut self, dy: f64, map: &Map, canvas_height: f64) -> bool {
        self.position.y += dy;

        if dy > 0.0 && self.handle_ground_collision(map) {
            return false;
        }

        if dy < 0.0 && self.handle_ceiling_collision(map) {
            return false;
        }

        if self.check_vertical_bounds(canvas_height) {
            return false;
        }

        true
    }

    fn can_move_horizontally(&self, x: f64, map: &Map) -> bool {
        let top = self.position.y + 1.0;
        let bottom = self.position.y + self.height - 1.0;

        let left = x;
        let right = x + self.width;

        for &y in &[top, bottom] {
            if !map.is_solid_at(left, y) && !map.is_solid_at(right, y) {
                continue;
            } else {
                return false;
            }
        }
        true
    }
}

impl GravityObject for Player {
    fn apply_physics(&mut self, map: &Map, canvas_height: f64) {
        self.apply_gravity();
        self.apply_vertical_movement(map, canvas_height);
    }

    fn apply_gravity(&mut self) {
        const GRAVITY: f64 = 0.5;
        self.velocity_y += GRAVITY;
    }

    fn apply_vertical_movement(&mut self, map: &Map, canvas_height: f64) {
        const MAX_STEP: f64 = 1.0; // субкрок — не більше 1px за раз

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

    fn is_on_ground(&self, map: &Map) -> bool {
        let feet_y = self.position.y + self.height + 1.0;
        let mut x = self.position.x + 1.0;
        while x <= self.position.x + self.width {
            if map.is_solid_at(x, feet_y) {
                return true;
            }
            x += 19.0;
        }
        // Also check the very right edge in case width is not a multiple of 4
        if map.is_solid_at(self.position.x + self.width - 1.0, feet_y) {
            return true;
        }
        false
    }

    fn handle_ground_collision(&mut self, map: &Map) -> bool {
        let on_ground = self.is_on_ground(map);

        if on_ground {
            loop {
                self.position.y -= 0.1;
                if !self.is_on_ground(map) {
                    self.position.y += 0.1;
                    break;
                }
            }
            self.velocity_y = 0.0;
            true
        } else {
            false
        }
    }

    fn handle_ceiling_collision(&mut self, map: &Map) -> bool {
        let head_y = self.position.y;
        let check_points = [
            self.position.x + 1.0,
            self.position.x + self.width / 2.0,
            self.position.x + self.width - 1.0,
        ];

        for &px in &check_points {
            if map.is_solid_at(px, head_y) {
                self.position.y = head_y.ceil();
                return true;
            }
        }
        false
    }

    fn check_vertical_bounds(&mut self, canvas_height: f64) -> bool {
        if self.position.y + self.height >= canvas_height {
            self.position.y = canvas_height - self.height;
            return true;
        }
        if self.position.y < 0.0 {
            self.position.y = 0.0;
            return true;
        }
        false
    }
}

impl AnimatedObject for Player {
    fn set_animation_row(&mut self, row: u32) {
        if let Some(anim) = &mut self.animation {
            anim.set_animation_row(row as usize);
        }
    }

    fn update_animation_state(&mut self, is_moving: bool, is_on_ground: bool) {
        if !is_on_ground {
            self.set_animation_row(3); // jump / falling
        } else if is_moving {
            self.set_animation_row(2); // walking
        } else {
            self.set_animation_row(1); // idle
        }
    }

    fn draw_animation(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
        if let Some(anim) = &self.animation {
            anim.draw(ctx, x, y, self.width, self.height);
        } else {
            ctx.set_fill_style(&JsValue::from_str("blue"));
            ctx.fill_rect(x, y, self.width, self.height);
        }
    }
}

impl InputControlledObject for Player {
    fn is_moving_horizontally(&self) -> bool {
        self.is_left_pressed() || self.is_right_pressed()
    }

    fn is_left_pressed(&self) -> bool {
        self.pressed_keys.contains("ArrowLeft") || self.pressed_keys.contains("KeyA")
    }

    fn is_right_pressed(&self) -> bool {
        self.pressed_keys.contains("ArrowRight") || self.pressed_keys.contains("KeyD")
    }

    fn is_jump_pressed(&self) -> bool {
        self.pressed_keys.contains("Space")
            || self.pressed_keys.contains("KeyW")
            || self.pressed_keys.contains("ArrowUp")
    }

    fn handle_horizontal_movement(&mut self, map: &Map, canvas_height: f64) {
        const MOVE_SPEED: f64 = 5.0;
        if self.is_left_pressed() {
            self.facing_left = true;
            self.change_position(-MOVE_SPEED, 0.0, map, canvas_height);
        }

        if self.is_right_pressed() {
            self.facing_left = false;
            self.change_position(MOVE_SPEED, 0.0, map, canvas_height);
        }
    }

    fn handle_jump(&mut self, map: &Map, canvas_height: f64, is_on_ground: bool) {
        if self.is_jump_pressed() && is_on_ground {
            self.jump(map, canvas_height);
        }
    }
}

impl Player {
    pub async fn new() -> Result<Player, JsValue> {
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

        let animation = Animation::new(img, 20.0, 20.0, vec![4, 4, 6, 3, 2, 6], 0.1, 1);
        // let weapon = Weapon::new(
        //     50.0,
        //     50.0,
        //     vec![
        //         (
        //             "idle".to_string(),
        //             "animations/guns/[IDLE] AK 47.png".to_string(),
        //         ),
        //         (
        //             "shoot".to_string(),
        //             "animations/guns/[SHOOT WITH MUZZLE FLASH] AK 47.png".to_string(),
        //         ),
        //     ],
        // )
        // .await?;

        Ok(Player {
            position: Position { x: 50.0, y: 300.0 },
            velocity_y: 0.0,
            width: 64.0,
            height: 64.0,
            animation: Some(animation),
            pressed_keys: HashSet::new(),
            facing_left: false,
            weapon: None,
        })
    }

    pub fn jump(&mut self, map: &Map, canvas_height: f64) {
        if self.check_if_on_ground(map, canvas_height) {
            self.velocity_y = -10.0;
        }
    }

    pub fn set_pressed_keys(&mut self, keys: HashSet<String>) {
        self.pressed_keys = keys;
    }

    fn check_if_on_ground(&self, map: &Map, canvas_height: f64) -> bool {
        self.is_on_ground(map) || self.position.y + self.height >= canvas_height
    }
}
