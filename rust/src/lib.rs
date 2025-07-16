mod animation;

mod utils;
mod models;

use std::cell::RefCell;
use std::rc::Rc;

use crate::models::map::Map;
use crate::models::player::Player;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    window, CanvasRenderingContext2d, ErrorEvent, Event, HtmlCanvasElement, MessageEvent, WebSocket,
};

use crate::models::map;
use crate::models::game::Game;
use crate::models::player;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// thread_local! {
//     static PLAYER: RefCell<Player> = RefCell::new(Player::new());
// }

thread_local! {

static GAME: RefCell<Option<Game>> = RefCell::new(None);
static KEYS_PRESSED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}
#[wasm_bindgen]
pub fn play() -> Result<(), JsValue> {
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

    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    GAME.with(|game| {
        *game.borrow_mut() = Some(Game::new(canvas_width, canvas_height, Rc::new(ctx)));
    });


    let img = document
        .create_element("img")?
        .dyn_into::<web_sys::HtmlImageElement>()?;
    img.set_src("/animations/NuclearLeak_CharacterAnim_1.2/character_20x20_red.png");

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
pub fn apply_physics() -> Result<(), JsValue> {
    // TODO make this global
    const GRAVITY: f64 = 0.5;

    // TODO  this must be in game and player
    // PLAYER.with(|p| {
    //     if let Some(player) = p.borrow_mut().as_mut() {
    //         let is_on_ground =
    //             player.is_on_ground(&map) || player.y + player.height >= canvas_height;
    //         let is_moving = KEYS_PRESSED.with(|keys| {
    //             let keys = keys.borrow();
    //             keys.contains("ArrowLeft")
    //                 || keys.contains("ArrowRight")
    //                 || keys.contains("KeyA")
    //                 || keys.contains("KeyD")
    //         });

    //         player.update_animation_state(is_moving, is_on_ground);
    //         player.apply_physics(&map, canvas_height);
    //         player.update(0.016);
    //     }
    // });

    Ok(())
}


// TODO make as one function
// #[wasm_bindgen]
// pub fn move_left() {
//     let window = window().unwrap();
//     let document = window.document().unwrap();
//     let canvas = document
//         .get_element_by_id("mycanvas")
//         .unwrap()
//         .dyn_into::<HtmlCanvasElement>()
//         .unwrap();

//     let map = Map::new(canvas.width() as f64, canvas.height() as f64);

//     PLAYER.with(|p| {
//         if let Some(player) = p.borrow_mut().as_mut() {
//             player.move_left(&map);
//         }
//     });
// }

// #[wasm_bindgen]
// pub fn move_right() {
//     let window = window().unwrap();
//     let document = window.document().unwrap();
//     let canvas = document
//         .get_element_by_id("mycanvas")
//         .unwrap()
//         .dyn_into::<HtmlCanvasElement>()
//         .unwrap();

//     let map = Map::new(canvas.width() as f64, canvas.height() as f64);

//     PLAYER.with(|p| {
//         if let Some(player) = p.borrow_mut().as_mut() {
//             player.move_right(&map);
//         }
//     });
// }

// #[wasm_bindgen]
// pub fn jump() {
//     let window = window().unwrap();
//     let document = window.document().unwrap();
//     let canvas = document
//         .get_element_by_id("mycanvas")
//         .unwrap()
//         .dyn_into::<HtmlCanvasElement>()
//         .unwrap();

//     let canvas_height = canvas.height() as f64;
//     let canvas_width = canvas.width() as f64;
//     let map = Map::new(canvas_width, canvas_height);

//     PLAYER.with(|p| {
//         if let Some(player) = p.borrow_mut().as_mut() {
//             player.jump(&map, canvas_height);
//         }
//     });
// }

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
pub fn press_key(key: &str) {
    KEYS_PRESSED.with(|keys| {
        keys.borrow_mut().insert(key.to_string());
    });
}

#[wasm_bindgen]
pub fn release_key(key: &str) {
    KEYS_PRESSED.with(|keys| {
        keys.borrow_mut().remove(key);
    });
}
