#![allow(clippy::unused_unit)] // the wasm_bindgen macro is expanding to code that clippy doesnt like

use brawllib_rs::high_level_fighter::HighLevelSubaction;
use brawllib_rs::renderer::app::App;

use js_sys::Uint8Array;
use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::{Request, RequestInit, RequestMode, Response};

mod dom_ui;
mod hitbox_table_angles;

#[wasm_bindgen]
pub fn run(subaction_bin_path: String) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Warn).expect("could not initialize logger");

    wasm_bindgen_futures::spawn_local(run_async(subaction_bin_path));
}

async fn run_async(subaction_bin_path: String) {
    let document = web_sys::window().unwrap().document().unwrap();
    hitbox_table_angles::draw_hitbox_table_angles(&document);

    let subaction = get_subaction(&subaction_bin_path).await;

    run_renderer(document, subaction).await;
}

async fn get_subaction(subaction_bin_path: &str) -> HighLevelSubaction {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(subaction_bin_path, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let js_value = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
    let data = Uint8Array::new(&js_value).to_vec();

    bincode::deserialize_from(&*data).unwrap()
}

pub async fn run_renderer(document: Document, subaction: HighLevelSubaction) {
    let visualiser_span = document.get_element_by_id("fighter-render").unwrap();
    let frames_len = subaction.frames.len();
    let mut app = App::new_insert_into_element(visualiser_span, subaction).await;

    app.set_event_handler(dom_ui::init(&document, app.get_event_tx(), frames_len));

    app.run();
}
