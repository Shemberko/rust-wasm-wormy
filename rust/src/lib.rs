mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{
    window, HtmlCanvasElement, CanvasRenderingContext2d,
    WebSocket, MessageEvent, Event, ErrorEvent,
};
use std::cell::RefCell;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

thread_local! {
    static X: RefCell<f64> = RefCell::new(100.0);
    static Y: RefCell<f64> = RefCell::new(50.0);
    static VELOCITY_Y: RefCell<f64> = RefCell::new(0.0);
}

const GROUND_LEVEL: f64 = 390.0;
const PLATFORM_X: f64 = 300.0;
const PLATFORM_WIDTH: f64 = 150.0;
const PLATFORM_HEIGHT: f64 = 20.0;

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

    let (x, y) = (
        X.with(|x| *x.borrow()),
        Y.with(|y| *y.borrow()),
    );

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

            // Межі гравця після руху вліво
            let player_left = new_x;
            let player_bottom = y_pos + PLAYER_HEIGHT;

            // Межі платформи
            let window = window().unwrap();
            let document = window.document().unwrap();
            let canvas = document
                .get_element_by_id("mycanvas").unwrap()
                .dyn_into::<HtmlCanvasElement>().unwrap();
            let canvas_height = canvas.height() as f64;

            let platform_top = canvas_height - PLATFORM_HEIGHT - 10.0;
            let platform_bottom = platform_top + PLATFORM_HEIGHT;

            // Чи гравець вертикально на рівні платформи?
            let on_platform_vertically = player_bottom > platform_top && y_pos < platform_bottom;

            // Якщо гравець не заходить всередину платформи зліва — рух дозволений
            let will_collide = player_left < PLATFORM_X + PLATFORM_WIDTH &&
                               player_left > PLATFORM_X &&
                               on_platform_vertically;

            if !will_collide {
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

            // Межі гравця після руху вправо
            let player_right = new_x + PLAYER_WIDTH;
            let player_bottom = y_pos + PLAYER_HEIGHT;

            let window = window().unwrap();
            let document = window.document().unwrap();
            let canvas = document
                .get_element_by_id("mycanvas").unwrap()
                .dyn_into::<HtmlCanvasElement>().unwrap();
            let canvas_height = canvas.height() as f64;

            let platform_top = canvas_height - PLATFORM_HEIGHT - 10.0;
            let platform_bottom = platform_top + PLATFORM_HEIGHT;

            let on_platform_vertically = player_bottom > platform_top && y_pos < platform_bottom;

            // Якщо гравець не заходить всередину платформи справа — рух дозволений
            let will_collide = player_right > PLATFORM_X &&
                               player_right < PLATFORM_X + PLATFORM_WIDTH &&
                               on_platform_vertically;

            if !will_collide {
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
                let canvas = window().unwrap()
                    .document().unwrap()
                    .get_element_by_id("mycanvas").unwrap()
                    .dyn_into::<HtmlCanvasElement>().unwrap();
                let canvas_height = canvas.height() as f64;

                let ground_level = canvas_height - PLAYER_HEIGHT;
                let platform_y = canvas_height - PLATFORM_HEIGHT - 10.0;

                let x_pos = *x.borrow();
                let y_pos = *y.borrow();

                let is_on_ground = y_pos >= ground_level;
                let is_on_platform = 
                    y_pos + PLAYER_HEIGHT >= platform_y &&
                    y_pos + PLAYER_HEIGHT <= platform_y + PLATFORM_HEIGHT &&
                    x_pos + PLAYER_WIDTH >= PLATFORM_X &&
                    x_pos <= PLATFORM_X + PLATFORM_WIDTH;

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
    let canvas = document.get_element_by_id("mycanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;

    let canvas_height = canvas.height() as f64;
    let ground_level = canvas_height - PLAYER_HEIGHT;
    let platform_y = canvas_height - PLATFORM_HEIGHT - 10.0;

    let (x, y) = (
        X.with(|x| *x.borrow()),
        Y.with(|y| *y.borrow()),
    );

    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    
    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill_rect(x, y, PLAYER_WIDTH, PLAYER_HEIGHT);

    ctx.set_fill_style(&JsValue::from_str("green"));
    ctx.fill_rect(PLATFORM_X, platform_y, PLATFORM_WIDTH, PLATFORM_HEIGHT);

    Ok(())
}

#[wasm_bindgen]
pub fn apply_physics() -> Result<(), JsValue> {
    const GRAVITY: f64 = 0.5; // прискорення падіння

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("mycanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let canvas_height = canvas.height() as f64;
    let ground_level = canvas_height - PLAYER_HEIGHT;
    let platform_y = canvas_height - PLATFORM_HEIGHT - 10.0;

    VELOCITY_Y.with(|v| {
        Y.with(|y| {
            X.with(|x| {
                let mut vy = *v.borrow();
                let mut y_pos = *y.borrow();
                let x_pos = *x.borrow();

                vy += GRAVITY;
                y_pos += vy;

                let is_on_platform = 
                    y_pos + PLAYER_HEIGHT >= platform_y &&
                    y_pos + PLAYER_HEIGHT <= platform_y + PLATFORM_HEIGHT &&
                    x_pos + PLAYER_WIDTH >= PLATFORM_X &&
                    x_pos <= PLATFORM_X + PLATFORM_WIDTH &&
                    vy >= 0.0; // падає вниз

                if is_on_platform {
                    y_pos = platform_y - PLAYER_HEIGHT;
                    vy = 0.0;
                } else if y_pos >= ground_level {
                    y_pos = ground_level;
                    vy = 0.0;
                }

                *v.borrow_mut() = vy;
                *y.borrow_mut() = y_pos;
            });
        });
    });

    Ok(())
}