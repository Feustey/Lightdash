use wasm_bindgen::prelude::*;
use yew::prelude::*;
use wasm_logger;
use crate::components::dashboard::Dashboard;
use crate::services::api::ApiService;
use web_sys::window;
use js_sys::Reflect;

pub mod models;
pub mod services;
pub mod components;

use components::nav::Nav;
use components::actions::Actions;

fn get_api_url() -> String {
    if let Some(window) = window() {
        if let Ok(env) = Reflect::get(&window, &wasm_bindgen::JsValue::from_str("ENV")) {
            if let Ok(api_url) = Reflect::get(&env, &wasm_bindgen::JsValue::from_str("API_URL")) {
                if let Some(url) = api_url.as_string() {
                    return url;
                }
            }
        }
    }
    String::from("https://api.lightdash.vercel.app")
}

#[function_component(App)]
pub fn app() -> Html {
    use_effect_with_deps(
        |_| {
            wasm_logger::init(wasm_logger::Config::default());
            || ()
        },
        (),
    );

    let api_service = ApiService::new(get_api_url());

    html! {
        <div class="min-h-screen bg-gray-100">
            <Nav />
            <main class="container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 gap-8">
                    <Dashboard api_service={api_service} />
                    <Actions api_service={api_service.clone()} />
                </div>
            </main>
        </div>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
    Ok(())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
