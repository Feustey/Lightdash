use yew::prelude::*;
use crate::services::api::ApiService;
use crate::models::{SimulationResult, FeeSimulation, ImpactSeverity};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use gloo_timers::callback::Timeout;

#[derive(Properties, PartialEq)]
pub struct FeeSimulatorProps {
    pub api_service: ApiService,
}

pub struct FeeSimulator {
    base_fee: u64,
    fee_rate: u64,
    simulation_result: Option<SimulationResult>,
    is_simulating: bool,
    error: Option<String>,
    simulation_timeout: Option<Timeout>,
}

pub enum Msg {
    UpdateBaseFee(u64),
    UpdateFeeRate(u64),
    SimulateButtonClicked,
    SimulationReceived(Result<SimulationResult, String>),
    ResetSimulation,
}

impl Component for FeeSimulator {
    type Message = Msg;
    type Properties = FeeSimulatorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            base_fee: 0,
            fee_rate: 0,
            simulation_result: None,
            is_simulating: false,
            error: None,
            simulation_timeout: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBaseFee(value) => {
                self.base_fee = value;
                true
            }
            Msg::UpdateFeeRate(value) => {
                self.fee_rate = value;
                true
            }
            Msg::SimulateButtonClicked => {
                self.is_simulating = true;
                self.error = None;
                
                let api_service = ctx.props().api_service.clone();
                let base_fee = self.base_fee;
                let fee_rate = self.fee_rate;
                
                ctx.link().send_future(async move {
                    match api_service.simulate_fees(base_fee, fee_rate).await {
                        Ok(result) => Msg::SimulationReceived(Ok(result)),
                        Err(e) => Msg::SimulationReceived(Err(e)),
                    }
                });
                
                true
            }
            Msg::SimulationReceived(result) => {
                self.is_simulating = false;
                match result {
                    Ok(simulation) => {
                        self.simulation_result = Some(simulation);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                        self.simulation_result = None;
                    }
                }
                true
            }
            Msg::ResetSimulation => {
                self.simulation_result = None;
                self.error = None;
                self.is_simulating = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="bg-white shadow rounded-lg p-6">
                <h2 class="text-2xl font-bold text-gray-900 mb-6">{"Simulateur de Frais"}</h2>

                // Formulaire de simulation
                <div class="space-y-4 mb-8">
                    <div>
                        <label for="base_fee" class="block text-sm font-medium text-gray-700">
                            {"Frais de base (sats)"}
                        </label>
                        <div class="mt-1">
                            <input
                                type="number"
                                id="base_fee"
                                class="shadow-sm focus:ring-blue-500 focus:border-blue-500 block w-full sm:text-sm border-gray-300 rounded-md"
                                value={self.base_fee.to_string()}
                                onchange={ctx.link().callback(|e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateBaseFee(input.value().parse().unwrap_or(0))
                                })}
                            />
                        </div>
                    </div>

