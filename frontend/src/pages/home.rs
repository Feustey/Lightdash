use yew::prelude::*;
use crate::components::{Navbar, Card, Chart};
use crate::types::NodeStats;

#[function_component(Home)]
pub fn home() -> Html {
    let stats = use_state(|| NodeStats {
    alias: String::new(),
    avg_channel_size: 0,
    pubkey: String::new(),
    total_capacity: 0,
    });

        <div class="min-h-screen bg-dark">
            <Navbar />
            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <div class="px-4 py-6 sm:px-0">
                        <Card title="Balance locale">
                            <div class="text-2xl font-bold text-white">
                                {format!("{:.2} sats", stats.local_balance)}
                            </div>
                        </Card>
                        <Card title="Balance distante">
                            <div class="text-2xl font-bold text-white">
                                {format!("{:.2} sats", stats.remote_balance)}
                            </div>
                        </Card>
                        <Card title="Capacité totale">
                            <div class="text-2xl font-bold text-white">
                                {format!("{:.2} sats", stats.total_capacity)}
                            </div>
                        </Card>
                        <Card title="Canaux actifs">
                            <div class="text-2xl font-bold text-white">
                                {stats.num_channels}
                            </div>
                        </Card>
                        <Card title="Canaux en attente">
                            <div class="text-2xl font-bold text-white">
                                {stats.num_channels}
                            </div>
                        </Card>
                        <Card title="Canaux inactifs">
                            <div class="text-2xl font-bold text-white">
                                {stats.num_channels}
                            </div>
                        </Card>
                    </div>
                    <div class="mt-8 grid grid-cols-1 gap-6 lg:grid-cols-2">
                        <Card title="Distribution des fonds">
                            <Chart
                                title="Distribution des fonds"
                                data={vec![stats.local_balance as f64, stats.remote_balance as f64]}
                                labels={vec!["Balance locale".to_string(), "Balance distante".to_string()]}
                            />
                        </Card>
                        <Card title="Liquidité par canal">
                            <Chart
                                title="Liquidité par canal"
                                data={vec![stats.total_capacity as f64 / stats.num_channels as f64]}
                                labels={vec!["Capacité moyenne".to_string()]}
                            />
                        </Card>
                    </div>
                </div>
            </main>
        </div>
    }
