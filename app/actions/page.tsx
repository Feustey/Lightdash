"use client";

import { useState } from "react";
import { Layout } from "@/components/layout";
import { Card, CardContent, CardDescription, CardHeader, CardTitle, CardFooter } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Lightbulb, RefreshCw, ArrowRight, Zap } from "lucide-react";
import { generateResponse } from "@/app/utils/openai";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Badge } from "@/components/ui/badge";
import { fetchSparkSeerRecommendations } from "@/app/utils/sparkseer";

export default function ActionsPage() {
  const [recommendations, setRecommendations] = useState<string | null>(null);
  const [sparkSeerRecos, setSparkSeerRecos] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [sparkSeerLoading, setSparkSeerLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [sparkSeerError, setSparkSeerError] = useState<string | null>(null);

  // Données fictives pour la démonstration
  const nodeData = {
    total_capacity: "5,000,000",
    channel_count: "12",
    channels: [
      { pubkey: "03a...b2c", alias: "ACINQ", capacity: "1,200,000" },
      { pubkey: "02f...e7d", alias: "LN+", capacity: "800,000" },
      { pubkey: "03c...a1b", alias: "Bitfinex", capacity: "600,000" }
    ],
    connectivity_score: "85/100",
    betweenness_centrality: "0.72",
    node_alias: "LightningDash",
    node_country: "France",
    avg_base_fee: "1000",
    avg_fee_rate: "200",
    activity_history: "Actif depuis 14 mois",
    inbound_liquidity: "2,800,000",
    outbound_liquidity: "2,200,000",
    rebalance_rate: "15",
    routing_fees_earned: "42,500",
    low_performance_channels: "2",
    high_performance_channels: "4"
  };

  const generateRecommendations = async () => {
    setLoading(true);
    setError(null);
    try {
      const prompt = `Tu es un expert en analyse des performances des nœuds Lightning Network et en optimisation de la rentabilité des canaux. 

### 🔍 Contexte :
Je possède un nœud Lightning et je souhaite optimiser ses performances et sa rentabilité. Voici les données récupérées depuis **1ML** et **Sparkseer** concernant mon nœud :

#### 📡 Données issues de 1ML :
- **Capacité totale** : ${nodeData.total_capacity} sats
- **Nombre de canaux ouverts** : ${nodeData.channel_count}
- **Liste des canaux avec leurs capacités** :
  ${nodeData.channels.map(c => `- ${c.pubkey} (${c.alias}) : ${c.capacity} sats`).join('\n  ')}
- **Score de connectivité** : ${nodeData.connectivity_score}
- **Centralité du nœud dans le graphe du réseau** : ${nodeData.betweenness_centrality}
- **Alias du nœud** : ${nodeData.node_alias}
- **Pays d'hébergement** : ${nodeData.node_country}
- **Politique de frais (fee policy) moyenne** :
  - Base fee : ${nodeData.avg_base_fee} msats
  - Fee rate : ${nodeData.avg_fee_rate} ppm
- **Historique d'activité** : ${nodeData.activity_history}

#### 📊 Données issues de Sparkseer :
- **Flux de liquidité entrants/sortants** :
  - Inbound liquidity : ${nodeData.inbound_liquidity} sats
  - Outbound liquidity : ${nodeData.outbound_liquidity} sats
- **Taux de rebalance automatique effectué** : ${nodeData.rebalance_rate}%
- **Gains générés par les frais de routage** : ${nodeData.routing_fees_earned} sats
- **Canaux sous-performants** : ${nodeData.low_performance_channels}
- **Canaux très actifs** : ${nodeData.high_performance_channels}

### 🎯 Objectif :
- Maximiser la rentabilité en ajustant les fees de manière optimale
- Éviter l'épuisement des liquidités tout en gardant des canaux actifs
- Fermer ou rééquilibrer les canaux peu performants
- Identifier les meilleurs pairs pour ouvrir de nouveaux canaux

### 🚀 Ta mission :
À partir des données ci-dessus, analyse la situation et propose une liste de **recommandations détaillées** classées par priorité.  
Chaque recommandation doit être claire, actionable et justifiée par les données fournies.  

Génère **5 à 10 recommandations pertinentes** et ajoute un **score d'impact** (faible, moyen, élevé) à chaque action.`;

      const result = await generateResponse(prompt);
      setRecommendations(result);
    } catch (err) {
      console.error("Erreur lors de la génération des recommandations:", err);
      setError("Erreur lors de la génération des recommandations. Veuillez réessayer.");
    } finally {
      setLoading(false);
    }
  };

  const fetchSparkSeerData = async () => {
    setSparkSeerLoading(true);
    setSparkSeerError(null);
    try {
      const testPubkey = '02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b';
      const data = await fetchSparkSeerRecommendations(testPubkey);
      if (data) {
        setSparkSeerRecos(data);
      } else {
        setSparkSeerError("Impossible de récupérer les recommandations SparkSeer");
      }
    } catch (err) {
      console.error("Erreur lors de la récupération des recommandations SparkSeer:", err);
      setSparkSeerError("Erreur lors de la récupération des recommandations SparkSeer");
    } finally {
      setSparkSeerLoading(false);
    }
  };

  // Formater les recommandations pour l'affichage
  const formatRecommendations = (text: string) => {
    if (!text) return [];
    
    // Diviser le texte en recommandations individuelles
    const recommendations = text.split(/\d+\.\s+/).filter(Boolean);
    
    return recommendations.map(rec => {
      // Extraire le score d'impact s'il existe
      const impactMatch = rec.match(/Impact\s*:\s*(Faible|Moyen|Élevé)/i);
      const impact = impactMatch ? impactMatch[1].toLowerCase() : "moyen";
      
      return {
        text: rec.trim(),
        impact
      };
    });
  };

  const formattedRecommendations = recommendations ? formatRecommendations(recommendations) : [];

  // Obtenir la couleur du badge en fonction de l'impact
  const getImpactColor = (impact: string) => {
    switch (impact.toLowerCase()) {
      case 'faible':
        return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300';
      case 'moyen':
        return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300';
      case 'élevé':
      case 'elevé':
        return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300';
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
    }
  };

  return (
    <Layout>
      <div className="container mx-auto p-4">
        <h1 className="text-2xl font-bold mb-4">Actions Recommandées</h1>
        
        <Tabs defaultValue="recommendations" className="w-full">
          <TabsList className="mb-4">
            <TabsTrigger value="recommendations">Recommandations</TabsTrigger>
            <TabsTrigger value="sparkseer">Recos SparkSeer</TabsTrigger>
            <TabsTrigger value="data">Données du Nœud</TabsTrigger>
          </TabsList>
          
          <TabsContent value="recommendations">
            <Card>
              <CardHeader>
                <div className="flex items-center justify-between">
                  <div className="flex items-center">
                    <Lightbulb className="mr-2 h-5 w-5" />
                    <CardTitle>Recommandations Intelligentes</CardTitle>
                  </div>
                  <Button 
                    onClick={generateRecommendations} 
                    disabled={loading}
                    className="flex items-center"
                  >
                    {loading ? (
                      <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                    ) : (
                      <RefreshCw className="mr-2 h-4 w-4" />
                    )}
                    {loading ? "Génération..." : "Générer des recommandations"}
                  </Button>
                </div>
                <CardDescription>
                  Optimisez votre nœud Lightning avec des recommandations basées sur l'IA
                </CardDescription>
              </CardHeader>
              <CardContent>
                {error && (
                  <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                    {error}
                  </div>
                )}
                
                {!recommendations && !loading && !error && (
                  <div className="text-center py-8">
                    <Lightbulb className="mx-auto h-12 w-12 text-gray-400 mb-4" />
                    <p className="text-gray-500">
                      Cliquez sur "Générer des recommandations" pour obtenir des conseils personnalisés pour votre nœud Lightning.
                    </p>
                  </div>
                )}
                
                {loading && (
                  <div className="text-center py-8">
                    <RefreshCw className="mx-auto h-12 w-12 text-blue-500 animate-spin mb-4" />
                    <p className="text-gray-500">
                      Analyse des données de votre nœud en cours...
                    </p>
                  </div>
                )}
                
                {formattedRecommendations.length > 0 && (
                  <div className="space-y-4">
                    {formattedRecommendations.map((rec, index) => (
                      <div key={index} className="border rounded-lg p-4 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">
                        <div className="flex items-start">
                          <div className="flex-shrink-0 pt-1">
                            <ArrowRight className="h-4 w-4 text-blue-500" />
                          </div>
                          <div className="ml-3 flex-1">
                            <p className="text-sm">{rec.text}</p>
                            <div className="mt-2">
                              <Badge className={`${getImpactColor(rec.impact)} mt-1`}>
                                Impact: {rec.impact.charAt(0).toUpperCase() + rec.impact.slice(1)}
                              </Badge>
                            </div>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </CardContent>
            </Card>
          </TabsContent>
          
          <TabsContent value="sparkseer">
            <Card>
              <CardHeader>
                <div className="flex items-center justify-between">
                  <div className="flex items-center">
                    <Zap className="mr-2 h-5 w-5" />
                    <CardTitle>Recommandations SparkSeer</CardTitle>
                  </div>
                  <Button 
                    onClick={fetchSparkSeerData} 
                    disabled={sparkSeerLoading}
                    className="flex items-center"
                  >
                    {sparkSeerLoading ? (
                      <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                    ) : (
                      <RefreshCw className="mr-2 h-4 w-4" />
                    )}
                    {sparkSeerLoading ? "Chargement..." : "Actualiser"}
                  </Button>
                </div>
                <CardDescription>
                  Recommandations basées sur l'analyse des données SparkSeer
                </CardDescription>
              </CardHeader>
              <CardContent>
                {sparkSeerError && (
                  <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                    {sparkSeerError}
                  </div>
                )}
                
                {!sparkSeerRecos && !sparkSeerLoading && !sparkSeerError && (
                  <div className="text-center py-8">
                    <Zap className="mx-auto h-12 w-12 text-gray-400 mb-4" />
                    <p className="text-gray-500">
                      Cliquez sur "Actualiser" pour charger les recommandations SparkSeer.
                    </p>
                  </div>
                )}
                
                {sparkSeerLoading && (
                  <div className="text-center py-8">
                    <RefreshCw className="mx-auto h-12 w-12 text-blue-500 animate-spin mb-4" />
                    <p className="text-gray-500">
                      Chargement des recommandations SparkSeer...
                    </p>
                  </div>
                )}
                
                {sparkSeerRecos && (
                  <div className="space-y-4">
                    {sparkSeerRecos.info.map((reco: any, index: number) => (
                      <div key={index} className="border rounded-lg p-4 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">
                        <div className="flex items-start">
                          <div className="flex-shrink-0 pt-1">
                            <ArrowRight className="h-4 w-4 text-blue-500" />
                          </div>
                          <div className="ml-3 flex-1">
                            <div className="grid grid-cols-2 gap-4">
                              <div>
                                <p className="text-sm font-medium">Capacité minimale</p>
                                <p className="text-sm text-gray-600 dark:text-gray-400">
                                  {reco.minimum_viable_capacity.toLocaleString()} sats
                                </p>
                              </div>
                              <div>
                                <p className="text-sm font-medium">Capacité idéale</p>
                                <p className="text-sm text-gray-600 dark:text-gray-400">
                                  {reco.ideal_capacity.toLocaleString()} sats
                                </p>
                              </div>
                              <div>
                                <p className="text-sm font-medium">Frais passifs</p>
                                <p className="text-sm text-gray-600 dark:text-gray-400">
                                  {reco.passive_fee_ppm} ppm
                                </p>
                              </div>
                              <div>
                                <p className="text-sm font-medium">Frais actifs</p>
                                <p className="text-sm text-gray-600 dark:text-gray-400">
                                  {reco.active_fee_ppm} ppm
                                </p>
                              </div>
                            </div>
                            <div className="mt-4">
                              <p className="text-sm font-medium">Gains potentiels</p>
                              <div className="flex gap-2 mt-2">
                                <div className="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300">
                                  Betweenness: +{reco.gain_in_betweenness_rank}
                                </div>
                                <div className="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300">
                                  Closeness: +{reco.gain_in_closeness_rank}
                                </div>
                                <div className="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-300">
                                  Eigenvector: +{reco.gain_in_eigenvector_rank}
                                </div>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </CardContent>
            </Card>
          </TabsContent>
          
          <TabsContent value="data">
            <Card>
              <CardHeader>
                <CardTitle>Données de votre nœud Lightning</CardTitle>
                <CardDescription>
                  Informations utilisées pour générer les recommandations
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="border rounded-lg p-4">
                    <h3 className="font-medium mb-2">Données 1ML</h3>
                    <ul className="space-y-1 text-sm">
                      <li><span className="font-medium">Capacité totale:</span> {nodeData.total_capacity} sats</li>
                      <li><span className="font-medium">Nombre de canaux:</span> {nodeData.channel_count}</li>
                      <li><span className="font-medium">Score de connectivité:</span> {nodeData.connectivity_score}</li>
                      <li><span className="font-medium">Centralité:</span> {nodeData.betweenness_centrality}</li>
                      <li><span className="font-medium">Base fee:</span> {nodeData.avg_base_fee} msats</li>
                      <li><span className="font-medium">Fee rate:</span> {nodeData.avg_fee_rate} ppm</li>
                    </ul>
                  </div>
                  <div className="border rounded-lg p-4">
                    <h3 className="font-medium mb-2">Données Sparkseer</h3>
                    <ul className="space-y-1 text-sm">
                      <li><span className="font-medium">Liquidité entrante:</span> {nodeData.inbound_liquidity} sats</li>
                      <li><span className="font-medium">Liquidité sortante:</span> {nodeData.outbound_liquidity} sats</li>
                      <li><span className="font-medium">Taux de rebalance:</span> {nodeData.rebalance_rate}%</li>
                      <li><span className="font-medium">Frais de routage gagnés:</span> {nodeData.routing_fees_earned} sats</li>
                      <li><span className="font-medium">Canaux sous-performants:</span> {nodeData.low_performance_channels}</li>
                      <li><span className="font-medium">Canaux très actifs:</span> {nodeData.high_performance_channels}</li>
                    </ul>
                  </div>
                </div>
                <div className="mt-4 border rounded-lg p-4">
                  <h3 className="font-medium mb-2">Canaux</h3>
                  <div className="overflow-x-auto">
                    <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                      <thead>
                        <tr>
                          <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Alias</th>
                          <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Pubkey</th>
                          <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Capacité</th>
                        </tr>
                      </thead>
                      <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                        {nodeData.channels.map((channel, index) => (
                          <tr key={index}>
                            <td className="px-4 py-2 text-sm">{channel.alias}</td>
                            <td className="px-4 py-2 text-sm font-mono">{channel.pubkey}</td>
                            <td className="px-4 py-2 text-sm">{channel.capacity} sats</td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
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