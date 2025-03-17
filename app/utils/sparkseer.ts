const SPARKSEER_API_KEY = 'oMxFGpBwDo5ZgCZf5uB48eW71wxBsce5DATe7uDbKGmh6GGZAWvIF2XKvxAlFTev';
const BASE_URL = 'https://api.sparkseer.space/v1';

interface NodeRank {
  capacity: number;
  channelCount: number;
  age: number;
  growth: number;
  availability: number;
}

interface SparkSeerStats {
  total_capacity: number;
  num_channels: number;
  mean_channel_capacity: number;
  median_channel_capacity: number;
  mean_outbound_base_fee: number;
  mean_outbound_fee_rate: number;
  median_outbound_base_fee: number;
  median_outbound_fee_rate: number;
  mean_inbound_base_fee: number;
  mean_inbound_fee_rate: number;
  median_inbound_base_fee: number;
  median_inbound_fee_rate: number;
  betweenness_rank: number;
  closeness_rank: number;
  eigenvector_rank: number;
  weighted_betweenness_rank: number;
  weighted_closeness_rank: number;
  weighted_eigenvector_rank: number;
  htlc_response_time_mean: number;
  htlc_response_time_median: number;
  liquidity_flexibility_score: number;
  effective_outbound_balance: number;
  effective_inbound_balance: number;
}

interface ChannelRecommendation {
  minimum_viable_capacity: number;
  ideal_capacity: number;
  passive_fee_ppm: number;
  active_fee_ppm: number;
}

interface Channel {
  publicKey: string;
  alias: string;
  capacity: number;
  channelCount: number;
  nodeRank: NodeRank;
  addresses: {
    network: string;
    addr: string;
  }[];
  color: string;
  sparkSeerStats?: SparkSeerStats;
  channelRecommendations?: ChannelRecommendation[];
}

interface SparkSeerNodeStats {
  pubkey: string;
  total_capacity: number;
  num_channels: number;
  mean_channel_capacity: number;
  median_channel_capacity: number;
  mean_outbound_base_fee: number;
  mean_outbound_fee_rate: number;
  median_outbound_base_fee: number;
  median_outbound_fee_rate: number;
  mean_inbound_base_fee: number;
  mean_inbound_fee_rate: number;
  median_inbound_base_fee: number;
  median_inbound_fee_rate: number;
  alias: string;
  ipv4: string;
  ipv6: string;
  torv3: string;
  last_update: number;
  betweenness_rank: number;
  closeness_rank: number;
  eigenvector_rank: number;
  weighted_betweenness_rank: number;
  weighted_closeness_rank: number;
  weighted_eigenvector_rank: number;
  htlc_response_time_mean: number;
  htlc_response_time_median: number;
  liquidity_flexibility_score: number;
  effective_outbound_balance: number;
  effective_inbound_balance: number;
}

interface SparkSeerChannelRecommendation {
  pubkey: string;
  info: {
    gain_in_betweenness_rank: number;
    gain_in_closeness_rank: number;
    gain_in_eigenvector_rank: number;
    minimum_viable_capacity: number;
    ideal_capacity: number;
    passive_fee_ppm: number;
    active_fee_ppm: number;
  }[];
}

export interface SparkSeerRecommendation {
  pubkey: string;
  info: {
    gain_in_betweenness_rank: number;
    gain_in_closeness_rank: number;
    gain_in_eigenvector_rank: number;
    minimum_viable_capacity: number;
    ideal_capacity: number;
    passive_fee_ppm: number;
    active_fee_ppm: number;
  }[];
}

