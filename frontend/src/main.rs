mod routes;
use routes::AppRoute;
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
    Home,
    #[at("/actions")]
    Actions,
    #[at("/alby")]
    Alby,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Dashboard /> },
        Route::Actions => html! { <Actions /> },
        Route::Alby => html! { <Alby /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen bg-gray-900">
                <Navbar />
                <main class="container mx-auto px-4 py-8">
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