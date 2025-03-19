use yew::prelude::*;
use crate::services::ApiService;
use crate::models::Recommendation;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
pub struct ActionsProps {
    pub api_service: ApiService,
}

#[function_component(Actions)]
pub fn actions(props: &ActionsProps) -> Html {
    let recommendations = use_state(|| Vec::<Recommendation>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let recommendations = recommendations.clone();
        let loading = loading.clone();
        let error = error.clone();
        let api_service = props.api_service.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match api_service.get_recommendations().await {
                        Ok(recs) => {
                            recommendations.set(recs);
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e.to_string()));
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
        <div class="p-6 bg-white rounded-lg shadow-lg">
            <h2 class="text-2xl font-bold mb-4">{"Actions Recommandées"}</h2>
            
            if *loading {
                <div class="text-center py-8 text-gray-500">
                    {"Chargement des recommandations..."}
                </div>
            } else if let Some(err) = (*error).as_ref() {
                <div class="text-center py-8 text-red-500">
                    {format!("Erreur: {}", err)}
                </div>
            } else if recommendations.is_empty() {
                <div class="text-center py-8 text-gray-500">
                    {"Aucune recommandation pour le moment."}
                </div>
            } else {
                <div class="space-y-4">
                    {for recommendations.iter().map(|rec| {
                        let severity_class = match rec.severity {
                            crate::models::ImpactSeverity::High => "bg-red-100 border-red-500",
                            crate::models::ImpactSeverity::Medium => "bg-yellow-100 border-yellow-500",
                            crate::models::ImpactSeverity::Low => "bg-green-100 border-green-500",
                        };
                        
                        html! {
                            <div class={format!("p-4 border-l-4 rounded {}", severity_class)}>
                                <h3 class="font-semibold">{&rec.title}</h3>
                                <p class="text-gray-600 mt-1">{&rec.description}</p>
                            </div>
                        }
                    })}
                </div>
            }
        </div>
    }
} 