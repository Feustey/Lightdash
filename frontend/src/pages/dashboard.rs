use crate::components::chart::ChartComponent;
use web_sys::ChartType;
use crate::services::sparkseer::SparkSeerService;
use crate::types::NodeStats;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub struct Dashboard {
    node_stats: NodeStats,
    loading: bool,
    error: Option<String>,
}

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let node_stats = NodeStats {
            pubkey: String::new(),
            alias: String::new(),
            capacity: 0.0,
            total_channels: 0,
            fee_rates: vec![],
            routing_fees: vec![],
            routing_volume: vec![],
            last_update: String::new(),
            uptime: 0.0,
            avg_fee_rate: 0.0,
            total_routing_fees: 0.0,
            total_routing_volume: 0.0,
        };

        let service = SparkSeerService::new();
        let node_pubkey = "YOUR_NODE_PUBKEY".to_string(); // À remplacer par la vraie clé publique

        spawn_local(async move {
            if let Ok(stats) = service.get_node_stats(&node_pubkey).await {
                // Mettre à jour les stats
            }
        });

        Self {
            node_stats,
            loading: true,
            error: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="dashboard">
                <h1>{"Tableau de bord"}</h1>
                
                if self.loading {
                    <div class="loading">{"Chargement..."}</div>
                } else if let Some(error) = &self.error {
                    <div class="error">{error}</div>
                } else {
                    <div class="node-info">
                        <h2>{&self.node_stats.alias}</h2>
                        <p>{"Clé publique: "}{&self.node_stats.pubkey}</p>
                        <p>{"Dernière mise à jour: "}{&self.node_stats.last_update}</p>
                        <p>{"Uptime: "}{format!("{:.2}%", self.node_stats.uptime)}</p>
                    </div>

                    <div class="stats-grid">
                        <div class="stat-card">
                            <h3>{"Capacité totale"}</h3>
                            <p>{format!("{:.2} BTC", self.node_stats.capacity)}</p>
                        </div>
                        <div class="stat-card">
                            <h3>{"Canaux actifs"}</h3>
                            <p>{self.node_stats.total_channels}</p>
                        </div>
                        <div class="stat-card">
                            <h3>{"Frais de routage totaux"}</h3>
                            <p>{format!("{:.2} sats", self.node_stats.total_routing_fees)}</p>
                        </div>
                        <div class="stat-card">
                            <h3>{"Volume de routage"}</h3>
                            <p>{format!("{:.2} BTC", self.node_stats.total_routing_volume)}</p>
                        </div>
                    </div>

                    <div class="charts-container">
                        <div class="chart-wrapper">
                            <h3>{"Capacité et Canaux"}</h3>
                            <ChartComponent
                                data={vec![self.node_stats.capacity, self.node_stats.total_channels as f64]}
                                labels={vec!["Capacité (BTC)".to_string(), "Canaux".to_string()]}
                                title="Statistiques du nœud".to_string()
                                chart_type={ChartType::Bar}
                            />
                        </div>
                        <div class="chart-wrapper">
                            <h3>{"Frais de routage"}</h3>
                            <ChartComponent
                                data={self.node_stats.routing_fees.clone()}
                                labels={self.node_stats.routing_fees.iter().enumerate().map(|(i, _)| format!("Jour {}", i + 1)).collect()}
                                title="Frais de routage quotidiens".to_string()
                                chart_type={ChartType::Line}
                            />
                        </div>
                        <div class="chart-wrapper">
                            <h3>{"Volume de routage"}</h3>
                            <ChartComponent
                                data={self.node_stats.routing_volume.clone()}
                                labels={self.node_stats.routing_volume.iter().enumerate().map(|(i, _)| format!("Jour {}", i + 1)).collect()}
                                title="Volume de routage quotidien".to_string()
                                chart_type={ChartType::Line}
                            />
                        </div>
                        <div class="chart-wrapper">
                            <h3>{"Taux de frais"}</h3>
                            <ChartComponent
                                data={self.node_stats.fee_rates.clone()}
                                labels={self.node_stats.fee_rates.iter().enumerate().map(|(i, _)| format!("Jour {}", i + 1)).collect()}
                                title="Taux de frais quotidiens".to_string()
                                chart_type={ChartType::Line}
                            />
                        </div>
                    </div>
                }
            </div>
        }
    }
} 