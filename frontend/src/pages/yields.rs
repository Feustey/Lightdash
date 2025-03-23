use yew::prelude::*;
use crate::components::{Navbar, Card};
use crate::types::NodeStats;
use crate::services::fetch_node_stats;

#[function_component(YieldsPageComponent)]
pub fn yields_page() -> Html {
    let node_stats = use_state(|| None::<NodeStats>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    {
        let node_stats = node_stats.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with(
            (),
            move |_| {
                let node_stats = node_stats.clone();
                let error = error.clone();
                let loading = loading.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    match fetch_node_stats().await {
                        Ok(stats) => {
                            node_stats.set(Some(stats));
                            error.set(None);
                        }
                        Err(e) => {
                            error.set(Some(e.to_string()));
                            node_stats.set(None);
                        }
                    }
                    loading.set(false);
                });
                || ()
            },
        );
    }

    html! {
        <div class="container mx-auto px-4 py-8">
            <Navbar current_page="yields" />
            <div class="mt-8">
                <h1 class="text-3xl font-bold mb-6">{"Rendements"}</h1>
                
                if *loading {
                    <div class="text-center">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"></div>
                    </div>
                } else if let Some(err) = &*error {
                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                        <strong class="font-bold">{"Erreur !"}</strong>
                        <span class="block sm:inline">{" "}{err}</span>
                    </div>
                } else if let Some(stats) = &*node_stats {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <Card title="Statistiques du nœud" class="h-full">
                            <div class="space-y-4">
                                <div>
                                    <span class="font-semibold">{"Capacité totale :"}</span>
                                    <span class="ml-2">{format!("{:.2} sats", stats.total_capacity as f64 / 100_000_000.0)}</span>
                                </div>
                                <div>
                                    <span class="font-semibold">{"Nombre de canaux :"}</span>
                                    <span class="ml-2">{stats.num_channels}</span>
                                </div>
                                <div>
                                    <span class="font-semibold">{"Balance locale :"}</span>
                                    <span class="ml-2">{format!("{:.2} sats", stats.total_local_balance as f64 / 100_000_000.0)}</span>
                                </div>
                                <div>
                                    <span class="font-semibold">{"Balance distante :"}</span>
                                    <span class="ml-2">{format!("{:.2} sats", stats.total_remote_balance as f64 / 100_000_000.0)}</span>
                                </div>
                            </div>
                        </Card>
                    </div>
                }
            </div>
        </div>
    }
}