use std::rc::Rc;

use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::models::player::Player;

pub struct Map {
    pub canvas: Rc<CanvasRenderingContext2d>,

    pub canvas_width: u32,
    pub canvas_height: u32,

    pub image_data: ImageData,

    pub camera_x: u32,
    pub camera_y: u32,
}

impl Map {
    pub fn new(
        canvas_width: u32,
        canvas_height: u32,
        canvas: Rc<CanvasRenderingContext2d>,
        data: ImageData,
    ) -> Self {
        Self {
            canvas,
            canvas_width,
            canvas_height,
            image_data: data,
            camera_x: 0,
            camera_y: 0,
        }
    }

    pub fn draw(&self) {
        if let Some(ref img_data) = self.cropped_visible_image() {
            let _ = self.canvas.put_image_data(img_data, 0.0, 0.0);
        }
    }

    /// Crops the image data to the visible area and returns a new ImageData
    pub fn cropped_visible_image(&self) -> Option<ImageData> {
        let image_width = self.image_data.width();
        let image_height = self.image_data.height();
        let data = self.image_data.data();
        let data_len = data.len();

        let mut left = self.camera_x as i32;
        let mut top = self.camera_y as i32;

        left = left.max(0);
        top = top.max(0);

        let right = (left + self.canvas_width as i32).min(image_width as i32);
        let bottom = (top + self.canvas_height as i32).min(image_height as i32);

        let mut cropped_pixels =
            Vec::with_capacity((self.canvas_width * self.canvas_height * 4) as usize);

        for y in top..bottom {
            for x in left..right {
                let idx = ((y as u32 * image_width + x as u32) * 4) as usize;
                if idx + 4 <= data_len {
                    cropped_pixels.extend_from_slice(&data[idx..idx + 4]);
                } else {
                    // Заповнити прозорим, якщо поза межами
                    cropped_pixels.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }

        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&cropped_pixels),
            self.canvas_width,
            self.canvas_height,
        )
        .ok()
    }

    pub fn update_camera(&mut self, player: &Player) {
        // let center_x = (self.canvas_width / 2) as f64;
        // let mut camera_x = player.position.x + player.width / 2.0 - center_x;

        // if camera_x < 0.0 {
        //     camera_x = 0.0;
        // }

        // let max_camera_x = (self.width as f64) - (self.canvas_width as f64);
        // if camera_x > max_camera_x {
        //     camera_x = max_camera_x;
        // }

        // self.camera_x = camera_x as u32;
        // let center_y = (self.canvas_height / 2) as f64;
        // let mut camera_y = player.position.y + player.height / 2.0 - center_y;

        // if camera_y < 0.0 {
        //     camera_y = 0.0;
        // }

        // let max_camera_y = (self.height as f64) - (self.canvas_height as f64);
        // if camera_y > max_camera_y {
        //     camera_y = max_camera_y;
        // }

        // self.camera_x = camera_x as u32;
        // self.camera_y = camera_y as u32;

        self.camera_x = 0 as u32;
        self.camera_y = 500 as u32;
    }

    pub fn is_solid_at(&self, x: f64, y: f64) -> bool {
        let px = x.floor() as i32;
        let py = y.floor() as i32;

        if px < 0
            || py < 0
            || px >= self.image_data.width() as i32
            || py >= self.image_data.height() as i32
        {
            return false;
        }

        let idx = ((py as u32 * self.image_data.width() + px as u32) * 4 + 3) as usize;
        let data = self.image_data.data();

        let result = data.get(idx).map_or(false, |&alpha| alpha > 0);

        result
    }
}
