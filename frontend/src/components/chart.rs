use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, Chart, ChartConfiguration, ChartData, ChartOptions, ChartType, ChartDataset};
use yew::prelude::*;

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