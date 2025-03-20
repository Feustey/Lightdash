use yew::prelude::*;
use crate::pages::{DashboardPage, ChannelsPage, ActionsPage, RecommendationsPage, YieldsPage};

pub mod chart;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_page: String,
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let current_page = use_state(|| "dashboard".to_string());
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
                <h1>{"Lightdash"}</h1>
            </div>
            <button class="mobile-menu-button" onclick={toggle_mobile_menu}>
                <span class={if *is_mobile_menu_open { "open" } else { "" }}></span>
                <span class={if *is_mobile_menu_open { "open" } else { "" }}></span>
                <span class={if *is_mobile_menu_open { "open" } else { "" }}></span>
            </button>
            <div class={classes!(
                "navbar-links",
                if *is_mobile_menu_open { "open" } else { "" }
            )}>
                <a href="/" class={if *current_page == "dashboard" { "active" } else { "" }}>
                    {"Tableau de bord"}
                </a>
                <a href="/channels" class={if *current_page == "channels" { "active" } else { "" }}>
                    {"Canaux"}
                </a>
                <a href="/yields" class={if *current_page == "yields" { "active" } else { "" }}>
                    {"Rendements"}
                </a>
                <a href="/actions" class={if *current_page == "actions" { "active" } else { "" }}>
                    {"Actions"}
                </a>
                <a href="/recommendations" class={if *current_page == "recommendations" { "active" } else { "" }}>
                    {"Recommandations"}
                </a>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: String,
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="card">
            <div class="card-header">
                <h2>{&props.title}</h2>
            </div>
            <div class="card-content">
                {for props.children.iter()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub variant: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.clone();
    html! {
        <button 
            class={format!("button {}", props.variant)}
            onclick={Callback::from(move |_| onclick.emit(()))}
        >
            {&props.label}
        </button>
    }
} 