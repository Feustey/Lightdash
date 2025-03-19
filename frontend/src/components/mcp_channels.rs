use yew::prelude::*;
use thousands::Separable;
use crate::models::{McpChannel, McpOffer};
use crate::services::ApiService;
use chrono::{DateTime, Utc};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_service: ApiService,
}

pub enum Msg {
    LoadChannels,
    LoadOffers,
    ChannelsLoaded(Result<Vec<McpChannel>, String>),
    OffersLoaded(Result<Vec<McpOffer>, String>),
    CreateOffer,
    AcceptOffer(String),
    OfferCreated(Result<McpOffer, String>),
    ChannelCreated(Result<McpChannel, String>),
    UpdateCapacity(String),
    UpdateLeaseFeeBasis(String),
    UpdateLeaseRate(String),
    UpdateLeaseDuration(String),
    UpdateMinUptime(String),
    UpdateCapacityFilter(String),
    UpdateTypeFilter(String),
    UpdateSortBy(String),
}

pub struct McpChannelsComponent {
    channels: Vec<McpChannel>,
    offers: Vec<McpOffer>,
    error: Option<String>,
    new_offer: NewOfferForm,
    filters: Filters,
}

#[derive(Default)]
struct Filters {
    min_capacity: Option<u64>,
    channel_type: Option<String>,
    sort_by: String,
}

struct NewOfferForm {
    capacity: u64,
    lease_fee_base: u64,
    lease_fee_rate: u32,
    lease_duration: u32,
    min_uptime: u32,
}

