use yew::prelude::*;
use yew::html;
use wasm_bindgen_futures::spawn_local;
use crate::components::{NavbarComponent as Navbar, CardComponent as Card, ChartComponent as Chart, Button};
use crate::types::{OutboundLiquidityValue, SuggestedFees, NodeStats};
use crate::services::{fetch_outbound_liquidity_value, fetch_suggested_fees, fetch_node_stats};

#[derive(Properties, PartialEq)]
pub struct YieldsPageProps {}

#[function_component(YieldsPageComponent)]
pub fn yields() -> Html {
    let node_stats = use_state(|| None::<NodeStats>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let node_stats = node_stats.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let node_stats = node_stats.clone();
            let error = error.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match fetch_node_stats().await {
                    Ok(stats) => node_stats.set(Some(stats)),
                    Err(e) => error.set(Some(e.to_string())),
                }

                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="container">
            <div class="columns">
                <div class="column">
                    <Card class={"yields-card".to_string()} title={"Rendements".to_string()}>
                        <div class="yields-content">
                            if *loading {
                                <div class="loading">{"Chargement des rendements..."}</div>
                            } else if let Some(err) = &*error {
                                <div class="error">
                                    <p>{err}</p>
                                    <Button onclick={Callback::from(move |_| {
                                        let node_stats = node_stats.clone();
                                        let error = error.clone();
                                        let loading = loading.clone();

                                        wasm_bindgen_futures::spawn_local(async move {
                                            loading.set(true);
                                            error.set(None);
                                            match fetch_node_stats().await {
                                                Ok(stats) => node_stats.set(Some(stats)),
                                                Err(e) => error.set(Some(e.to_string())),
                                            }
                                            loading.set(false);
                                        });
                                    })}>
                                        {"Réessayer"}
                                    </Button>
                                </div>
                            } else if let Some(stats) = &*node_stats {
                                <div class="yields-grid">
                                    <div class="yield-card">
                                        <h3>{"Rendement 30 jours"}</h3>
                                        <p class="yield-value">{format!("{:.2}%", stats.yield_30d)}</p>
                                    </div>
                                    <div class="yield-card">
                                        <h3>{"Capacité totale"}</h3>
                                        <p class="yield-value">{format!("{} sats", stats.capacity)}</p>
                                    </div>
                        {recs.iter().map(|rec| {
                            html! {
                                <tr key={rec.node_pubkey.clone()}>
                                    <td>{rec.alias.clone()}</td>
                                    <td>{rec.capacity.to_string()}</td>
                                    <td>{rec.channel_count.to_string()}</td>
                                    <td>{"rec.node_pubkey"}</td>
                                </tr>
                            }
                        }).collect::<Html>()}
                </div>
                <div class="column">
                    <Card class={"stats-card".to_string()} title={"Statistiques".to_string()}>
                        <div class="stats-content">
                            <p>{"Rendement total : 0.5%"}</p>
                            <p>{"Rendement moyen : 0.1%"}</p>
                            <p>{"Nombre de transactions : 100"}</p>
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
} 
}
