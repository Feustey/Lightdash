use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};
use yew::prelude::*;
use js_sys::{Array, Object};
use serde_json::json;
use gloo_utils::format::JsValueSerdeExt;

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub data: Vec<f64>,
    pub labels: Vec<String>,
    pub title: String,
    #[prop_or("line".to_string())]
    pub chart_type: String,
    #[prop_or("rgba(74, 144, 226, 0.2)".to_string())]
    pub background_color: String,
    #[prop_or("rgba(74, 144, 226, 1)".to_string())]
    pub border_color: String,
}

#[function_component(ChartComponent)]
pub fn chart(props: &ChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let data = props.data.clone();
    let labels = props.labels.clone();
    let title = props.title.clone();
    let chart_type = props.chart_type.clone();
    let background_color = props.background_color.clone();
    let border_color = props.border_color.clone();

    {
        let canvas_ref = canvas_ref.clone();
        
        use_effect_with(
            (data.clone(), labels.clone(), title.clone(), chart_type.clone(), background_color.clone(), border_color.clone()),
            move |(data, labels, title, chart_type, background_color, border_color)| {
                let canvas = canvas_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("Le canvas devrait exister");

                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                // Création des données pour le graphique
                let data_array = Array::new();
                for value in data {
                    data_array.push(&JsValue::from_f64(*value));
                }

                let labels_array = Array::new();
                for label in labels {
                    labels_array.push(&JsValue::from_str(&label));
                }

                // Configuration du graphique
                let config = Object::new();
                js_sys::Reflect::set(&config, &JsValue::from_str("type"), &JsValue::from_str(&chart_type))
                    .expect("La configuration du type devrait fonctionner");

                let data_obj = Object::new();
                js_sys::Reflect::set(&data_obj, &JsValue::from_str("labels"), &labels_array)
                    .expect("La configuration des labels devrait fonctionner");

                let dataset = Object::new();
                js_sys::Reflect::set(&dataset, &JsValue::from_str("data"), &data_array)
                    .expect("La configuration des données devrait fonctionner");
                js_sys::Reflect::set(&dataset, &JsValue::from_str("label"), &JsValue::from_str(&title))
                    .expect("La configuration du titre devrait fonctionner");
                js_sys::Reflect::set(&dataset, &JsValue::from_str("backgroundColor"), &JsValue::from_str(&background_color))
                    .expect("La configuration de la couleur de fond devrait fonctionner");
                js_sys::Reflect::set(&dataset, &JsValue::from_str("borderColor"), &JsValue::from_str(&border_color))
                    .expect("La configuration de la couleur de bordure devrait fonctionner");

                let datasets = Array::new();
                datasets.push(&dataset);

                js_sys::Reflect::set(&data_obj, &JsValue::from_str("datasets"), &datasets)
                    .expect("La configuration des datasets devrait fonctionner");

                js_sys::Reflect::set(&config, &JsValue::from_str("data"), &data_obj)
                    .expect("La configuration des données devrait fonctionner");

                // Options du graphique
                let options = Object::new();
                let scales = Object::new();
                let y_axis = Object::new();
                js_sys::Reflect::set(&y_axis, &JsValue::from_str("beginAtZero"), &JsValue::from_bool(true))
                    .expect("La configuration de l'axe Y devrait fonctionner");
                js_sys::Reflect::set(&scales, &JsValue::from_str("y"), &y_axis)
                    .expect("La configuration des échelles devrait fonctionner");
                js_sys::Reflect::set(&options, &JsValue::from_str("scales"), &scales)
                    .expect("La configuration des options devrait fonctionner");

                js_sys::Reflect::set(&config, &JsValue::from_str("options"), &options)
                    .expect("La configuration des options devrait fonctionner");

                // Création du graphique
                let chart_js = window()
                    .unwrap()
                    .get("Chart")
                    .expect("Chart.js devrait être disponible");

                let chart_constructor = chart_js.dyn_ref::<js_sys::Function>().unwrap();
                let _ = chart_constructor.call2(
                    &JsValue::NULL,
                    &canvas,
                    &config,
                ).expect("La création du graphique devrait fonctionner");

                || ()
            },
        );
    }

    html! {
        <div class="chart-container">
            <canvas ref={canvas_ref}></canvas>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct YieldChartProps {
    pub data: Vec<(String, f64)>,
    pub title: String,
    pub color: String,
}

#[function_component(YieldChart)]
pub fn yield_chart(props: &YieldChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let data = props.data.clone();
    let title = props.title.clone();
    let color = props.color.clone();

    {
        let canvas_ref = canvas_ref.clone();
        use_effect_with(
            (data.clone(), title.clone(), color.clone()),
            move |(data, title, color)| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let context = canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();

                    // Configuration du graphique
                    let width = canvas.width() as f64;
                    let height = canvas.height() as f64;
                    let padding = 40.0;
                    let chart_width = width - (padding * 2.0);
                    let chart_height = height - (padding * 2.0);

                    // Effacer le canvas
                    context.clear_rect(0.0, 0.0, width, height);

                    // Dessiner le titre
                    context.set_fill_style(&"#ffffff".into());
                    context.set_font("16px Arial");
                    context.set_text_align("center");
                    context.fill_text(&title, width / 2.0, padding / 2.0).unwrap();

                    // Calculer les échelles
                    let max_value = data.iter().map(|(_, v)| *v).fold(0.0, f64::max);
                    let x_scale = chart_width / (data.len() - 1) as f64;
                    let y_scale = chart_height / max_value;

                    // Dessiner la grille
                    context.set_stroke_style(&"#2a2b35".into());
                    context.set_line_width(1.0);
                    for i in 0..=4 {
                        let y = height - padding - (chart_height * i as f64 / 4.0);
                        context.begin_path();
                        context.move_to(padding, y);
                        context.line_to(width - padding, y);
                        context.stroke();
                    }

                    // Dessiner les labels de l'axe Y
                    context.set_fill_style(&"#8f98a7".into());
                    context.set_font("12px Arial");
                    context.set_text_align("right");
                    for i in 0..=4 {
                        let y = height - padding - (chart_height * i as f64 / 4.0);
                        let value = (max_value * (4 - i) as f64 / 4.0) as u64;
                        context.fill_text(&format!("{}", value), padding - 10.0, y + 4.0).unwrap();
                    }

                    // Dessiner les labels de l'axe X
                    context.set_text_align("center");
                    for (i, (label, _)) in data.iter().enumerate() {
                        let x = padding + (i as f64 * x_scale);
                        context.fill_text(label, x, height - padding + 20.0).unwrap();
                    }

                    // Dessiner la ligne
                    context.set_stroke_style(&color.into());
                    context.set_line_width(2.0);
                    context.begin_path();
                    for (i, (_, value)) in data.iter().enumerate() {
                        let x = padding + (i as f64 * x_scale);
                        let y = height - padding - (value * y_scale);
                        if i == 0 {
                            context.move_to(x, y);
                        } else {
                            context.line_to(x, y);
                        }
                    }
                    context.stroke();

                    // Dessiner les points
                    context.set_fill_style(&color.into());
                    for (i, (_, value)) in data.iter().enumerate() {
                        let x = padding + (i as f64 * x_scale);
                        let y = height - padding - (value * y_scale);
                        context.begin_path();
                        context.arc(x, y, 4.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                        context.fill();
                    }
                }

                move || ()
            },
        );
    }

    html! {
        <div class="chart-container">
            <canvas
                ref={canvas_ref}
                width={400}
                height={300}
            />
        </div>
    }
} 