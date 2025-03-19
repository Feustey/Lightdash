use yew::prelude::*;
use crate::services::api::ApiService;
use crate::models::{SimulationResult, ImpactSeverity};
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, Clone, PartialEq)]
pub struct FeeSimulatorProps {
    pub api_service: ApiService,
}

#[function_component(FeeSimulator)]
pub fn fee_simulator(props: &FeeSimulatorProps) -> Html {
    let base_fee = use_state(|| 1000u64);
    let fee_rate = use_state(|| 0.0001f64);
    let result = use_state(|| None::<SimulationResult>);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_simulate = {
        let base_fee = base_fee.clone();
        let fee_rate = fee_rate.clone();
        let result = result.clone();
        let loading = loading.clone();
        let error = error.clone();
        let api_service = props.api_service.clone();

        Callback::from(move |_| {
            loading.set(true);
            error.set(None);
            let base_fee_val = *base_fee;
            let fee_rate_val = *fee_rate;
            let result = result.clone();
            let loading = loading.clone();
            let error = error.clone();
            let api_service = api_service.clone();

            spawn_local(async move {
                match api_service.simulate_fees(base_fee_val, fee_rate_val).await {
                    Ok(simulation_result) => {
                        result.set(Some(simulation_result));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.as_string().unwrap_or_else(|| "Une erreur est survenue".to_string())));
                        loading.set(false);
                    }
                }
            });
        })
    };

    let on_base_fee_change = {
        let base_fee = base_fee.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u64>() {
                base_fee.set(value);
            }
        })
    };

    let on_fee_rate_change = {
        let fee_rate = fee_rate.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f64>() {
                fee_rate.set(value);
            }
        })
    };

    html! {
        <div class="bg-white shadow rounded-lg p-6">
            <h2 class="text-2xl font-bold mb-6">{"Simulateur de frais"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2" for="base-fee">
                        {"Frais de base (sats)"}
                    </label>
                    <input
                        type="number"
                        id="base-fee"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={(*base_fee).to_string()}
                        onchange={on_base_fee_change}
                        min="0"
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2" for="fee-rate">
                        {"Taux de frais (ppm)"}
                    </label>
                    <input
                        type="number"
                        id="fee-rate"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={(*fee_rate).to_string()}
                        onchange={on_fee_rate_change}
                        step="0.0001"
                        min="0"
                    />
                </div>
            </div>
            <div class="flex justify-center mb-6">
                <button
                    onclick={on_simulate}
                    disabled={*loading}
                    class="px-6 py-3 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {
                        if *loading {
                            "Simulation en cours..."
                        } else {
                            "Simuler"
                        }
                    }
                </button>
            </div>
            {
                if let Some(error_msg) = (*error).clone() {
                    html! {
                        <div class="text-red-500 text-center mb-6">
                            {error_msg}
                        </div>
                    }
                } else if let Some(sim_result) = (*result).clone() {
                    html! {
                        <div class="border rounded-lg p-6">
                            <h3 class="text-xl font-semibold mb-4">{"Résultats de la simulation"}</h3>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div>
                                    <p class="text-sm text-gray-600 mb-1">{"Revenus estimés (par mois)"}</p>
                                    <p class="text-2xl font-bold">{format!("{} sats", sim_result.estimated_revenue)}</p>
                                </div>
                                <div>
                                    <p class="text-sm text-gray-600 mb-1">{"Impact sur le routage"}</p>
                                    {
                                        let (color_class, text) = match sim_result.routing_impact.severity {
                                            ImpactSeverity::Positive => ("text-green-600", "Positif"),
                                            ImpactSeverity::Neutral => ("text-gray-600", "Neutre"),
                                            ImpactSeverity::Negative => ("text-red-600", "Négatif"),
                                        };
                                        html! {
                                            <p class={format!("text-2xl font-bold {}", color_class)}>{text}</p>
                                        }
                                    }
                                </div>
                            </div>
                            <div class="mt-4">
                                <p class="text-sm text-gray-600 mb-1">{"Variation des revenus"}</p>
                                {
                                    let current = sim_result.current_revenue;
                                    let simulated = sim_result.estimated_revenue;
                                    let percentage = (simulated as f64 - current as f64) / current as f64 * 100.0;
                                    let (color_class, sign) = if percentage >= 0.0 {
                                        ("text-green-600", "+")
                                    } else {
                                        ("text-red-600", "")
                                    };
                                    html! {
                                        <p class={format!("text-lg font-semibold {}", color_class)}>
                                            {format!("{}{}%", sign, percentage.abs().round())}
                                        </p>
                                    }
                                }
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
} 