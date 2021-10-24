use brawllib_rs::high_level_fighter::HighLevelSubaction;
use brawllib_rs::renderer::app::App;

use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::{Request, RequestInit, RequestMode, Response};

mod dom_ui;
mod hitbox_table_angles;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Warn).expect("could not initialize logger");

    let document = web_sys::window().unwrap().document().unwrap();

    hitbox_table_angles::draw_hitbox_table_angles(&document);

    let global = JsValue::from(js_sys::global());
    let subaction_json = js_sys::Reflect::get(&global, &"fighter_subaction_data_string".into())
        .unwrap()
        .as_string()
        .unwrap();

    let subaction = serde_json::from_str(&subaction_json).unwrap();

    wasm_bindgen_futures::spawn_local(run_renderer(document, subaction));
}

// TODO: hmmmmm if we have to do some slow javascript conversion,
// then there is no gaurantee this will be faster than just parsing json
// So lets wait till we have more complete functionality before attempting this optimization
#[allow(unused)]
async fn get_subaction(subaction_name: &str) -> HighLevelSubaction {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("{}.bin", subaction_name);

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

    todo!("How do I get binary data from the json?")
}

pub async fn run_renderer(document: Document, subaction: HighLevelSubaction) {
    let visualiser_span = document.get_element_by_id("fighter-render").unwrap();
    let frames_len = subaction.frames.len();
    let mut app = App::new_insert_into_element(visualiser_span, subaction).await;

    app.set_event_handler(dom_ui::init(&document, app.get_event_tx(), frames_len));

    app.run();
}
