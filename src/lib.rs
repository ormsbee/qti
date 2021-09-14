mod model;
mod parse;
mod render;
mod utils;

use pyo3::prelude::*;
use wasm_bindgen::prelude::*;
use xmltree::{Element, XMLNode};

use crate::render::RenderHtml;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, qti!");
}
*/

#[pyfunction]
fn render_qti_as_html3(qti_xml: String) -> PyResult<String> {
    if let Ok(root) = Element::parse(qti_xml.as_bytes()) {
        let choice_interaction = parse::parse_choice_interaction(&root);
        return PyResult::Ok(choice_interaction.html().to_string());
    }
    PyResult::Ok("Parse Error".to_string())  // Yeah, should be an Err
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn qti(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_qti_as_html3, m)?)?;

    Ok(())
}

#[wasm_bindgen]
pub fn render_qti_as_html(qti_xml: String) -> String {
    if let Ok(root) = Element::parse(qti_xml.as_bytes()) {
        let choice_interaction = parse::parse_choice_interaction(&root);
        return choice_interaction.html().to_string()
    }
    "Parse Error".to_string()
}

#[wasm_bindgen]
pub fn render_qti_as_html2(qti_xml: &str) -> String {
    if let Ok(root) = Element::parse(qti_xml.as_bytes()) {
        let choice_interaction = parse::parse_choice_interaction(&root);
        return choice_interaction.html().to_string()
    }
    "Parse Error".to_string()
}
