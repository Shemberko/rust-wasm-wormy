mod animation;

mod models;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use crate::models::map::Map;
use crate::models::player::create_player;

use js_sys::Array;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    window, CanvasRenderingContext2d, ErrorEvent, Event, HtmlCanvasElement, ImageData,
    MessageEvent, WebSocket,
};

use crate::models::game::Game;
use crate::models::player;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

thread_local! {
    static GAME: RefCell<Option<Game>> = RefCell::new(None);
}

#[wasm_bindgen]
pub async fn play() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("mycanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let canvas_width = canvas.width() as f64;
    let canvas_height = canvas.height() as f64;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    ctx.set_image_smoothing_enabled(false);

    GAME.with(|game| {
        *game.borrow_mut() = Some(Game::new(
            canvas_width as u32,
            canvas_height as u32,
            Rc::new(ctx),
        ));
    });

    let player = create_player().await?;

    GAME.with(|game| {
        if let Some(g) = &mut *game.borrow_mut() {
            g.add_player(player);
            g.draw();
        }
    });

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

    let canvas_width = canvas.width() as f64;
    let canvas_height = canvas.height() as f64;

    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);

    Ok(())
}

#[wasm_bindgen]
pub fn update(pressed_keys: Array) -> Result<(), JsValue> {
    GAME.with(|game| {
        if let Some(g) = &mut *game.borrow_mut() {
            if let Some(player) = g.get_current_player_mut() {
                let mut keys_set = HashSet::new();
                for key in pressed_keys.iter() {
                    if let Some(s) = key.as_string() {
                        keys_set.insert(s);
                    }
                }
                player.set_pressed_keys(keys_set);
            }
            g.update();
            g.draw();
        }
    });

    Ok(())
}

#[wasm_bindgen]
pub async fn init_player() -> Result<(), JsValue> {
    let player = crate::player::create_player().await?;

    GAME.with(|game| {
        if let Some(ref mut g) = *game.borrow_mut() {
            g.add_player(player);
        }
    });
    Ok(())
}

#[wasm_bindgen]
pub fn resize(width: f64, height: f64) -> Result<(), JsValue> {
    GAME.with(|game| {
        if let Some(g) = &mut *game.borrow_mut() {
            g.canvas_width = width as u32;
            g.canvas_height = height as u32;
        }
    });
    Ok(())
}

#[wasm_bindgen]
pub fn set_image_data(data: ImageData) {
    GAME.with(|game| {
        if let Some(g) = &mut *game.borrow_mut() {
            g.map.image_data = data;
        }
    });
}
