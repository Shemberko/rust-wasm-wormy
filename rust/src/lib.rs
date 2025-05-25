mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{
    window, HtmlCanvasElement, CanvasRenderingContext2d,
    WebSocket, MessageEvent, Event, ErrorEvent,
};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

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
    ctx.begin_path();
    ctx.move_to(100.0, 50.0);
    ctx.line_to(300.0, 150.0);
    ctx.line_to(100.0, 250.0);
    ctx.close_path();
    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill();

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
