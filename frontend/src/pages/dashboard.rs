use yew::prelude::*;
use crate::components::{Navbar, Card, Chart};
use crate::types::{NodeStats, OutboundLiquidityValue, SuggestedFees};
use crate::services::{fetch_node_stats, fetch_outbound_liquidity_value, fetch_suggested_fees};

#[function_component(DashboardPageComponent)]
pub fn dashboard_page() -> Html {
    let node_stats = use_state(|| None::<NodeStats>);
    let liquidity = use_state(|| None::<OutboundLiquidityValue>);
    let fees = use_state(|| None::<SuggestedFees>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    {
        let node_stats = node_stats.clone();
        let liquidity = liquidity.clone();
        let fees = fees.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                
                // Fetch node stats
                match fetch_node_stats().await {
                    Ok(stats) => {
                        node_stats.set(Some(stats));
                    }
                    Err(e) => {
                        error.set(Some(format!("Erreur lors de la récupération des statistiques : {}", e)));
                    }
                }

                // Fetch liquidity data
                match fetch_outbound_liquidity_value().await {
                    Ok(liq) => {
                        liquidity.set(Some(liq));
                    }
                    Err(e) => {
                        error.set(Some(format!("Erreur lors de la récupération de la liquidité : {}", e)));
                    }
                }

                // Fetch suggested fees
                match fetch_suggested_fees().await {
                    Ok(f) => {
                        fees.set(Some(f));
                    }
                    Err(e) => {
                        error.set(Some(format!("Erreur lors de la récupération des frais : {}", e)));
                    }
                }

                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="container mx-auto px-4 py-8">
            <Navbar />
            <div class="mt-8">
                <h1 class="text-3xl font-bold mb-6">{"Tableau de bord"}</h1>
                
                if *loading {
                    <div class="text-center">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"></div>
                    </div>
                } else if let Some(err) = &*error {
                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                        <strong class="font-bold">{"Erreur !"}</strong>
                        <span class="block sm:inline">{" "}{err}</span>
                    </div>
                } else {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        if let Some(stats) = &*node_stats {
                            <Card title="Statistiques du nœud">
                                <div class="space-y-4">
                                    <div>
                                        <span class="font-semibold">{"Capacité totale :"}</span>
                                        <span class="ml-2">{format!("{} sats", stats.total_capacity)}</span>
                                    </div>
                                    <div>
                                        <span class="font-semibold">{"Nombre de canaux :"}</span>
                                        <span class="ml-2">{stats.num_channels}</span>
                                    </div>
                                    <div>
                                        <span class="font-semibold">{"Balance locale :"}</span>
                                        <span class="ml-2">{format!("{} sats", stats.local_balance)}</span>
                                    </div>
                                    <div>
                                        <span class="font-semibold">{"Balance distante :"}</span>
                                        <span class="ml-2">{format!("{} sats", stats.remote_balance)}</span>
                                    </div>
                                    <div>
                                        <span class="font-semibold">{"Disponibilité :"}</span>
                                        <span class="ml-2">{format!("{:.1}%", stats.uptime_percentage)}</span>
                                    </div>
                                </div>
                            </Card>

                            <Card title="Distribution des fonds">
                                <Chart 
                                    
                                    data={vec![
                                        ("Balance locale".to_string(), stats.local_balance as f64),
                                        ("Balance distante".to_string(), stats.remote_balance as f64),
                                    ]}
                                    options={serde_json::json!({
                                        "responsive": true,
                                        "plugins": {
                                            "legend": {
                                                "position": "bottom"
                                            }
                                        }
                                    })}
                                />
                            </Card>
                        }

                        if let Some(liq) = &*liquidity {
                            <Card title="Liquidité sortante">
                                <Chart 
                                    
                                    data={liq.value_per_channel.iter().map(|(channel, value)| 
                                        (channel.clone(), *value as f64)
                                    ).collect()}
                                    options={serde_json::json!({
                                        "responsive": true,
                                        "plugins": {
                                            "legend": {
                                                "display": false
                                            }
                                        },
                                        "scales": {
                                            "y": {
                                                "beginAtZero": true,
                                                "title": {
                                                    "display": true,
                                                    "text": "Sats"
                                                }
                                            }
                                        }
                                    })}
                                />
                            </Card>
                        }

                        if let Some(fee_data) = &*fees {
                            <Card title="Frais suggérés">
                                <div class="space-y-4">
                                    <div>
                                        <span class="font-semibold">{"Frais de base :"}</span>
                                        <span class="ml-2">{format!("{} msat", fee_data.base_fee_msat)}</span>
                                    </div>
                                    <div>
                                        <span class="font-semibold">{"Taux de frais :"}</span>
                                        <span class="ml-2">{format!("{} ppm", fee_data.fee_rate_ppm)}</span>
                                    </div>
                                    <div>
                                        <span class="font-semibold">{"Delta de verrouillage :"}</span>
                                        <span class="ml-2">{fee_data.time_lock_delta}</span>
                                    </div>
                                    <div>
                                        <span class="font-semibold">{"Raison :"}</span>
                                        <span class="ml-2">{&"Unknown Reason"}</span>
                                    </div>
                                </div>
                            </Card>
                        }
                    </div>
                }
            </div>
        </div>
    }
} 