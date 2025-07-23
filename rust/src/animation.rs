use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub struct Animation {
    image: HtmlImageElement,
    frame_width: f64,
    frame_height: f64,
    frame_counts: Vec<u32>, // ← наприклад [6, 6, 3] для 3 рядків
    current_frame: u32,
    animation_row: usize,
    timer: f64,
    frame_duration: f64,
}

impl Animation {
    pub fn new(
        image: HtmlImageElement,
        frame_width: f64,
        frame_height: f64,
        frame_counts: Vec<u32>,
        frame_duration: f64,
        animation_row: usize,
    ) -> Self {
        Self {
            image,
            frame_width,
            frame_height,
            frame_counts,
            current_frame: 0,
            animation_row,
            timer: 0.0,
            frame_duration,
        }
    }

    pub fn update(
        &mut self,
        delta_time: f64,
        is_moving: bool,
        is_on_ground: bool,
        velocity_y: f64,
    ) {
        if !is_on_ground {
            // Якщо у повітрі: 1 кадр для стрибка, 2 кадр для падіння
            if velocity_y < 0.0 {
                self.current_frame = 1;
            } else {
                self.current_frame = 2;
            }
            self.timer = 0.0;
            return;
        }

        self.timer += delta_time;
        if self.timer >= self.frame_duration {
            self.timer = 0.0;
            let row_frame_count = self.frame_counts[self.animation_row];
            self.current_frame = (self.current_frame + 1) % row_frame_count;
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64) {
        let sx = self.current_frame as f64 * self.frame_width;
        let sy = self.animation_row as f64 * self.frame_height;
        ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &self.image,
            sx,
            sy,
            self.frame_width,
            self.frame_height,
            x,
            y,
            width,
            height,
        )
        .unwrap();
    }

    pub fn set_animation_row(&mut self, row: usize) {
        if self.animation_row != row {
            self.animation_row = row;
            self.current_frame = 0;
            self.timer = 0.0;
        }
    }
}
