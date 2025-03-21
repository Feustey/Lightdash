use yew::prelude::*;
use yew::html;
use web_sys::MouseEvent;

mod navbar;
mod card;
mod button;
mod chart;

#[derive(Properties, PartialEq, Clone)]
pub struct NavbarProps {
    pub current_page: String,
}

pub struct NavbarComponent {
    props: NavbarProps,
}

impl Component for NavbarComponent {
    type Message = ();
    type Properties = NavbarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_mobile_menu_open = use_state(|| false).clone();

        let toggle_mobile_menu = {
            let is_mobile_menu_open = is_mobile_menu_open.clone();
            Callback::from(move |_| {
                is_mobile_menu_open.set(!*is_mobile_menu_open.borrow());
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
                <div class={classes!("navbar-menu", if *is_mobile_menu_open.borrow() { "is-active" } else { "" })}>
                    <div class="navbar-start">
                        <a href="/" class={classes!("navbar-item", if self.props.current_page == "dashboard" { "is-active" } else { "" })}>
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="mdi mdi-view-dashboard"></i>
                                </span>
                                <span>{"Tableau de bord"}</span>
                            </span>
                        </a>
                        <a href="/channels" class={classes!("navbar-item", if self.props.current_page == "channels" { "is-active" } else { "" })}>
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="mdi mdi-connection"></i>
                                </span>
                                <span>{"Canaux"}</span>
                            </span>
                        </a>
                        <a href="/actions" class={classes!("navbar-item", if self.props.current_page == "actions" { "is-active" } else { "" })}>
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="mdi mdi-play-circle"></i>
                                </span>
                                <span>{"Actions"}</span>
                            </span>
                        </a>
                        <a href="/recommendations" class={classes!("navbar-item", if self.props.current_page == "recommendations" { "is-active" } else { "" })}>
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="mdi mdi-lightbulb"></i>
                                </span>
                                <span>{"Recommandations"}</span>
                            </span>
                        </a>
                        <a href="/yields" class={classes!("navbar-item", if self.props.current_page == "yields" { "is-active" } else { "" })}>
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="mdi mdi-chart-line"></i>
                                </span>
                                <span>{"Rendements"}</span>
                            </span>
                        </a>
                        <a href="/alby" class={classes!("navbar-item", if self.props.current_page == "alby" { "is-active" } else { "" })}>
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
}

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    pub title: String,
    pub children: Children,
    pub class: Option<String>,
}

pub struct CardComponent {
    props: CardProps,
}

impl Component for CardComponent {
    type Message = ();
    type Properties = CardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let class = self.props.class.clone().unwrap_or_default();
        html! {
            <div class={format!("card {}", class)}>
                <div class="card-header">
                    <h2 class="card-title">{&self.props.title}</h2>
                </div>
                <div class="card-content">
                    {for self.props.children.iter()}
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

pub struct ButtonComponent {
    props: ButtonProps,
}

impl Component for ButtonComponent {
    type Message = ();
    type Properties = ButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <button class="button" onclick={self.props.onclick.clone()}>
                {self.props.children.clone()}
            </button>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ChartProps {
    pub title: String,
    pub data: Vec<f64>,
    pub labels: Vec<String>,
}

pub struct ChartComponent {
    props: ChartProps,
}

impl Component for ChartComponent {
    type Message = ();
    type Properties = ChartProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="chart">
                <h3>{&self.props.title}</h3>
                <canvas id="chart"></canvas>
            </div>
        }
    }
}

pub type Navbar = NavbarComponent;
pub type Card = CardComponent;
pub type Button = ButtonComponent;
pub type Chart = ChartComponent; 