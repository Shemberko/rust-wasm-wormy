mod utils;


use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn play() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("mycanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    ctx.begin_path();
    ctx.move_to(100.0, 50.0);    // Перша вершина
    ctx.line_to(300.0, 150.0);   // Друга вершина
    ctx.line_to(100.0, 250.0);   // Третя вершина
    ctx.close_path();

    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill();

    Ok(())
}
