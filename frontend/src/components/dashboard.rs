use yew::prelude::*;
use crate::services::api::ApiService;
use crate::models::SparkSeerStats;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, Clone, PartialEq)]
pub struct DashboardProps {
    pub api_service: ApiService,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let stats = use_state(|| None::<SparkSeerStats>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let stats = stats.clone();
        let loading = loading.clone();
        let error = error.clone();
        let api_service = props.api_service.clone();

        use_effect_with_deps(
            move |_| {
                loading.set(true);
                spawn_local(async move {
                    match api_service.get_node_health().await {
                        Ok(data) => {
                            stats.set(Some(data));
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e.as_string().unwrap_or_else(|| "Une erreur est survenue".to_string())));
                            loading.set(false);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="bg-white shadow rounded-lg p-6">
            <h2 class="text-2xl font-bold mb-4">{"État du nœud"}</h2>
            {
                if *loading {
                    html! {
                        <div class="flex justify-center items-center h-32">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
                        </div>
                    }
                } else if let Some(error_msg) = (*error).clone() {
                    html! {
                        <div class="text-red-500 text-center">
                            {error_msg}
                        </div>
                    }
                } else if let Some(stats) = (*stats).clone() {
                    html! {
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <div class="bg-gray-50 p-4 rounded">
                                <h3 class="text-lg font-semibold mb-2">{"Capacité totale"}</h3>
                                <p class="text-2xl">{format!("{} sats", stats.total_capacity)}</p>
                            </div>
                            <div class="bg-gray-50 p-4 rounded">
                                <h3 class="text-lg font-semibold mb-2">{"Nombre de canaux"}</h3>
                                <p class="text-2xl">{stats.num_channels}</p>
                            </div>
                            <div class="bg-gray-50 p-4 rounded">
                                <h3 class="text-lg font-semibold mb-2">{"Score de santé"}</h3>
                                <p class="text-2xl">{format!("{}%", stats.health_score)}</p>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-gray-500 text-center">
                            {"Aucune donnée disponible"}
                        </div>
                    }
                }
            }
        </div>
    }
} 