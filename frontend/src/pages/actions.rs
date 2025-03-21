use yew::prelude::*;
use yew::html;
use wasm_bindgen_futures::spawn_local;
use crate::components::{Navbar, Button, Card};
use crate::types::{Action, NodeStats};
use crate::services::{fetch_node_stats, fetch_actions, execute_action};

const PREMIUM_PRICE: u64 = 10_000;

#[derive(Clone, PartialEq)]
enum Tab {
    Overview,
    Actions,
    Settings,
}

#[derive(Properties, PartialEq)]
pub struct ActionsPageProps {}

#[function_component(ActionsPageComponent)]
pub fn actions() -> Html {
    let actions = use_state(|| None::<Vec<Action>>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let actions = actions.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let actions = actions.clone();
            let error = error.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match fetch_actions().await {
                    Ok(actions_data) => actions.set(Some(actions_data)),
                    Err(e) => error.set(Some(e.to_string())),
                }

                loading.set(false);
            });
            || ()
        });
    }

    let on_execute_action = {
        let actions = actions.clone();
        let error = error.clone();
        let loading = loading.clone();

        Callback::from(move |action_id: String| {
            let actions = actions.clone();
            let error = error.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match execute_action(&action_id).await {
                    Ok(_) => {
                        if let Ok(actions_data) = fetch_actions().await {
                            actions.set(Some(actions_data));
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }

                loading.set(false);
            });
        })
    };

    html! {
        <div class="container">
            <div class="columns">
                <div class="column">
                    <Card class={"actions-card".to_string()} title={"Actions".to_string()}>
                        <div class="actions-content">
                            if *loading {
                                <div class="loading">{"Chargement des actions..."}</div>
                            } else if let Some(err) = &*error {
                                <div class="error">
                                    <p>{err}</p>
                                    <Button onclick={Callback::from(move |_| {
                                        let actions = actions.clone();
                                        let error = error.clone();
                                        let loading = loading.clone();

                                        wasm_bindgen_futures::spawn_local(async move {
                                            loading.set(true);
                                            error.set(None);
                                            match fetch_actions().await {
                                                Ok(actions_data) => actions.set(Some(actions_data)),
                                                Err(e) => error.set(Some(e.to_string())),
                                            }
                                            loading.set(false);
                                        });
                                    })}>
                                        {"Réessayer"}
                                    </Button>
                                </div>
                            } else if let Some(actions_data) = &*actions {
                                <div class="actions-table">
                                    <table>
                                        <thead>
                                            <tr>
                                                <th>{"ID"}</th>
                                                <th>{"Type"}</th>
                                                <th>{"Statut"}</th>
                                                <th>{"Date"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                        {actions_data.iter().map(|action| {
                            html! {
                                <tr key={action.id.clone()}>
                                    <td>{action.id.clone()}</td>
                                    <td>{action.type_.to_string()}</td>
                                    <td>{action.status.to_string()}</td>
                                    <td>{action.created_at.to_string()}</td>
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