use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::services::api::ApiService;
use crate::components::dashboard::Dashboard;

mod components;
mod services;
mod models;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::set_once));

    let window = window().unwrap();
    let document = window.document().unwrap();
    let element = document.get_element_by_id("app").unwrap();

    let api_service = ApiService::new();
    
    yew::Renderer::<Dashboard>::with_root_and_props(element, DashboardProps {
        api_service,
    }).render();

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
