use wasm_bindgen::JsCast;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Properties, PartialEq, Clone)]
pub struct ChartProps {
    pub title: String,
    pub data: Vec<f64>,
    pub labels: Vec<String>,
}

#[function_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let props = props.clone();

    {
        let canvas_ref = canvas_ref.clone();
        use_effect_with(props.clone(), move |props| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                // Clear canvas
                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

                // Draw chart
                let width = canvas.width() as f64;
                let height = canvas.height() as f64;
                let bar_width = width / props.data.len() as f64;
                let max_value = props.data.iter().fold(0.0f64, |a, let max_value = props.data.iter().fold(0.0f64, |a, (0.0f64, |a, (0.0f64, |a, (0.0f64, |a, (0.0, |a, &b| a.max(b))b| a.max(b))b| a.max(b))b| a.max(b))b| a.max(b));b| a.max(b));

                for (i, (value, label)) in props.data.iter().zip(props.labels.iter()).enumerate() {
                    let x = i as f64 * bar_width;
                    let bar_height = (value / max_value) * height;
                    let y = height - bar_height;

                    // Draw bar
                    context.set_fill_style(&"#3B82F6".into());
                    context.fill_rect(x, y, bar_width - 2.0, bar_height);

                    // Draw label
                    context.set_fill_style(&"#FFFFFF".into());
                    context.set_font("12px Arial");
                    context.fill_text(label, x + bar_width / 2.0 - 20.0, height - 5.0).unwrap();
                }
            }
            || ()
        });
    }

    html! {
        <div class="w-full h-64 bg-dark-lighter rounded-lg p-4">
            <h3 class="text-lg font-semibold text-white mb-4">{props.title}</h3>
            <canvas ref={canvas_ref} class="w-full h-48" width="800" height="300"></canvas>
        </div>
    }
} 