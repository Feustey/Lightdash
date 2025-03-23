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
    let recommendations = use_state(|| Vec::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let recommendations = recommendations.clone();
        let loading = loading.clone();
        let error = error.clone();
        let api_service = props.api_service.clone();

        use_effect_with(
            api_service,
            move |api_service| {
                let recommendations = recommendations.clone();
                let loading = loading.clone();
                let error = error.clone();

                spawn_local(async move {
                    match api_service.get_recommendations().await {
                        Ok(recs) => {
                            recommendations.set(Some(recs));
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
        );
    }

    html! {
        <div class={"actions"}>
            if *loading {
                <div class={"loading"}>{"Chargement des recommandations..."}</div>
            } else if let Some(err) = &*error {
                <div class={"error"}>{err}</div>
            } else if recommendations.is_empty() {
                <div class={"empty"}>{"Aucune recommandation disponible."}</div>
            } else {
                <div class={"recommendations"}>
                    {for recommendations.iter().map(move |rec| {
                        let severity_class = match rec.severity {
                            crate::models::RecommendationSeverity::High => "high",
                            crate::models::RecommendationSeverity::Medium => "medium",
                            crate::models::RecommendationSeverity::Low => "low",
                        };
                        
                        html! {
                            <div class={format!("recommendation {}", severity_class)}>
                                <h3>{&rec.title}</h3>
                                <p>{&rec.description}</p>
                            </div>
                        }
                    })}
                </div>
            }
        </div>
    }
} 