use std::sync::mpsc::Sender;

use brawllib_rs::high_level_fighter::HighLevelSubaction;
use brawllib_rs::renderer::app::state::{AppEvent, State};
use brawllib_rs::renderer::app::App;
use brawllib_rs::renderer::camera::CharacterFacing;

use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlElement};
use web_sys::{Request, RequestInit, RequestMode, Response};

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Warn).expect("could not initialize logger");

    let document = web_sys::window().unwrap().document().unwrap();
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
    let app = App::new_insert_into_element(visualiser_span, subaction).await;

    let event_tx = app.get_event_tx();

    setup_frame_buttons(&document, event_tx.clone(), frames_len);
    setup_run_toggle(&document, event_tx.clone());
    setup_previous_frame_button(&document, event_tx.clone());
    setup_next_frame_button(&document, event_tx.clone());
    setup_face_left_button(&document, event_tx.clone());
    setup_face_right_button(&document, event_tx.clone());

    // TODO: receive start/stop messages and set button_move accordingly
    // TODO: receive current frame messages and update set_frame to add class .current-frame-background

    event_tx.send(AppEvent::SetState(State::Pause)).unwrap();

    app.run();
}

fn setup_frame_buttons(document: &Document, event_tx: Sender<AppEvent>, frames_len: usize) {
    for i in 0..frames_len {
        let event_tx = event_tx.clone();
        set_button_on_click(
            document,
            &format!("set_frame_{}", i + 1),
            Box::new(move || {
                event_tx.send(AppEvent::SetState(State::Pause)).unwrap();
                event_tx.send(AppEvent::SetFrame(i)).unwrap();
            }) as Box<dyn FnMut()>,
        );
    }
}

fn setup_run_toggle(document: &Document, event_tx: Sender<AppEvent>) {
    let button = document.get_element_by_id("run-toggle").unwrap();
    let button_move = button.clone();
    button_move.set_inner_html("Run");
    let run_toggle = Closure::wrap(Box::new(move || {
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
        .set_onclick(Some(run_toggle.as_ref().unchecked_ref()));

    // Need to forget closure otherwise the destructor destroys it ;-;
    run_toggle.forget();
}

fn setup_previous_frame_button(document: &Document, event_tx: Sender<AppEvent>) {
    set_button_on_click(
        document,
        "previous-frame",
        Box::new(move || {
            event_tx
                .send(AppEvent::SetState(State::StepBackward))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_next_frame_button(document: &Document, event_tx: Sender<AppEvent>) {
    set_button_on_click(
        document,
        "next-frame",
        Box::new(move || {
            event_tx
                .send(AppEvent::SetState(State::StepForward))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_face_left_button(document: &Document, event_tx: Sender<AppEvent>) {
    set_button_on_click(
        document,
        "face-left",
        Box::new(move || {
            event_tx
                .send(AppEvent::ResetCamera(CharacterFacing::Left))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_face_right_button(document: &Document, event_tx: Sender<AppEvent>) {
    set_button_on_click(
        document,
        "face-right",
        Box::new(move || {
            event_tx
                .send(AppEvent::ResetCamera(CharacterFacing::Right))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn set_button_on_click(document: &Document, id: &str, closure: Box<dyn FnMut()>) {
    let closure = Closure::wrap(closure);
    document
        .get_element_by_id(id)
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(closure.as_ref().unchecked_ref()));

    // Need to forget closure otherwise the destructor destroys it ;-;
    closure.forget();
}
