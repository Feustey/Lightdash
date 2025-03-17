export interface ChannelRecommendation {
  gain_in_betweenness_rank: number;
  gain_in_closeness_rank: number;
  gain_in_eigenvector_rank: number;
  minimum_viable_capacity: number;
  ideal_capacity: number;
  passive_fee_ppm: number;
  active_fee_ppm: number;
}

export interface NodeRank {
  capacity: number;
  age: number;
  growth: number;
  availability: number;
}

export interface SparkSeerStats {
  mean_channel_capacity: number;
  mean_outbound_base_fee: number;
  mean_outbound_fee_rate: number;
  htlc_response_time_mean: number;
  liquidity_flexibility_score: number;
  effective_outbound_balance: number;
  betweenness_rank: number;
  closeness_rank: number;
  eigenvector_rank: number;
}

export interface Channel {
  publicKey: string;
  alias: string;
  capacity: number;
  channelCount: number;
  nodeRank: NodeRank;
  sparkSeerStats?: SparkSeerStats;
  channelRecommendations?: ChannelRecommendation[];
} 