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
    let channels = use_state(|| Vec::new());
    let transactions = use_state(|| Vec::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let node_stats = node_stats.clone();
        let channels = channels.clone();
        let transactions = transactions.clone();
        let loading = loading.clone();
        let error = error.clone();
        let api_service = props.api_service.clone();

        use_effect_with(
            api_service,
            move |api_service| {
                let node_stats = node_stats.clone();
                let channels = channels.clone();
                let transactions = transactions.clone();
                let loading = loading.clone();
                let error = error.clone();

                spawn_local(async move {
                    let stats_result = api_service.get_node_stats().await;
                    let channels_result = api_service.get_channels().await;
                    let transactions_result = api_service.get_transactions().await;

                    match (stats_result, channels_result, transactions_result) {
                        (Ok(stats), Ok(chans), Ok(txs)) => {
                            node_stats.set(Some(stats));
                            channels.set(Some(chans));
                            transactions.set(Some(txs));
                            loading.set(false);
                        }
                        (stats_err, channels_err, transactions_err) => {
                            let error_msg = format!(
                                "Erreur lors de la récupération des données : {:?}, {:?}, {:?}",
                                stats_err.err(),
                                channels_err.err(),
                                transactions_err.err()
                            );
                            error.set(Some(error_msg));
                            loading.set(false);
                        }
                    }
                });

                || ()
            },
        );
    }

    html! {
        <div class={"dashboard"}>
            if *loading {
                <div class={"loading"}>{"Chargement..."}</div>
            } else if let Some(err) = &*error {
                <div class={"error"}>{err}</div>
            } else {
                <div class={"dashboard-content"}>
                    if let Some(stats) = &*node_stats {
                        <div class={"stats"}>
                            <h2>{"Statistiques du nœud"}</h2>
                            <div class={"stats-grid"}>
                                <div class={"stat-item"}>
                                    <span class={"stat-label"}>{"Alias"}</span>
                                    <span class={"stat-value"}>{&stats.alias}</span>
                                </div>
                                <div class={"stat-item"}>
                                    <span class={"stat-label"}>{"Capacité totale"}</span>
                                    <span class={"stat-value"}>{format!("{} sats", stats.capacity)}</span>
                                </div>
                                <div class={"stat-item"}>
                                    <span class={"stat-label"}>{"Nombre de canaux"}</span>
                                    <span class={"stat-value"}>{stats.channel_count}</span>
                                </div>
                                <div class={"stat-item"}>
                                    <span class={"stat-label"}>{"Canaux actifs"}</span>
                                    <span class={"stat-value"}>{stats.active_channels}</span>
                                </div>
                            </div>
                        </div>
                    }
                    <div class={"channels"}>
                        <h2>{"Canaux"}</h2>
                        <div class={"table-container"}>
                            <table>
                                <thead>
                                    <tr>
                                        <th>{"ID"}</th>
                                        <th>{"Capacité"}</th>
                                        <th>{"Statut"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {for channels.iter().map(move |channel| {
                                        let status_class = match channel.status {
                                            crate::models::ChannelStatus::Active => "text-green-800 bg-green-100",
                                            crate::models::ChannelStatus::Inactive => "text-red-800 bg-red-100",
                                            crate::models::ChannelStatus::Pending => "text-yellow-800 bg-yellow-100",
                                        };
                                        
                                        html! {
                                            <tr>
                                                <td class={"px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"}>
                                                    {&channel.id}
                                                </td>
                                                <td class={"px-6 py-4 whitespace-nowrap text-sm text-gray-500"}>
                                                    {format!("{} sats", channel.capacity)}
                                                </td>
                                                <td class={"px-6 py-4 whitespace-nowrap"}>
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
                    </div>
                </div>
            }
        </div>
    }
} 