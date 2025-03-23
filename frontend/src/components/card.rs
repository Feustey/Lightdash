use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: String,
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class={format!("bg-dark-lighter border border-dark-lighter rounded-lg shadow-lg p-6 {}", props.class)}>
            <h3 class="text-lg font-semibold text-white mb-4">{&props.title}</h3>
            <div class="text-gray-300">
                {props.children.clone()}
            </div>
        </div>
    }
} 