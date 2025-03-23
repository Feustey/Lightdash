use yew::prelude::*;
use crate::types::Page;

mod dashboard;
mod actions;
mod alby;
pub mod home;
pub mod recommendations;
pub mod channels;
pub mod about;

pub use dashboard::Dashboard;
pub use actions::Actions;
pub use alby::Alby;
pub use home::Home;
pub use recommendations::RecommendationsPage;
pub use channels::ChannelsPage;
pub use about::AboutPage;

pub fn render_page(page: &Page) -> Html {
    match page {
        Page::Dashboard => html! { <DashboardPageComponent /> },
        Page::Actions => html! { <ActionsPageComponent /> },
        Page::Alby => html! { <AlbyPageComponent /> },
    }
} 