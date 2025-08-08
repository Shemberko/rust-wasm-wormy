use js_sys::Promise;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, CanvasRenderingContext2d, HtmlImageElement};

use crate::models::{map::Map, position::Position, traits::CanvasObject};

pub struct Bullet {
    pub position: Position,
    pub speed: f64,
    pub velocity_y: f64,
    pub facing_left: bool,
    pub width: f64,
    pub height: f64,
    pub image: HtmlImageElement,
    pub is_active: bool,
}

impl CanvasObject for Bullet {
    fn draw(&self, ctx: &CanvasRenderingContext2d, map: &Map) {
        ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &self.image,
            self.position.x,
            self.position.y,
            self.width,
            self.height,
        )
        .unwrap();
    }

    fn update(&mut self, _delta_time: f64, map: &Map, canvas_height: f64) {
        if self.facing_left {
            self.position.x -= self.speed;
        } else {
            self.position.x += self.speed;
        }

        self.velocity_y += 0.05;
        let new_y = self.position.y + self.velocity_y;

        if new_y + self.height >= canvas_height {
            self.position.y = canvas_height - self.height;
            self.velocity_y = 0.0;
            self.is_active = false;
        } else if self.collides_vertically(new_y, map) {
            self.velocity_y = 0.0;
            self.is_active = false;
        } else {
            self.position.y = new_y;
        }

        if self.collides_with_map(map) {
            self.is_active = false;
        }
    }
}

impl Bullet {
    pub fn new(x: f64, y: f64, facing_left: bool, image: HtmlImageElement) -> Self {
        Self {
            position: Position { x, y },
            speed: 20.0,
            velocity_y: 0.0,
            facing_left,
            width: 18.0,
            height: 10.0,
            image,
            is_active: true,
        }
    }

    pub async fn load_bullet_image() -> Result<HtmlImageElement, JsValue> {
        let document = window().unwrap().document().unwrap();
        let img = document
            .create_element("img")?
            .dyn_into::<HtmlImageElement>()?;

        let promise = Promise::new(&mut |resolve, reject| {
            let onload = Closure::once_into_js(move || {
                let _ = resolve.call0(&JsValue::NULL);
            });

            let onerror = Closure::once_into_js(move || {
                let _ = reject.call1(&JsValue::NULL, &"Failed to load bullet image".into());
            });

            img.set_onload(Some(onload.unchecked_ref()));
            img.set_onerror(Some(onerror.unchecked_ref()));
        });

        img.set_src("/assets/Guns_V1.01 - Commission - Copy/01 - Individual sprites/Bullets & Ammo/AK 47/Casing & Bullet.png");
        JsFuture::from(promise).await?;
        Ok(img)
    }

    fn collides_with_map(&self, map: &Map) -> bool {
        let left = self.position.x;
        let right = self.position.x + self.width;
        let top = self.position.y;
        let bottom = self.position.y + self.height;

        let mut x = left;
        while x <= right {
            let mut y = top;
            while y <= bottom {
                if map.is_solid_at(x, y) {
                    return true;
                }
                y += 4.0;
            }
            x += 4.0;
        }
        false
    }
    fn collides_vertically(&self, new_y: f64, map: &Map) -> bool {
        let left = self.position.x;
        let right = self.position.x + self.width;
        let top = new_y;
        let bottom = new_y + self.height;

        let mut x = left;
        while x <= right {
            let mut y = top;
            while y <= bottom {
                if map.is_solid_at(x, y) {
                    return true;
                }
                y += 4.0;
            }
            x += 4.0;
        }
        false
    }
}
