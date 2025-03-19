use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::services::ApiService;
use gloo_timers::callback::Interval;
use crate::services::api::ApiService as ApiService;
use crate::models::{SparkSeerStats, FeeHistory, PeerComparison, SuggestedPeer, Recommendation, RecommendationSeverity};
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use js_sys::Date;
use crate::components::fee_simulator::FeeSimulator;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealth {
    pub total_capacity: u64,
    pub active_channels: u32,
    pub online_peers: u32,
    pub inbound_liquidity: u64,
    pub outbound_liquidity: u64,
    pub fee_earnings: u64,
    pub uptime_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRecommendation {
    pub priority: String,
    pub action: String,
    pub description: String,
    pub impact: String,
}

pub struct ActionsComponent {
    health_data: Option<NodeHealth>,
    recommendations: Vec<ActionRecommendation>,
    error: Option<String>,
    _interval: Option<Interval>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ActionsProps {
    pub api_service: ApiService,
}

pub enum Msg {
    LoadData,
    DataLoaded(Result<NodeHealth, String>),
    RecommendationsReceived(Result<Vec<ActionRecommendation>, String>),
    Tick,
}

impl Component for ActionsComponent {
    type Message = Msg;
    type Properties = ActionsProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadData);
        
        // Mise à jour toutes les 5 minutes
        let interval = {
            let link = ctx.link().clone();
            Interval::new(300_000, move || link.send_message(Msg::Tick))
        };

        Self {
            health_data: None,
            recommendations: Vec::new(),
            error: None,
            _interval: Some(interval),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadData => {
                let callback = ctx.link().callback(Msg::DataLoaded);
                ctx.props().api_service.get_node_health(callback);
                false
            }
            Msg::DataLoaded(result) => {
                match result {
                    Ok(health) => {
                        self.health_data = Some(health.clone());
                        let callback = ctx.link().callback(Msg::RecommendationsReceived);
                        ctx.props().api_service.get_ai_recommendations(health, callback);
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            }
            Msg::RecommendationsReceived(result) => {
                match result {
                    Ok(recommendations) => {
                        self.recommendations = recommendations;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            }
            Msg::Tick => {
                ctx.link().send_message(Msg::LoadData);
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="space-y-6">
                <div class="bg-white shadow rounded-lg p-6">
                    <h2 class="text-2xl font-bold mb-4">{"Actions recommandées"}</h2>
                    
                    // Santé du nœud
                    {if let Some(health) = &self.health_data {
                        html! {
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                                <div class="bg-blue-50 p-4 rounded-lg">
                                    <p class="text-sm text-blue-600">{"Liquidité entrante/sortante"}</p>
                                    <p class="text-lg font-semibold">{format!("{}% / {}%",
                                        (health.inbound_liquidity as f64 / health.total_capacity as f64 * 100.0).round(),
                                        (health.outbound_liquidity as f64 / health.total_capacity as f64 * 100.0).round()
                                    )}</p>
                                </div>
                                <div class="bg-green-50 p-4 rounded-lg">
                                    <p class="text-sm text-green-600">{"Disponibilité"}</p>
                                    <p class="text-lg font-semibold">{format!("{}%", health.uptime_percentage.round())}</p>
                                </div>
                                <div class="bg-purple-50 p-4 rounded-lg">
                                    <p class="text-sm text-purple-600">{"Canaux actifs"}</p>
                                    <p class="text-lg font-semibold">{health.active_channels}{" / "}{health.online_peers}</p>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="animate-pulse">
                                <div class="h-24 bg-gray-200 rounded"></div>
                            </div>
                        }
                    }}

                    // Recommandations
                    <div class="space-y-4">
                        {for self.recommendations.iter().map(|rec| {
                            let priority_color = match rec.priority.as_str() {
                                "Haute" => "red",
                                "Moyenne" => "yellow",
                                _ => "green"
                            };
                            
                            html! {
                                <div class="border rounded-lg p-4 hover:shadow-md transition-shadow">
                                    <div class="flex items-start space-x-4">
                                        <div class={format!("w-2 h-2 mt-2 rounded-full bg-{}-500", priority_color)}></div>
                                        <div class="flex-1">
                                            <h3 class="font-semibold text-lg">{&rec.action}</h3>
                                            <p class="text-gray-600 mt-1">{&rec.description}</p>
                                            <div class="mt-2">
                                                <span class={format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-{}-100 text-{}-800", priority_color, priority_color)}>
                                                    {"Impact : "}{&rec.impact}
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        })}
                    </div>

                    {if self.recommendations.is_empty() && self.error.is_none() {
                        html! {
                            <div class="text-center py-8 text-gray-500">
                                {"Chargement des recommandations..."}
                            </div>
                        }
                    }}
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
}

#[function_component(Actions)]
pub fn actions(props: &ActionsProps) -> Html {
    let recommendations = use_state(|| Vec::<Recommendation>::new());
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let api_service = props.api_service.clone();
        let recommendations = recommendations.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            loading.set(true);
            error.set(None);

            spawn_local(async move {
                match api_service.get_ai_recommendations().await {
                    Ok(recs) => {
                        recommendations.set(recs);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.as_string().unwrap_or_else(|| "Une erreur est survenue".to_string())));
                        loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    let get_severity_class = |severity: &RecommendationSeverity| {
        match severity {
            RecommendationSeverity::High => "bg-red-100 text-red-800 border-red-300",
            RecommendationSeverity::Medium => "bg-yellow-100 text-yellow-800 border-yellow-300",
            RecommendationSeverity::Low => "bg-green-100 text-green-800 border-green-300",
        }
    };

    html! {
        <div class="bg-white shadow rounded-lg p-6">
            <h2 class="text-2xl font-bold mb-6">{"Actions recommandées"}</h2>

            if *loading {
                <div class="flex justify-center items-center h-32">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-500"></div>
                </div>
            } else if let Some(error_msg) = (*error).clone() {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                    <strong class="font-bold">{"Erreur! "}</strong>
                    <span class="block sm:inline">{error_msg}</span>
                </div>
            } else if (*recommendations).is_empty() {
                <div class="text-center py-8 text-gray-500">
                    {"Aucune action recommandée pour le moment."}
                </div>
            } else {
                <div class="space-y-4">
                    {
                        (*recommendations).iter().map(|rec| {
                            let severity_class = get_severity_class(&rec.severity);
                            html! {
                                <div class={format!("p-4 rounded-lg border {}", severity_class)}>
                                    <h3 class="font-semibold mb-2">{&rec.title}</h3>
                                    <p class="text-sm">{&rec.description}</p>
                                    if let Some(channel_id) = &rec.channel_id {
                                        <p class="text-sm mt-2">
                                            {"Canal concerné : "}
                                            <span class="font-mono">{channel_id}</span>
                                        </p>
                                    }
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </div>
    }
}

fn render_fee_history_chart(fee_history: &[FeeHistory]) -> Html {
    use_effect_with_deps(
        move |_| {
            if let Some(canvas) = web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.get_element_by_id("feeHistoryChart"))
                .and_then(|element| element.dyn_into::<HtmlCanvasElement>().ok())
            {
                // Ici, nous utiliserions Chart.js pour dessiner le graphique
                // Pour l'instant, nous laissons cette partie en commentaire car elle nécessite
                // l'intégration de Chart.js
            }
            || ()
        },
        fee_history.len(),
    );

    html! {}
}

fn get_fee_optimization_class(base_fee: f64, fee_rate: f64) -> &'static str {
    if base_fee > 1000.0 || fee_rate > 1000.0 {
        "border-red-500 bg-red-50"
    } else if base_fee > 500.0 || fee_rate > 500.0 {
        "border-yellow-500 bg-yellow-50"
    } else {
        "border-green-500 bg-green-50"
    }
}

fn get_flexibility_class(score: f64) -> &'static str {
    if score < 10.0 {
        "border-red-500 bg-red-50"
    } else if score < 20.0 {
        "border-yellow-500 bg-yellow-50"
    } else {
        "border-green-500 bg-green-50"
    }
}

fn get_outbound_fee_suggestion(mean_base: f64, mean_rate: f64, median_base: u64, median_rate: u64) -> String {
    if mean_base > (median_base as f64) * 2.0 || mean_rate > (median_rate as f64) * 2.0 {
        format!(
            "Vos frais sortants sont significativement plus élevés que la médiane du réseau. \
            Considérez une réduction pour améliorer votre compétitivité."
        )
    } else if mean_base < (median_base as f64) * 0.5 || mean_rate < (median_rate as f64) * 0.5 {
        format!(
            "Vos frais sortants sont très bas par rapport à la médiane du réseau. \
            Vous pourriez augmenter vos frais tout en restant compétitif."
        )
    } else {
        format!(
            "Vos frais sortants sont bien alignés avec la médiane du réseau. \
            Continuez à surveiller les tendances du marché."
        )
    }
}

fn get_inbound_fee_suggestion(mean_base: f64, mean_rate: f64, median_base: u64, median_rate: u64) -> String {
    if mean_base > (median_base as f64) * 2.0 || mean_rate > (median_rate as f64) * 2.0 {
        format!(
            "Vos frais entrants sont significativement plus élevés que la médiane du réseau. \
            Cela pourrait décourager les paiements entrants."
        )
    } else if mean_base < (median_base as f64) * 0.5 || mean_rate < (median_rate as f64) * 0.5 {
        format!(
            "Vos frais entrants sont très bas. Vous pourriez les augmenter \
            progressivement pour optimiser vos revenus."
        )
    } else {
        format!(
            "Vos frais entrants sont bien équilibrés. Maintenez cette stratégie \
            et ajustez en fonction du trafic."
        )
    }
}

fn get_flexibility_suggestion(score: f64, outbound_balance: f64) -> String {
    if score < 10.0 {
        format!(
            "Votre score de flexibilité est bas. Considérez d'ouvrir des canaux \
            avec des nœuds plus centraux et diversifiez vos connexions."
        )
    } else if score < 20.0 {
        if outbound_balance > 70.0 {
            format!(
                "Score de flexibilité moyen avec une forte balance sortante. \
                Cherchez à équilibrer vos canaux ou à ouvrir de nouveaux canaux entrants."
            )
        } else if outbound_balance < 30.0 {
            format!(
                "Score de flexibilité moyen avec une faible balance sortante. \
                Considérez d'augmenter votre liquidité sortante pour plus de flexibilité."
            )
        } else {
            format!(
                "Score de flexibilité moyen avec une balance équilibrée. \
                Continuez à optimiser vos connexions pour améliorer votre score."
            )
        }
    } else {
        format!(
            "Excellent score de flexibilité ! Votre nœud est bien positionné dans le réseau. \
            Maintenez cette stratégie et ajustez en fonction des changements du réseau."
        )
    }
} 