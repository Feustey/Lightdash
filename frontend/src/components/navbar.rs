use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct NavbarProps {
    pub current_page: String,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let is_mobile_menu_open = use_state(|| false);

    let toggle_mobile_menu = {
        let is_mobile_menu_open = is_mobile_menu_open.clone();
        Callback::from(move |_| {
            is_mobile_menu_open.set(!*is_mobile_menu_open);
        })
    };

    html! {
        <nav class="navbar">
            <div class="navbar-brand">
                <a href="/" class="navbar-item">
                    <span class="icon-text">
                        <span class="icon">
                            <i class="mdi mdi-lightning-bolt"></i>
                        </span>
                        <span>{"Lightdash"}</span>
                    </span>
                </a>
                <button class="navbar-burger" onclick={toggle_mobile_menu}>
                    <span></span>
                    <span></span>
                    <span></span>
                </button>
            </div>
            <div class={classes!("navbar-menu", if *is_mobile_menu_open { "is-active" } else { "" })}>
                <div class="navbar-start">
                    <a href="/" class={classes!("navbar-item", if props.current_page == "dashboard" { "is-active" } else { "" })}>
                        <span class="icon-text">
                            <span class="icon">
                                <i class="mdi mdi-view-dashboard"></i>
                            </span>
                            <span>{"Tableau de bord"}</span>
                        </span>
                    </a>
                    <a href="/channels" class={classes!("navbar-item", if props.current_page == "channels" { "is-active" } else { "" })}>
                        <span class="icon-text">
                            <span class="icon">
                                <i class="mdi mdi-connection"></i>
                            </span>
                            <span>{"Canaux"}</span>
                        </span>
                    </a>
                    <a href="/actions" class={classes!("navbar-item", if props.current_page == "actions" { "is-active" } else { "" })}>
                        <span class="icon-text">
                            <span class="icon">
                                <i class="mdi mdi-play-circle"></i>
                            </span>
                            <span>{"Actions"}</span>
                        </span>
                    </a>
                    <a href="/recommendations" class={classes!("navbar-item", if props.current_page == "recommendations" { "is-active" } else { "" })}>
                        <span class="icon-text">
                            <span class="icon">
                                <i class="mdi mdi-star"></i>
                            </span>
                            <span>{"Recommandations"}</span>
                        </span>
                    </a>
                    <a href="/yields" class={classes!("navbar-item", if props.current_page == "yields" { "is-active" } else { "" })}>
                        <span class="icon-text">
                            <span class="icon">
                                <i class="mdi mdi-chart-line"></i>
                            </span>
                            <span>{"Rendements"}</span>
                        </span>
                    </a>
                    <a href="/alby" class={classes!("navbar-item", if props.current_page == "alby" { "is-active" } else { "" })}>
                        <span class="icon-text">
                            <span class="icon">
                                <i class="mdi mdi-wallet"></i>
                            </span>
                            <span>{"Alby"}</span>
                        </span>
                    </a>
                </div>
            </div>
        </nav>
    }
} 