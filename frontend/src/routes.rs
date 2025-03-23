use yew_router::Routable;

#[derive(Clone, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/channels")]
    Channels,
    #[at("/transactions")]
    Transactions,
    #[at("/yields")]
    Yields,
    #[at("/recommendations")]
    Recommendations,
    #[at("/about")]
    About,
    #[at("/alby")]
    Alby,
    #[not_found]
    NotFound,
}
