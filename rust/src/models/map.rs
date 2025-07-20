use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Map {
    pub tile_size: f64,
    pub data: Vec<Vec<u8>>,
    pub canvas: Rc<CanvasRenderingContext2d>,
}

impl Map {
    pub fn new(
        canvas_width: f64,
        canvas_height: f64,
        canvas: Rc<CanvasRenderingContext2d>,
    ) -> Self {
        let tile_size = 5.0; // замість 10.0
        let cols = (canvas_width / tile_size).floor() as usize;
        let rows = (canvas_height / tile_size).floor() as usize;

        let mut data = vec![vec![0; cols]; rows];

        // Нижні горбочки (синусоїда) — параметри теж можна підкоригувати, щоб виглядало плавно
        for c in 0..cols {
            let wave_height = ((c as f64 / 10.0).sin() * 6.0).round() as isize; // більша амплітуда, довша хвиля
            let base_row = rows as isize - 1;
            let platform_rows = 5 + wave_height; // більше "платформ" по висоті
            for r in (base_row - platform_rows + 1)..=base_row {
                if r >= 0 && (r as usize) < rows {
                    data[r as usize][c] = 1;
                }
            }
        }
        let mountain_base_col = 100;
        let mountain_base_row = rows - 1;
        let mountain_height = 17;
        let mountain_width = 30;

        for step in 0..mountain_height {
            let row = mountain_base_row - step;
            let col_start = mountain_base_col + step / 2;
            let col_end = col_start + mountain_width - step;
            if row >= 0 {
                for c in col_start..col_end.min(cols) {
                    data[row][c] = 1;
                }
            }
        }

        let stair_start_col = 50;
        let stair_start_row = rows - 20;
        let stair_height = 40;
        for step in 0..stair_height {
            let row = stair_start_row - step;
            let col_start = stair_start_col + step * 4;
            for c in col_start..(col_start + 10) {
                if row < rows && c < cols {
                    data[row][c] = 1;
                }
            }
        }

        let mid_row = stair_start_row - stair_height - 5;
        let center_start_col = stair_start_col + stair_height * 4;
        for c in center_start_col..(center_start_col + 30) {
            if c < cols {
                data[mid_row][c] = 1;
            }
        }

        let side_row = mid_row - 40;
        let left_start = stair_start_col + 10;
        let right_start = center_start_col + 40;

        for c in left_start..(left_start + 20) {
            if side_row < rows && c < cols {
                data[side_row][c] = 1;
            }
        }

        for c in right_start..(right_start + 20) {
            if side_row < rows && c < cols {
                data[side_row][c] = 1;
            }
        }

        // Маленькі платформи
        let jump_row = mid_row - 10;
        let p_start = center_start_col + 40;

        for c in p_start..(p_start + 12) {
            if jump_row < rows && c < cols {
                data[jump_row][c] = 1;
            }
        }

        Self {
            tile_size,
            data,
            canvas,
        }
    }

    pub fn draw(&self) {
        self.canvas.set_fill_style(&JsValue::from_str("green"));
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, &tile) in row.iter().enumerate() {
                if tile == 1 {
                    let x = col_idx as f64 * self.tile_size;
                    let y = row_idx as f64 * self.tile_size;
                    self.canvas.fill_rect(x, y, self.tile_size, self.tile_size);
                }
            }
        }
    }

    pub fn is_solid_at(&self, x: f64, y: f64) -> bool {
        let col = (x / self.tile_size).floor() as usize;
        let row = (y / self.tile_size).floor() as usize;
        self.data.get(row).and_then(|r| r.get(col)).copied() == Some(1)
    }

    pub fn can_move_to(&self, x: f64, y: f64, width: f64, height: f64) -> bool {
        // Перевіряємо всі тайли, що перекриваються прямокутником
        let left_tile = (x / self.tile_size).floor() as isize;
        let right_tile = ((x + width - 1.0) / self.tile_size).floor() as isize;
        let top_tile = (y / self.tile_size).floor() as isize;
        let bottom_tile = ((y + height - 1.0) / self.tile_size).floor() as isize;

        for row in top_tile..=bottom_tile {
            for col in left_tile..=right_tile {
                if self.is_solid_tile(col, row) {
                    return false;
                }
            }
        }
        true
    }

    fn is_solid_tile(&self, col: isize, row: isize) -> bool {
        if row < 0 || col < 0 {
            return false;
        }
        if let Some(row_vec) = self.data.get(row as usize) {
            if let Some(&tile) = row_vec.get(col as usize) {
                return tile == 1;
            }
        }
        false
    }
}
