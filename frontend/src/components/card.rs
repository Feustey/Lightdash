use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    pub class: String,
    pub title: String,
    pub children: Children,
}

#[function_component(CardComponent)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class={classes!("card", props.class.clone())}>
            <header class="card-header">
                <h2 class="card-header-title">{&props.title}</h2>
            </header>
            <div class="card-content">
                {props.children.clone()}
            </div>
        </div>
    }
} 