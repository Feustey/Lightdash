"use client";

import * as React from "react";
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from "recharts";
import { Layout } from "@/components/layout";
import { Card } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Activity, Bolt, MessageCircle, Settings, Search, Check } from "lucide-react";
import { Input } from "@/components/ui/input";

interface Channel {
  publicKey: string;
  alias: string;
  capacity: number;
  channelCount: number;
  nodeRank: {
    capacity: number;
    channelCount: number;
    age: number;
    growth: number;
    availability: number;
  };
  addresses: {
    network: string;
    addr: string;
  }[];
  color: string;
}

interface DataPoint {
  name: string;
  revenue: number;
  volume: number;
}

// Fonction pour récupérer les données depuis 1ml.com
const fetchNodeData = async (pubkey: string): Promise<Channel | null> => {
  try {
    const response = await fetch(`https://1ml.com/node/${pubkey}/json`);
    if (!response.ok) {
      throw new Error('Erreur lors de la récupération des données');
    }
    const data = await response.json();
    return {
      publicKey: data.pub_key,
      alias: data.alias,
      capacity: data.capacity / 100000000, // Conversion des sats en BTC
      channelCount: data.channelcount,
      nodeRank: data.noderank,
      addresses: data.addresses,
      color: data.color
    };
  } catch (error) {
    console.error('Erreur:', error);
    return null;
  }
};

