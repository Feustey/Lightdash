use yew::prelude::*;
use crate::components::{Navbar, Card, Button, SearchInput};
use crate::types::ChannelRecommendation;

#[function_component(RecommendationsPage)]
pub fn recommendations() -> Html {
    let recommendations = use_state(Vec::new);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let search_query = use_state(String::new);

    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |query: String| {
            search_query.set(query);
        })
    };

    let filtered_recommendations = recommendations
        .iter()
        .filter(|rec: &&ChannelRecommendation| {
            search_query.is_empty()
                || rec.alias.to_lowercase().contains(&search_query.to_lowercase())
                || rec.pubkey.to_lowercase().contains(&search_query.to_lowercase())
        })
        .collect::<Vec<_>>();

    html! {
        <div class="flex flex-col min-h-screen bg-dark">
            <Navbar />
            <main class="flex-grow container mx-auto px-4 py-8">
                <div class="space-y-6">
                    <div class="flex justify-between items-center">
                        <h1 class="text-2xl font-bold text-white">{"Recommandations"}</h1>
                        <Button variant="primary" onclick={Callback::from(|_| {})}>
                            {"Actualiser"}
                        </Button>
                    </div>

                    <Card title="Recherche de recommandations">
                        <div class="space-y-4">
                            <SearchInput on_search={on_search} placeholder="Rechercher un nœud..." />
                            <div class="text-sm text-gray-400">
                                {"Entrez l'alias ou la clé publique d'un nœud pour filtrer les recommandations"}
                            </div>
                        </div>
                    </Card>

                    if *loading {
                        <div class="flex justify-center items-center h-64">
                            <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary"></div>
                        </div>
                    } else if let Some(err) = &*error {
                        <div class="bg-red-900 border border-red-700 text-red-100 px-4 py-3 rounded relative" role="alert">
                            <strong class="font-bold">{"Erreur : "}</strong>
                            <span class="block sm:inline">{err}</span>
                            <Button variant="secondary" onclick={Callback::from(|_| {})}>
                                {"Réessayer"}
                            </Button>
                        </div>
                    } else {
                        <Card title="Liste des recommandations">
                            <div class="overflow-x-auto">
                                <table class="min-w-full divide-y divide-dark-light">
                                    <thead>
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                                {"Nœud"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                                {"Capacité"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                                {"Score"}
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                                {"Clé publique"}
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-dark-light">
                                        {filtered_recommendations.iter().map(|rec| {
                                            html! {
                                                <tr>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                                                        {rec.alias.clone()}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-400">
                                                        {format!("{} sats", rec.capacity)}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-400">
                                                        {format!("{:.2}", rec.score)}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-400">
                                                        {rec.pubkey.clone()}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Html>()}
                                    </tbody>
                                </table>
                            </div>
                        </Card>
                    }
                </div>
            </main>
        </div>
    }
}