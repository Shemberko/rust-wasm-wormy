use std::rc::Rc;

use wasm_bindgen::{Clamped, JsValue};
use web_sys::{CanvasRenderingContext2d, ImageData};

pub struct Map {
    pub canvas: Rc<CanvasRenderingContext2d>,

    pub canvas_width: u32,
    pub canvas_height: u32,

    pub image_data: ImageData,
    pub width: u32,
    pub height: u32,

    pub camera_x: u32,
    pub camera_y: u32,
}

impl Map {
    pub fn new(
        canvas_width: u32,
        canvas_height: u32,
        canvas: Rc<CanvasRenderingContext2d>,
        map_width: u32,
        map_height: u32,
    ) -> Self {
        let mut pixels = vec![0u8; (map_width * map_height * 4) as usize];

        // Генеруємо землю знизу (все, що нижче половини карти — суцільний шар)
        for y in 0..map_height {
            for x in 0..map_width {
                let idx = ((y * map_width + x) * 4) as usize;
                if y > map_height / 2 {
                    // коричнева земля
                    pixels[idx] = 139; // R
                    pixels[idx + 1] = 69; // G
                    pixels[idx + 2] = 19; // B
                    pixels[idx + 3] = 255; // A (непрозора)
                } else {
                    // повітря (прозоре)
                    pixels[idx + 3] = 0;
                }
            }
        }

        // Створюємо ImageData з пікселів
        let image_data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pixels), map_width, map_height)
                .expect("failed to create ImageData");

        Self {
            canvas,
            canvas_width,
            canvas_height,
            image_data,
            width: map_width,
            height: map_height,
            camera_x: 0,
            camera_y: 0,
        }
    }

    pub fn draw(&self) {
        let _ = self.canvas.put_image_data(
            &self.image_data,
            0.0 - self.camera_x as f64,
            0.0 - self.camera_y as f64,
        );
    }

    pub fn is_solid_at(&self, x: f64, y: f64) -> bool {
        let px = x.floor() as i32;
        let py = y.floor() as i32;

        if px < 0 || py < 0 || px >= self.width as i32 || py >= self.height as i32 {
            return false;
        }

        let idx = ((py as u32 * self.width + px as u32) * 4 + 3) as usize;
        let data = self.image_data.data();

        let result = data.get(idx).map_or(false, |&alpha| alpha > 0);

        result
    }

    pub fn can_move_to(&self, x: f64, y: f64, width: f64, height: f64) -> bool {
        let left = x.floor() as i32;
        let right = (x + width - 0.01).ceil() as i32;
        let top = y.floor() as i32;
        let bottom = (y + height - 0.01).ceil() as i32;

        for px in left..right {
            for py in top..bottom {
                if self.is_solid_at(px as f64, py as f64) {
                    return false;
                }
            }
        }
        true
    }
}
