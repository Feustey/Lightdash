use yew::prelude::*;
use web_sys::MouseEvent;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub class: String,
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button 
            class={classes!("button", &props.class)}
            onclick={&props.onclick}
        >
            {for props.children.iter()}
        </button>
    }
} 