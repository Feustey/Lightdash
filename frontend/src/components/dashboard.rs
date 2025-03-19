use yew::prelude::*;
use crate::services::api::ApiService;
use crate::models::SparkSeerStats;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub api_service: ApiService,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let stats = use_state(|| None::<SparkSeerStats>);
    let error = use_state(|| None::<String>);

    {
        let stats = stats.clone();
        let error = error.clone();
        let api_service = props.api_service.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match api_service.get_node_stats().await {
                        Ok(node_stats) => stats.set(Some(node_stats)),
                        Err(e) => error.set(Some(e.to_string())),
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="p-6 space-y-6">
            <h1 class="text-3xl font-bold text-gray-900">{"Tableau de Bord Lightning"}</h1>
            
            if let Some(node_stats) = (*stats).as_ref() {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    // Carte des informations générales
                    <div class="bg-white rounded-lg shadow p-6">
                        <h2 class="text-xl font-semibold mb-4">{"Informations Générales"}</h2>
                        <div class="space-y-2">
                            <p class="flex justify-between">
                                <span class="text-gray-600">{"Alias:"}</span>
                                <span class="font-medium">{&node_stats.alias}</span>
                            </p>
                            <p class="flex justify-between">
                                <span class="text-gray-600">{"Nombre de canaux:"}</span>
                                <span class="font-medium">{node_stats.num_channels}</span>
                            </p>
                            <p class="flex justify-between">
                                <span class="text-gray-600">{"Capacité totale:"}</span>
                                <span class="font-medium">{format!("{} sats", node_stats.total_capacity)}</span>
                            </p>
                        </div>
                    </div>

                    // Carte des métriques de liquidité
                    <div class="bg-white rounded-lg shadow p-6">
                        <h2 class="text-xl font-semibold mb-4">{"Liquidité"}</h2>
                        <div class="space-y-4">
                            <div class="relative pt-1">
                                <div class="flex mb-2 items-center justify-between">
                                    <div>
                                        <span class="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-blue-600 bg-blue-200">
                                            {"Balance Sortante"}
                                        </span>
                                    </div>
                                    <div class="text-right">
                                        <span class="text-xs font-semibold inline-block text-blue-600">
                                            {format!("{}%", node_stats.effective_outbound_balance)}
                                        </span>
                                    </div>
                                </div>
                                <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-blue-200">
                                    <div style={format!("width:{}%", node_stats.effective_outbound_balance)}
                                         class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500">
                                    </div>
                                </div>
                            </div>
                            <p class="flex justify-between">
                                <span class="text-gray-600">{"Score de flexibilité:"}</span>
                                <span class="font-medium">{node_stats.liquidity_flexibility_score}</span>
                            </p>
                        </div>
                    </div>

                    // Carte des frais
                    <div class="bg-white rounded-lg shadow p-6">
                        <h2 class="text-xl font-semibold mb-4">{"Frais moyens"}</h2>
                        <div class="space-y-2">
                            <div class="border-b pb-2">
                                <h3 class="text-sm font-medium text-gray-600 mb-2">{"Frais sortants"}</h3>
                                <p class="flex justify-between">
                                    <span>{"Base:"}</span>
                                    <span class="font-medium">{format!("{} sats", node_stats.mean_outbound_base_fee)}</span>
                                </p>
                                <p class="flex justify-between">
                                    <span>{"Taux:"}</span>
                                    <span class="font-medium">{format!("{} ppm", node_stats.mean_outbound_fee_rate)}</span>
                                </p>
                            </div>
                            <div>
                                <h3 class="text-sm font-medium text-gray-600 mb-2">{"Frais entrants"}</h3>
                                <p class="flex justify-between">
                                    <span>{"Base:"}</span>
                                    <span class="font-medium">{format!("{} sats", node_stats.mean_inbound_base_fee)}</span>
                                </p>
                                <p class="flex justify-between">
                                    <span>{"Taux:"}</span>
                                    <span class="font-medium">{format!("{} ppm", node_stats.mean_inbound_fee_rate)}</span>
                                </p>
                            </div>
                        </div>
                    </div>

                    // Carte des classements réseau
                    <div class="bg-white rounded-lg shadow p-6">
                        <h2 class="text-xl font-semibold mb-4">{"Classements Réseau"}</h2>
                        <div class="space-y-2">
                            <p class="flex justify-between">
                                <span class="text-gray-600">{"Centralité:"}</span>
                                <span class="font-medium">{node_stats.betweenness_rank}</span>
                            </p>
                            <p class="flex justify-between">
                                <span class="text-gray-600">{"Vecteur propre:"}</span>
                                <span class="font-medium">{node_stats.eigenvector_rank}</span>
                            </p>
                            <p class="flex justify-between">
                                <span class="text-gray-600">{"Proximité:"}</span>
                                <span class="font-medium">{node_stats.closeness_rank}</span>
                            </p>
                        </div>
                    </div>
                </div>
            } else if let Some(err) = (*error).as_ref() {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative">
                    {format!("Erreur lors du chargement des données: {}", err)}
                </div>
            } else {
                <div class="flex justify-center items-center h-32">
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                </div>
            }
        </div>
    }
} 