impl Component for McpChannelsComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadChannels);
        ctx.link().send_message(Msg::LoadOffers);
        Self {
            channels: Vec::new(),
            offers: Vec::new(),
            error: None,
            new_offer: NewOfferForm {
                capacity: 100_000,
                lease_fee_base: 1000,
                lease_fee_rate: 1,
                lease_duration: 2592000, // 30 jours en secondes
                min_uptime: 90,
            },
            filters: Filters {
                min_capacity: None,
                channel_type: None,
                sort_by: "capacity".to_string(),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadChannels => {
                let callback = ctx.link().callback(Msg::ChannelsLoaded);
                ctx.props().api_service.get_mcp_channels(callback);
                false
            }
            Msg::LoadOffers => {
                let callback = ctx.link().callback(Msg::OffersLoaded);
                ctx.props().api_service.get_mcp_offers(callback);
                false
            }
            Msg::ChannelsLoaded(result) => {
                match result {
                    Ok(mut channels) => {
                        // Appliquer les filtres et le tri
                        if let Some(min_cap) = self.filters.min_capacity {
                            channels.retain(|c| c.capacity >= min_cap);
                        }
                        if let Some(type_filter) = &self.filters.channel_type {
                            channels.retain(|c| format!("{:?}", c.channel_type) == *type_filter);
                        }
                        match self.filters.sort_by.as_str() {
                            "capacity" => channels.sort_by(|a, b| b.capacity.cmp(&a.capacity)),
                            "balance" => channels.sort_by(|a, b| b.local_balance.cmp(&a.local_balance)),
                            "expiry" => channels.sort_by(|a, b| b.lease_expiry.cmp(&a.lease_expiry)),
                            _ => {}
                        }
                        self.channels = channels;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            }
            Msg::OffersLoaded(result) => {
                match result {
                    Ok(offers) => {
                        self.offers = offers;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            }
            Msg::CreateOffer => {
                let callback = ctx.link().callback(Msg::OfferCreated);
                ctx.props().api_service.create_mcp_offer(
                    self.new_offer.capacity,
                    self.new_offer.lease_fee_base,
                    self.new_offer.lease_fee_rate,
                    self.new_offer.lease_duration,
                    self.new_offer.min_uptime,
                    callback,
                );
                false
            }
            Msg::AcceptOffer(offer_id) => {
                let callback = ctx.link().callback(Msg::ChannelCreated);
                ctx.props().api_service.accept_mcp_offer(&offer_id, callback);
                false
            }
            Msg::OfferCreated(result) => {
                match result {
                    Ok(offer) => {
                        self.offers.push(offer);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            }
            Msg::ChannelCreated(result) => {
                match result {
                    Ok(channel) => {
                        self.channels.push(channel);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                true
            }
            Msg::UpdateCapacity(value) => {
                if let Ok(capacity) = value.parse() {
                    self.new_offer.capacity = capacity;
                }
                true
            }
            Msg::UpdateLeaseFeeBasis(value) => {
                if let Ok(fee) = value.parse() {
                    self.new_offer.lease_fee_base = fee;
                }
                true
            }
            Msg::UpdateLeaseRate(value) => {
                if let Ok(rate) = value.parse() {
                    self.new_offer.lease_fee_rate = rate;
                }
                true
            }
            Msg::UpdateLeaseDuration(value) => {
                if let Ok(duration) = value.parse() {
                    self.new_offer.lease_duration = duration;
                }
                true
            }
            Msg::UpdateMinUptime(value) => {
                if let Ok(uptime) = value.parse() {
                    self.new_offer.min_uptime = uptime;
                }
                true
            }
            Msg::UpdateCapacityFilter(value) => {
                self.filters.min_capacity = if value.is_empty() {
                    None
                } else {
                    value.parse().ok()
                };
                ctx.link().send_message(Msg::LoadChannels);
                false
            }
            Msg::UpdateTypeFilter(value) => {
                self.filters.channel_type = if value == "all" {
                    None
                } else {
                    Some(value)
                };
                ctx.link().send_message(Msg::LoadChannels);
                false
            }
            Msg::UpdateSortBy(value) => {
                self.filters.sort_by = value;
                ctx.link().send_message(Msg::LoadChannels);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="space-y-6">
                // Section Création d'offre
                <div class="bg-white shadow rounded-lg p-6">
                    <h2 class="text-2xl font-bold mb-4">{"Créer une offre MCP"}</h2>
                    <div class="space-y-4">
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-700">{"Capacité (sats)"}</label>
                                <input
                                    type="number"
                                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
                                    value={self.new_offer.capacity.to_string()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateCapacity(input.value())
                                    })}
                                />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700">{"Frais de base (sats)"}</label>
                                <input
                                    type="number"
                                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
                                    value={self.new_offer.lease_fee_base.to_string()}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateLeaseFeeBasis(input.value())
                                    })}
                                />
                            </div>
                        </div>
                        <button
                            class="w-full bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
                            onclick={ctx.link().callback(|_| Msg::CreateOffer)}
                        >
                            {"Créer l'offre"}
                        </button>
                    </div>
                </div>

                // Section Offres actives
                <div class="bg-white shadow rounded-lg p-6">
                    <h2 class="text-2xl font-bold mb-4">{"Offres MCP actives"}</h2>
                    <div class="space-y-4">
                        {for self.offers.iter().map(|offer| {
                            let offer_id = offer.offer_id.clone();
                            html! {
                                <div class="border rounded p-4">
                                    <div class="flex justify-between items-center">
                                        <div>
                                            <p class="font-semibold">{"Capacité: "}{offer.capacity.separate_with_spaces()}{" sats"}</p>
                                            <p>{"Frais: "}{offer.lease_fee_base.separate_with_spaces()}{" sats"}</p>
                                            <p>{"Durée: "}{offer.lease_duration / 86400}{" jours"}</p>
                                        </div>
                                        <button
                                            class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700"
                                            onclick={ctx.link().callback(move |_| Msg::AcceptOffer(offer_id.clone()))}
                                        >
                                            {"Accepter"}
                                        </button>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                </div>

                // Section Canaux MCP avec filtres
                <div class="bg-white shadow rounded-lg p-6">
                    <h2 class="text-2xl font-bold mb-4">{"Canaux MCP"}</h2>
                    
                    // Filtres
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                        <div>
                            <label class="block text-sm font-medium text-gray-700">{"Capacité minimale"}</label>
                            <input
                                type="number"
                                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
                                placeholder="Entrer la capacité minimale"
                                onchange={ctx.link().callback(|e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateCapacityFilter(input.value())
                                })}
                            />
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700">{"Type de canal"}</label>
                            <select
                                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
                                onchange={ctx.link().callback(|e: Event| {
                                    let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                    Msg::UpdateTypeFilter(select.value())
                                })}
                            >
                                <option value="all">{"Tous"}</option>
                                <option value="Provider">{"Fournisseur"}</option>
                                <option value="Taker">{"Preneur"}</option>
                                <option value="Standard">{"Standard"}</option>
                            </select>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700">{"Trier par"}</label>
                            <select
                                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
                                onchange={ctx.link().callback(|e: Event| {
                                    let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                    Msg::UpdateSortBy(select.value())
                                })}
                            >
                                <option value="capacity">{"Capacité"}</option>
                                <option value="balance">{"Balance locale"}</option>
                                <option value="expiry">{"Date d'expiration"}</option>
                            </select>
                        </div>
                    </div>

                    // Liste des canaux
                    <div class="space-y-4">
                        {for self.channels.iter().map(|channel| {
                            html! {
                                <div class="border rounded p-4">
                                    <div class="space-y-2">
                                        <p class="font-semibold">{"ID: "}{&channel.channel_id}</p>
                                        <p>{"Capacité: "}{channel.capacity.separate_with_spaces()}{" sats"}</p>
                                        <p>{"Balance locale: "}{channel.local_balance.separate_with_spaces()}{" sats"}</p>
                                        <p>{"Type: "}{format!("{:?}", channel.channel_type)}</p>
                                        {if let Some(expiry) = channel.lease_expiry {
                                            html! {
                                                <p>{"Expiration: "}{expiry.format("%Y-%m-%d %H:%M:%S").to_string()}</p>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                </div>

                {if let Some(error) = &self.error {
                    html! {
                        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                            {error}
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }
} 