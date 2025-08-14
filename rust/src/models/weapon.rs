use std::collections::HashMap;

use crate::animation::Animation;
use crate::models::traits::AnimatedObject;
use crate::models::{map::Map, position::Position, traits::CanvasObject};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::CanvasRenderingContext2d;
use web_sys::{window, HtmlImageElement};

pub struct Weapon {
    pub position: Position,
    pub width: f64,
    pub height: f64,
    pub animations: HashMap<String, Animation>,
    pub current_anim: String,
    pub facing_left: bool,
}

impl Weapon {
    pub async fn new(
        x: f64,
        y: f64,
        animations_src: Vec<(String, String)>,
    ) -> Result<Self, JsValue> {
        let document = window().unwrap().document().unwrap();

        let mut animations = HashMap::new();

        for (name, src) in animations_src {
            let img = document
                .create_element("img")?
                .dyn_into::<HtmlImageElement>()?;

            let promise = Promise::new(&mut |resolve, reject| {
                // Потрібно клонувати `name` і `src`, бо вони використовуються у замиканні
                let src_clone = src.clone();
                let onload = Closure::once_into_js(move || {
                    resolve.call0(&JsValue::NULL).unwrap();
                });

                let onerror = Closure::once_into_js(move || {
                    reject
                        .call1(
                            &JsValue::NULL,
                            &format!("Failed to load image {}", src_clone).into(),
                        )
                        .unwrap();
                });

                img.set_onload(Some(onload.unchecked_ref()));
                img.set_onerror(Some(onerror.unchecked_ref()));
            });

            img.set_src(&src);
            JsFuture::from(promise).await?;

            let animation = Animation::new(img, 96.0, 48.0, vec![12], 0.1, 0);

            animations.insert(name, animation);
        }

        Ok(Self {
            position: Position { x, y },
            width: 96.0,
            height: 48.0,
            animations,
            current_anim: "idle".to_string(),
            facing_left: false,
        })
    }

    pub fn update_position(&mut self, player_x: f64, player_y: f64, facing_left: bool) {
        self.facing_left = facing_left;

        self.position.x = if facing_left {
            player_x - self.width + 34.0
        } else {
            player_x + 24.0
        };
        self.position.y = player_y + 35.0;
    }
}

impl CanvasObject for Weapon {
    fn draw(&self, ctx: &CanvasRenderingContext2d, map: &Map) {
        let draw_x = self.position.x - map.camera_x as f64;
        let draw_y = self.position.y - map.camera_y as f64;

        ctx.save();

        if let Some(anim) = self.animations.get(&self.current_anim) {
            if self.facing_left {
                ctx.scale(-1.0, 1.0).unwrap();
                anim.draw(ctx, -draw_x - self.width, draw_y, self.width, self.height);
            } else {
                anim.draw(ctx, draw_x, draw_y, self.width, self.height);
            }
        }

        ctx.restore();
    }

    fn update(&mut self, delta_time: f64, _map: &Map, _canvas_height: f64) {
        if let Some(anim) = self.animations.get_mut(&self.current_anim) {
            anim.update(delta_time, true, true, 0.0);
        }
    }
}

impl AnimatedObject for Weapon {
    fn set_animation_row(&mut self, row: u32) {
        if let Some(anim) = self.animations.get_mut(&self.current_anim) {
            anim.set_animation_row(row as usize);
        }
    }

    fn update_animation_state(&mut self, is_moving: bool, is_on_ground: bool) {
        if self.current_anim == "shoot" {
            if let Some(anim) = self.animations.get(&self.current_anim) {
                if anim.is_finished() {
                    self.current_anim = "idle".to_string();
                }
            }
        }
    }

    fn draw_animation(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
        if let Some(anim) = self.animations.get(&self.current_anim) {
            anim.draw(ctx, x, y, self.width, self.height);
        }
    }
}
