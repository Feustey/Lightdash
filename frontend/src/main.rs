use yew::prelude::*;
use yew_router::prelude::*;
use pages::{DashboardPage, ChannelsPage, ActionsPage, RecommendationsPage, YieldsPage, AlbyPage};

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
    #[at("/actions")]
    Actions,
    #[at("/recommendations")]
    Recommendations,
    #[at("/yields")]
    Yields,
    #[at("/alby")]
    Alby,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Channels => html! { <ChannelsPage /> },
        Route::Actions => html! { <ActionsPage /> },
        Route::Recommendations => html! { <RecommendationsPage /> },
        Route::Yields => html! { <YieldsPage /> },
        Route::Alby => html! { <AlbyPage /> },
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