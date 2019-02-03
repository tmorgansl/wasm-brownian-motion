use crate::dom::{element, request_animation_frame};
use crate::state::State;
use std::cell::{Cell, RefCell};
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const BACKGROUND_COLOUR: &str = "rgba(135, 206, 230, 1.0)";
const BORDER_COLOUR: &str = "rgba(70, 130, 180, 1.0)";
const PARTICLE_COLOUR_FILL: &str = "rgba(238, 232, 170, 1.0)";
const PARTICLE_COLOUR_BORDER: &str = "rgba(128, 128, 0, 1.0)";
const PARTICLE_RADIUS: f64 = 4.0;

#[derive(Debug, Clone)]
pub struct Canvas {
    html_element: web_sys::HtmlCanvasElement,
    width: f64,
    height: f64,
}

impl Canvas {
    pub fn new(width: f64, height: f64) -> Result<Canvas, JsValue> {
        let canvas = element("canvas");
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        let style = canvas.style();
        style.set_property("border", "solid")?;
        style.set_property("max-width", "95%")?;
        style.set_property("max-height", "95%")?;
        style.set_property("background-color", BACKGROUND_COLOUR)?;
        style.set_property("border-color", BORDER_COLOUR)?;

        Ok(Canvas {
            html_element: canvas,
            width,
            height,
        })
    }

    pub fn html_element(&self) -> &web_sys::HtmlCanvasElement {
        &self.html_element
    }

    pub fn animate(
        &self,
        mut state: State,
        num_particles: Rc<Cell<usize>>,
        speed: Rc<Cell<f64>>,
        is_paused: Rc<Cell<bool>>,
    ) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let context = self
            .html_element()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let js_particle_colour_fill = JsValue::from(PARTICLE_COLOUR_FILL);
        let js_particle_colour_border = JsValue::from(PARTICLE_COLOUR_BORDER);

        let width = self.width;
        let height = self.height;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if !is_paused.get() {
                let num_particles_value = num_particles.get();
                if num_particles_value != state.particles().len() {
                    state.update_num_particles(num_particles_value);
                }

                let speed_value = speed.get();
                if speed_value != state.speed() {
                    state.update_speed(speed_value);
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
            }
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(())
    }
}
