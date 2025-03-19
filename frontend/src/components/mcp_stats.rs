use yew::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use gloo_timers::callback::Interval;
use crate::models::McpChannel;
use crate::services::ApiService;
use thousands::Separable;

pub struct McpStatsComponent {
    channels: Vec<McpChannel>,
    error: Option<String>,
    _interval: Option<Interval>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_service: ApiService,
}

pub enum Msg {
    LoadChannels,
    ChannelsLoaded(Result<Vec<McpChannel>, String>),
    Tick,
}

impl Component for McpStatsComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadChannels);
        
        // Mise à jour automatique toutes les 30 secondes
        let interval = {
            let link = ctx.link().clone();
            Interval::new(30_000, move || link.send_message(Msg::Tick))
        };

        Self {
            channels: Vec::new(),
            error: None,
            _interval: Some(interval),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadChannels => {
                let callback = ctx.link().callback(Msg::ChannelsLoaded);
                ctx.props().api_service.get_mcp_channels(callback);
                false
            }
            Msg::ChannelsLoaded(result) => {
                match result {
                    Ok(channels) => {
                        self.channels = channels;
                        self.error = None;
                        self.draw_charts();
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            }
            Msg::Tick => {
                ctx.link().send_message(Msg::LoadChannels);
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let total_capacity: u64 = self.channels.iter().map(|c| c.capacity).sum();
        let total_local_balance: u64 = self.channels.iter().map(|c| c.local_balance).sum();
        let channel_count = self.channels.len();
        let avg_capacity = if channel_count > 0 {
            total_capacity / channel_count as u64
        } else {
            0
        };

        html! {
            <div class="space-y-6">
                <div class="bg-white shadow rounded-lg p-6">
                    <h2 class="text-2xl font-bold mb-4">{"Statistiques MCP"}</h2>
                    
                    // Résumé des statistiques
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                        <div class="bg-blue-50 p-4 rounded-lg">
                            <p class="text-sm text-blue-600">{"Capacité totale"}</p>
                            <p class="text-2xl font-bold">{total_capacity.separate_with_spaces()}{" sats"}</p>
                        </div>
                        <div class="bg-green-50 p-4 rounded-lg">
                            <p class="text-sm text-green-600">{"Balance locale totale"}</p>
                            <p class="text-2xl font-bold">{total_local_balance.separate_with_spaces()}{" sats"}</p>
                        </div>
                        <div class="bg-purple-50 p-4 rounded-lg">
                            <p class="text-sm text-purple-600">{"Capacité moyenne"}</p>
                            <p class="text-2xl font-bold">{avg_capacity.separate_with_spaces()}{" sats"}</p>
                        </div>
                    </div>

                    // Graphiques
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <h3 class="text-lg font-semibold mb-2">{"Distribution des capacités"}</h3>
                            <canvas id="capacity-chart" width="400" height="300"></canvas>
                        </div>
                        <div>
                            <h3 class="text-lg font-semibold mb-2">{"Balance locale vs distante"}</h3>
                            <canvas id="balance-chart" width="400" height="300"></canvas>
                        </div>
                    </div>
                </div>

                {if let Some(error) = &self.error {
                    html! {
                        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                            {error}
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render || !self.channels.is_empty() {
            self.draw_charts();
        }
    }
}

impl McpStatsComponent {
    fn draw_charts(&self) {
        if let Some(canvas) = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("capacity-chart"))
            .and_then(|element| element.dyn_into::<HtmlCanvasElement>().ok())
        {
            if let Ok(backend) = CanvasBackend::with_canvas_object(canvas) {
                let root = backend.into_drawing_area();
                root.fill(&WHITE).unwrap();

                let channels = &self.channels;
                if !channels.is_empty() {
                    let max_capacity = channels.iter().map(|c| c.capacity).max().unwrap_or(0);
                    let mut chart = ChartBuilder::on(&root)
                        .caption("Distribution des capacités", ("sans-serif", 20))
                        .margin(10)
                        .x_label_area_size(30)
                        .y_label_area_size(60)
                        .build_cartesian_2d(
                            0..channels.len(),
                            0..max_capacity,
                        )
                        .unwrap();

                    chart
                        .configure_mesh()
                        .disable_x_mesh()
                        .draw()
                        .unwrap();

                    chart
                        .draw_series(
                            channels.iter().enumerate().map(|(i, channel)| {
                                Rectangle::new(
                                    [(i, 0), (i + 1, channel.capacity)],
                                    BLUE.mix(0.3).filled(),
                                )
                            }),
                        )
                        .unwrap();
                }
            }
        }

        if let Some(canvas) = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("balance-chart"))
            .and_then(|element| element.dyn_into::<HtmlCanvasElement>().ok())
        {
            if let Ok(backend) = CanvasBackend::with_canvas_object(canvas) {
                let root = backend.into_drawing_area();
                root.fill(&WHITE).unwrap();

                let channels = &self.channels;
                if !channels.is_empty() {
                    let max_balance = channels
                        .iter()
                        .map(|c| c.local_balance.max(c.remote_balance))
                        .max()
                        .unwrap_or(0);

                    let mut chart = ChartBuilder::on(&root)
                        .caption("Balance locale vs distante", ("sans-serif", 20))
                        .margin(10)
                        .x_label_area_size(30)
                        .y_label_area_size(60)
                        .build_cartesian_2d(
                            0..channels.len(),
                            0..max_balance,
                        )
                        .unwrap();

                    chart
                        .configure_mesh()
                        .disable_x_mesh()
                        .draw()
                        .unwrap();

                    // Balance locale
                    chart
                        .draw_series(
                            channels.iter().enumerate().map(|(i, channel)| {
                                Rectangle::new(
                                    [(i, 0), (i + 1, channel.local_balance)],
                                    GREEN.mix(0.3).filled(),
                                )
                            }),
                        )
                        .unwrap()
                        .label("Balance locale");

                    // Balance distante
                    chart
                        .draw_series(
                            channels.iter().enumerate().map(|(i, channel)| {
                                Rectangle::new(
                                    [(i, channel.local_balance), (i + 1, channel.remote_balance)],
                                    RED.mix(0.3).filled(),
                                )
                            }),
                        )
                        .unwrap()
                        .label("Balance distante");

                    chart
                        .configure_series_labels()
                        .background_style(&WHITE.mix(0.8))
                        .border_style(&BLACK)
                        .draw()
                        .unwrap();
                }
            }
        }
    }
} 