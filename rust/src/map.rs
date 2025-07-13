use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Map {
    pub tile_size: f64,
    pub data: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
        let tile_size = 10.0;
        let cols = (canvas_width / tile_size).floor() as usize;
        let rows = (canvas_height / tile_size).floor() as usize;

        let mut data = vec![vec![0; cols]; rows];

        // 1. Нижні горбочки (синусоїда)
        for c in 0..cols {
            let wave_height = ((c as f64 / 5.0).sin() * 2.0).round() as isize;
            let base_row = rows as isize - 1;
            let platform_rows = 2 + wave_height;
            for r in (base_row - platform_rows + 1)..=base_row {
                if r >= 0 && (r as usize) < rows {
                    data[r as usize][c] = 1;
                }
            }
        }

        let stair_start_col = 15;
        let stair_start_row = rows - 5; // трохи над горбками
        let stair_height = 10;
        for step in 0..stair_height {
            let row = stair_start_row - step;
            let col_start = stair_start_col + step * 2;
            for c in col_start..(col_start + 5) {
                if row < rows {
                    data[row][c] = 1;
                }
            }
        }

        let mid_row = stair_start_row - stair_height - 2;
        let center_start_col = stair_start_col + stair_height * 2;
        for c in center_start_col..(center_start_col + 15) {
            data[mid_row][c] = 1;
        }

        // 4. Додаткові повітряні платформи зліва і справа
        let side_row = mid_row - 16; // трохи вище
        let left_start = stair_start_col + 3;
        let right_start = center_start_col + 20;

        for c in left_start..(left_start + 10) {
            data[side_row][c] = 1;
        }

        for c in right_start..(right_start + 10) {
            data[side_row][c] = 1;
        }

        // 5. Маленькі "перепригункові" платформи
        let jump_row = mid_row - 5;
        let p_start = center_start_col + 18;

        for c in p_start..(p_start + 6) {
            data[jump_row][c] = 1;
        }

        Self { tile_size, data }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from_str("green"));
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, &tile) in row.iter().enumerate() {
                if tile == 1 {
                    let x = col_idx as f64 * self.tile_size;
                    let y = row_idx as f64 * self.tile_size;
                    ctx.fill_rect(x, y, self.tile_size, self.tile_size);
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
