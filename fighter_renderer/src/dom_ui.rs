use std::sync::mpsc::Sender;

use brawllib_rs::renderer::app::state::{
    AppEventIncoming, AppEventOutgoing, AppEventOutgoingHandler, InvulnerableType, State,
};
use brawllib_rs::renderer::camera::CharacterFacing;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, HtmlInputElement, HtmlSelectElement};

pub fn init(
    document: &Document,
    event_tx: Sender<AppEventIncoming>,
    frames_len: usize,
) -> AppEventOutgoingHandler {
    setup_frame_buttons(&document, event_tx.clone(), frames_len);
    setup_run_toggle(&document, event_tx.clone());
    setup_previous_frame_button(&document, event_tx.clone());
    setup_next_frame_button(&document, event_tx.clone());
    setup_face_left_button(&document, event_tx.clone());
    setup_face_right_button(&document, event_tx.clone());
    setup_invulnerable_select(&document, event_tx.clone());
    setup_wireframe_checkbox(&document, event_tx.clone());
    setup_ecb_checkbox(&document, event_tx.clone());
    setup_perspective_checkbox(&document, event_tx.clone());

    event_tx
        .send(AppEventIncoming::SetState(State::Pause))
        .unwrap();
    event_tx.send(AppEventIncoming::SetFrame(0)).unwrap();

    Box::new(move |event| app_event_handler(event, frames_len))
}

fn app_event_handler(event: AppEventOutgoing, frames_len: usize) {
    let document = web_sys::window().unwrap().document().unwrap();

    match event {
        AppEventOutgoing::NewState(State::Pause) => {
            document
                .get_element_by_id("run-toggle")
                .unwrap()
                .set_inner_html("Run");
        }
        AppEventOutgoing::NewState(State::Play) => {
            document
                .get_element_by_id("run-toggle")
                .unwrap()
                .set_inner_html("Stop");
        }
        AppEventOutgoing::NewState(_) => {}
        AppEventOutgoing::NewFrame(frame) => {
            for i in 0..frames_len {
                document
                    .get_element_by_id(&format!("set_frame_{}", i + 1))
                    .unwrap()
                    .class_list()
                    .remove_1("current-frame-button")
                    .unwrap();
            }
            document
                .get_element_by_id(&format!("set_frame_{}", frame + 1))
                .unwrap()
                .class_list()
                .add_1("current-frame-button")
                .unwrap();
        }
        AppEventOutgoing::NewInvulnerableType(invulnerable_type) => {
            let checkbox = document.get_element_by_id("invulnerable-select").unwrap();
            checkbox
                .dyn_ref::<HtmlSelectElement>()
                .unwrap()
                .set_value(match invulnerable_type {
                    InvulnerableType::Hit => "Hit",
                    InvulnerableType::Grab => "Grab",
                    InvulnerableType::TrapItem => "Trap Item",
                });
        }
        AppEventOutgoing::NewWireframe(wireframe) => {
            let checkbox = document.get_element_by_id("wireframe-checkbox").unwrap();
            checkbox
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .set_checked(wireframe);
        }
        AppEventOutgoing::NewRenderEcb(ecb) => {
            let checkbox = document.get_element_by_id("ecb-checkbox").unwrap();
            checkbox
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .set_checked(ecb);
        }
        AppEventOutgoing::NewPerspective(perspective) => {
            let checkbox = document.get_element_by_id("perspective-checkbox").unwrap();
            checkbox
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .set_checked(perspective);
        }
    }
}

fn setup_frame_buttons(document: &Document, event_tx: Sender<AppEventIncoming>, frames_len: usize) {
    for i in 0..frames_len {
        let event_tx = event_tx.clone();
        set_button_on_click(
            document,
            &format!("set_frame_{}", i + 1),
            Box::new(move || {
                event_tx
                    .send(AppEventIncoming::SetState(State::Pause))
                    .unwrap();
                event_tx.send(AppEventIncoming::SetFrame(i)).unwrap();
            }) as Box<dyn FnMut()>,
        );
    }
}

