use yew::prelude::*;
use crate::components::{Navbar, Card, Button, YieldChart};
use crate::types::{Dashboard, Channel, Action, Recommendation, NodeStats, ChannelRecommendation, OutboundLiquidityValue, SuggestedFees};
use crate::services::{fetch_all_data, fetch_channels, get_ai_recommendations};

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
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

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
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
                            stats.set(None);
                            recommendations.set(None);
                            liquidity.set(None);
                            fees.set(None);
                        }
                    }
                    loading.set(false);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="dashboard">
            <Navbar />
            <div class="container">
                if *loading {
                    <div class="loading">{"Chargement des données..."}</div>
                } else if let Some(err) = &*error {
                    <div class="error">
                        <p>{err}</p>
                        <Button onclick={Callback::from(|_| {
                            // Recharger les données
                            let stats = stats.clone();
                            let recommendations = recommendations.clone();
                            let liquidity = liquidity.clone();
                            let fees = fees.clone();
                            let error = error.clone();
                            let loading = loading.clone();

                            wasm_bindgen_futures::spawn_local(async move {
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
                } else {
                    <div class="dashboard-content">
                        if let Some(node_stats) = &*stats {
                            <div class="dashboard-grid">
                                <Card title="Statistiques générales">
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
                                            <p>{format!("{:.2} sats", node_stats.mean_channel_capacity)}</p>
                                        </div>
                                        <div class="stat-item">
                                            <h3>{"Capacité médiane"}</h3>
                                            <p>{format!("{} sats", node_stats.median_channel_capacity)}</p>
                                        </div>
                                    </div>
                                </Card>

                                <Card title="Rangs et Métriques">
                                    <div class="stats-grid">
                                        <div class="stat-item">
                                            <h3>{"Score de flexibilité"}</h3>
                                            <p>{format!("{:.2}", node_stats.liquidity_flexibility_score)}</p>
                                        </div>
                                        <div class="stat-item">
                                            <h3>{"Rang de betweenness"}</h3>
                                            <p>{node_stats.betweenness_rank}</p>
                                        </div>
                                        <div class="stat-item">
                                            <h3>{"Rang de closeness"}</h3>
                                            <p>{node_stats.closeness_rank}</p>
                                        </div>
                                        <div class="stat-item">
                                            <h3>{"Rang d'eigenvector"}</h3>
                                            <p>{node_stats.eigenvector_rank}</p>
                                        </div>
                                    </div>
                                </Card>

                                <Card title="Frais">
                                    <div class="stats-grid">
                                        <div class="stat-item">
                                            <h3>{"Frais de base moyen"}</h3>
                                            <p>{format!("{} msats", node_stats.mean_outbound_base_fee)}</p>
                                        </div>
                                        <div class="stat-item">
                                            <h3>{"Taux de frais moyen"}</h3>
                                            <p>{format!("{} ppm", node_stats.mean_outbound_fee_rate)}</p>
                                        </div>
                                        <div class="stat-item">
                                            <h3>{"Frais de base médian"}</h3>
                                            <p>{format!("{} msats", node_stats.median_outbound_base_fee)}</p>
                                        </div>
                                        <div class="stat-item">
                                            <h3>{"Taux de frais médian"}</h3>
                                            <p>{format!("{} ppm", node_stats.median_outbound_fee_rate)}</p>
                                        </div>
                                    </div>
                                </Card>

                                if let Some(liquidity) = &*liquidity {
                                    <Card title="Valeur de la liquidité sortante">
                                        <div class="liquidity-grid">
                                            {for liquidity.channel_peers.iter().map(|peer| {
                                                html! {
                                                    <div class="peer-item">
                                                        <h4>{format!("Nœud: {}", peer.pubkey)}</h4>
                                                        <div class="ppm-values">
                                                            {for peer.outbound_ppm_value.iter().map(|value| {
                                                                html! {
                                                                    <div class="ppm-value">
                                                                        <span>{"Valeur PPM: "}</span>
                                                                        <span>{format!("{:.2}", value.value)}</span>
                                                                    </div>
                                                                }
                                                            })}
                                                        </div>
                                                    </div>
                                                }
                                            })}
                                        </div>
                                    </Card>
                                }

                                if let Some(fees) = &*fees {
                                    <Card title="Frais suggérés">
                                        <div class="fees-grid">
                                            <div class="fee-item">
                                                <h3>{"Frais de base suggérés"}</h3>
                                                <p>{format!("{} msats", fees.base_fee)}</p>
                                            </div>
                                            <div class="fee-item">
                                                <h3>{"Taux de frais suggérés"}</h3>
                                                <p>{format!("{} ppm", fees.fee_rate)}</p>
                                            </div>
                                        </div>
                                    </Card>
                                }
                            </div>
                        }
                    </div>
                }
            }
        </div>
    }
}

#[function_component(ChannelsPage)]
pub fn channels_page() -> Html {
    let channels = use_state(|| None::<Vec<Channel>>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let channels = channels.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    error.set(None);
                    match fetch_channels().await {
                        Ok(channels_data) => {
                            channels.set(Some(channels_data));
                            error.set(None);
                        }
                        Err(e) => {
                            error.set(Some(format!("Erreur lors de la récupération des canaux : {}", e)));
                        }
                    }
                    loading.set(false);
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
                
                if *loading {
                    <div class="loading">{"Chargement des canaux..."}</div>
                } else if let Some(err) = &*error {
                    <div class="error">
                        <p>{err}</p>
                        <Button onclick={Callback::from(|_| {
                            let channels = channels.clone();
                            let error = error.clone();
                            let loading = loading.clone();

                            wasm_bindgen_futures::spawn_local(async move {
                                loading.set(true);
                                error.set(None);
                                match fetch_channels().await {
                                    Ok(channels_data) => {
                                        channels.set(Some(channels_data));
                                        error.set(None);
                                    }
                                    Err(e) => {
                                        error.set(Some(format!("Erreur lors de la récupération des canaux : {}", e)));
                                    }
                                }
                                loading.set(false);
                            });
                        })}>
                            {"Réessayer"}
                        </Button>
                    </div>
                } else if let Some(channels_data) = &*channels {
                    <div class="channels-summary">
                        <div class="summary-item">
                            <h3>{"Nombre total de canaux"}</h3>
                            <p>{channels_data.len()}</p>
                        </div>
                        <div class="summary-item">
                            <h3>{"Capacité totale"}</h3>
                            <p>{format!("{} sats", channels_data.iter().map(|c| c.capacity).sum::<u64>())}</p>
                        </div>
                        <div class="summary-item">
                            <h3>{"Balance totale locale"}</h3>
                            <p>{format!("{} sats", channels_data.iter().map(|c| c.local_balance).sum::<u64>())}</p>
                        </div>
                        <div class="summary-item">
                            <h3>{"Balance totale distante"}</h3>
                            <p>{format!("{} sats", channels_data.iter().map(|c| c.remote_balance).sum::<u64>())}</p>
                        </div>
                    </div>

                    <div class="channels-grid">
                        {for channels_data.iter().map(|channel| {
                            let balance_ratio = if channel.capacity > 0 {
                                (channel.local_balance as f64 / channel.capacity as f64) * 100.0
                            } else {
                                0.0
                            };

                            html! {
                                <Card title={format!("Canal avec {}", channel.pubkey)}>
                                    <div class="channel-info">
                                        <div class="channel-header">
                                            <div class="channel-status">
                                                <span class={format!("status-indicator status-{}", 
                                                    if channel.active { "active" } else { "inactive" }
                                                )}></span>
                                                <span>{if channel.active { "Actif" } else { "Inactif" }}</span>
                                            </div>
                                            <div class="channel-balance">
                                                <div class="balance-bar">
                                                    <div class="balance-fill" style={format!("width: {}%", balance_ratio)}></div>
                                                </div>
                                                <div class="balance-labels">
                                                    <span>{format!("{}%", balance_ratio)}</span>
                                                    <span>{format!("{} / {}", channel.local_balance, channel.capacity)}</span>
                                                </div>
                                            </div>
                                        </div>

                                        <div class="info-grid">
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
            }
        </div>
    }
}

#[derive(Clone, PartialEq)]
enum SortBy {
    YieldRate,
    DailyYield,
    Capacity,
    FeeRate,
}

#[derive(Clone, PartialEq)]
enum FilterStatus {
    All,
    Active,
    Inactive,
}

#[function_component(YieldsPage)]
pub fn yields_page() -> Html {
    let channels = use_state(|| None::<Vec<Channel>>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let sort_by = use_state(|| SortBy::YieldRate);
    let filter_status = use_state(|| FilterStatus::All);
    let time_range = use_state(|| "7d".to_string());

    {
        let channels = channels.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    error.set(None);
                    match fetch_channels().await {
                        Ok(channels_data) => {
                            channels.set(Some(channels_data));
                            error.set(None);
                        }
                        Err(e) => {
                            error.set(Some(format!("Erreur lors de la récupération des canaux : {}", e)));
                        }
                    }
                    loading.set(false);
                });
                || ()
            },
            (),
        );
    }

    let filtered_and_sorted_channels = channels.as_ref().map(|channels_data| {
        let mut filtered = channels_data.clone();
        
        // Appliquer le filtre de statut
        filtered = filtered.into_iter()
            .filter(|c| match *filter_status {
                FilterStatus::All => true,
                FilterStatus::Active => c.active,
                FilterStatus::Inactive => !c.active,
            })
            .collect();

        // Trier selon le critère sélectionné
        filtered.sort_by(|a, b| {
            match *sort_by {
                SortBy::YieldRate => {
                    let yield_a = if a.capacity > 0 {
                        (a.fee_rate as f64 * a.capacity as f64 / 1_000_000.0 / a.capacity as f64) * 100.0
                    } else { 0.0 };
                    let yield_b = if b.capacity > 0 {
                        (b.fee_rate as f64 * b.capacity as f64 / 1_000_000.0 / b.capacity as f64) * 100.0
                    } else { 0.0 };
                    yield_b.partial_cmp(&yield_a).unwrap_or(std::cmp::Ordering::Equal)
                },
                SortBy::DailyYield => {
                    let yield_a = a.fee_rate as f64 * a.capacity as f64 / 1_000_000.0;
                    let yield_b = b.fee_rate as f64 * b.capacity as f64 / 1_000_000.0;
                    yield_b.partial_cmp(&yield_a).unwrap_or(std::cmp::Ordering::Equal)
                },
                SortBy::Capacity => b.capacity.cmp(&a.capacity),
                SortBy::FeeRate => b.fee_rate.cmp(&a.fee_rate),
            }
        });

        filtered
    });

    // Données pour les graphiques
    let yield_data = filtered_and_sorted_channels.as_ref().map(|channels| {
        channels.iter()
            .take(10)
            .map(|c| {
                let daily_yield = c.fee_rate as f64 * c.capacity as f64 / 1_000_000.0;
                (format!("Canal {}", c.pubkey[..8].to_string()), daily_yield)
            })
            .collect::<Vec<_>>()
    });

    html! {
        <div class="page">
            <Navbar current_page={"yields".to_string()} />
            <div class="content">
                <h1>{"Rendements des Canaux"}</h1>
                
                if *loading {
                    <div class="loading">{"Chargement des données de rendement..."}</div>
                } else if let Some(err) = &*error {
                    <div class="error">
                        <p>{err}</p>
                        <Button onclick={Callback::from(|_| {
                            let channels = channels.clone();
                            let error = error.clone();
                            let loading = loading.clone();

                            wasm_bindgen_futures::spawn_local(async move {
                                loading.set(true);
                                error.set(None);
                                match fetch_channels().await {
                                    Ok(channels_data) => {
                                        channels.set(Some(channels_data));
                                        error.set(None);
                                    }
                                    Err(e) => {
                                        error.set(Some(format!("Erreur lors de la récupération des canaux : {}", e)));
                                    }
                                }
                                loading.set(false);
                            });
                        })}>
                            {"Réessayer"}
                        </Button>
                    </div>
                } else if let Some(channels_data) = &*channels {
                    // Calcul des métriques de rendement
                    let total_yield: f64 = channels_data.iter()
                        .map(|c| c.fee_rate as f64 * c.capacity as f64 / 1_000_000.0)
                        .sum();
                    
                    let avg_yield_rate: f64 = if !channels_data.is_empty() {
                        total_yield / channels_data.iter().map(|c| c.capacity as f64).sum::<f64>() * 100.0
                    } else {
                        0.0
                    };

                    <div class="yields-summary">
                        <div class="summary-item">
                            <h3>{"Rendement total estimé"}</h3>
                            <p>{format!("{:.2} sats/jour", total_yield)}</p>
                        </div>
                        <div class="summary-item">
                            <h3>{"Taux de rendement moyen"}</h3>
                            <p>{format!("{:.2}%", avg_yield_rate)}</p>
                        </div>
                        <div class="summary-item">
                            <h3>{"Nombre de canaux actifs"}</h3>
                            <p>{channels_data.iter().filter(|c| c.active).count()}</p>
                        </div>
                        <div class="summary-item">
                            <h3>{"Capacité totale"}</h3>
                            <p>{format!("{} sats", channels_data.iter().map(|c| c.capacity).sum::<u64>())}</p>
                        </div>
                    </div>

                    <div class="yields-filters">
                        <div class="filter-group">
                            <label>{"Trier par:"}</label>
                            <select 
                                value={match *sort_by {
                                    SortBy::YieldRate => "yield_rate",
                                    SortBy::DailyYield => "daily_yield",
                                    SortBy::Capacity => "capacity",
                                    SortBy::FeeRate => "fee_rate",
                                }}
                                onchange={Callback::from(move |e: Event| {
                                    let select = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                                    sort_by.set(match select.value().as_str() {
                                        "yield_rate" => SortBy::YieldRate,
                                        "daily_yield" => SortBy::DailyYield,
                                        "capacity" => SortBy::Capacity,
                                        "fee_rate" => SortBy::FeeRate,
                                        _ => SortBy::YieldRate,
                                    });
                                })}
                            >
                                <option value="yield_rate">{"Taux de rendement"}</option>
                                <option value="daily_yield">{"Rendement quotidien"}</option>
                                <option value="capacity">{"Capacité"}</option>
                                <option value="fee_rate">{"Taux de frais"}</option>
                            </select>
                        </div>

                        <div class="filter-group">
                            <label>{"Statut:"}</label>
                            <select 
                                value={match *filter_status {
                                    FilterStatus::All => "all",
                                    FilterStatus::Active => "active",
                                    FilterStatus::Inactive => "inactive",
                                }}
                                onchange={Callback::from(move |e: Event| {
                                    let select = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                                    filter_status.set(match select.value().as_str() {
                                        "all" => FilterStatus::All,
                                        "active" => FilterStatus::Active,
                                        "inactive" => FilterStatus::Inactive,
                                        _ => FilterStatus::All,
                                    });
                                })}
                            >
                                <option value="all">{"Tous"}</option>
                                <option value="active">{"Actifs"}</option>
                                <option value="inactive">{"Inactifs"}</option>
                            </select>
                        </div>

                        <div class="filter-group">
                            <label>{"Période:"}</label>
                            <select 
                                value={(*time_range).clone()}
                                onchange={Callback::from(move |e: Event| {
                                    let select = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                                    time_range.set(select.value());
                                })}
                            >
                                <option value="24h">{"24 heures"}</option>
                                <option value="7d">{"7 jours"}</option>
                                <option value="30d">{"30 jours"}</option>
                            </select>
                        </div>
                    </div>

                    <div class="yields-charts">
                        if let Some(data) = yield_data {
                            <div class="chart-grid">
                                <Card title="Top 10 des canaux par rendement quotidien">
                                    <YieldChart 
                                        data={data.clone()}
                                        title="Rendement quotidien (sats)".to_string()
                                        color="#3498db".to_string()
                                    />
                                </Card>
                                <Card title="Évolution des rendements">
                                    <YieldChart 
                                        data={data}
                                        title="Rendement cumulé (sats)".to_string()
                                        color="#2ecc71".to_string()
                                    />
                                </Card>
                            </div>
                        }
                    </div>

                    <div class="yields-grid">
                        {for filtered_and_sorted_channels.iter().flatten().map(|channel| {
                            let daily_yield = channel.fee_rate as f64 * channel.capacity as f64 / 1_000_000.0;
                            let yield_rate = if channel.capacity > 0 {
                                (daily_yield / channel.capacity as f64) * 100.0
                            } else {
                                0.0
                            };

                            html! {
                                <Card title={format!("Canal avec {}", channel.pubkey)}>
                                    <div class="yield-info">
                                        <div class="yield-header">
                                            <div class="yield-status">
                                                <span class={format!("status-indicator status-{}", 
                                                    if channel.active { "active" } else { "inactive" }
                                                )}></span>
                                                <span>{if channel.active { "Actif" } else { "Inactif" }}</span>
                                            </div>
                                            <div class="yield-rate">
                                                <span class="rate-label">{"Taux de rendement"}</span>
                                                <span class="rate-value">{format!("{:.2}%", yield_rate)}</span>
                                            </div>
                                        </div>

                                        <div class="yield-details">
                                            <div class="detail-item">
                                                <span class="detail-label">{"Rendement quotidien"}</span>
                                                <span class="detail-value">{format!("{:.2} sats", daily_yield)}</span>
                                            </div>
                                            <div class="detail-item">
                                                <span class="detail-label">{"Capacité"}</span>
                                                <span class="detail-value">{format!("{} sats", channel.capacity)}</span>
                                            </div>
                                            <div class="detail-item">
                                                <span class="detail-label">{"Taux de frais"}</span>
                                                <span class="detail-value">{format!("{} ppm", channel.fee_rate)}</span>
                                            </div>
                                            <div class="detail-item">
                                                <span class="detail-label">{"Frais de base"}</span>
                                                <span class="detail-value">{format!("{} msats", channel.base_fee)}</span>
                                            </div>
                                        </div>

                                        <div class="yield-balance">
                                            <div class="balance-bar">
                                                <div class="balance-fill" style={format!("width: {}%", 
                                                    if channel.capacity > 0 {
                                                        (channel.local_balance as f64 / channel.capacity as f64) * 100.0
                                                    } else {
                                                        0.0
                                                    }
                                                )}></div>
                                            </div>
                                            <div class="balance-labels">
                                                <span>{"Balance locale"}</span>
                                                <span>{format!("{} / {}", channel.local_balance, channel.capacity)}</span>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="yield-actions">
                                        <Button 
                                            label={"Optimiser les frais".to_string()}
                                            onclick={Callback::from(|_| {})}
                                        />
                                        <Button 
                                            label={"Voir l'historique".to_string()}
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