use yew::prelude::*;
use crate::components::{Navbar, Card, Button, SearchInput};
use crate::types::NodeStats;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
}

#[function_component(AlbyPage)]
pub fn alby() -> Html {
    let stats = use_state(|| NodeStats {
        local_balance: 0.0,
        remote_balance: 0.0,
        total_capacity: 0,
        num_channels: 0,
        pubkey: String::new(),
        alias: String::new(),
        avg_channel_size: 0,
        uptime_percentage: 0.0,
    });
    let search_query = use_state(String::new);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |value: String| {
            search_query.set(value);
        })
    };

    html! {
        <div class="min-h-screen bg-dark">
            <Navbar />
            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <div class="px-4 py-6 sm:px-0">
                    <div class="flex justify-between items-center mb-6">
                        <h1 class="text-2xl font-bold text-white">{"Alby"}</h1>
                        <Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| {})}>
                            {"Connecter un nœud"}
                        </Button>
                    </div>
                    <Card title="Default Title">
                        <div class="mb-6">
                            <SearchInput
                                on_search={on_search}
                                placeholder="Entrez la clé publique du nœud"
                            />
                        </div>
                        if *loading {
                            <div class="flex justify-center items-center h-32">
                                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-white"></div>
                            </div>
                        } else if let Some(err) = &*error {
                            <div class="text-red-500 text-center py-4">{err}</div>
                        } else {
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div class="bg-dark-lighter p-4 rounded-lg">
                                    <h3 class="text-lg font-semibold text-white mb-2">{"Balance locale"}</h3>
                                    <p class="text-2xl font-bold text-white">{format!("{:.2} sats", stats.local_balance)}</p>
                                </div>
                                <div class="bg-dark-lighter p-4 rounded-lg">
                                    <h3 class="text-lg font-semibold text-white mb-2">{"Balance distante"}</h3>
                                    <p class="text-2xl font-bold text-white">{format!("{:.2} sats", stats.remote_balance)}</p>
                                </div>
                                <div class="bg-dark-lighter p-4 rounded-lg">
                                    <h3 class="text-lg font-semibold text-white mb-2">{"Capacité totale"}</h3>
                                    <p class="text-2xl font-bold text-white">{format!("{:.2} sats", stats.total_capacity)}</p>
                                </div>
                                <div class="bg-dark-lighter p-4 rounded-lg">
                                    <h3 class="text-lg font-semibold text-white mb-2">{"Canaux actifs"}</h3>
                                    <p class="text-2xl font-bold text-white">{format!("{}/{}", stats.num_channels, stats.num_channels)}</p>
                                </div>
                            </div>
                        }
                    </Card>
                </div>
            </main>
        </div>
    }
} 