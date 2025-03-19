use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::api::ApiService;
use crate::models::SimulationResult;

#[derive(Properties, Clone, PartialEq)]
pub struct FeeSimulatorProps {
    pub api_service: ApiService,
}

#[function_component(FeeSimulator)]
pub fn fee_simulator(props: &FeeSimulatorProps) -> Html {
    let channel_id = use_state(|| String::new());
    let fee_rate = use_state(|| 0.0);
    let simulation_result = use_state(|| None::<SimulationResult>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let on_submit = {
        let api_service = props.api_service.clone();
        let channel_id = channel_id.clone();
        let fee_rate = fee_rate.clone();
        let simulation_result = simulation_result.clone();
        let error = error.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let api_service = api_service.clone();
            let channel_id = (*channel_id).clone();
            let fee_rate = *fee_rate;
            let simulation_result = simulation_result.clone();
            let error = error.clone();
            let loading = loading.clone();

            loading.set(true);
            error.set(None);

            spawn_local(async move {
                match api_service.simulate_fees(channel_id, fee_rate).await {
                    Ok(result) => {
                        simulation_result.set(Some(result));
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

    let on_channel_id_change = {
        let channel_id = channel_id.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            channel_id.set(input.value());
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
            
            <form onsubmit={on_submit} class="space-y-4">
                <div>
                    <label for="channel_id" class="block text-sm font-medium text-gray-700">
                        {"ID du canal"}
                    </label>
                    <input
                        type="text"
                        id="channel_id"
                        value={(*channel_id).clone()}
                        onchange={on_channel_id_change}
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                        required=true
                    />
                </div>

                <div>
                    <label for="fee_rate" class="block text-sm font-medium text-gray-700">
                        {"Taux de frais (%)"}
                    </label>
                    <input
                        type="number"
                        id="fee_rate"
                        value={(*fee_rate).to_string()}
                        onchange={on_fee_rate_change}
                        step="0.001"
                        min="0"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                        required=true
                    />
                </div>

                <button
                    type="submit"
                    disabled={*loading}
                    class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                >
                    if *loading {
                        {"Simulation en cours..."}
                    } else {
                        {"Simuler"}
                    }
                </button>
            </form>

            if let Some(error_msg) = (*error).clone() {
                <div class="mt-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                    <strong class="font-bold">{"Erreur! "}</strong>
                    <span class="block sm:inline">{error_msg}</span>
                </div>
            }

            if let Some(result) = (*simulation_result).clone() {
                <div class="mt-6 space-y-4">
                    <h3 class="text-lg font-semibold">{"Résultats de la simulation"}</h3>
                    <div class="bg-gray-50 p-4 rounded-lg">
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <p class="text-sm text-gray-500">{"Revenus actuels"}</p>
                                <p class="text-lg font-medium">{format!("{} sats", result.current_revenue)}</p>
                            </div>
                            <div>
                                <p class="text-sm text-gray-500">{"Impact sur le routage"}</p>
                                <p class="text-lg font-medium">{format!("{:.2}%", result.routing_impact * 100.0)}</p>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
} 