export default function Home(): React.ReactElement {
  const [searchQuery, setSearchQuery] = React.useState("");
  const [selectedPubkey, setSelectedPubkey] = React.useState("");
  const [showSuggestions, setShowSuggestions] = React.useState(false);
  const [suggestions, setSuggestions] = React.useState<Channel[]>([]);
  const [selectedChannel, setSelectedChannel] = React.useState<Channel | null>(null);
  const [loading, setLoading] = React.useState(false);

  // Effet pour charger les données quand selectedPubkey change
  React.useEffect(() => {
    const loadChannelData = async () => {
      if (selectedPubkey) {
        setLoading(true);
        const data = await fetchNodeData(selectedPubkey);
        setSelectedChannel(data);
        setLoading(false);
      }
    };
    loadChannelData();
  }, [selectedPubkey]);

  const btcToSats = (btc: number): number => {
    return Math.round(btc * 100000000);
  };

  const formatSats = (sats: number): string => {
    return sats.toLocaleString('fr-FR');
  };

  const handleSearchChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setSearchQuery(value);
    
    // Validation du format de la clé publique (33 caractères hexadécimaux commençant par 02 ou 03)
    if (value.length === 33 && /^[02|03][0-9a-fA-F]{32}$/.test(value)) {
      setLoading(true);
      const data = await fetchNodeData(value);
      if (data) {
        setSuggestions([data]);
      } else {
        setSuggestions([]);
      }
      setLoading(false);
    } else {
      setSuggestions([]);
    }
  };

  const handlePubkeySelect = (pubkey: string) => {
    setSelectedPubkey(pubkey);
    setSearchQuery(pubkey);
    setShowSuggestions(false);
  };

  return (
    <Layout>
      <div className="flex-1 space-y-4 p-4 md:p-8 pt-6">
        <div className="flex items-center justify-between space-y-2">
          <h2 className="text-3xl font-bold tracking-tight">Dashboard</h2>
          <div className="relative w-96">
            <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input
              type="text"
              placeholder="Entrez la clé publique du nœud (33 caractères hexadécimaux)..."
              value={searchQuery}
              onChange={handleSearchChange}
              className="pl-8 font-mono text-sm"
              onFocus={() => setShowSuggestions(true)}
            />
            {showSuggestions && searchQuery && (
              <div className="absolute w-full mt-1 bg-background border rounded-md shadow-lg z-50">
                {loading ? (
                  <div className="px-4 py-2 text-muted-foreground">
                    Chargement...
                  </div>
                ) : suggestions.length > 0 ? (
                  suggestions.map((channel) => (
                    <div
                      key={channel.publicKey}
                      className="px-4 py-2 hover:bg-accent cursor-pointer flex items-center justify-between"
                      onClick={() => handlePubkeySelect(channel.publicKey)}
                    >
                      <div className="flex items-center">
                        <div 
                          className="w-3 h-3 rounded-full mr-2" 
                          style={{ backgroundColor: channel.color }}
                        />
                        <span className="font-medium">{channel.alias}</span>
                      </div>
                      <span className="text-sm text-muted-foreground font-mono">
                        {channel.publicKey.slice(0, 8)}...{channel.publicKey.slice(-8)}
                      </span>
                    </div>
                  ))
                ) : searchQuery.length === 33 && !/^[02|03][0-9a-fA-F]{32}$/.test(searchQuery) ? (
                  <div className="px-4 py-2 text-red-500">
                    Format de clé publique invalide. Doit commencer par 02 ou 03 et contenir 33 caractères hexadécimaux.
                  </div>
                ) : (
                  <div className="px-4 py-2 text-muted-foreground">
                    Aucun nœud trouvé
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
        <Tabs defaultValue="overview" className="space-y-4">
          <TabsList>
            <TabsTrigger value="overview">
              <Activity className="mr-2 h-4 w-4" />
              Overview
            </TabsTrigger>
            <TabsTrigger value="channels">
              <Bolt className="mr-2 h-4 w-4" />
              Channels
            </TabsTrigger>
            <TabsTrigger value="messages">
              <MessageCircle className="mr-2 h-4 w-4" />
              Messages
            </TabsTrigger>
            <TabsTrigger value="settings">
              <Settings className="mr-2 h-4 w-4" />
              Settings
            </TabsTrigger>
          </TabsList>
          <TabsContent value="overview" className="space-y-4">
            {loading ? (
              <div className="flex items-center justify-center h-64">
                <div className="text-muted-foreground">Chargement des données...</div>
              </div>
            ) : selectedChannel ? (
              <>
                <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                  <Card className="p-6">
                    <h3 className="text-sm font-medium">Channel Capacity</h3>
                    <div className="mt-2 text-2xl font-bold">{formatSats(btcToSats(selectedChannel.capacity))} sats</div>
                    <p className="text-xs text-muted-foreground">{selectedChannel.channelCount} active channels</p>
                  </Card>
                  <Card className="p-6">
                    <h3 className="text-sm font-medium">Node Rank</h3>
                    <div className="mt-2 text-2xl font-bold">#{selectedChannel.nodeRank.capacity}</div>
                    <p className="text-xs text-muted-foreground">Capacity ranking</p>
                  </Card>
                  <Card className="p-6">
                    <h3 className="text-sm font-medium">Channel Count Rank</h3>
                    <div className="mt-2 text-2xl font-bold">#{selectedChannel.nodeRank.channelCount}</div>
                    <p className="text-xs text-muted-foreground">Among all nodes</p>
                  </Card>
                  <Card className="p-6">
                    <h3 className="text-sm font-medium">Node Age</h3>
                    <div className="mt-2 text-2xl font-bold">#{selectedChannel.nodeRank.age}</div>
                    <p className="text-xs text-muted-foreground">Age ranking</p>
                  </Card>
                </div>
                <div className="grid gap-4 md:grid-cols-2">
                  <Card className="p-6">
                    <h3 className="text-sm font-medium mb-4">Node Information</h3>
                    <div className="space-y-4">
                      <div>
                        <h4 className="text-sm font-medium">Addresses</h4>
                        <ul className="mt-2 space-y-1">
                          {selectedChannel.addresses.map((addr, index) => (
                            <li key={index} className="text-sm text-muted-foreground">
                              {addr.network}: {addr.addr}
                            </li>
                          ))}
                        </ul>
                      </div>
                      <div>
                        <h4 className="text-sm font-medium">Node Color</h4>
                        <div className="mt-2 flex items-center">
                          <div 
                            className="w-4 h-4 rounded-full mr-2" 
                            style={{ backgroundColor: selectedChannel.color }}
                          />
                          <span className="text-sm text-muted-foreground">{selectedChannel.color}</span>
                        </div>
                      </div>
                    </div>
                  </Card>
                  <Card className="p-6">
                    <h3 className="text-sm font-medium mb-4">Node Rankings</h3>
                    <div className="space-y-4">
                      <div className="space-y-2">
                        <div className="flex items-center">
                          <span className="text-sm font-medium">Growth</span>
                          <span className="ml-auto text-sm">#{selectedChannel.nodeRank.growth}</span>
                        </div>
                        <div className="h-2 bg-secondary rounded-full">
                          <div 
                            className="h-2 bg-primary rounded-full" 
                            style={{ width: `${selectedChannel.nodeRank.growth / 100}%` }} 
                          />
                        </div>
                      </div>
                      <div className="space-y-2">
                        <div className="flex items-center">
                          <span className="text-sm font-medium">Availability</span>
                          <span className="ml-auto text-sm">#{selectedChannel.nodeRank.availability}</span>
                        </div>
                        <div className="h-2 bg-secondary rounded-full">
                          <div 
                            className="h-2 bg-primary rounded-full" 
                            style={{ width: `${selectedChannel.nodeRank.availability / 100}%` }} 
                          />
                        </div>
                      </div>
                    </div>
                  </Card>
                </div>
              </>
            ) : (
              <div className="flex items-center justify-center h-64">
                <div className="text-muted-foreground">Sélectionnez un nœud pour voir ses informations</div>
              </div>
            )}
          </TabsContent>
        </Tabs>
      </div>
    </Layout>
  );
}