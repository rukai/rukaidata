#![allow(clippy::unused_unit)] // the wasm_bindgen macro is expanding to code that clippy doesnt like

use base64::{engine::general_purpose, Engine as _};
use brawllib_rs::high_level_fighter::HighLevelSubaction;
use brawllib_rs::renderer::app::App;
use log::Level;
use wasm_bindgen::prelude::*;
use web_sys::Document;

mod dom_ui;
mod hitbox_table_angles;

#[wasm_bindgen]
pub fn run(subaction_bincode: String) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Warn).expect("could not initialize logger");

    wasm_bindgen_futures::spawn_local(run_async(subaction_bincode));
}

async fn run_async(subaction_bincode: String) {
    let document = web_sys::window().unwrap().document().unwrap();
    hitbox_table_angles::draw_hitbox_table_angles(&document);

    let subaction = get_subaction(&subaction_bincode).await;

    run_renderer(document, subaction).await;
}

async fn get_subaction(subaction_bincode: &str) -> HighLevelSubaction {
    let data = general_purpose::STANDARD.decode(subaction_bincode).unwrap();
    bincode::deserialize_from(data.as_slice()).unwrap()
}

pub async fn run_renderer(document: Document, subaction: HighLevelSubaction) {
    let visualiser_span = document.get_element_by_id("fighter-render").unwrap();
    let frames_len = subaction.frames.len();
    let mut app = App::new_insert_into_element(visualiser_span, subaction).await;

    app.set_event_handler(dom_ui::init(&document, app.get_event_tx(), frames_len));

    app.run();
}