export async function fetchSparkSeerNodeStats(pubkey: string): Promise<SparkSeerNodeStats | null> {
  try {
    console.log('Appel API SparkSeer pour le nœud:', pubkey);
    const response = await fetch(`${BASE_URL}/node/current-stats/${pubkey}`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${SPARKSEER_API_KEY}`,
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      mode: 'cors',
      credentials: 'include'
    });

    console.log('Statut de la réponse:', response.status);
    console.log('Headers de la réponse:', Object.fromEntries(response.headers.entries()));

    if (!response.ok) {
      const errorText = await response.text();
      console.error(`Erreur HTTP: ${response.status}`, errorText);
      return null;
    }

    const data = await response.json();
    console.log('Données reçues:', data);
    
    // La réponse est un tableau, on prend le premier élément
    if (Array.isArray(data) && data.length > 0) {
      return data[0];
    }
    
    return null;
  } catch (error) {
    console.error('Erreur détaillée lors de la récupération des données SparkSeer:', error);
    return null;
  }
}

export async function fetchChannelRecommendations(pubkey: string): Promise<SparkSeerChannelRecommendation | null> {
  try {
    const response = await fetch(`${BASE_URL}/recommendations/${pubkey}`, {
      headers: {
        'Authorization': `Bearer ${SPARKSEER_API_KEY}`,
        'Content-Type': 'application/json'
      }
    });

    if (!response.ok) {
      console.error('Erreur lors de la récupération des recommandations:', response.status);
      return null;
    }

    const data = await response.json();
    if (!data || !data.info || data.info.length === 0) {
      return null;
    }

    const recommendation = data.info[0];
    return {
      pubkey: recommendation.pubkey,
      info: [recommendation]
    };
  } catch (error) {
    console.error('Erreur lors de la récupération des recommandations:', error);
    return null;
  }
}

export async function fetchOutboundLiquidityValue(pubkey: string): Promise<any> {
  try {
    const response = await fetch(`${BASE_URL}/services/outbound-liquidity-value`, {
      headers: {
        'api-key': SPARKSEER_API_KEY,
        'Accept': 'application/json',
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      console.error(`Erreur HTTP: ${response.status}`);
      return null;
    }

    const data = await response.json();
    return data.channel_peers.find((peer: any) => peer.pubkey === pubkey) || null;
  } catch (error) {
    console.error('Erreur lors de la récupération de la valeur de liquidité sortante:', error);
    return null;
  }
}

export interface NodeData {
  alias: string;
  publicKey: string;
  capacity: number;
  channelCount: number;
  nodeRank: {
    capacity: number;
    age: number;
    growth: number;
    availability: number;
  };
  sparkSeerStats?: {
    mean_channel_capacity: number;
    mean_outbound_base_fee: number;
    mean_outbound_fee_rate: number;
    htlc_response_time_mean: number;
    liquidity_flexibility_score: number;
    effective_outbound_balance: number;
    betweenness_rank: number;
    closeness_rank: number;
    eigenvector_rank: number;
  };
  channelRecommendations?: Array<{
    minimum_viable_capacity: number;
    ideal_capacity: number;
    passive_fee_ppm: number;
    active_fee_ppm: number;
  }>;
}

export async function fetchNodeData(pubkey: string): Promise<NodeData | null> {
  try {
    console.log('Début de la récupération des données pour le nœud:', pubkey);
    
    const [sparkSeerStats, channelRecommendations] = await Promise.all([
      fetchSparkSeerNodeStats(pubkey),
      fetchChannelRecommendations(pubkey)
    ]);

    console.log('Réponse SparkSeer complète:', sparkSeerStats);
    console.log('Réponse recommandations complète:', channelRecommendations);

    if (!sparkSeerStats) {
      console.error('Impossible de récupérer les statistiques du nœud depuis SparkSeer');
      return null;
    }

    if (!sparkSeerStats.pubkey) {
      console.error('Les données du nœud ne contiennent pas de clé publique');
      return null;
    }

    const nodeData: NodeData = {
      alias: sparkSeerStats.alias || "Nœud inconnu",
      publicKey: pubkey,
      capacity: sparkSeerStats.total_capacity, // En satoshis
      channelCount: sparkSeerStats.num_channels,
      nodeRank: {
        capacity: 0,
        age: 0,
        growth: 0,
        availability: 0
      },
      sparkSeerStats: {
        mean_channel_capacity: sparkSeerStats.mean_channel_capacity, // En satoshis
        mean_outbound_base_fee: sparkSeerStats.mean_outbound_base_fee,
        mean_outbound_fee_rate: sparkSeerStats.mean_outbound_fee_rate,
        htlc_response_time_mean: sparkSeerStats.htlc_response_time_mean,
        liquidity_flexibility_score: sparkSeerStats.liquidity_flexibility_score,
        effective_outbound_balance: sparkSeerStats.effective_outbound_balance, // En pourcentage
        betweenness_rank: sparkSeerStats.betweenness_rank,
        closeness_rank: sparkSeerStats.closeness_rank,
        eigenvector_rank: sparkSeerStats.eigenvector_rank
      },
      channelRecommendations: channelRecommendations?.info || []
    };

    console.log('Données du nœud formatées:', nodeData);
    return nodeData;
  } catch (error) {
    console.error('Erreur détaillée lors de la récupération des données du nœud:', error);
    return null;
  }
}

export async function fetchDefaultNodeData(): Promise<Channel | null> {
  try {
    const testPubkey = '02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b';
    console.log('Appel API SparkSeer pour le nœud:', testPubkey);
    
    const sparkSeerStats = await fetchSparkSeerNodeStats(testPubkey);
    if (!sparkSeerStats) {
      console.error('Impossible de récupérer les statistiques du nœud depuis SparkSeer');
      return null;
    }

    const channelRecommendations = await fetchChannelRecommendations(testPubkey);
    
    const nodeData: Channel = {
      publicKey: testPubkey,
      alias: sparkSeerStats.alias || 'Nœud Lightning',
      capacity: sparkSeerStats.total_capacity,
      channelCount: sparkSeerStats.num_channels,
      nodeRank: {
        capacity: sparkSeerStats.betweenness_rank,
        channelCount: sparkSeerStats.closeness_rank,
        age: 0,
        growth: 0,
        availability: 0
      },
      addresses: [
        {
          network: 'torv3',
          addr: sparkSeerStats.torv3 || ''
        }
      ],
      color: '#000000',
      sparkSeerStats: {
        total_capacity: sparkSeerStats.total_capacity,
        num_channels: sparkSeerStats.num_channels,
        mean_channel_capacity: sparkSeerStats.mean_channel_capacity,
        median_channel_capacity: sparkSeerStats.median_channel_capacity,
        mean_outbound_base_fee: sparkSeerStats.mean_outbound_base_fee,
        mean_outbound_fee_rate: sparkSeerStats.mean_outbound_fee_rate,
        median_outbound_base_fee: sparkSeerStats.median_outbound_base_fee,
        median_outbound_fee_rate: sparkSeerStats.median_outbound_fee_rate,
        mean_inbound_base_fee: sparkSeerStats.mean_inbound_base_fee,
        mean_inbound_fee_rate: sparkSeerStats.mean_inbound_fee_rate,
        median_inbound_base_fee: sparkSeerStats.median_inbound_base_fee,
        median_inbound_fee_rate: sparkSeerStats.median_inbound_fee_rate,
        betweenness_rank: sparkSeerStats.betweenness_rank,
        closeness_rank: sparkSeerStats.closeness_rank,
        eigenvector_rank: sparkSeerStats.eigenvector_rank,
        weighted_betweenness_rank: sparkSeerStats.weighted_betweenness_rank,
        weighted_closeness_rank: sparkSeerStats.weighted_closeness_rank,
        weighted_eigenvector_rank: sparkSeerStats.weighted_eigenvector_rank,
        htlc_response_time_mean: 0,
        htlc_response_time_median: 0,
        liquidity_flexibility_score: sparkSeerStats.liquidity_flexibility_score,
        effective_outbound_balance: sparkSeerStats.effective_outbound_balance,
        effective_inbound_balance: sparkSeerStats.effective_inbound_balance
      },
      channelRecommendations: channelRecommendations ? channelRecommendations.info : []
    };

    console.log('Données du nœud formatées:', nodeData);
    return nodeData;
  } catch (error) {
    console.error('Erreur lors de la récupération des données du nœud:', error);
    return null;
  }
}

export async function fetchSparkSeerRecommendations(pubkey: string): Promise<SparkSeerRecommendation | null> {
  try {
    console.log('Appel API SparkSeer pour les recommandations du nœud:', pubkey);
    const response = await fetch(`${BASE_URL}/recommendations/${pubkey}`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${SPARKSEER_API_KEY}`,
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      mode: 'cors',
      credentials: 'include'
    });

    console.log('Statut de la réponse:', response.status);
    console.log('Headers de la réponse:', Object.fromEntries(response.headers.entries()));

    if (!response.ok) {
      const errorText = await response.text();
      console.error(`Erreur HTTP: ${response.status}`, errorText);
      return null;
    }

    const data = await response.json();
    console.log('Données reçues:', data);
    
    if (!data || !data.info || data.info.length === 0) {
      return null;
    }

    return {
      pubkey: data.pubkey,
      info: data.info
    };
  } catch (error) {
    console.error('Erreur détaillée lors de la récupération des recommandations SparkSeer:', error);
    return null;
  }
} 