// Définition des modules
mod components;
mod pages;
mod services;
mod types;
mod config;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Dashboard,
    #[at("/channels")]
    Channels,
    #[at("/actions")]
    Actions,
    #[at("/recommendations")]
    Recommendations,
    #[at("/yields")]
    Yields,
    #[at("/alby")]
    Alby,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! { <pages::DashboardPage /> },
        Route::Channels => html! { <pages::ChannelsPage /> },
        Route::Actions => html! { <pages::ActionsPage /> },
        Route::Recommendations => html! { <pages::RecommendationsPage /> },
        Route::Yields => html! { <pages::YieldsPage /> },
        Route::Alby => html! { <pages::AlbyPage /> },
        Route::NotFound => html! { <h1>{"404 - Page non trouvée"}</h1> },
    }
}

// Fonction d'initialisation
pub fn init() {
    // Code d'initialisation ici
} 