                    <div>
                        <label for="fee_rate" class="block text-sm font-medium text-gray-700">
                            {"Taux de frais (ppm)"}
                        </label>
                        <div class="mt-1">
                            <input
                                type="number"
                                id="fee_rate"
                                class="shadow-sm focus:ring-blue-500 focus:border-blue-500 block w-full sm:text-sm border-gray-300 rounded-md"
                                value={self.fee_rate.to_string()}
                                onchange={ctx.link().callback(|e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateFeeRate(input.value().parse().unwrap_or(0))
                                })}
                            />
                        </div>
                    </div>

                    <div class="flex justify-end space-x-4">
                        <button
                            type="button"
                            class="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                            onclick={ctx.link().callback(|_| Msg::ResetSimulation)}
                            disabled={self.is_simulating}
                        >
                            {"Réinitialiser"}
                        </button>
                        <button
                            type="button"
                            class="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                            onclick={ctx.link().callback(|_| Msg::SimulateButtonClicked)}
                            disabled={self.is_simulating}
                        >
                            {if self.is_simulating {
                                "Simulation en cours..."
                            } else {
                                "Simuler"
                            }}
                        </button>
                    </div>
                </div>

                // Résultats de la simulation
                {if let Some(result) = &self.simulation_result {
                    self.view_simulation_results(result)
                } else if let Some(error) = &self.error {
                    html! {
                        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative">
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

impl FeeSimulator {
    fn view_simulation_results(&self, result: &SimulationResult) -> Html {
        html! {
            <div class="space-y-6">
                <div class="border-t border-gray-200 pt-6">
                    <h3 class="text-lg font-medium text-gray-900">{"Résultats de la simulation"}</h3>
                    
                    // Comparaison des métriques clés
                    <div class="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2">
                        {self.view_metric_comparison("Revenus estimés", 
                            result.current.estimated_revenue, 
                            result.simulated.estimated_revenue, 
                            "sats")}
                        
                        {self.view_metric_comparison("Volume de routage", 
                            result.current.estimated_routing_volume, 
                            result.simulated.estimated_routing_volume, 
                            "sats")}
                            
                        {self.view_metric_comparison("Taux de succès", 
                            (result.current.estimated_success_rate * 100.0) as u64, 
                            (result.simulated.estimated_success_rate * 100.0) as u64, 
                            "%")}
                            
                        {self.view_metric_comparison("Score de compétitivité", 
                            (result.current.competitive_score * 100.0) as u64, 
                            (result.simulated.competitive_score * 100.0) as u64, 
                            "%")}
                    </div>

                    // Analyse d'impact
                    <div class="mt-6">
                        <h4 class="text-sm font-medium text-gray-900">{"Analyse d'impact"}</h4>
                        <div class="mt-2 space-y-4">
                            {for result.impact_analysis.iter().map(|impact| {
                                let (bg_color, text_color) = match impact.severity {
                                    ImpactSeverity::Positive => ("bg-green-50", "text-green-800"),
                                    ImpactSeverity::Neutral => ("bg-gray-50", "text-gray-800"),
                                    ImpactSeverity::Negative => ("bg-red-50", "text-red-800"),
                                };
                                
                                html! {
                                    <div class={format!("p-4 rounded-lg {}", bg_color)}>
                                        <div class="flex">
                                            <div class="flex-1">
                                                <h5 class={format!("text-sm font-medium {}", text_color)}>
                                                    {&impact.metric}
                                                </h5>
                                                <p class="mt-1 text-sm text-gray-600">
                                                    {&impact.description}
                                                </p>
                                            </div>
                                            <div class={format!("ml-4 text-sm font-medium {}", text_color)}>
                                                {format!("{:+.1}%", impact.change)}
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>

                    // Recommandation
                    <div class="mt-6 bg-blue-50 p-4 rounded-lg">
                        <div class="flex">
                            <div class="flex-shrink-0">
                                <svg class="h-5 w-5 text-blue-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                                </svg>
                            </div>
                            <div class="ml-3">
                                <h4 class="text-sm font-medium text-blue-800">{"Recommandation"}</h4>
                                <p class="mt-2 text-sm text-blue-700">
                                    {&result.recommendation}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn view_metric_comparison(&self, label: &str, current: u64, simulated: u64, unit: &str) -> Html {
        let change_percentage = if current > 0 {
            ((simulated as f64 - current as f64) / current as f64 * 100.0)
        } else {
            0.0
        };

        let (change_color, arrow) = if change_percentage > 0.0 {
            ("text-green-600", "↑")
        } else if change_percentage < 0.0 {
            ("text-red-600", "↓")
        } else {
            ("text-gray-600", "→")
        };

        html! {
            <div class="bg-gray-50 rounded-lg p-4">
                <div class="text-sm font-medium text-gray-500">{label}</div>
                <div class="mt-1 flex items-baseline justify-between">
                    <div class="text-2xl font-semibold text-gray-900">
                        {format!("{} {}", simulated, unit)}
                    </div>
                    <div class={format!("text-sm font-medium {}", change_color)}>
                        {format!("{} {:.1}%", arrow, change_percentage.abs())}
                    </div>
                </div>
                <div class="text-sm text-gray-500 mt-1">
                    {format!("Actuellement: {} {}", current, unit)}
                </div>
            </div>
        }
    }
} 