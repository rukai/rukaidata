use brawllib_rs::renderer::app::state::{AppEvent, State};
use brawllib_rs::renderer::app::App;
use brawllib_rs::high_level_fighter::HighLevelSubaction;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");

    let document = web_sys::window().unwrap().document().unwrap();
    //document.get_bytes_from_somewhere

    let fighter_bytes = include_bytes!("subaction_data.bin");
    let subaction = bincode::deserialize(fighter_bytes).unwrap();

    wasm_bindgen_futures::spawn_local(run_renderer(document, subaction));
}

pub async fn run_renderer(document: Document, subaction: HighLevelSubaction) {
    let visualiser_span = document.get_element_by_id("fighter-render").unwrap();
    let app = App::new_insert_into_element(visualiser_span, subaction).await;

    let event_tx = app.get_event_tx();

    let frame = document.get_element_by_id("frame").unwrap();
    let frame_move = frame.clone();
    frame_move.set_inner_html("Frame: 0");

    let button = document.get_element_by_id("run").unwrap();
    let button_move = button.clone();
    button_move.set_inner_html("Run");
    let do_thing = Closure::wrap(Box::new(move || {
        if button_move.inner_html() == "Stop" {
            event_tx.send(AppEvent::SetState(State::Pause)).unwrap();
            button_move.set_inner_html("Run");
        } else {
            event_tx.send(AppEvent::SetState(State::Play)).unwrap();
            button_move.set_inner_html("Stop");
        }
    }) as Box<dyn FnMut()>);
    button
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(do_thing.as_ref().unchecked_ref()));

    app.get_event_tx()
        .send(AppEvent::SetState(State::Pause))
        .unwrap();

    app.run();
}