fn setup_run_toggle(document: &Document, event_tx: Sender<AppEventIncoming>) {
    let button = document.get_element_by_id("run-toggle").unwrap();
    let button_move = button.clone();
    let run_toggle = Closure::wrap(Box::new(move || {
        if button_move.inner_html() == "Stop" {
            event_tx
                .send(AppEventIncoming::SetState(State::Pause))
                .unwrap();
        } else {
            event_tx
                .send(AppEventIncoming::SetState(State::Play))
                .unwrap();
        }
    }) as Box<dyn FnMut()>);

    button
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(run_toggle.as_ref().unchecked_ref()));

    // Need to forget closure otherwise the destructor destroys it ;-;
    run_toggle.forget();
}

fn setup_previous_frame_button(document: &Document, event_tx: Sender<AppEventIncoming>) {
    set_button_on_click(
        document,
        "previous-frame",
        Box::new(move || {
            event_tx
                .send(AppEventIncoming::SetState(State::StepBackward))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_next_frame_button(document: &Document, event_tx: Sender<AppEventIncoming>) {
    set_button_on_click(
        document,
        "next-frame",
        Box::new(move || {
            event_tx
                .send(AppEventIncoming::SetState(State::StepForward))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_face_left_button(document: &Document, event_tx: Sender<AppEventIncoming>) {
    set_button_on_click(
        document,
        "face-left",
        Box::new(move || {
            event_tx
                .send(AppEventIncoming::ResetCamera(CharacterFacing::Left))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_face_right_button(document: &Document, event_tx: Sender<AppEventIncoming>) {
    set_button_on_click(
        document,
        "face-right",
        Box::new(move || {
            event_tx
                .send(AppEventIncoming::ResetCamera(CharacterFacing::Right))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_invulnerable_select(document: &Document, event_tx: Sender<AppEventIncoming>) {
    let select = document.get_element_by_id("invulnerable-select").unwrap();

    let select_clone = select.clone();
    let closure = Closure::wrap(Box::new(move || {
        event_tx
            .send(AppEventIncoming::SetInvulnerableType(
                match select_clone
                    .dyn_ref::<HtmlSelectElement>()
                    .unwrap()
                    .value()
                    .as_ref()
                {
                    "Hit" => InvulnerableType::Hit,
                    "Grab" => InvulnerableType::Grab,
                    "Trap Item" => InvulnerableType::TrapItem,
                    _ => unreachable!(),
                },
            ))
            .unwrap();
    }) as Box<dyn FnMut()>);

    select
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onchange(Some(closure.as_ref().unchecked_ref()));

    // Need to forget closure otherwise the destructor destroys it ;-;
    closure.forget();
}

fn setup_wireframe_checkbox(document: &Document, event_tx: Sender<AppEventIncoming>) {
    let checkbox = document.get_element_by_id("wireframe-checkbox").unwrap();
    set_button_on_click(
        document,
        "wireframe-checkbox",
        Box::new(move || {
            event_tx
                .send(AppEventIncoming::SetWireframe(
                    checkbox.dyn_ref::<HtmlInputElement>().unwrap().checked(),
                ))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_ecb_checkbox(document: &Document, event_tx: Sender<AppEventIncoming>) {
    let checkbox = document.get_element_by_id("ecb-checkbox").unwrap();
    set_button_on_click(
        document,
        "ecb-checkbox",
        Box::new(move || {
            event_tx
                .send(AppEventIncoming::SetRenderEcb(
                    checkbox.dyn_ref::<HtmlInputElement>().unwrap().checked(),
                ))
                .unwrap();
        }) as Box<dyn FnMut()>,
    );
}

fn setup_perspective_checkbox(document: &Document, event_tx: Sender<AppEventIncoming>) {
    let checkbox = document.get_element_by_id("perspective-checkbox").unwrap();
    set_button_on_click(
        document,
        "perspective-checkbox",
        Box::new(move || {
            event_tx
                .send(AppEventIncoming::SetPerspective(
                    checkbox.dyn_ref::<HtmlInputElement>().unwrap().checked(),
                ))
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
