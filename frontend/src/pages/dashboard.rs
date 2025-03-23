use yew::prelude::*;
use crate::components::{Card, Chart};
use crate::types::NodeStats;
use crate::services::sparkseer::SparkseerService;

#[function_component(DashboardPage)]
pub fn dashboard() -> Html {
    let stats = use_state(|| None::<NodeStats>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let stats = stats.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let sparkseer = SparkseerService::new();
            wasm_bindgen_futures::spawn_local(async move {
                match sparkseer.get_node_stats().await {
                    Ok(data) => {
                        stats.set(Some(data));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                        loading.set(false);
                    }
                }
            });
        });
    }

    // Données pour les graphiques
    let balance_data = stats.as_ref().map(|s| vec![
        s.local_balance as f64,
        s.remote_balance as f64,
    ]).unwrap_or_default();
    let balance_labels = vec!["Balance locale".to_string(), "Balance distante".to_string()];

    let channel_data = stats.as_ref().map(|s| vec![
        s.num_channels as f64,
        s.avg_channel_size as f64,
    ]).unwrap_or_default();
    let channel_labels = vec!["Nombre de canaux".to_string(), "Taille moyenne".to_string()];

    let uptime_data = stats.as_ref().map(|s| vec![s.uptime_percentage]).unwrap_or_default();
    let uptime_labels = vec!["Uptime".to_string()];

    html! {
        <div class="min-h-screen bg-dark">
            <main class="container mx-auto px-4 py-8">
                if *loading {
                    <div class="flex justify-center items-center h-64">
                        <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary"></div>
                    </div>
                } else if let Some(error) = &*error {
                    <div class="bg-red-900/50 border border-red-500 text-red-200 px-4 py-3 rounded relative" role="alert">
                        <strong class="font-bold">{"Erreur !"}</strong>
                        <span class="block sm:inline">{" "}{error}</span>
                    </div>
                } else if let Some(stats) = &*stats {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
                        <Card title="Balance locale">
                            <div class="text-2xl font-bold text-primary">
                                {format!("{:.8} BTC", stats.local_balance as f64 / 100_000_000.0)}
                            </div>
                        </Card>
                        <Card title="Balance distante">
                            <div class="text-2xl font-bold text-primary">
                                {format!("{:.8} BTC", stats.remote_balance as f64 / 100_000_000.0)}
                            </div>
                        </Card>
                        <Card title="Capacité totale">
                            <div class="text-2xl font-bold text-primary">
                                {format!("{:.8} BTC", stats.total_capacity as f64 / 100_000_000.0)}
                            </div>
                        </Card>
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
                        <Chart
                            title="Distribution des balances"
                            data={balance_data}
                            labels={balance_labels}
                            chart_type={ChartType::Pie}
                        />
                        <Chart
                            title="Statistiques des canaux"
                            data={channel_data}
                            labels={channel_labels}
                            chart_type={ChartType::Bar}
                        />
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        <Chart
                            title="Uptime du nœud"
                            data={uptime_data}
                            labels={uptime_labels}
                            chart_type={ChartType::Doughnut}
                        />
                        <Card title="Informations du nœud">
                            <div class="space-y-4">
                                <div>
                                    <div class="text-sm text-gray-400">{"Clé publique"}</div>
                                    <div class="text-white font-mono">{&stats.pubkey}</div>
                                </div>
                                <div>
                                    <div class="text-sm text-gray-400">{"Alias"}</div>
                                    <div class="text-white">{&stats.alias}</div>
                                </div>
                                <div>
                                    <div class="text-sm text-gray-400">{"Nombre de canaux"}</div>
                                    <div class="text-white">{stats.num_channels}</div>
                                </div>
                                <div>
                                    <div class="text-sm text-gray-400">{"Taille moyenne des canaux"}</div>
                                    <div class="text-white">
                                        {format!("{:.8} BTC", stats.avg_channel_size as f64 / 100_000_000.0)}
                                    </div>
                                </div>
                                <div>
                                    <div class="text-sm text-gray-400">{"Uptime"}</div>
                                    <div class="text-white">{format!("{:.1}%", stats.uptime_percentage)}</div>
                                </div>
                            </div>
                        </Card>
                    </div>
                }
            </main>
        </div>
    }
} 