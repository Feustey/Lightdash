use yew::prelude::*;
use crate::services::ApiService;
use crate::models::{NodeStats, Channel, Transaction, Recommendation};

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let api_service = use_state(|| ApiService::new());
    let node_stats = use_state(|| None::<NodeStats>);
    let channels = use_state(|| Vec::<Channel>::new());
    let transactions = use_state(|| Vec::<Transaction>::new());
    let recommendations = use_state(|| Vec::<Recommendation>::new());

    {
        let api_service = api_service.clone();
        let node_stats = node_stats.clone();
        use_effect_with_deps(
            move |_| {
                let api = (*api_service).clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(stats) = api.get_node_stats().await {
                        node_stats.set(Some(stats));
                    }
                });
                || ()
            },
            (),
        );
    }

    {
        let api_service = api_service.clone();
        let channels = channels.clone();
        use_effect_with_deps(
            move |_| {
                let api = (*api_service).clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(ch) = api.get_channels().await {
                        channels.set(ch);
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-8">{"Tableau de bord Lightning"}</h1>
            
            // Node Stats Section
            <div class="bg-white rounded-lg shadow-md p-6 mb-8">
                <h2 class="text-xl font-semibold mb-4">{"Statistiques du nœud"}</h2>
                {
                    if let Some(stats) = (*node_stats).clone() {
                        html! {
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                <div class="p-4 bg-gray-50 rounded-lg">
                                    <p class="text-gray-600">{"Capacité totale"}</p>
                                    <p class="text-2xl font-bold">{stats.total_capacity}</p>
                                </div>
                                <div class="p-4 bg-gray-50 rounded-lg">
                                    <p class="text-gray-600">{"Nombre de canaux"}</p>
                                    <p class="text-2xl font-bold">{stats.num_channels}</p>
                                </div>
                                <div class="p-4 bg-gray-50 rounded-lg">
                                    <p class="text-gray-600">{"Alias"}</p>
                                    <p class="text-2xl font-bold">{stats.alias}</p>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <p>{"Chargement des statistiques..."}</p>
                        }
                    }
                }
            </div>

            // Channels Section
            <div class="bg-white rounded-lg shadow-md p-6 mb-8">
                <h2 class="text-xl font-semibold mb-4">{"Canaux"}</h2>
                <div class="overflow-x-auto">
                    <table class="min-w-full">
                        <thead>
                            <tr class="bg-gray-50">
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                    {"ID du canal"}
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                    {"Capacité"}
                                </th>
                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                    {"Statut"}
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                (*channels).iter().map(|channel| {
                                    html! {
                                        <tr class="border-t">
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {&channel.channel_id}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {channel.capacity}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                                {&channel.status}
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
} 