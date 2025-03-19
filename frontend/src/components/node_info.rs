use yew::prelude::*;
use thousands::Separable;
use crate::models::NodeInfo;
use crate::services::ApiService;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_service: ApiService,
}

pub enum Msg {
    LoadNodeInfo,
    NodeInfoLoaded(Result<NodeInfo, String>),
}

pub struct NodeInfoComponent {
    node_info: Option<NodeInfo>,
    error: Option<String>,
}

impl Component for NodeInfoComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadNodeInfo);
        Self {
            node_info: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadNodeInfo => {
                let callback = ctx.link().callback(Msg::NodeInfoLoaded);
                ctx.props().api_service.get_node_info(callback);
                false
            }
            Msg::NodeInfoLoaded(result) => {
                match result {
                    Ok(info) => {
                        self.node_info = Some(info);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="bg-white shadow rounded-lg p-6">
                <h2 class="text-2xl font-bold mb-4">{"Informations du Nœud"}</h2>
                {
                    if let Some(error) = &self.error {
                        html! {
                            <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                                {error}
                            </div>
                        }
                    } else if let Some(info) = &self.node_info {
                        html! {
                            <div class="space-y-4">
                                <div class="flex justify-between">
                                    <span class="font-semibold">{"Alias:"}</span>
                                    <span>{&info.alias}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="font-semibold">{"Clé publique:"}</span>
                                    <span class="font-mono text-sm">{&info.pubkey}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="font-semibold">{"Capacité totale:"}</span>
                                    <span>{info.capacity.separate_with_spaces()} {" sats"}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="font-semibold">{"Nombre de canaux:"}</span>
                                    <span>{info.channels}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="font-semibold">{"Version:"}</span>
                                    <span>{&info.version}</span>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="flex justify-center items-center h-32">
                                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                            </div>
                        }
                    }
                }
            </div>
        }
    }
} 