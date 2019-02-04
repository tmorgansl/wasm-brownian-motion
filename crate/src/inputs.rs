use crate::dom::element;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const PAUSE: &str = "Pause";
const UNPAUSE: &str = "Unpause";
const BUTTON_COLOUR: &str = "rgba(70, 130, 180, 1.0)";

pub struct PauseInput {
    html_element: web_sys::HtmlButtonElement,
}

impl PauseInput {
    pub fn new() -> Result<PauseInput, JsValue> {
        let button = element("button");
        let button = button
            .dyn_into::<web_sys::HtmlButtonElement>()
            .map_err(|_| ())
            .unwrap();

        button.set_inner_html(PAUSE);

        let style = button.style();
        style.set_property("width", "100%")?;
        style.set_property("background-color", BUTTON_COLOUR)?;
        style.set_property("color", "white")?;
        style.set_property("padding", "15px 0px 15px 0px")?;
        style.set_property("text-align", "center")?;
        style.set_property("text-decoration", "none")?;
        style.set_property("border", "none")?;
        style.set_property("display", "inline-block")?;
        style.set_property("font-size", "16px")?;

        Ok(PauseInput {
            html_element: button,
        })
    }

    pub fn html_element(&self) -> &web_sys::HtmlButtonElement {
        &self.html_element
    }

    pub fn add_event_listener(&self, is_paused: Rc<Cell<bool>>) -> Result<(), JsValue> {
        let html_element = self.html_element.clone();

        let rc_html_element = Rc::new(RefCell::new(html_element));

        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            is_paused.set(!is_paused.get());
            let html_element = rc_html_element.borrow_mut();
            if is_paused.get() {
                html_element.set_inner_html(UNPAUSE);
            } else {
                html_element.set_inner_html(PAUSE);
            }
        }) as Box<dyn FnMut(_)>);

        self.html_element()
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }
}

pub struct SpeedInput {
    html_element: web_sys::HtmlInputElement,
}

impl SpeedInput {
    pub fn new() -> Result<SpeedInput, JsValue> {
        let slider = element("input");
        let slider = slider
            .dyn_into::<web_sys::HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();
        slider.set_type("range");
        slider.set_value("1.0");
        slider.set_min("0.1");
        slider.set_max("10.0");
        slider.set_step("0.1");

        slider.style().set_property("width", "100%")?;

        Ok(SpeedInput {
            html_element: slider,
        })
    }

    pub fn html_element(&self) -> &web_sys::HtmlInputElement {
        &self.html_element
    }

    pub fn add_event_listener(&self, speed: Rc<Cell<f64>>) -> Result<(), JsValue> {
        let html_element = self.html_element.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
            let value = html_element.value();
            speed.set(value.parse::<f64>().unwrap());
        }) as Box<dyn FnMut(_)>);

        self.html_element()
            .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }
}

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
        slider.set_value("2");
        slider.set_min("1");
        slider.set_max("4");
        slider.set_step("0.01");

        slider.style().set_property("width", "100%")?;

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
            let power = value.parse::<f64>().unwrap();
            num_particles.set(10_f64.powf(power) as usize);
        }) as Box<dyn FnMut(_)>);

        self.html_element()
            .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }
}
