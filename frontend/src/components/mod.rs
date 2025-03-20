use yew::prelude::*;

pub mod chart;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_page: String,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    html! {
        <nav class="navbar">
            <div class="navbar-brand">
                <h1>{"Lightdash"}</h1>
            </div>
            <div class="navbar-menu">
                <a href="/" class={if props.current_page == "dashboard" { "active" } else { "" }}>
                    {"Tableaux de bord"}
                </a>
                <a href="/channels" class={if props.current_page == "channels" { "active" } else { "" }}>
                    {"Canaux"}
                </a>
                <a href="/actions" class={if props.current_page == "actions" { "active" } else { "" }}>
                    {"Actions"}
                </a>
                <a href="/recommendations" class={if props.current_page == "recommendations" { "active" } else { "" }}>
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