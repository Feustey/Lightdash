use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{DashboardPage, ChannelsPage, ActionsPage, RecommendationsPage};

mod components;
mod pages;
mod types;

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
}

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Channels => html! { <ChannelsPage /> },
        Route::Actions => html! { <ActionsPage /> },
        Route::Recommendations => html! { <RecommendationsPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
} 