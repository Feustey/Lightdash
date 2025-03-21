use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement};
use js_sys::{Object, Reflect};
use gloo_utils::format::JsValueSerdeExt;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn createChart(canvas: &HtmlCanvasElement, title: &str, data: &JsValue, labels: &JsValue) -> JsValue;
}

#[derive(Properties, PartialEq, Clone)]
pub struct ChartProps {
    pub title: String,
    pub data: Vec<f64>,
    pub labels: Vec<String>,
}

#[function_component(ChartComponent)]
pub fn chart(props: &ChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let props = props.clone();

    use_effect_with((canvas_ref.clone(), props.clone()), move |(canvas_ref, props)| {
        if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
            let data = JsValue::from_serde(&props.data).unwrap();
            let labels = JsValue::from_serde(&props.labels).unwrap();
            let _chart = createChart(&canvas, &props.title, &data, &labels);
        }
        || ()
    });

    html! {
        <div class="chart-container">
            <canvas ref={canvas_ref}></canvas>
        </div>
    }
} 