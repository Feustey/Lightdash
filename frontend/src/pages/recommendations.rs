use yew::prelude::*;
use yew::html;
use wasm_bindgen_futures::spawn_local;
use crate::components::{Navbar, Card, Button};
use crate::types::ChannelRecommendation;
use crate::services::fetch_channel_recommendations;

#[derive(Properties, PartialEq)]
pub struct RecommendationsPageProps {}

#[function_component(RecommendationsPageComponent)]
pub fn recommendations() -> Html {
    let recommendations = use_state(|| None::<Vec<ChannelRecommendation>>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let recommendations = recommendations.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let recommendations = recommendations.clone();
            let error = error.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match fetch_channel_recommendations().await {
                    Ok(recs) => recommendations.set(Some(recs)),
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
                    <Card class={"recommendations-card".to_string()} title={"Recommandations".to_string()}>
                        <div class="recommendations-content">
                            if *loading {
                                <div class="loading">{"Chargement des recommandations..."}</div>
                            } else if let Some(err) = &*error {
                                <div class="error">
                                    <p>{err}</p>
                                    <Button onclick={Callback::from(move |_| {
                                        let recommendations = recommendations.clone();
                                        let error = error.clone();
                                        let loading = loading.clone();

                                        wasm_bindgen_futures::spawn_local(async move {
                                            loading.set(true);
                                            error.set(None);
                                            match fetch_channel_recommendations().await {
                                                Ok(recs) => recommendations.set(Some(recs)),
                                                Err(e) => error.set(Some(e.to_string())),
                                            }
                                            loading.set(false);
                                        });
                                    })}>
                                        {"Réessayer"}
                                    </Button>
                                </div>
                            } else if let Some(recs) = &*recommendations {
                                <div class="recommendations-table">
                                    <table>
                                        <thead>
                                            <tr>
                                                <th>{"Nœud"}</th>
                                                <th>{"Capacité"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                        {recs.iter().map(|rec| {
                            html! {
                                <tr key={rec.node_pubkey.clone()}>
                                    <td>{rec.node_alias.clone()}</td>
                                    <td>{rec.suggested_size.to_string()}</td>
                                    <td>{rec.confidence_score.to_string()}</td>
                                    <td>{rec.node_pubkey.clone()}</td>
                                </tr>
                            }
                        }).collect::<Html>()}
                                        </tbody>
                                    </table>
                                </div>
                            }
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
} 