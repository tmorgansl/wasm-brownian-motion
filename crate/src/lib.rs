mod canvas;
mod dom;
mod particle;
mod state;

#[macro_use]
extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

use crate::canvas::Canvas;
use crate::dom::{body, element, request_animation_frame};
use crate::state::State;
use std::cell::{Cell, RefCell};
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const PARTICLE_COLOUR_FILL: &str = "rgba(238, 232, 170, 1.0)";
const PARTICLE_COLOUR_BORDER: &str = "rgba(128, 128, 0, 1.0)";
const PARTICLE_RADIUS: f64 = 4.0;

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
    create_elements(&canvas)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let context = canvas
        .html_element()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let js_particle_colour_fill = JsValue::from(PARTICLE_COLOUR_FILL);
    let js_particle_colour_border = JsValue::from(PARTICLE_COLOUR_BORDER);
    let is_paused = Rc::new(Cell::new(false));

    let is_paused = is_paused.clone();
    let is_paused_action = is_paused.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if is_paused.get() {
            return;
        }

        state.tick();

        context.clear_rect(0.0, 0.0, width, height);

        let particles = state.particles();

        for particle in particles.iter() {
            context.begin_path();

            context
                .arc(
                    particle.pos()[0],
                    particle.pos()[1],
                    PARTICLE_RADIUS,
                    0.0,
                    f64::consts::PI * 2.0,
                )
                .unwrap();
            context.fill();
            context.set_fill_style(&js_particle_colour_fill);
            context.set_stroke_style(&js_particle_colour_border);
            context.stroke();
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        is_paused_action.set(!is_paused_action.get());
        if !is_paused_action.get() {
            request_animation_frame(g.borrow().as_ref().unwrap());
        }
    }) as Box<dyn FnMut(_)>);
    canvas
        .html_element()
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn create_elements(canvas: &Canvas) -> Result<(), JsValue> {
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

    let inputs = create_inputs()?;
    canvas_container.append_child(&inputs)?;

    canvas_container.append_child(canvas.html_element())?;

    Ok(())
}

fn create_inputs() -> Result<web_sys::HtmlElement, JsValue> {
    let container = element("div");
    let container = container
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();

    let slider = element("input");
    let slider = slider
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| ())
        .unwrap();
    slider.set_type("range");
    container.append_child(&slider)?;
    Ok(container)
}
