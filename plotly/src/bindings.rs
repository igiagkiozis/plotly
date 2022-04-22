//! Bindings to the underlying plotly.js Javascript API. To be used in a WASM context, where it is assumed that a 
//! remote copy of the Javascript Plotly library is available, (i.e. via a CDN).

use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::Plot;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = Plotly, js_name = newPlot)]
    async fn new_plot_(id: &str, obj: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = Plotly, js_name = react)]
    async fn react_(id: &str, obj: &Object) -> Result<JsValue, JsValue>;
}

/// A wrapper around the plotly.js [newPlot](https://plotly.com/javascript/plotlyjs-function-reference/#plotlynewplot)
/// function.
///
/// The function signature is slightly constrained in that `id` is a &str which represents
/// the `id` of an existing HTML `div` element, rather than also allowing an instance of a `div`
/// element, itself.
pub async fn new_plot(id: &str, plot: &Plot) {
    // Convert the strongly typed Plot struct into a JS object via JSON. The only reason this
    // could fail is if the plotly library produces structurally incorrect JSON.
    let plot_obj = js_sys::JSON::parse(&plot.to_json())
        .expect("Invalid JSON")
        .dyn_into::<Object>()
        .expect("Invalid JSON structure - expected an top-level Object");

    // This will only fail if the Rust Plotly library has produced plotly-incompatible JSON. An error here
    // should have been handled by the library, rather than down here.
    new_plot_(id, &plot_obj)
        .await
        .expect("Error plotting chart");
}

/// A wrapper around the plotly.js [react](https://plotly.com/javascript/plotlyjs-function-reference/#react)
/// function.
///
/// The function signature is slightly constrained in that `id` is a &str which represents
/// the `id` of an existing HTML `div` element, rather than also allowing an instance of a `div`
/// element, itself.
pub async fn react(id: &str, plot: &Plot) {
    // Convert the strongly typed Plot struct into a JS object via JSON. The only reason this
    // could fail is if the plotly library produces structurally incorrect JSON.
    let plot_obj = js_sys::JSON::parse(&plot.to_json())
        .expect("Invalid JSON")
        .dyn_into::<Object>()
        .expect("Invalid JSON structure - expected a top-level Object");

    // This will only fail if the Rust Plotly library has produced plotly-incompatible JSON. An error here
    // should have been handled by the library, rather than down here.
    react_(id, &plot_obj).await.expect("Error plotting chart");
}
