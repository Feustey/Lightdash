use yew::prelude::*;
use web_sys::window;
use wasm_bindgen::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use crate::components::Meta;

const PREMIUM_PRICE: u64 = 10_000;

#[derive(Clone, PartialEq)]
enum Tab {
    Free,
    Premium,
}

#[function_component(ActionsPage)]
pub fn actions_page() -> Html {
    let active_tab = use_state(|| Tab::Free);
    let is_premium_unlocked = use_state(|| LocalStorage::get("premium_unlocked").unwrap_or(false));
    let is_connecting = use_state(|| false);

    let switch_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: Tab| {
            active_tab.set(tab);
        })
    };

    let handle_payment = {
        let is_premium_unlocked = is_premium_unlocked.clone();
        let is_connecting = is_connecting.clone();
        Callback::from(move |_| {
            is_connecting.set(true);
            let window = window().unwrap();
            
            // Récupérer l'URL de connexion Alby depuis les variables d'environnement
            let js_code = format!(
                r#"
                try {{
                    const albyConnectUrl = process.env.ALBY_CONNECT_URL;
                    if (!albyConnectUrl) {{
                        throw new Error('Configuration Alby manquante');
                    }}
                    
                    if (typeof window.nostr !== 'undefined') {{
                        window.nostr.enable().then(() => {{
                            const invoice = {{
                                amount: {},
                                defaultMemo: "Accès Premium aux Actions Lightdash",
                                connector: albyConnectUrl,
                            }};
                            window.nostr.signEvent(invoice).then((response) => {{
                                if (response.success) {{
                                    localStorage.setItem('premium_unlocked', 'true');
                                    window.alert('Paiement réussi ! Accès Premium débloqué.');
                                    window.location.reload();
                                }} else {{
                                    window.alert('Erreur lors du paiement : ' + response.error);
                                }}
                            }}).catch(err => {{
                                window.alert('Erreur lors de la signature : ' + err);
                            }});
                        }}).catch(err => {{
                            window.alert('Erreur lors de la connexion à Alby : ' + err);
                        }});
                    }} else {{
                        window.location.href = albyConnectUrl;
                    }}
                }} catch (err) {{
                    window.alert('Une erreur est survenue : ' + err);
                    console.error('Erreur Alby:', err);
                }}
                "#,
                PREMIUM_PRICE
            );

            let _ = js_sys::eval(&js_code);
            is_connecting.set(false);
        })
    };

    html! {
        <>
            <Meta
                title="Actions et Optimisations"
                description="Optimisez votre nœud Lightning Network avec des actions intelligentes alimentées par l'IA. Accédez à des recommandations personnalisées pour maximiser vos rendements."
                keywords="lightning network, bitcoin, node management, AI optimization, crypto, blockchain"
                og_image="/images/actions-preview.jpg"
            />
            <div class="actions-container">
                <div class="actions-tabs">
                    <button 
                        class={if *active_tab == Tab::Free { "tab-button active" } else { "tab-button" }}
                        onclick={let switch_tab = switch_tab.clone(); move |_| switch_tab.emit(Tab::Free)}
                    >
                        {"Actions Gratuites"}
                    </button>
                    <button 
                        class={if *active_tab == Tab::Premium { "tab-button active" } else { "tab-button" }}
                        onclick={let switch_tab = switch_tab.clone(); move |_| switch_tab.emit(Tab::Premium)}
                    >
                        {"Actions Premium 🔒"}
                    </button>
                </div>

                <div class="actions-content">
                    {match *active_tab {
                        Tab::Free => html! {
                            <div class="free-actions">
                                <h2>{"Actions de Base pour votre Nœud Lightning"}</h2>
                                <div class="actions-grid">
                                    <div class="action-card">
                                        <h3>{"Équilibrage Simple"}</h3>
                                        <p>{"Rééquilibrez vos canaux en fonction des ratios de base."}</p>
                                        <button class="button">{"Équilibrer"}</button>
                                    </div>
                                    <div class="action-card">
                                        <h3>{"Surveillance des Canaux"}</h3>
                                        <p>{"Vérifiez l'état de vos canaux et identifiez les problèmes évidents."}</p>
                                        <button class="button">{"Analyser"}</button>
                                    </div>
                                    <div class="action-card">
                                        <h3>{"Gestion des Frais"}</h3>
                                        <p>{"Ajustez vos frais selon les recommandations de base."}</p>
                                        <button class="button">{"Ajuster"}</button>
                                    </div>
                                </div>
                            </div>
                        },
                        Tab::Premium => html! {
                            <div class="premium-actions">
                                {if !*is_premium_unlocked {
                                    html! {
                                        <div class="premium-lock">
                                            <div class="premium-marketing">
                                                <h2>{"🚀 Débloquez l'Intelligence Artificielle pour votre Nœud Lightning"}</h2>
                                                <div class="premium-features">
                                                    <div class="feature">
                                                        <h3>{"🤖 IA Avancée"}</h3>
                                                        <p>{"Accédez aux recommandations personnalisées générées par OpenAI et DeepSeek."}</p>
                                                    </div>
                                                    <div class="feature">
                                                        <h3>{"📈 Optimisation Intelligente"}</h3>
                                                        <p>{"Maximisez vos revenus grâce à des stratégies d'optimisation avancées."}</p>
                                                    </div>
                                                    <div class="feature">
                                                        <h3>{"🎯 Analyses Prédictives"}</h3>
                                                        <p>{"Anticipez les tendances du réseau et positionnez-vous stratégiquement."}</p>
                                                    </div>
                                                    <div class="feature">
                                                        <h3>{"⚡ Recommandations en Temps Réel"}</h3>
                                                        <p>{"Recevez des suggestions d'actions basées sur l'analyse en temps réel de votre nœud."}</p>
                                                    </div>
                                                </div>
                                                <div class="premium-cta">
                                                    <p class="price">{"Seulement 10,000 sats"}</p>
                                                    <button 
                                                        class={classes!("button", "premium-button", is_connecting.then(|| "loading"))} 
                                                        onclick={handle_payment}
                                                        disabled={*is_connecting}
                                                    >
                                                        {if *is_connecting {
                                                            "Connexion à Alby..."
                                                        } else {
                                                            "Débloquer l'Accès Premium avec Alby"
                                                        }}
                                                    </button>
                                                    <p class="note">{"Un paiement unique pour un accès permanent aux fonctionnalités premium"}</p>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <div class="premium-content">
                                            <h2>{"Actions Premium Alimentées par l'IA"}</h2>
                                            <div class="actions-grid">
                                                <div class="action-card premium">
                                                    <h3>{"Optimisation IA des Frais"}</h3>
                                                    <p>{"Optimisez vos frais avec l'aide de l'IA pour maximiser vos revenus."}</p>
                                                    <button class="button premium-action">{"Optimiser"}</button>
                                                </div>
                                                <div class="action-card premium">
                                                    <h3>{"Équilibrage Intelligent"}</h3>
                                                    <p>{"Équilibrez vos canaux avec des suggestions basées sur l'apprentissage automatique."}</p>
                                                    <button class="button premium-action">{"Équilibrer"}</button>
                                                </div>
                                                <div class="action-card premium">
                                                    <h3>{"Analyse Prédictive"}</h3>
                                                    <p>{"Obtenez des prédictions sur les tendances futures du réseau."}</p>
                                                    <button class="button premium-action">{"Analyser"}</button>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }}
                            </div>
                        }
                    }}
                </div>
            </div>
        </>
    }
} 