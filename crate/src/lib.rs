mod particle;
mod state;

#[macro_use]
extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

use crate::state::State;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const PARTICLE_COLOUR: &str = "rgba(256, 0, 0, 1.0)";
const PARTICLE_RADIUS: f64 = 4.0;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

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

    let canvas = create_canvas()?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let mut state = State::new(width, height);
    let js_particle_colour = JsValue::from(PARTICLE_COLOUR);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
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

            context.set_stroke_style(&js_particle_colour);
            context.fill();
            context.set_fill_style(&js_particle_colour);
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn create_canvas() -> Result<web_sys::HtmlCanvasElement, JsValue> {
    let width = web_sys::window()
        .unwrap()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap();

    let height = web_sys::window()
        .unwrap()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap();

    let canvas = document().create_element("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
    canvas.style().set_property("border", "solid")?;

    body().append_child(&canvas)?;
    Ok(canvas)
}
