mod models;
mod services;
mod components;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use services::ApiService;
use components::node_info::NodeInfoComponent;
use components::mcp_channels::McpChannelsComponent;
use components::mcp_stats::McpStatsComponent;
use components::actions::ActionsComponent;
use components::dashboard::Dashboard;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}

#[function_component(App)]
pub fn app() -> Html {
    let api_service = ApiService::new();

    html! {
        <div class="min-h-screen bg-gray-100">
            <nav class="bg-white shadow-sm">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between h-16">
                        <div class="flex">
                            <div class="flex-shrink-0 flex items-center">
                                <span class="text-xl font-bold text-blue-600">{"⚡ Lightdash"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>

            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <Dashboard api_service={api_service} />
            </main>
        </div>
    }
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
