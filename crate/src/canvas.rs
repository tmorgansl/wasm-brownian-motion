use crate::dom::element;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const BACKGROUND_COLOUR: &str = "rgba(135, 206, 230, 1.0)";
const BORDER_COLOUR: &str = "rgba(70, 130, 180, 1.0)";

pub struct Canvas {
    html_element: web_sys::HtmlCanvasElement,
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
        })
    }

    pub fn html_element(&self) -> &web_sys::HtmlCanvasElement {
        &self.html_element
    }
}
