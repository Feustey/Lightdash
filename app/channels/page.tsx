"use client";

import { useState, useEffect } from "react";
import { usePathname } from "next/navigation";
import { Layout } from "@/components/layout";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { 
  Zap,
  AlertTriangle,
  ArrowUpRight,
  ArrowDownRight,
  RefreshCw,
  Loader2
} from "lucide-react";
import { fetchDefaultNodeData } from "@/app/utils/sparkseer";

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
  sparkSeerStats?: {
    effective_outbound_balance: number;
    effective_inbound_balance: number;
    liquidity_flexibility_score: number;
  };
}

export default function ChannelsPage() {
  const [channel, setChannel] = useState<Channel | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const pathname = usePathname();

  useEffect(() => {
    const loadChannel = async () => {
      try {
        const data = await fetchDefaultNodeData();
        if (data) {
          setChannel(data);
        }
      } catch (err) {
        setError("Erreur lors du chargement des données du nœud");
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    loadChannel();
  }, []);

  const handleRefresh = async () => {
    setLoading(true);
    try {
      const data = await fetchDefaultNodeData();
      if (data) {
        setChannel(data);
      }
    } catch (err) {
      setError("Erreur lors de l'actualisation des données");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <Layout>
        <div className="flex items-center justify-center h-screen">
          <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-gray-900"></div>
        </div>
      </Layout>
    );
  }

  if (error) {
    return (
      <Layout>
        <div className="flex items-center justify-center h-screen">
          <div className="text-red-500">{error}</div>
        </div>
      </Layout>
    );
  }

  if (!channel) {
    return (
      <Layout>
        <div className="flex items-center justify-center h-screen">
          <div className="text-gray-500">Aucune donnée disponible</div>
        </div>
      </Layout>
    );
  }

  const isActive = channel.sparkSeerStats?.liquidity_flexibility_score && 
                   channel.sparkSeerStats.liquidity_flexibility_score > 0.7;

  return (
    <Layout>
      <div className="container mx-auto p-4 space-y-6">
        <div className="flex justify-between items-center">
          <h1 className="text-2xl font-bold">Gestion des Canaux</h1>
          <Button onClick={handleRefresh}>
            <RefreshCw className="mr-2 h-4 w-4" />
            Actualiser
          </Button>
        </div>

        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">Total des canaux</CardTitle>
              <Zap className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{channel.channelCount}</div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">État du nœud</CardTitle>
              {isActive ? (
                <ArrowUpRight className="h-4 w-4 text-green-500" />
              ) : (
                <ArrowDownRight className="h-4 w-4 text-red-500" />
              )}
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">
                {isActive ? "Actif" : "Action requise"}
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">Score de flexibilité</CardTitle>
              <AlertTriangle className="h-4 w-4 text-yellow-500" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">
                {channel.sparkSeerStats?.liquidity_flexibility_score 
                  ? (channel.sparkSeerStats.liquidity_flexibility_score * 100).toFixed(1)
                  : 0}%
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">Capacité totale</CardTitle>
              <Zap className="h-4 w-4 text-blue-500" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">
                {channel.capacity.toFixed(2)} BTC
              </div>
            </CardContent>
          </Card>
        </div>

        <Tabs defaultValue="active" className="space-y-4">
          <TabsList>
            <TabsTrigger value="active">Informations du Nœud</TabsTrigger>
            <TabsTrigger value="needs-action">Actions Nécessaires</TabsTrigger>
          </TabsList>

          <TabsContent value="active" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Informations du Nœud</CardTitle>
                <CardDescription>
                  Détails du nœud {channel.alias}
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div className="border rounded-lg p-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <h3 className="font-medium">{channel.alias}</h3>
                        <p className="text-sm text-muted-foreground">{channel.publicKey}</p>
                      </div>
                      <div className={`ml-2 px-2 py-1 rounded-full ${
                        isActive ? "bg-green-500" : "bg-red-500"
                      } text-white text-sm`}>
                        {isActive ? "Actif" : "Action requise"}
                      </div>
                    </div>
                    <div className="mt-4 grid grid-cols-2 gap-4">
                      <div>
                        <p className="text-sm text-muted-foreground">Capacité</p>
                        <p className="font-medium">{channel.capacity.toFixed(2)} BTC</p>
                      </div>
                      <div>
                        <p className="text-sm text-muted-foreground">Score de flexibilité</p>
                        <p className="font-medium">
                          {channel.sparkSeerStats?.liquidity_flexibility_score 
                            ? (channel.sparkSeerStats.liquidity_flexibility_score * 100).toFixed(1)
                            : "N/A"}%
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="needs-action" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Actions Nécessaires</CardTitle>
                <CardDescription>
                  Recommandations pour améliorer la performance du nœud
                </CardDescription>
              </CardHeader>
              <CardContent>
                {!isActive ? (
                  <div className="space-y-4">
                    <div className="border rounded-lg p-4">
                      <div className="flex items-center justify-between">
                        <div>
                          <h3 className="font-medium">Amélioration de la liquidité</h3>
                          <p className="text-sm text-muted-foreground">
                            Le score de flexibilité actuel est inférieur à 70%
                          </p>
                        </div>
                        <div className="ml-2 px-2 py-1 rounded-full bg-red-500 text-white text-sm">
                          Action requise
                        </div>
                      </div>
                      <div className="mt-4">
                        <p className="text-sm text-muted-foreground">
                          Recommandations :
                        </p>
                        <ul className="mt-2 space-y-2">
                          <li className="text-sm">• Augmenter la capacité des canaux existants</li>
                          <li className="text-sm">• Ouvrir de nouveaux canaux avec des nœuds bien connectés</li>
                          <li className="text-sm">• Rééquilibrer la liquidité entre les canaux</li>
                        </ul>
                      </div>
                    </div>
                  </div>
                ) : (
                  <div className="text-center py-8 text-muted-foreground">
                    Aucune action requise pour le moment
                  </div>
                )}
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </div>
    </Layout>
  );
} 