use yew::prelude::*;
use crate::components::Navbar;
use crate::types::Route;

mod dashboard;
mod channels;
mod actions;
mod recommendations;
mod yields;
mod alby;

pub use dashboard::DashboardPageComponent;
pub use channels::ChannelsPageComponent;
pub use actions::ActionsPageComponent;
pub use recommendations::RecommendationsPageComponent;
pub use yields::YieldsPageComponent;
pub use alby::AlbyPageComponent;

#[derive(Clone, PartialEq)]
pub enum Page {
    Dashboard,
    Channels,
    Transactions,
    Actions,
    Recommendations,
    Yields,
    Alby,
}

impl Page {
    pub fn to_string(&self) -> String {
        match self {
            Page::Dashboard => "dashboard".to_string(),
            Page::Channels => "channels".to_string(),
            Page::Transactions => "transactions".to_string(),
            Page::Actions => "actions".to_string(),
            Page::Recommendations => "recommendations".to_string(),
            Page::Yields => "yields".to_string(),
            Page::Alby => "alby".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Page> {
        match s {
            "dashboard" => Some(Page::Dashboard),
            "channels" => Some(Page::Channels),
            "transactions" => Some(Page::Transactions),
            "actions" => Some(Page::Actions),
            "recommendations" => Some(Page::Recommendations),
            "yields" => Some(Page::Yields),
            "alby" => Some(Page::Alby),
            _ => None,
        }
    }
}

pub fn render_page(page: &Page) -> Html {
    match page {
        Page::Dashboard => html! { <DashboardPageComponent /> },
        Page::Channels => html! { <ChannelsPageComponent /> },
        Page::Transactions => html! { <TransactionsPageComponent /> },
        Page::Actions => html! { <ActionsPageComponent /> },
        Page::Recommendations => html! { <RecommendationsPageComponent /> },
        Page::Yields => html! { <YieldsPageComponent /> },
        Page::Alby => html! { <AlbyPageComponent /> },
    }
} 