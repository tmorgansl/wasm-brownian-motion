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
    let pause_input = inputs::PauseInput::new()?;

    create_elements(
        &canvas.html_element(),
        &num_particles_input.html_element(),
        &speed_input.html_element(),
        &pause_input.html_element(),
    )?;

    let num_particles = Rc::new(Cell::new(state.particles().len()));
    let num_particles_action = num_particles.clone();

    num_particles_input.add_event_listener(num_particles_action)?;

    let speed = Rc::new(Cell::new(state.speed()));
    let speed_action = speed.clone();

    speed_input.add_event_listener(speed_action)?;

    let is_paused = Rc::new(Cell::new(false));
    let is_paused_action = is_paused.clone();

    pause_input.add_event_listener(is_paused_action)?;

    canvas.animate(state, num_particles, speed, is_paused)?;

    Ok(())
}

fn create_elements(
    canvas: &web_sys::HtmlCanvasElement,
    num_particle_input: &web_sys::HtmlInputElement,
    speed_input: &web_sys::HtmlInputElement,
    pause_input: &web_sys::HtmlButtonElement,
) -> Result<(), JsValue> {
    let canvas_container = container_element();

    let style = canvas_container.style();
    style.set_property("width", "100%")?;
    style.set_property("height", "100%")?;
    style.set_property("display", "flex")?;
    style.set_property("justify-content", "center")?;
    style.set_property("align-items", "center")?;

    body().append_child(&canvas_container)?;

    let input_container = container_element();

    let style = input_container.style();
    style.set_property("display", "flex")?;
    style.set_property("height", "50%")?;
    style.set_property("flex-direction", "column")?;
    style.set_property("justify-content", "space-around")?;

    let num_particle_container = container_element();

    let num_particle_title = title_element("Number of Particles")?;
    num_particle_container.append_child(&num_particle_title)?;
    num_particle_container.append_child(num_particle_input)?;

    let speed_container = container_element();

    let speed_title = title_element("Particle Speed")?;
    speed_container.append_child(&speed_title)?;
    speed_container.append_child(speed_input)?;

    input_container.append_child(pause_input)?;
    input_container.append_child(&num_particle_container)?;
    input_container.append_child(&speed_container)?;
    canvas_container.append_child(&input_container)?;

    canvas_container.append_child(canvas)?;

    Ok(())
}

fn title_element(title: &str) -> Result<web_sys::HtmlElement, JsValue> {
    let title_element = container_element();
    title_element.style().set_property("text-align", "center")?;

    title_element.set_inner_html(title);
    Ok(title_element)
}

fn container_element() -> web_sys::HtmlElement {
    let div = element("div");
    let div = div
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    div
}
