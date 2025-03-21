use yew::prelude::*;
use yew::html;
use wasm_bindgen_futures::spawn_local;
use crate::components::{Navbar, Card, Button};
use crate::types::{Channel, ChannelStatus};
use crate::services::fetch_channels;

#[derive(Properties, PartialEq)]
pub struct ChannelsPageProps {}

#[function_component(ChannelsPageComponent)]
pub fn channels() -> Html {
    let channels = use_state(|| None::<Vec<Channel>>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let channels = channels.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let channels = channels.clone();
            let error = error.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match fetch_channels().await {
                    Ok(channels_data) => channels.set(Some(channels_data)),
                    Err(e) => error.set(Some(e.to_string())),
                }

                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="container">
            <div class="columns">
                <div class="column">
                    <Card class={"channels-card".to_string()} title={"Canaux".to_string()}>
                        <div class="channels-content">
                            if *loading {
                                <div class="loading">{"Chargement des canaux..."}</div>
                            } else if let Some(err) = &*error {
                                <div class="error">
                                    <p>{err}</p>
                                    <Button onclick={Callback::from(move |_| {
                                        let channels = channels.clone();
                                        let error = error.clone();
                                        let loading = loading.clone();

                                        wasm_bindgen_futures::spawn_local(async move {
                                            loading.set(true);
                                            error.set(None);
                                            match fetch_channels().await {
                                                Ok(channels_data) => channels.set(Some(channels_data)),
                                                Err(e) => error.set(Some(e.to_string())),
                                            }
                                            loading.set(false);
                                        });
                                    })}>
                                        {"Réessayer"}
                                    </Button>
                                </div>
                            } else if let Some(channels_data) = &*channels {
                                <div class="channels-grid">
                        {channels_data.iter().map(|channel| {
                            html! {
                                <tr key={channel.channel_id.clone()}>
                                    <td>{channel.channel_id.clone()}</td>
                                    <td>{channel.remote_alias.clone()}</td>
                                    <td>{channel.capacity.to_string()}</td>
                                    <td>{channel.remote_pubkey.clone()}</td>
                                </tr>
                            }
                        }).collect::<Html>()}
                                </div>
                            }
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
} 