use yew::prelude::*;
use yew::html;
use crate::components::{Navbar, Card, Chart, Button};
use crate::types::{Channel, NodeStats, ChannelRecommendation, OutboundLiquidityValue, SuggestedFees};
use crate::services::{fetch_all_data, fetch_channels, fetch_outbound_liquidity_value, fetch_channel_recommendations, fetch_suggested_fees};
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {}

#[function_component(DashboardPageComponent)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let stats = use_state(|| None::<NodeStats>);
    let recommendations = use_state(|| None::<Vec<ChannelRecommendation>>);
    let liquidity = use_state(|| None::<OutboundLiquidityValue>);
    let fees = use_state(|| None::<SuggestedFees>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let stats = stats.clone();
        let recommendations = recommendations.clone();
        let liquidity = liquidity.clone();
        let fees = fees.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let node_stats = stats.clone();
            let liquidity = liquidity.clone();
            let recommendations = recommendations.clone();
            let suggested_fees = fees.clone();

            spawn_local(async move {
                loading.set(true);
                error.set(None);

                if let Ok(stats) = fetch_all_data().await {
                    node_stats.set(Some(stats.0));
                }
                if let Ok(liquidity_value) = fetch_outbound_liquidity_value().await {
                    liquidity.set(Some(liquidity_value));
                }
                if let Ok(recs) = fetch_channel_recommendations().await {
                    recommendations.set(Some(recs));
                }
                if let Ok(fees) = fetch_suggested_fees().await {
                    suggested_fees.set(Some(fees));
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="dashboard">
            <Navbar current_page={"dashboard".to_string()} />
            <div class="container">
                if *loading {
                    <div class="loading">{"Chargement des données..."}</div>
                } else if let Some(err) = &*error {
                    <div class="error">
                        <p>{err}</p>
                        <Button onclick={Callback::from(move |_| {
                            let stats = stats.clone();
                            let recommendations = recommendations.clone();
                            let liquidity = liquidity.clone();
                            let fees = fees.clone();
                            let error = error.clone();
                            let loading = loading.clone();

                            spawn_local(async move {
                                loading.set(true);
                                error.set(None);
                                match fetch_all_data().await {
                                    Ok((node_stats, channel_recommendations, outbound_liquidity, suggested_fees)) => {
                                        stats.set(Some(node_stats));
                                        recommendations.set(Some(channel_recommendations));
                                        liquidity.set(Some(outbound_liquidity));
                                        fees.set(Some(suggested_fees));
                                        error.set(None);
                                    }
                                    Err(e) => {
                                        error.set(Some(format!("Erreur lors de la récupération des données : {}", e)));
                                    }
                                }
                                loading.set(false);
                            });
                        })}>
                            {"Réessayer"}
                        </Button>
                    </div>
                } else if let Some(node_stats) = &*stats {
                    <div class="dashboard-content">
                        <div class="dashboard-grid">
                            <Card title={"Statistiques générales".to_string()}>
                                <div class="stats-grid">
                                    <div class="stat-item">
                                        <h3>{"Capacité totale"}</h3>
                                        <p>{format!("{} sats", node_stats.total_capacity)}</p>
                                    </div>
                                    <div class="stat-item">
                                        <h3>{"Nombre de canaux"}</h3>
                                        <p>{node_stats.num_channels}</p>
                                    </div>
                                    <div class="stat-item">
                                        <h3>{"Capacité moyenne"}</h3>
                                        <p>{format!("{} sats", node_stats.avg_channel_size)}</p>
                                    </div>
                                    <div class="stat-item">
                                        <h3>{"Capacité médiane"}</h3>
                                        <p>{format!("{} sats", node_stats.median_channel_size)}</p>
                                    </div>
                                </div>
                            </Card>

                            <Card title={"Liquidité".to_string()}>
                                <div class="stats-grid">
                                    <div class="stat-item">
                                        <h3>{"Balance locale"}</h3>
                                        <p>{format!("{} sats", node_stats.total_local_balance)}</p>
                                    </div>
                                    <div class="stat-item">
                                        <h3>{"Balance distante"}</h3>
                                        <p>{format!("{} sats", node_stats.total_remote_balance)}</p>
                                    </div>
                                    <div class="stat-item">
                                        <h3>{"Nombre de pairs"}</h3>
                                        <p>{node_stats.num_peers}</p>
                                    </div>
                                    <div class="stat-item">
                                        <h3>{"Uptime"}</h3>
                                        <p>{format!("{:.1}%", node_stats.uptime_percentage)}</p>
                                    </div>
                                </div>
                            </Card>

                            if let Some(liquidity) = &*liquidity {
                                <Card class={"liquidity-card".to_string()} title={"Liquidité".to_string()}>
                                    <div class="liquidity-grid">
                                        <div class="total-value">
                                            <h3>{"Valeur totale"}</h3>
                                            <p>{format!("{} sats", liquidity.total_value)}</p>
                                        </div>
                                        {for liquidity.value_per_channel.iter().map(move |channel| {
                                            html! {
                                                <div class="channel-value">
                                                    <h4>{format!("Canal: {}", channel.channel_id)}</h4>
                                                    <p>{format!("Score: {:.2}", channel.value_score)}</p>
                                                </div>
                                            }
                                        })}
                                    </div>
                                </Card>
                            }

                            if let Some(fees) = &*fees {
                                <Card title={"Frais suggérés".to_string()}>
                                    <div class="fees-grid">
                                        <div class="fee-item">
                                            <h3>{"Frais de base"}</h3>
                                            <p>{format!("{} msats", fees.base_fee_msat)}</p>
                                        </div>
                                        <div class="fee-item">
                                            <h3>{"Taux de frais"}</h3>
                                            <p>{format!("{} ppm", fees.fee_rate_ppm)}</p>
                                        </div>
                                        <div class="fee-item">
                                            <h3>{"Delta de verrouillage"}</h3>
                                            <p>{fees.time_lock_delta}</p>
                                        </div>
                                    </div>
                                </Card>
                            }

                            <Card class={"uptime-card".to_string()} title={"Disponibilité".to_string()}>
                                <div class="uptime-content">
                                    <p>{format!("{:.1}%", node_stats.uptime_percentage)}</p>
                                </div>
                            </Card>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
} 