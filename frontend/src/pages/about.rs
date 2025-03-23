use yew::prelude::*;
use crate::components::{Navbar, Card};

#[function_component(AboutPage)]
pub fn about() -> Html {
    html! {
        <div class="min-h-screen bg-dark">
            <Navbar />
            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <div class="px-4 py-6 sm:px-0">
                    <div class="grid grid-cols-1 gap-6">
                        <Card title="Genèse du projet">
                            <p class="text-gray-300">
                                {"Lightdash est né d'une vision : démocratiser l'accès aux outils de gestion de nœuds Lightning. 
                                En tant que développeur passionné par le Bitcoin et le Lightning Network, j'ai constaté le besoin 
                                d'une interface utilisateur intuitive et puissante pour gérer les nœuds Lightning."}
                            </p>
                        </Card>
                        <Card title="Notre mission">
                            <p class="text-gray-300">
                                {"Notre mission est de fournir une plateforme accessible et efficace pour gérer les nœuds Lightning, 
                                permettant aux utilisateurs de se concentrer sur l'essentiel : la croissance et l'optimisation de leur réseau."}
                            </p>
                        </Card>
                        <Card title="Innovation technique">
                            <p class="text-gray-300">
                                {"Construit avec Rust et WebAssembly, Lightdash offre des performances exceptionnelles et une 
                                expérience utilisateur fluide. Notre architecture modulaire permet une évolution continue et 
                                l'intégration de nouvelles fonctionnalités."}
                            </p>
                        </Card>
                        <Card title="Impact sur l'écosystème">
                            <p class="text-gray-300">
                                {"En simplifiant la gestion des nœuds Lightning, Lightdash contribue à la croissance et à la 
                                décentralisation du réseau. Notre outil aide les opérateurs de nœuds à maintenir et optimiser 
                                leurs canaux de paiement."}
                            </p>
                        </Card>
                        <Card title="Développements futurs">
                            <p class="text-gray-300">
                                {"Nous prévoyons d'ajouter des fonctionnalités avancées comme :"}
                            </p>
                            <ul class="mt-4 list-disc list-inside text-gray-300">
                                <li>{"Analyse avancée des métriques de performance"}</li>
                                <li>{"Gestion automatique des rebalancements"}</li>
                                <li>{"Intégration avec d'autres services Lightning"}</li>
                                <li>{"Support multi-nœuds"}</li>
                            </ul>
                        </Card>
                        <Card title="Collaboration communautaire">
                            <p class="text-gray-300">
                                {"Lightdash est un projet open source qui encourage la participation de la communauté. 
                                Nous accueillons les contributions, les suggestions et les retours d'expérience pour 
                                améliorer continuellement notre plateforme."}
                            </p>
                        </Card>
                        <Card title="Conclusion">
                            <p class="text-gray-300">
                                {"Lightdash représente une étape importante dans l'évolution des outils de gestion de 
                                nœuds Lightning. Notre engagement envers l'excellence technique et l'expérience utilisateur 
                                nous permet de contribuer activement à l'écosystème Bitcoin."}
                            </p>
                        </Card>
                    </div>
                </div>
            </main>
        </div>
    }
} 