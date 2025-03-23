use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct SearchInputProps {
    pub on_search: Callback<String>,
    pub placeholder: String,
    #[prop_or_default]
    pub class: String,
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
        <div class={format!("flex space-x-2 {}", props.class)}>
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