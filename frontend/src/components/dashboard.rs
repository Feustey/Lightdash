use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::api::ApiService;
use crate::models::{SparkSeerStats, FeeHistory, PeerComparison};
use crate::components::{actions::Actions, fee_simulator::FeeSimulator};

#[derive(Properties, Clone, PartialEq)]
pub struct DashboardProps {
    pub api_service: ApiService,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let stats = use_state(|| None::<SparkSeerStats>);
    let fee_history = use_state(|| None::<FeeHistory>);
    let peer_comparison = use_state(|| None::<PeerComparison>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let api_service = props.api_service.clone();
        let stats = stats.clone();
        let fee_history = fee_history.clone();
        let peer_comparison = peer_comparison.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            loading.set(true);
            error.set(None);

            spawn_local(async move {
                let stats_result = api_service.get_stats().await;
                let history_result = api_service.get_fee_history().await;
                let peers_result = api_service.get_peer_comparison().await;

                match (stats_result, history_result, peers_result) {
                    (Ok(s), Ok(h), Ok(p)) => {
                        stats.set(Some(s));
                        fee_history.set(Some(h));
                        peer_comparison.set(Some(p));
                        loading.set(false);
                    }
                    (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
                        error.set(Some(e.as_string().unwrap_or_else(|| "Une erreur est survenue".to_string())));
                        loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    html! {
        <div class="container mx-auto px-4 py-8">
            if *loading {
                <div class="flex justify-center items-center h-64">
                    <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-indigo-500"></div>
                </div>
            } else if let Some(error_msg) = (*error).clone() {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                    <strong class="font-bold">{"Erreur! "}</strong>
                    <span class="block sm:inline">{error_msg}</span>
                </div>
            } else {
                <div class="space-y-8">
                    // Statistiques générales
                    if let Some(stats) = (*stats).clone() {
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                            <div class="bg-white shadow rounded-lg p-6">
                                <h3 class="text-lg font-semibold text-gray-900 mb-4">{"Canaux actifs"}</h3>
                                <p class="text-3xl font-bold text-indigo-600">{stats.active_channels}</p>
                            </div>
                            <div class="bg-white shadow rounded-lg p-6">
                                <h3 class="text-lg font-semibold text-gray-900 mb-4">{"Capacité totale"}</h3>
                                <p class="text-3xl font-bold text-indigo-600">{format!("{} sats", stats.total_capacity)}</p>
                            </div>
                            <div class="bg-white shadow rounded-lg p-6">
                                <h3 class="text-lg font-semibold text-gray-900 mb-4">{"Revenus mensuels"}</h3>
                                <p class="text-3xl font-bold text-indigo-600">{format!("{} sats", stats.monthly_revenue)}</p>
                            </div>
                        </div>
                    }

                    // Historique des frais et comparaison des pairs
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        if let Some(history) = (*fee_history).clone() {
                            <div class="bg-white shadow rounded-lg p-6">
                                <h3 class="text-lg font-semibold text-gray-900 mb-4">{"Historique des revenus"}</h3>
                                <div class="space-y-4">
                                    {
                                        history.entries.into_iter().map(|entry| {
                                            html! {
                                                <div class="flex justify-between items-center">
                                                    <span class="text-gray-600">{entry.date}</span>
                                                    <span class="font-medium">{format!("{} sats", entry.revenue)}</span>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>
                        }

                        if let Some(peers) = (*peer_comparison).clone() {
                            <div class="bg-white shadow rounded-lg p-6">
                                <h3 class="text-lg font-semibold text-gray-900 mb-4">{"Comparaison avec les pairs"}</h3>
                                <div class="space-y-4">
                                    {
                                        peers.suggested_peers.into_iter().map(|peer| {
                                            html! {
                                                <div class="flex justify-between items-center">
                                                    <span class="text-gray-600">{peer.alias}</span>
                                                    <span class="font-medium">{format!("{:.2}% de similarité", peer.similarity * 100.0)}</span>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>
                        }
                    </div>

                    // Actions recommandées et simulateur de frais
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        <Actions api_service={props.api_service.clone()} />
                        <FeeSimulator api_service={props.api_service.clone()} />
                    </div>
                </div>
            }
        </div>
    }
} 