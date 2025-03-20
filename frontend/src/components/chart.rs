use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, Chart, ChartConfiguration, ChartData, ChartOptions, ChartType, ChartDataset};
use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d};

pub struct ChartComponent {
    chart: Option<Chart>,
    canvas_ref: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub data: Vec<f64>,
    pub labels: Vec<String>,
    pub title: String,
    pub chart_type: ChartType,
}

impl Component for ChartComponent {
    type Message = ();
    type Properties = ChartProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            chart: None,
            canvas_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="chart-container">
                <canvas ref={self.canvas_ref.clone()}></canvas>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                let data = ChartData::new();
                let mut datasets = js_sys::Array::new();
                let dataset = ChartDataset::new();
                dataset.data(&ctx.props().data);
                dataset.label(&ctx.props().title);
                datasets.push(&dataset);
                data.datasets(&datasets);
                data.labels(&ctx.props().labels);

                let options = ChartOptions::new();
                options.responsive(true);
                options.maintain_aspect_ratio(false);

                let config = ChartConfiguration::new(
                    ctx.props().chart_type,
                    &data,
                    Some(&options),
                );

                let chart = Chart::new(&canvas, &config);
                self.chart = Some(chart);
            }
        }
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
        use_effect_with_deps(
            move |_| {
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
                || ()
            },
            (data, title, color),
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