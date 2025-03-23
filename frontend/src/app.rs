use yew::prelude::*;
use yew_router::prelude::*;
use crate::types::Route;
use crate::pages::{
    home::HomePage,
    actions::ActionsPage,
    alby::AlbyPage,
    recommendations::RecommendationsPage,
    channels::ChannelsPage,
    about::AboutPage,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Actions => html! { <ActionsPage /> },
        Route::Alby => html! { <AlbyPage /> },
        Route::Recommendations => html! { <RecommendationsPage /> },
        Route::Channels => html! { <ChannelsPage /> },
        Route::About => html! { <AboutPage /> },
    }
} 