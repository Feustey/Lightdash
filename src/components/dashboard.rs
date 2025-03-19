use yew::prelude::*;
use crate::services::ApiService;
use crate::models::{NodeStats, Channel, Transaction};
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub api_service: ApiService,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let node_stats = use_state(|| None::<NodeStats>);
    let channels = use_state(|| Vec::<Channel>::new());
    let transactions = use_state(|| Vec::<Transaction>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let node_stats = node_stats.clone();
        let channels = channels.clone();
        let transactions = transactions.clone();
        let loading = loading.clone();
        let error = error.clone();
        let api_service = props.api_service.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let stats_result = api_service.get_node_stats().await;
                    let channels_result = api_service.get_channels().await;
                    let transactions_result = api_service.get_transactions().await;

                    match (stats_result, channels_result, transactions_result) {
                        (Ok(stats), Ok(chans), Ok(txs)) => {
                            node_stats.set(Some(stats));
                            channels.set(chans);
                            transactions.set(txs);
                            loading.set(false);
                        }
                        (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
                            error.set(Some(e));
                            loading.set(false);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="space-y-8">
            <div class="bg-white rounded-lg shadow-lg p-6">
                <h2 class="text-2xl font-bold mb-4">{"Statistiques du Nœud"}</h2>
                
                if *loading {
                    <div class="text-center py-8 text-gray-500">
                        {"Chargement des statistiques..."}
                    </div>
                } else if let Some(err) = (*error).as_ref() {
                    <div class="text-center py-8 text-red-500">
                        {format!("Erreur: {}", err)}
                    </div>
                } else if let Some(stats) = (*node_stats).as_ref() {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        <div class="p-4 bg-blue-50 rounded-lg">
                            <h3 class="text-lg font-semibold text-blue-800">{"Alias"}</h3>
                            <p class="text-2xl font-bold text-blue-900">{&stats.alias}</p>
                        </div>
                        <div class="p-4 bg-green-50 rounded-lg">
                            <h3 class="text-lg font-semibold text-green-800">{"Nombre de Canaux"}</h3>
                            <p class="text-2xl font-bold text-green-900">{stats.num_channels}</p>
                        </div>
                        <div class="p-4 bg-purple-50 rounded-lg">
                            <h3 class="text-lg font-semibold text-purple-800">{"Capacité Totale"}</h3>
                            <p class="text-2xl font-bold text-purple-900">{format!("{} sats", stats.total_capacity)}</p>
                        </div>
                    </div>
                }
            </div>

            <div class="bg-white rounded-lg shadow-lg p-6">
                <h2 class="text-2xl font-bold mb-4">{"Canaux"}</h2>
                
                if *loading {
                    <div class="text-center py-8 text-gray-500">
                        {"Chargement des canaux..."}
                    </div>
                } else if channels.is_empty() {
                    <div class="text-center py-8 text-gray-500">
                        {"Aucun canal trouvé."}
                    </div>
                } else {
                    <div class="overflow-x-auto">
                        <table class="min-w-full divide-y divide-gray-200">
                            <thead class="bg-gray-50">
                                <tr>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"ID du Canal"}
                                    </th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"Capacité"}
                                    </th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"Statut"}
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="bg-white divide-y divide-gray-200">
                                {for channels.iter().map(|channel| {
                                    let status_class = match channel.status {
                                        crate::models::ChannelStatus::Active => "text-green-800 bg-green-100",
                                        crate::models::ChannelStatus::Inactive => "text-red-800 bg-red-100",
                                        crate::models::ChannelStatus::Pending => "text-yellow-800 bg-yellow-100",
                                    };
                                    
                                    html! {
                                        <tr>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                {&channel.channel_id}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                {format!("{} sats", channel.capacity)}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap">
                                                <span class={format!("px-2 inline-flex text-xs leading-5 font-semibold rounded-full {}", status_class)}>
                                                    {format!("{}", channel.status)}
                                                </span>
                                            </td>
                                        </tr>
                                    }
                                })}
                            </tbody>
                        </table>
                    </div>
                }
            </div>
        </div>
    }
} 