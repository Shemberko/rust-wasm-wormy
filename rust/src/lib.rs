mod map;
mod utils;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    window, CanvasRenderingContext2d, ErrorEvent, Event, HtmlCanvasElement, MessageEvent, WebSocket,
};

use crate::map::Map;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

thread_local! {
    static X: RefCell<f64> = RefCell::new(100.0);
    static Y: RefCell<f64> = RefCell::new(50.0);
    static VELOCITY_Y: RefCell<f64> = RefCell::new(0.0);
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub velocity_y: f64,
    pub width: f64,
    pub height: f64,
}

const PLAYER_WIDTH: f64 = 50.0;
const PLAYER_HEIGHT: f64 = 100.0;

#[wasm_bindgen]
pub fn play() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("mycanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // 🎨 Малюємо трикутник
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    let (x, y) = (X.with(|x| *x.borrow()), Y.with(|y| *y.borrow()));

    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill_rect(x, y, PLAYER_WIDTH, PLAYER_HEIGHT);

    // 🌐 Підключення WebSocket
    let ws = WebSocket::new("ws://127.0.0.1:3000/ws")?; // заміни IP, якщо потрібно

    // 📩 Обробка вхідних повідомлень
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            web_sys::console::log_1(&format!("📨 Повідомлення від сервера: {}", txt).into());
        }
    }) as Box<dyn FnMut(_)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget(); // не викидаємо callback

    // ⚠️ Обробка помилок
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        web_sys::console::log_1(&format!("❌ WebSocket помилка: {:?}", e.message()).into());
    }) as Box<dyn FnMut(_)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    // 🔗 Обробка відкриття з'єднання
    let onopen_callback = Closure::wrap(Box::new(move |_: Event| {
        web_sys::console::log_1(&"✅ WebSocket з'єднано!".into());
    }) as Box<dyn FnMut(_)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn move_left() {
    X.with(|x| {
        Y.with(|y| {
            let mut x_pos = *x.borrow();
            let y_pos = *y.borrow();

            let new_x = x_pos - 5.0;

            let window = window().unwrap();
            let document = window.document().unwrap();
            let canvas = document
                .get_element_by_id("mycanvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            let canvas_width = canvas.width() as f64;
            let canvas_height = canvas.height() as f64;

            let map = Map::new(canvas_width, canvas_height);

            // Якщо рух вліво не призведе до колізій, оновлюємо позицію
            if map.can_move_to(new_x, y_pos, PLAYER_WIDTH, PLAYER_HEIGHT) {
                x_pos = new_x;
            }

            *x.borrow_mut() = x_pos;
        });
    });
}

#[wasm_bindgen]
pub fn move_right() {
    X.with(|x| {
        Y.with(|y| {
            let mut x_pos = *x.borrow();
            let y_pos = *y.borrow();

            let new_x = x_pos + 5.0;

            let window = window().unwrap();
            let document = window.document().unwrap();
            let canvas = document
                .get_element_by_id("mycanvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            let canvas_width = canvas.width() as f64;
            let canvas_height = canvas.height() as f64;

            let map = Map::new(canvas_width, canvas_height);

            if map.can_move_to(new_x, y_pos, PLAYER_WIDTH, PLAYER_HEIGHT) {
                x_pos = new_x;
            }

            *x.borrow_mut() = x_pos;
        });
    });
}

#[wasm_bindgen]
pub fn jump() -> Result<(), JsValue> {
    VELOCITY_Y.with(|v| {
        Y.with(|y| {
            X.with(|x| {
                let canvas = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("mycanvas")
                    .unwrap()
                    .dyn_into::<HtmlCanvasElement>()
                    .unwrap();
                let canvas_height = canvas.height() as f64;
                let canvas_width = canvas.width() as f64;

                let map = Map::new(canvas_width, canvas_height);

                let x_pos = *x.borrow();
                let y_pos = *y.borrow();

                let is_on_ground = y_pos + PLAYER_HEIGHT >= canvas_height;
                let is_on_platform = is_player_on_platform(&map, x_pos, y_pos);

                if is_on_ground || is_on_platform {
                    *v.borrow_mut() = -10.0;
                }
            });
        });
    });

    Ok(())
}

#[wasm_bindgen]
pub fn move_down() {
    Y.with(|y| {
        *y.borrow_mut() += 5.0;
    });
}

#[wasm_bindgen]
pub fn draw() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("mycanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let (x, y) = (X.with(|x| *x.borrow()), Y.with(|y| *y.borrow()));

    let canvas_width = canvas.width() as f64;
    let canvas_height = canvas.height() as f64;

    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);

    let map = Map::new(canvas_width, canvas_height);
    map.draw(&ctx);

    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill_rect(x, y, PLAYER_WIDTH, PLAYER_HEIGHT);

    Ok(())
}

#[wasm_bindgen]
pub fn apply_physics() -> Result<(), JsValue> {
    const GRAVITY: f64 = 0.5;

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("mycanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let canvas_width = canvas.width() as f64;
    let canvas_height = canvas.height() as f64;

    let map = Map::new(canvas_width, canvas_height);

    VELOCITY_Y.with(|v| {
        Y.with(|y| {
            X.with(|x| {
                let mut vy = *v.borrow();
                let mut y_pos = *y.borrow();
                let x_pos = *x.borrow();

                vy += GRAVITY;
                y_pos += vy;

                if vy >= 0.0 && is_player_on_platform(&map, x_pos, y_pos) {
                    // Колізія знизу (платформа або земля)
                    let feet_y = y_pos + PLAYER_HEIGHT + 1.0;
                    let tile_row = (feet_y / map.tile_size).floor();
                    y_pos = tile_row * map.tile_size - PLAYER_HEIGHT;
                    vy = 0.0;
                } else if vy < 0.0 {
                    // Перевірка колізії зверху
                    // Перевіримо 3 точки зверху: ліву, центр, праву
                    let head_y = y_pos;
                    let check_points = [
                        x_pos + 1.0,
                        x_pos + PLAYER_WIDTH / 2.0,
                        x_pos + PLAYER_WIDTH - 1.0,
                    ];
                    let mut hit_ceiling = false;
                    for &px in &check_points {
                        if map.is_solid_at(px, head_y) {
                            hit_ceiling = true;
                            break;
                        }
                    }
                    if hit_ceiling {
                        // Встановлюємо гравця безпосередньо під платформою
                        let tile_row = (head_y / map.tile_size).floor();
                        y_pos = (tile_row + 1.0) * map.tile_size;
                        vy = 0.0;
                    }
                }

                if y_pos + PLAYER_HEIGHT >= canvas_height {
                    y_pos = canvas_height - PLAYER_HEIGHT;
                    vy = 0.0;
                }

                *v.borrow_mut() = vy;
                *y.borrow_mut() = y_pos;
            });
        });
    });

    Ok(())
}

fn is_player_on_platform(map: &Map, player_x: f64, player_y: f64) -> bool {
    let feet_y = player_y + PLAYER_HEIGHT + 1.0; // 1 піксель під гравцем
                                                 // Перевіряємо кілька точок під ногами гравця (ліва, центр, права)
    let check_points = [
        player_x,                      // ліва нога
        player_x + PLAYER_WIDTH / 2.0, // центр
        player_x + PLAYER_WIDTH - 1.0, // права нога
    ];

    for &x in &check_points {
        if map.is_solid_at(x, feet_y) {
            return true;
        }
    }
    false
}
