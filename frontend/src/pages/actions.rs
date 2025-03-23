use yew::prelude::*;
use crate::components::{Navbar, Card, Button};
use crate::types::{Action, ActionType, ActionStatus, NodeStats, Channel};
use crate::services::{fetch_actions, execute_action, fetch_node_stats, fetch_channels, get_ai_actions};

const FEUSTEY_PUBKEY: &str = "0296b2db342fcf87ea94d981757fdf4d3e545bd5cef4919f58b5d38dfdd73bf5c9";

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
}

#[function_component(ActionsPage)]
pub fn actions() -> Html {
    let actions = use_state(Vec::new);
    let node_stats = use_state(|| None::<NodeStats>);
    let channels = use_state(|| Vec::new());
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);
    let ai_loading = use_state(|| false);

    // Charger les actions existantes
    {
        let actions = actions.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match fetch_actions().await {
                    Ok(fetched_actions) => {
                        actions.set(fetched_actions);
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(format!("Erreur lors de la récupération des actions : {}", e)));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    // Charger les données du nœud et des canaux
    {
        let node_stats = node_stats.clone();
        let channels = channels.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Récupérer les stats du nœud
                match fetch_node_stats().await {
                    Ok(stats) => {
                        node_stats.set(Some(stats));
                    }
                    Err(e) => {
                        error.set(Some(format!("Erreur lors de la récupération des statistiques : {}", e)));
                    }
                }

                // Récupérer les canaux
                match fetch_channels().await {
                    Ok(fetched_channels) => {
                        channels.set(fetched_channels);
                    }
                    Err(e) => {
                        error.set(Some(format!("Erreur lors de la récupération des canaux : {}", e)));
                    }
                }
            });
            || ()
        });
    }

    let on_get_ai_suggestions = {
        let node_stats = node_stats.clone();
        let channels = channels.clone();
        let actions = actions.clone();
        let error = error.clone();
        let ai_loading = ai_loading.clone();

        Callback::from(move |_| {
            let node_stats = node_stats.clone();
            let channels = channels.clone();
            let actions = actions.clone();
            let error = error.clone();
            let ai_loading = ai_loading.clone();

            if let Some(stats) = &*node_stats {
                ai_loading.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    match get_ai_actions(stats, &channels).await {
                        Ok(suggested_actions) => {
                            // Ajouter les actions suggérées à la liste existante
                            actions.set(suggested_actions.into_iter().chain((*actions).clone()).collect());
                            error.set(None);
                        }
                        Err(e) => {
                            error.set(Some(format!("Erreur lors de la génération des suggestions : {}", e)));
                        }
                    }
                    ai_loading.set(false);
                });
            }
        })
    };

    let on_new_action = {
        let actions = actions.clone();
        Callback::from(move |action_type: ActionType| {
            let actions = actions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(new_action) = execute_action(action_type, 1, "Action créée manuellement").await {
                    actions.set([new_action].into_iter().chain((*actions).clone()).collect());
                }
            });
        })
    };

    html! {
        <div class="min-h-screen bg-dark">
            <Navbar />
            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <div class="px-4 py-6 sm:px-0">
                    <div class="flex justify-between items-center mb-6">
                        <h1 class="text-2xl font-bold text-white">{"Actions"}</h1>
                        <Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| {})}>
                            {"Nouvelle action"}
                        </Button>
                    </div>
                    <Card title="Default Title">
                        if *loading {
                            <div class="flex justify-center items-center h-32">
                                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-white"></div>
                            </div>
                        } else if let Some(err) = &*error {
                            <div class="text-red-500 text-center py-4">{err}</div>
                        } else if actions.is_empty() {
                            <div class="text-gray-400 text-center py-4">{"Aucune action disponible"}</div>
                        } else {
                            <div class="overflow-x-auto">
                                <table class="min-w-full divide-y divide-gray-700">
                                    <thead>
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Type"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Description"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Priorité"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Impact"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Statut"}</th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">{"Actions"}</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-gray-700">
                                        {actions.iter().map(|action| {
                                            html! {
                                                <tr key={action.id.clone()}>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-white">{&action.action_type}</td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">{&action.description}</td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">{action.priority}</td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">{&action.impact}</td>
                                                    <td class="px-6 py-4 whitespace-nowrap">
                                                        <span class={classes!(
                                                            "px-2 inline-flex text-xs leading-5 font-semibold rounded-full",
                                                            match action.status.as_str() {
                                                                "pending" => "bg-yellow-100 text-yellow-800",
                                                                "completed" => "bg-green-100 text-green-800",
                                                                "failed" => "bg-red-100 text-red-800",
                                                                _ => "bg-gray-100 text-gray-800",
                                                            }
                                                        )}>
                                                            {&action.status}
                                                        </span>
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                                                        <Button variant={ButtonVariant::Secondary} onclick={Callback::from(|_| {})}>
                                                            {"Voir les détails"}
                                                        </Button>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Html>()}
                                    </tbody>
                                </table>
                            </div>
                        }
                    </Card>
                </div>
            </main>
        </div>
    }
} 