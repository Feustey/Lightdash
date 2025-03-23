use crate::routes::AppRoute;
use crate::routes::AppRoute;
// Définition des modules
mod components;
mod pages;
mod services;
mod types;
mod config;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{DashboardPageComponent, ActionsPageComponent, AlbyPageComponent};
use crate::types::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <DashboardPageComponent /> },
        Route::Actions => html! { <ActionsPageComponent /> },
        Route::Alby => html! { <AlbyPageComponent /> },
        Route::Home => html! { <h1>{"404 - Page non trouvée"}</h1> },
    }
}

// Fonction d'initialisation
pub fn init() {
    wasm_logger::init(wasm_logger::Config::default());
} 