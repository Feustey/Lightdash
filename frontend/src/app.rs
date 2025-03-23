use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::dashboard::DashboardPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Dashboard,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! { <DashboardPage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
} 