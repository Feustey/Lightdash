mod components;
mod pages;
mod services;
mod types;
mod chart;
mod config;

use components::{Navbar, Card, Button, Chart};
use pages::*;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_logger;

use crate::Route;
use crate::switch;

#[derive(Clone, Routable, PartialEq)]
enum Route {
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

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Channels => html! { <ChannelsPage /> },
        Route::Actions => html! { <ActionsPage /> },
        Route::Recommendations => html! { <RecommendationsPage /> },
        Route::Yields => html! { <YieldsPage /> },
        Route::Alby => html! { <AlbyPage /> },
        Route::NotFound => html! { <h1>{"404 - Page non trouvée"}</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app">
                <Navbar current_page="dashboard".to_string() />
                <main class="main">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
} 