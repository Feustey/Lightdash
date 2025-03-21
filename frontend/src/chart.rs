use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window, Document};
use serde_json::{json, to_value};
use gloo_utils::format::JsValueSerdeExt;

#[wasm_bindgen]
extern "C" {
    type Chart;

    #[wasm_bindgen(js_namespace = Chart)]
    fn new(ctx: &CanvasRenderingContext2d, config: &JsValue) -> Chart;
}

pub fn create_chart(canvas_id: &str, data: Vec<f64>, labels: Vec<String>, title: &str) -> Result<Chart, JsValue> {
    let window = window().ok_or("Pas de window")?;
    let document = window.document().ok_or("Pas de document")?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or("Canvas non trouvé")?
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .ok_or("Contexte 2d non trouvé")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let config = json!({
        "type": "line",
        "data": {
            "labels": labels,
            "datasets": [{
                "label": title,
                "data": data,
                "fill": false,
                "borderColor": "rgb(75, 192, 192)",
                "tension": 0.1
            }]
        },
        "options": {
            "responsive": true,
            "maintainAspectRatio": false,
            "plugins": {
                "legend": {
                    "position": "top"
                },
                "title": {
                    "display": true,
                    "text": title
                }
            },
            "scales": {
                "y": {
                    "beginAtZero": true,
                    "grid": {
                        "color": "rgba(0, 0, 0, 0.1)"
                    }
                },
                "x": {
                    "grid": {
                        "color": "rgba(0, 0, 0, 0.1)"
                    }
                }
            }
        }
    });

    let config = config.to_js_value().map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(Chart::new(&ctx, &config))
}

pub fn create_bar_chart(canvas_id: &str, data: Vec<f64>, labels: Vec<String>, title: &str) -> Result<Chart, JsValue> {
    let window = window().ok_or("Pas de window")?;
    let document = window.document().ok_or("Pas de document")?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or("Canvas non trouvé")?
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .ok_or("Contexte 2d non trouvé")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let config = json!({
        "type": "bar",
        "data": {
            "labels": labels,
            "datasets": [{
                "label": title,
                "data": data,
                "backgroundColor": "rgba(74, 144, 226, 0.6)",
                "borderColor": "rgba(74, 144, 226, 1)",
                "borderWidth": 1
            }]
        },
        "options": {
            "responsive": true,
            "maintainAspectRatio": false,
            "plugins": {
                "legend": {
                    "position": "top"
                },
                "title": {
                    "display": true,
                    "text": title
                }
            },
            "scales": {
                "y": {
                    "beginAtZero": true,
                    "grid": {
                        "color": "rgba(0, 0, 0, 0.1)"
                    }
                },
                "x": {
                    "grid": {
                        "color": "rgba(0, 0, 0, 0.1)"
                    }
                }
            }
        }
    });

    let config = to_value(&config).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(Chart::new(&ctx, &config))
}

pub fn create_pie_chart(canvas_id: &str, data: Vec<f64>, labels: Vec<String>, title: &str) -> Result<Chart, JsValue> {
    let window = window().ok_or("Pas de window")?;
    let document = window.document().ok_or("Pas de document")?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or("Canvas non trouvé")?
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .ok_or("Contexte 2d non trouvé")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let colors = vec![
        "rgba(74, 144, 226, 0.8)",
        "rgba(46, 204, 113, 0.8)",
        "rgba(241, 196, 15, 0.8)",
        "rgba(231, 76, 60, 0.8)",
        "rgba(155, 89, 182, 0.8)",
        "rgba(52, 152, 219, 0.8)",
        "rgba(230, 126, 34, 0.8)",
        "rgba(149, 165, 166, 0.8)"
    ];

    let config = json!({
        "type": "pie",
        "data": {
            "labels": labels,
            "datasets": [{
                "data": data,
                "backgroundColor": colors,
                "borderColor": "white",
                "borderWidth": 2
            }]
        },
        "options": {
            "responsive": true,
            "maintainAspectRatio": false,
            "plugins": {
                "legend": {
                    "position": "right"
                },
                "title": {
                    "display": true,
                    "text": title
                }
            }
        }
    });

    let config = to_value(&config).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(Chart::new(&ctx, &config))
} 