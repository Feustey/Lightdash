use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::components::{Navbar, Card, Button, SearchInput};
use crate::types::{Channel, ChannelStatus};
use crate::services::fetch_channels;

#[function_component(ChannelsPage)]
pub fn channels() -> Html {
    let channels = use_state(Vec::new);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let search_query = use_state(String::new);

    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |value: String| {
            search_query.set(value);
        })
    };

    let filtered_channels = channels
        .iter()
        .filter(|channel: &&Channel| {
            search_query.is_empty()
                || channel.pubkey.to_lowercase().contains(&search_query.to_lowercase())
        })
        .collect::<Vec<_>>();

    html! {
        <div class="min-h-screen bg-dark">
            <Navbar current_page={AppRoute::Channels} />
            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <div class="px-4 py-6 sm:px-0">
                    <div class="flex justify-between items-center mb-6">
                        <h1 class="text-2xl font-bold text-white">{"Canaux"}</h1>
                        <Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| {})}>
                            {"Nouveau canal"}
                        </Button>
                    </div>
                    <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
                        <Card title="Rechercher un canal">
                            <SearchInput
                                value={(*search_query).clone()}
                                on_search={on_search}
                                placeholder="Entrez l'alias ou la clé publique du nœud"
                            />
                        </Card>
                        <Card title="Liste des canaux">
                            if *loading {
                                <div class="flex justify-center items-center h-32">
                                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-white"></div>
                                </div>
                            } else if let Some(err) = &*error {
                                <div class="text-red-400 text-center">{err}</div>
                            } else if filtered_channels.is_empty() {
                                <div class="text-gray-400 text-center">{"Aucun canal trouvé"}</div>
                            } else {
                                <div class="overflow-x-auto">
                                    <table class="min-w-full divide-y divide-gray-700">
                                        <thead>
                                            <tr>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                                    {"ID"}
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                                    {"Nœud distant"}
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                                    {"Capacité"}
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                                    {"Balance locale"}
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                                    {"Balance distante"}
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                                    {"Statut"}
                                                </th>
                                                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                                    {"Actions"}
                                                </th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-gray-700">
                                            {filtered_channels.iter().map(|channel| {
                                                let status_class = match channel.status {
                                                    ChannelStatus::Active => "text-green-400",
                                                    ChannelStatus::Inactive => "text-red-400",
                                                    ChannelStatus::Pending => "text-yellow-400",
                                                };
                                                html! {
                                                    <tr key={channel.id.clone()} class="hover:bg-dark-lighter transition-colors duration-200">
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-white font-mono">
                                                            {channel.id.clone()}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                                                            {channel.pubkey.clone()}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                                                            {format!("{:.2} sats", channel.capacity)}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                                                            {format!("{:.2} sats", channel.local_balance)}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                                                            {format!("{:.2} sats", channel.remote_balance)}
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm">
                                                            <span class={status_class}>
                                                                {format!("{:?}", channel.status)}
                                                            </span>
                                                        </td>
                                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                                                            <Button variant={ButtonVariant::Secondary} onclick={Callback::from(|_| {})}>
                                                                {"Détails"}
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
                </div>
            </main>
        </div>
    }
}

#[derive(PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
}