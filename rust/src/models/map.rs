use std::rc::Rc;

use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::models::player::Player;

pub struct Map {
    pub canvas: Rc<CanvasRenderingContext2d>,

    pub canvas_width: u32,
    pub canvas_height: u32,

    pub image_data: ImageData,
    pub pixels: Vec<u8>,
    pub collision_map: Vec<bool>, // Нова кешована карта

    pub camera_x: u32,
    pub camera_y: u32,
}

impl Map {
    pub fn new(
        canvas_width: u32,
        canvas_height: u32,
        canvas: Rc<CanvasRenderingContext2d>,
        image_data: ImageData,
    ) -> Self {
        let width = image_data.width();
        let height = image_data.height();
        let data = image_data.data();

        // Заповнюємо кеш
        let mut collision_map = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4 + 3) as usize; // Альфа-канал
                collision_map.push(data[idx] > 0);
            }
        }

        let pixels: Vec<u8> = data.to_vec();

        Self {
            canvas,
            canvas_width,
            canvas_height,
            image_data: image_data,
            pixels: pixels,
            collision_map,
            camera_x: 0,
            camera_y: 0,
        }
    }

    pub fn draw(&self) {
        if let Some(ref img_data) = self.cropped_visible_image() {
            let _ = self.canvas.put_image_data(img_data, 0.0, 0.0);
        }
    }

    pub fn destroy_circle(&mut self, center_x: f64, center_y: f64, radius: f64) {
        let width = self.image_data.width() as usize;
        let height = self.image_data.height() as usize;
        let r2 = (radius * radius) as i32;

        let cx = center_x as i32;
        let cy = center_y as i32;

        for y in (cy - radius as i32).max(0)..(cy + radius as i32).min(height as i32) {
            for x in (cx - radius as i32).max(0)..(cx + radius as i32).min(width as i32) {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy <= r2 {
                    let idx = ((y as usize) * width + (x as usize)) * 4;
                    self.pixels[idx + 3] = 0; // альфа = 0
                    self.collision_map[y as usize * width + x as usize] = false;
                }
            }
        }

        self.image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.pixels),
            width as u32,
            height as u32,
        )
        .unwrap();
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
        let px = x as i32;
        let py = y as i32;

        if px < 0
            || py < 0
            || px >= self.image_data.width() as i32
            || py >= self.image_data.height() as i32
        {
            return false;
        }

        self.collision_map[py as usize * self.image_data.width() as usize + px as usize]
    }
}
