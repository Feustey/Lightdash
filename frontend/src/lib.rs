use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod models;
pub mod services;
pub mod components;

use components::nav::Nav;
use components::dashboard::Dashboard;
use components::actions::Actions;
use services::ApiService;

#[function_component(App)]
pub fn app() -> Html {
    let api_service = ApiService::new();

    html! {
        <div class="min-h-screen bg-gray-100">
            <Nav />
            <main class="container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 gap-8">
                    <Dashboard api_service={api_service.clone()} />
                    <Actions api_service={api_service.clone()} />
                </div>
            </main>
        </div>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
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
