use yew::prelude::*;
use crate::components::{Navbar, Card, Button};
use crate::types::{Dashboard, Channel, Action, Recommendation, NodeStats, ChannelRecommendation, OutboundLiquidityValue, SuggestedFees};
use crate::services::{fetch_all_data, fetch_channels, get_ai_recommendations};

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let stats = use_state(|| None::<NodeStats>);
    let recommendations = use_state(|| None::<Vec<ChannelRecommendation>>);
    let liquidity = use_state(|| None::<OutboundLiquidityValue>);
    let fees = use_state(|| None::<SuggestedFees>);
    let error = use_state(|| None::<String>);

    {
        let stats = stats.clone();
        let recommendations = recommendations.clone();
        let liquidity = liquidity.clone();
        let fees = fees.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_all_data().await {
                        Ok((node_stats, channel_recommendations, outbound_liquidity, suggested_fees)) => {
                            stats.set(Some(node_stats));
                            recommendations.set(Some(channel_recommendations));
                            liquidity.set(Some(outbound_liquidity));
                            fees.set(Some(suggested_fees));
                            error.set(None);
                        }
                        Err(e) => {
                            error.set(Some(e));
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="page">
            <Navbar current_page={"dashboard".to_string()} />
            <div class="content">
                <h1>{"Tableau de bord du nœud Lightning"}</h1>
                
                if let Some(error) = (*error).clone() {
                    <div class="error-message">
                        {format!("Erreur: {}", error)}
                    </div>
                }

                if let Some(node_stats) = (*stats).clone() {
                    <div class="stats-grid">
                        <Card title={"Statistiques générales".to_string()}>
                            <div class="stat-item">
                                <span class="stat-label">{"Capacité totale"}</span>
                                <span class="stat-value">{format!("{} sats", node_stats.total_capacity)}</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{"Nombre de canaux"}</span>
                                <span class="stat-value">{node_stats.num_channels}</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{"Score de flexibilité"}</span>
                                <span class="stat-value">{format!("{:.2}", node_stats.liquidity_flexibility_score)}</span>
                            </div>
                        </Card>

                        <Card title={"Rangs"}.to_string()>
                            <div class="stat-item">
                                <span class="stat-label">{"Rang de betweenness"}</span>
                                <span class="stat-value">{node_stats.betweenness_rank}</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{"Rang de closeness"}</span>
                                <span class="stat-value">{node_stats.closeness_rank}</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{"Rang d'eigenvector"}</span>
                                <span class="stat-value">{node_stats.eigenvector_rank}</span>
                            </div>
                        </Card>

                        <Card title={"Frais"}.to_string()>
                            <div class="stat-item">
                                <span class="stat-label">{"Frais de base moyen"}</span>
                                <span class="stat-value">{format!("{} msats", node_stats.mean_outbound_base_fee)}</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-label">{"Taux de frais moyen"}</span>
                                <span class="stat-value">{format!("{} ppm", node_stats.mean_outbound_fee_rate)}</span>
                            </div>
                        </Card>
                    </div>
                }

                if let Some(recs) = (*recommendations).clone() {
                    <h2>{"Recommandations de canaux"}</h2>
                    <div class="recommendations-grid">
                        {for recs.iter().map(|rec| {
                            html! {
                                <Card title={format!("Recommandation pour {}", rec.pubkey)}>
                                    {for rec.info.iter().map(|info| {
                                        html! {
                                            <div class="recommendation-item">
                                                <p>{format!("Gain de rang: {}", info.gain_in_betweenness_rank)}</p>
                                                <p>{format!("Capacité minimale: {} sats", info.minimum_viable_capacity)}</p>
                                                <p>{format!("Capacité idéale: {} sats", info.ideal_capacity)}</p>
                                                <p>{format!("Frais passifs suggérés: {} ppm", info.passive_fee_ppm)}</p>
                                            </div>
                                        }
                                    })}
                                </Card>
                            }
                        })}
                    </div>
                }
            </div>
        </div>
    }
}

#[function_component(ChannelsPage)]
pub fn channels_page() -> Html {
    let channels = use_state(|| None::<Vec<Channel>>);
    let error = use_state(|| None::<String>);

    {
        let channels = channels.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_channels().await {
                        Ok(channels_data) => {
                            channels.set(Some(channels_data));
                            error.set(None);
                        }
                        Err(e) => {
                            error.set(Some(e));
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="page">
            <Navbar current_page={"channels".to_string()} />
            <div class="content">
                <h1>{"Canaux Lightning"}</h1>
                
                if let Some(error) = (*error).clone() {
                    <div class="error-message">
                        {format!("Erreur: {}", error)}
                    </div>
                }

                if let Some(channels_data) = (*channels).clone() {
                    <div class="channels-grid">
                        {for channels_data.iter().map(|channel| {
                            html! {
                                <Card title={format!("Canal avec {}", channel.pubkey)}>
                                    <div class="channel-info">
                                        <div class="info-item">
                                            <span class="info-label">{"Capacité"}</span>
                                            <span class="info-value">{format!("{} sats", channel.capacity)}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">{"Balance locale"}</span>
                                            <span class="info-value">{format!("{} sats", channel.local_balance)}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">{"Balance distante"}</span>
                                            <span class="info-value">{format!("{} sats", channel.remote_balance)}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">{"Taux de frais"}</span>
                                            <span class="info-value">{format!("{} ppm", channel.fee_rate)}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">{"Frais de base"}</span>
                                            <span class="info-value">{format!("{} msats", channel.base_fee)}</span>
                                        </div>
                                        <div class="info-item">
                                            <span class="info-label">{"Dernière mise à jour"}</span>
                                            <span class="info-value">{format!("{}", channel.last_update)}</span>
                                        </div>
                                    </div>
                                    <div class="channel-actions">
                                        <Button 
                                            label={"Ajuster les frais".to_string()}
                                            onclick={Callback::from(|_| {})}
                                        />
                                        <Button 
                                            label={"Fermer le canal".to_string()}
                                            onclick={Callback::from(|_| {})}
                                        />
                                    </div>
                                </Card>
                            }
                        })}
                    </div>
                }
            </div>
        </div>
    }
}

#[function_component(ActionsPage)]
pub fn actions_page() -> Html {
    let actions = vec![
        Action {
            id: "1".to_string(),
            name: "Action par défaut".to_string(),
            description: Some("Description de l'action".to_string()),
            type_: crate::types::ActionType::Email,
            created_at: "2024-03-20".to_string(),
            updated_at: "2024-03-20".to_string(),
        },
    ];

    html! {
        <div class="page">
            <Navbar current_page={"actions".to_string()} />
            <div class="content">
                <h1>{"Actions"}</h1>
                <div class="actions-grid">
                    {for actions.iter().map(|action| {
                        html! {
                            <Card title={action.name.clone()}>
                                <p>{action.description.clone().unwrap_or_default()}</p>
                                <p>{format!("Type: {:?}", action.type_)}</p>
                                <Button 
                                    label={"Éditer".to_string()}
                                    onclick={Callback::from(|_| {})}
                                />
                            </Card>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}

#[function_component(RecommendationsPage)]
pub fn recommendations_page() -> Html {
    let recommendations = use_state(|| None::<Vec<Recommendation>>);
    let error = use_state(|| None::<String>);
    let stats = use_state(|| None::<NodeStats>);
    let channels = use_state(|| None::<Vec<Channel>>);

    {
        let recommendations = recommendations.clone();
        let error = error.clone();
        let stats = stats.clone();
        let channels = channels.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_all_data().await {
                        Ok((node_stats, _, _, _)) => {
                            stats.set(Some(node_stats));
                            match fetch_channels().await {
                                Ok(channels_data) => {
                                    channels.set(Some(channels_data));
                                    if let Some(stats_data) = &*stats {
                                        match get_ai_recommendations(stats_data, &channels_data).await {
                                            Ok(recs) => {
                                                recommendations.set(Some(recs));
                                                error.set(None);
                                            }
                                            Err(e) => {
                                                error.set(Some(e));
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    error.set(Some(e));
                                }
                            }
                        }
                        Err(e) => {
                            error.set(Some(e));
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="page">
            <Navbar current_page={"recommendations".to_string()} />
            <div class="content">
                <h1>{"Recommandations d'optimisation"}</h1>
                
                if let Some(error) = (*error).clone() {
                    <div class="error-message">
                        {format!("Erreur: {}", error)}
                    </div>
                }

                if let Some(recs) = (*recommendations).clone() {
                    <div class="recommendations-grid">
                        {for recs.iter().map(|rec| {
                            html! {
                                <Card title={rec.title.clone()}>
                                    <div class="recommendation-content">
                                        <p class="recommendation-description">{rec.description.clone()}</p>
                                        <div class="recommendation-meta">
                                            <span class={format!("priority-badge priority-{:?}", rec.priority).to_lowercase()}>
                                                {format!("{:?}", rec.priority)}
                                            </span>
                                            <span class="recommendation-date">
                                                {format!("Mise à jour: {}", rec.updated_at)}
                                            </span>
                                        </div>
                                        <Button 
                                            label={"Voir les détails".to_string()}
                                            onclick={Callback::from(|_| {})}
                                        />
                                    </div>
                                </Card>
                            }
                        })}
                    </div>
                }
            </div>
        </div>
    }
} 