use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_page: String,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    html! {
        <nav class="navbar">
            <div class="navbar-brand">
                <img src="/logo.png" alt="Logo" class="navbar-logo" />
                <h1>{"Lightdash"}</h1>
            </div>
            <div class="navbar-links">
                <Link<Route> to={Route::Dashboard} classes={if props.current_page == "dashboard" { "active" } else { "" }}>
                    {"Dashboard"}
                </Link<Route>>
                <Link<Route> to={Route::Channels} classes={if props.current_page == "channels" { "active" } else { "" }}>
                    {"Canaux"}
                </Link<Route>>
                <Link<Route> to={Route::Yields} classes={if props.current_page == "yields" { "active" } else { "" }}>
                    {"Rendements"}
                </Link<Route>>
                <Link<Route> to={Route::Actions} classes={if props.current_page == "actions" { "active" } else { "" }}>
                    {"Actions"}
                </Link<Route>>
                <Link<Route> to={Route::Recommendations} classes={if props.current_page == "recommendations" { "active" } else { "" }}>
                    {"Recommandations"}
                </Link<Route>>
                <Link<Route> to={Route::Alby} classes={if props.current_page == "alby" { "active" } else { "" }}>
                    {"Alby Hub"}
                </Link<Route>>
            </div>
        </nav>
    }
} 