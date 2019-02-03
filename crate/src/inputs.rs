use crate::dom::element;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct NumParticlesInput {
    html_element: web_sys::HtmlInputElement,
}

impl NumParticlesInput {
    pub fn new() -> Result<NumParticlesInput, JsValue> {
        let slider = element("input");
        let slider = slider
            .dyn_into::<web_sys::HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        slider.set_type("range");
        slider.set_value("100");
        slider.set_min("0");
        slider.set_max("10000");

        Ok(NumParticlesInput {
            html_element: slider,
        })
    }

    pub fn html_element(&self) -> &web_sys::HtmlInputElement {
        &self.html_element
    }

    pub fn add_event_listener(&self, num_particles: Rc<Cell<usize>>) -> Result<(), JsValue> {
        let html_element = self.html_element.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
            let value = html_element.value();
            num_particles.set(value.parse::<usize>().unwrap());
        }) as Box<dyn FnMut(_)>);

        self.html_element()
            .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }
}
