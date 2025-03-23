use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::components::{Navbar, Card, Button};
use crate::types::Transaction;
use crate::services::fetch_transactions;

#[function_component(TransactionsPageComponent)]
pub fn transactions() -> Html {
    let transactions = use_state(|| None::<Vec<Transaction>>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let transactions = transactions.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let transactions = transactions.clone();
            let error = error.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match fetch_transactions().await {
                    Ok(transactions_data) => transactions.set(Some(transactions_data)),
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
                    <Card class={"transactions-card".to_string()} title={"Transactions".to_string()}>
                        <div class="transactions-content">
                            if *loading {
                                <div class="loading">{"Chargement des transactions..."}</div>
                            } else if let Some(err) = &*error {
                                <div class="error">
                                    <p>{err}</p>
                                    <Button onclick={Callback::from(move |_| {
                                        let transactions = transactions.clone();
                                        let error = error.clone();
                                        let loading = loading.clone();

                                        wasm_bindgen_futures::spawn_local(async move {
                                            loading.set(true);
                                            error.set(None);
                                            match fetch_transactions().await {
                                                Ok(transactions_data) => transactions.set(Some(transactions_data)),
                                                Err(e) => error.set(Some(e.to_string())),
                                            }
                                            loading.set(false);
                                        });
                                    })}>
                                        {"Réessayer"}
                                    </Button>
                                </div>
                            } else if let Some(transactions_data) = &*transactions {
                                <div class="transactions-table">
                                    <table>
                                        <thead>
                                            <tr>
                                                <th>{"ID"}</th>
                                                <th>{"Type"}</th>
                                                <th>{"Montant"}</th>
                                                <th>{"Statut"}</th>
                                                <th>{"Date"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {transactions_data.iter().map(|transaction| {
                        {recs.iter().map(|rec| {
                            html! {
                                <tr key={rec.node_pubkey.clone()}>
                                    <td>{rec.alias.clone()}</td>
                                    <td>{rec.capacity.to_string()}</td>
                                    <td>{rec.channel_count.to_string()}</td>
                                    <td>{"rec.node_pubkey"}</td>
                                </tr>
                            }
                        }).collect::<Html>()}
                                                }
                                            }).collect::<Html>()}
                                        </tbody>
                                    </table>
                                </div>
                            }
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
} 