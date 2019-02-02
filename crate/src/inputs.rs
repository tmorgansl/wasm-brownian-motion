use crate::dom::element;
use wasm_bindgen::JsCast;

pub fn new_slider() -> web_sys::HtmlInputElement {
    let slider = element("input");
    let slider = slider
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| ())
        .unwrap();
    slider.set_type("range");
    slider.set_value("100");
    slider.set_min("0");
    slider.set_max("10000");
    slider
}
