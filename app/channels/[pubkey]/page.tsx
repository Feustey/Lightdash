"use client";

import * as React from "react";
import { Layout } from "@/components/layout";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { fetchNodeData } from "@/app/utils/sparkseer";
import { Loader2 } from "lucide-react";

interface NodeDetailsProps {
  params: {
    pubkey: string;
  };
}

export default function NodeDetailsPage({ params }: NodeDetailsProps) {
  const [nodeData, setNodeData] = React.useState<any>(null);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    const loadNodeData = async () => {
      try {
        console.log('Chargement des données pour le nœud:', params.pubkey);
        const data = await fetchNodeData(params.pubkey);
        
        if (data) {
          console.log('Données du nœud chargées avec succès:', data);
          setNodeData(data);
        } else {
          console.error('Aucune donnée reçue pour le nœud:', params.pubkey);
          setError("Nœud non trouvé ou données indisponibles");
        }
      } catch (err) {
        console.error('Erreur détaillée lors du chargement des données:', err);
        setError("Erreur lors du chargement des données du nœud. Veuillez réessayer plus tard.");
      } finally {
        setLoading(false);
      }
    };

    if (params.pubkey) {
      loadNodeData();
    } else {
      setError("Clé publique du nœud manquante");
      setLoading(false);
    }
  }, [params.pubkey]);

  if (loading) {
    return (
      <Layout>
        <div className="flex items-center justify-center h-[calc(100vh-4rem)]">
          <Loader2 className="h-8 w-8 animate-spin" />
        </div>
      </Layout>
    );
  }

  if (error) {
    return (
      <Layout>
        <div className="flex items-center justify-center h-[calc(100vh-4rem)]">
          <div className="text-center">
            <h2 className="text-2xl font-bold text-red-500 mb-2">Erreur</h2>
            <p className="text-gray-600">{error}</p>
          </div>
        </div>
      </Layout>
    );
  }

  return (
    <Layout>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold">{nodeData.alias || "Nœud anonyme"}</h1>
            <p className="text-sm text-gray-500">{nodeData.publicKey}</p>
          </div>
        </div>

        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-muted-foreground">
                Capacité totale
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{nodeData.capacity.toFixed(2)} BTC</div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-muted-foreground">
                Nombre de canaux
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{nodeData.channelCount}</div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-muted-foreground">
                Score de connectivité
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{nodeData.nodeRank.capacity}</div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-muted-foreground">
                Âge du nœud
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{nodeData.nodeRank.age} jours</div>
            </CardContent>
          </Card>
        </div>

        <Tabs defaultValue="overview" className="space-y-4">
          <TabsList>
            <TabsTrigger value="overview">Vue d'ensemble</TabsTrigger>
            <TabsTrigger value="channels">Canaux</TabsTrigger>
            <TabsTrigger value="analytics">Analytiques</TabsTrigger>
          </TabsList>

          <TabsContent value="overview" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Informations générales</CardTitle>
                <CardDescription>
                  Détails du nœud et de ses performances
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="grid gap-4 md:grid-cols-2">
                  <div>
                    <h3 className="font-medium mb-2">Statistiques de base</h3>
                    <ul className="space-y-2 text-sm">
                      <li>
                        <span className="text-muted-foreground">Capacité moyenne par canal:</span>{" "}
                        {nodeData.sparkSeerStats?.mean_channel_capacity.toFixed(2)} BTC
                      </li>
                      <li>
                        <span className="text-muted-foreground">Frais de base moyens:</span>{" "}
                        {nodeData.sparkSeerStats?.mean_outbound_base_fee} msats
                      </li>
                      <li>
                        <span className="text-muted-foreground">Taux de frais moyens:</span>{" "}
                        {nodeData.sparkSeerStats?.mean_outbound_fee_rate} ppm
                      </li>
                    </ul>
                  </div>
                  <div>
                    <h3 className="font-medium mb-2">Performance</h3>
                    <ul className="space-y-2 text-sm">
                      <li>
                        <span className="text-muted-foreground">Temps de réponse HTLC moyen:</span>{" "}
                        {nodeData.sparkSeerStats?.htlc_response_time_mean.toFixed(2)} ms
                      </li>
                      <li>
                        <span className="text-muted-foreground">Score de flexibilité:</span>{" "}
                        {nodeData.sparkSeerStats?.liquidity_flexibility_score.toFixed(2)}
                      </li>
                      <li>
                        <span className="text-muted-foreground">Liquidité effective:</span>{" "}
                        {nodeData.sparkSeerStats?.effective_outbound_balance.toFixed(2)} BTC
                      </li>
                    </ul>
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="channels" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Recommandations de canaux</CardTitle>
                <CardDescription>
                  Suggestions d'optimisation basées sur l'analyse du réseau
                </CardDescription>
              </CardHeader>
              <CardContent>
                {nodeData.channelRecommendations?.map((rec: any, index: number) => (
                  <div key={index} className="border-b py-4 last:border-0">
                    <h3 className="font-medium mb-2">Recommandation #{index + 1}</h3>
                    <ul className="space-y-1 text-sm">
                      <li>
                        <span className="text-muted-foreground">Capacité minimale:</span>{" "}
                        {rec.minimum_viable_capacity.toFixed(2)} BTC
                      </li>
                      <li>
                        <span className="text-muted-foreground">Capacité idéale:</span>{" "}
                        {rec.ideal_capacity.toFixed(2)} BTC
                      </li>
                      <li>
                        <span className="text-muted-foreground">Frais passifs recommandés:</span>{" "}
                        {rec.passive_fee_ppm} ppm
                      </li>
                      <li>
                        <span className="text-muted-foreground">Frais actifs recommandés:</span>{" "}
                        {rec.active_fee_ppm} ppm
                      </li>
                    </ul>
                  </div>
                ))}
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="analytics" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Analytiques avancées</CardTitle>
                <CardDescription>
                  Métriques détaillées et analyses du réseau
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="grid gap-4 md:grid-cols-2">
                  <div>
                    <h3 className="font-medium mb-2">Rangs dans le réseau</h3>
                    <ul className="space-y-2 text-sm">
                      <li>
                        <span className="text-muted-foreground">Rang de betweenness:</span>{" "}
                        {nodeData.sparkSeerStats?.betweenness_rank}
                      </li>
                      <li>
                        <span className="text-muted-foreground">Rang de closeness:</span>{" "}
                        {nodeData.sparkSeerStats?.closeness_rank}
                      </li>
                      <li>
                        <span className="text-muted-foreground">Rang d'eigenvector:</span>{" "}
                        {nodeData.sparkSeerStats?.eigenvector_rank}
                      </li>
                    </ul>
                  </div>
                  <div>
                    <h3 className="font-medium mb-2">Métriques de performance</h3>
                    <ul className="space-y-2 text-sm">
                      <li>
                        <span className="text-muted-foreground">Croissance du nœud:</span>{" "}
                        {nodeData.nodeRank.growth}%
                      </li>
                      <li>
                        <span className="text-muted-foreground">Disponibilité:</span>{" "}
                        {nodeData.nodeRank.availability}%
                      </li>
                      <li>
                        <span className="text-muted-foreground">Score de liquidité:</span>{" "}
                        {nodeData.sparkSeerStats?.liquidity_flexibility_score.toFixed(2)}
                      </li>
                    </ul>
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </div>
    </Layout>
  );
} 