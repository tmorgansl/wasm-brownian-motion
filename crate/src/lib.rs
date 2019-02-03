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
use std::cell::Cell;
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

    let state = State::new(width, height);
    let canvas = Canvas::new(width, height)?;

    let num_particles_input = inputs::NumParticlesInput::new()?;
    let speed_input = inputs::SpeedInput::new()?;
    create_elements(
        &canvas.html_element(),
        &num_particles_input.html_element(),
        &speed_input.html_element(),
    )?;

    let num_particles = Rc::new(Cell::new(state.particles().len()));
    let num_particles_action = num_particles.clone();

    num_particles_input.add_event_listener(num_particles_action)?;

    let speed = Rc::new(Cell::new(state.speed()));
    let speed_action = speed.clone();

    speed_input.add_event_listener(speed_action)?;
    canvas.animate(state, num_particles, speed)?;

    Ok(())
}

fn create_elements(
    canvas: &web_sys::HtmlCanvasElement,
    num_particle_input: &web_sys::HtmlInputElement,
    speed_input: &web_sys::HtmlInputElement,
) -> Result<(), JsValue> {
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

    input_container.append_child(num_particle_input)?;
    input_container.append_child(speed_input)?;
    canvas_container.append_child(&input_container)?;

    canvas_container.append_child(canvas)?;

    Ok(())
}
