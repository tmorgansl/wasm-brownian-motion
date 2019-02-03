mod canvas;
mod dom;
mod inputs;
mod particle;
mod state;

#[macro_use]
extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

use crate::canvas::Canvas;
use crate::dom::{body, element};
use crate::state::State;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();

    let inner_width = web_sys::window()
        .unwrap()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap();

    let inner_height = web_sys::window()
        .unwrap()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap();

    let width = 0.95 * inner_width;
    let height = 0.95 * inner_height;

    let mut state = State::new(width, height);
    let canvas = Canvas::new(width, height)?;

    let slider = inputs::new_slider();
    create_elements(&canvas, &slider)?;

    let num_particles = Rc::new(Cell::new(state.particles().len()));
    let num_particles_action = num_particles.clone();

    canvas.animate(state, num_particles)?;

    let slider_cell = Rc::new(RefCell::new(slider));

    let slider_cell2 = slider_cell.clone();

    let closure = Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
        let slider = slider_cell.borrow();
        let value = slider.value();
        num_particles_action.set(value.parse::<usize>().unwrap());
    }) as Box<dyn FnMut(_)>);
    slider_cell2
        .borrow_mut()
        .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn create_elements(canvas: &Canvas, slider: &web_sys::HtmlInputElement) -> Result<(), JsValue> {
    let canvas_container = element("div");
    let canvas_container = canvas_container
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();

    let style = canvas_container.style();
    style.set_property("width", "100%")?;
    style.set_property("height", "100%")?;
    style.set_property("display", "flex")?;
    style.set_property("justify-content", "center")?;
    style.set_property("align-items", "center")?;

    body().append_child(&canvas_container)?;

    let input_container = element("div");
    let input_container = input_container
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();

    input_container.append_child(&slider)?;
    canvas_container.append_child(&input_container)?;

    canvas_container.append_child(canvas.html_element())?;

    Ok(())
}
