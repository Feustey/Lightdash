use crate::routes::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::types::Route;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_page: String,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    html! {
        <nav class="bg-dark-lighter border-b border-dark-lighter">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <div class="flex">
                        <div class="flex-shrink-0 flex items-center">
                            <img src="/assets/logo.svg" alt="Lightdash" class="h-8 w-8" />
                            <span class="ml-2 text-xl font-bold text-primary">{"Lightdash"}</span>
                        </div>
                        <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                            <Link<AppRoute> 
                                to={Route::Home}
                                classes={classes!(
                                    "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                    if props.current_page == "home" {
                                        "border-primary text-white"
                                    } else {
                                        "border-transparent text-gray-300 hover:border-gray-300 hover:text-white"
                                    }
                                )}
                            >
                                {"Accueil"}
                            </Link<AppRoute>>
                            <Link<AppRoute>
                                to={Route::Actions}
                                classes={classes!(
                                    "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                    if props.current_page == "actions" {
                                        "border-primary text-white"
                                    } else {
                                        "border-transparent text-gray-300 hover:border-gray-300 hover:text-white"
                                    }
                                )}
                            >
                                {"Actions"}
                            </Link<AppRoute>>
                            <Link<AppRoute>
                                to={Route::Alby}
                                classes={classes!(
                                    "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                    if props.current_page == "alby" {
                                        "border-primary text-white"
                                    } else {
                                        "border-transparent text-gray-300 hover:border-gray-300 hover:text-white"
                                    }
                                )}
                            >
                                {"Alby"}
                            </Link<AppRoute>>
                            <Link<AppRoute>
                                to={Route::Recommendations}
                                classes={classes!(
                                    "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                    if props.current_page == "recommendations" {
                                        "border-primary text-white"
                                    } else {
                                        "border-transparent text-gray-300 hover:border-gray-300 hover:text-white"
                                    }
                                )}
                            >
                                {"Recommandations"}
                            </Link<AppRoute>>
                            <Link<AppRoute>
                                to={Route::Channels}
                                classes={classes!(
                                    "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                    if props.current_page == "channels" {
                                        "border-primary text-white"
                                    } else {
                                        "border-transparent text-gray-300 hover:border-gray-300 hover:text-white"
                                    }
                                )}
                            >
                                {"Canaux"}
                            </Link<AppRoute>>
                            <Link<AppRoute>
                                to={Route::About}
                                classes={classes!(
                                    "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                    if props.current_page == "about" {
                                        "border-primary text-white"
                                    } else {
                                        "border-transparent text-gray-300 hover:border-gray-300 hover:text-white"
                                    }
                                )}
                            >
                                {"À propos"}
                            </Link<AppRoute>>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
} 