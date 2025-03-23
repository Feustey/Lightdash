use yew::prelude::*;
use yew::html;
use web_sys::MouseEvent;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use crate::types::Route;

mod card;
mod navbar;
mod button;
mod chart;
mod search_input;

pub use card::Card;
pub use navbar::Navbar;
pub use button::{Button, ButtonVariant};
pub use chart::Chart;
pub use search_input::SearchInput;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: String,
    pub options: Option<serde_json::Value>,    pub options: Option<serde_json::Value>,    pub options: Option<serde_json::Value>,    pub options: Option<serde_json::Value>,    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="bg-dark-light border border-dark-lighter rounded-lg shadow-lg p-6">
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct NavbarProps {
    pub current_page: AppRoute,    pub current_page: AppRoute,    pub current_route: Route,
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="bg-dark-light border-b border-dark-lighter">
            <div class="container mx-auto px-4">
                <div class="flex justify-between items-center h-16">
                    <div class="flex items-center">
                        <img src="/logo.svg" alt="Lightdash" class="h-8 w-8 mr-2" />
                        <span class="text-xl font-bold text-primary">{"Lightdash"}</span>
                    </div>
                    
                    <div class="flex space-x-4">
                        <Link::<Route> to={Route::Home} classes="text-gray-300 hover:text-primary px-3 py-2 rounded-md text-sm font-medium">
                            {"Dashboard"}
                        </Link<Route>>
                        <Link::<Route> to={Route::Actions} classes="text-gray-300 hover:text-primary px-3 py-2 rounded-md text-sm font-medium">
                            {"Actions"}
                        </Link<Route>>
                        <Link::<Route> to={Route::Alby} classes="text-gray-300 hover:text-primary px-3 py-2 rounded-md text-sm font-medium">
                            {"Alby"}
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
    #[prop_or_default]
    pub variant: ButtonVariant,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = match props.variant {
        ButtonVariant::Primary => "bg-primary text-dark hover:bg-primary/90",
        ButtonVariant::Secondary => "bg-secondary text-dark hover:bg-secondary/90",
        ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
    };

    html! {
        <button
            onclick={props.onclick.clone()}
            class={format!("px-4 py-2 rounded-lg font-medium transition-colors duration-200 {}", classes)}
        >
            {props.children.clone()}
        </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ChartProps {
    pub title: String,
    pub options: Option<serde_json::Value>,    pub options: Option<serde_json::Value>,    pub options: Option<serde_json::Value>,    pub options: Option<serde_json::Value>,    pub data: Vec<f64>,
    pub labels: Vec<String>,
}

#[function_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    html! {
        <div class="chart">
            <h3>{&props.title}</h3>
            <canvas id="chart"></canvas>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SearchInputProps {
    pub value: String,    pub value: String,    pub on_search: Callback<String>,
    pub placeholder: String,
}

#[function_component(SearchInput)]
pub fn search_input(props: &SearchInputProps) -> Html {
    let onkeypress = {
        let on_search = props.on_search.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                on_search.emit(input.value());
            }
        })
    };

    let onclick = {
        let on_search = props.on_search.clone();
        Callback::from(move |_| {
            let input: HtmlInputElement = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("search-input")
                .unwrap()
                .dyn_into()
                .unwrap();
            on_search.emit(input.value());
        })
    };

    html! {
        <div class="flex space-x-2">
            <input
                id="search-input"
                type="text"
                placeholder={props.placeholder.clone()}
                class="flex-1 bg-dark-lighter border border-dark-lighter rounded-lg px-4 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent"
                onkeypress={onkeypress}
            />
            <Button onclick={onclick} variant={ButtonVariant::Primary}>
                {"Rechercher"}
            </Button>
        </div>
    }
} 