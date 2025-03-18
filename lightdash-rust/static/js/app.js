// Fonctions utilitaires
const utils = {
    formatSats(sats) {
        return new Intl.NumberFormat('fr-FR').format(sats) + ' sats';
    },

    formatDate(timestamp) {
        return new Date(timestamp * 1000).toLocaleString('fr-FR');
    },

    showNotification(message, type = 'info') {
        const notification = document.createElement('div');
        notification.className = `notification notification-${type} fade-in`;
        notification.textContent = message;
        document.body.appendChild(notification);

        setTimeout(() => {
            notification.classList.remove('fade-in');
            setTimeout(() => notification.remove(), 300);
        }, 3000);
    }
};

// Gestionnaire d'API
const api = {
    async fetch(endpoint, options = {}) {
        try {
            const response = await fetch(`/api/${endpoint}`, {
                ...options,
                headers: {
                    'Content-Type': 'application/json',
                    ...options.headers
                }
            });
            
            if (!response.ok) {
                throw new Error(`Erreur HTTP: ${response.status}`);
            }
            
            return await response.json();
        } catch (error) {
            console.error(`Erreur lors de la récupération des données de ${endpoint}:`, error);
            utils.showNotification(`Erreur lors de la récupération des données: ${error.message}`, 'error');
            throw error;
        }
    },

    // Informations du nœud
    async getNodeInfo() {
        return this.fetch('node/info');
    },

    // Gestion des canaux
    async getChannels() {
        return this.fetch('channels');
    },

    async openChannel(pubkey, capacity) {
        return this.fetch('channels', {
            method: 'POST',
            body: JSON.stringify({ pubkey, capacity })
        });
    },

    async closeChannel(channelId) {
        return this.fetch(`channels/${channelId}`, {
            method: 'DELETE'
        });
    },

    // Gestion des transactions
    async getTransactions() {
        return this.fetch('transactions');
    },

    async sendPayment(paymentRequest) {
        return this.fetch('payments', {
            method: 'POST',
            body: JSON.stringify({ payment_request: paymentRequest })
        });
    },

    async createInvoice(amount, description, expiry) {
        return this.fetch('invoices', {
            method: 'POST',
            body: JSON.stringify({ amount, description, expiry })
        });
    }
};

// Gestionnaire de graphiques
const charts = {
    transactions: null,

    initTransactionsChart() {
        const ctx = document.getElementById('transactions-chart')?.getContext('2d');
        if (!ctx) return;

        this.transactions = new Chart(ctx, {
            type: 'line',
            data: {
                labels: [],
                datasets: [{
                    label: 'Transactions',
                    data: [],
                    borderColor: 'rgb(59, 130, 246)',
                    tension: 0.1
                }]
            },
            options: {
                responsive: true,
                plugins: {
                    legend: {
                        position: 'top',
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true
                    }
                }
            }
        });
    },

    updateTransactionsChart(transactions) {
        if (!this.transactions) return;

        const labels = transactions.map(tx => utils.formatDate(tx.timestamp));
        const data = transactions.map(tx => tx.amount);

        this.transactions.data.labels = labels;
        this.transactions.data.datasets[0].data = data;
        this.transactions.update();
    }
};

// Gestionnaire de l'interface utilisateur
const ui = {
    async updateNodeStats() {
        try {
            const data = await api.getNodeInfo();
            
            document.getElementById('total-capacity').textContent = utils.formatSats(data.capacity);
            document.getElementById('channel-count').textContent = data.channels;
            document.getElementById('node-version').textContent = data.version;
        } catch (error) {
            console.error('Erreur lors de la mise à jour des statistiques:', error);
        }
    },

    async updateChannels() {
        try {
            const channels = await api.getChannels();
            
            const activeChannels = channels.filter(c => c.status === 'Active').length;
            const totalChannels = channels.length;
            const percentage = totalChannels > 0 ? (activeChannels / totalChannels) * 100 : 0;
            
            document.getElementById('active-channels').textContent = `${activeChannels}/${totalChannels}`;
            document.getElementById('active-channels-bar').style.width = `${percentage}%`;
        } catch (error) {
            console.error('Erreur lors de la mise à jour des canaux:', error);
        }
    },

    async updateTransactions() {
        try {
            const transactions = await api.getTransactions();
            
            const recentTransactions = transactions.slice(0, 5);
            const container = document.getElementById('recent-transactions');
            if (container) {
                container.innerHTML = recentTransactions.map(tx => `
                    <div class="flex justify-between items-center">
                        <span class="text-gray-600 dark:text-gray-300">${tx.type_}</span>
                        <span class="text-gray-900 dark:text-white">${utils.formatSats(tx.amount)}</span>
                    </div>
                `).join('');
            }

            charts.updateTransactionsChart(transactions);
        } catch (error) {
            console.error('Erreur lors de la mise à jour des transactions:', error);
        }
    },

    async updateAll() {
        await Promise.all([
            this.updateNodeStats(),
            this.updateChannels(),
            this.updateTransactions()
        ]);
    }
};

// Initialisation
document.addEventListener('DOMContentLoaded', () => {
    charts.initTransactionsChart();
    ui.updateAll();
    setInterval(() => ui.updateAll(), 30000);
}); 