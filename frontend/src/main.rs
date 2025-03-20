use yew::prelude::*;
use yew_router::prelude::*;
use pages::{DashboardPage, ChannelsPage, ActionsPage, RecommendationsPage, YieldsPage};

mod pages;
mod components;
mod services;
mod types;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Dashboard,
    #[at("/channels")]
    Channels,
    #[at("/yields")]
    Yields,
    #[at("/recommendations")]
    Recommendations,
    #[at("/actions")]
    Actions,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Channels => html! { <ChannelsPage /> },
        Route::Yields => html! { <YieldsPage /> },
        Route::Recommendations => html! { <RecommendationsPage /> },
        Route::Actions => html! { <ActionsPage /> },
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