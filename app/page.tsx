"use client";

import * as React from "react";
import { Layout } from "@/components/layout";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { generateResponse } from "./utils/openai";
import { fetchNodeData } from "@/app/utils/sparkseer";
import { LightningAuth } from "./components/LightningAuth";
import {
  Activity,
  Bolt,
  MessageCircle,
  Settings,
  Search,
  Check,
  Lightbulb,
  Loader2
} from "lucide-react";
import { Channel } from "@/app/utils/types";

const DEFAULT_PUBKEY = "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f";

interface DataPoint {
  name: string;
  revenue: number;
  volume: number;
}

export default function DashboardPage() {
  const [nodeData, setNodeData] = React.useState<Channel | null>(null);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    const loadNodeData = async () => {
      try {
        console.log('Début du chargement des données du nœud');
        const data = await fetchNodeData(DEFAULT_PUBKEY);
        console.log('Données reçues:', data);
        if (data) {
          setNodeData(data as Channel);
        } else {
          console.error('Aucune donnée reçue du nœud');
        }
      } catch (error) {
        console.error('Erreur lors du chargement des données du nœud:', error);
        setError('Erreur lors du chargement des données du nœud');
      } finally {
        setLoading(false);
      }
    };
    loadNodeData();
  }, []);

  if (loading) {
    return (
      <Layout>
        <div className="flex items-center justify-center h-screen">
          <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-gray-900"></div>
        </div>
      </Layout>
    );
  }

  return (
    <Layout>
      <div className="space-y-6">
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          <div className="rounded-lg border bg-card p-6">
            <h3 className="text-sm font-medium text-muted-foreground">Total des canaux</h3>
            <p className="text-2xl font-bold">{nodeData?.channelCount || 0}</p>
          </div>
          <div className="rounded-lg border bg-card p-6">
            <h3 className="text-sm font-medium text-muted-foreground">Capacité totale</h3>
            <p className="text-2xl font-bold">{nodeData?.capacity.toLocaleString()} sats</p>
          </div>
          <div className="rounded-lg border bg-card p-6">
            <h3 className="text-sm font-medium text-muted-foreground">Score de flexibilité</h3>
            <p className="text-2xl font-bold">
              {nodeData?.sparkSeerStats?.liquidity_flexibility_score 
                ? (nodeData.sparkSeerStats.liquidity_flexibility_score * 100).toFixed(1)
                : 0}%
            </p>
          </div>
          <div className="rounded-lg border bg-card p-6">
            <h3 className="text-sm font-medium text-muted-foreground">Rang de centralité</h3>
            <p className="text-2xl font-bold">
              {nodeData?.sparkSeerStats?.betweenness_rank 
                ? nodeData.sparkSeerStats.betweenness_rank.toFixed(0)
                : "N/A"}
            </p>
          </div>
        </div>

        <div className="grid gap-4 md:grid-cols-2">
          <div className="rounded-lg border bg-card p-6">
            <h3 className="text-lg font-medium">Statistiques des canaux</h3>
            <div className="mt-4 space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Capacité moyenne</p>
                  <p className="text-sm text-muted-foreground">Par canal</p>
                </div>
                <span className="text-sm font-medium">
                  {nodeData?.sparkSeerStats?.mean_channel_capacity 
                    ? nodeData.sparkSeerStats.mean_channel_capacity.toLocaleString()
                    : "N/A"} sats
                </span>
              </div>
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Frais moyens</p>
                  <p className="text-sm text-muted-foreground">Sortants</p>
                </div>
                <span className="text-sm font-medium">
                  {nodeData?.sparkSeerStats?.mean_outbound_fee_rate 
                    ? `${nodeData.sparkSeerStats.mean_outbound_fee_rate} ppm`
                    : "N/A"}
                </span>
              </div>
            </div>
          </div>

          <div className="rounded-lg border bg-card p-6">
            <h3 className="text-lg font-medium">Performance</h3>
            <div className="mt-4 space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Temps de réponse HTLC</p>
                  <p className="text-sm text-muted-foreground">Moyen</p>
                </div>
                <span className="text-sm font-medium">
                  {nodeData?.sparkSeerStats?.htlc_response_time_mean 
                    ? `${nodeData.sparkSeerStats.htlc_response_time_mean.toFixed(2)} ms`
                    : "N/A"}
                </span>
              </div>
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Liquidité effective</p>
                  <p className="text-sm text-muted-foreground">Sortante</p>
                </div>
                <span className="text-sm font-medium">
                  {nodeData?.sparkSeerStats?.effective_outbound_balance 
                    ? `${nodeData.sparkSeerStats.effective_outbound_balance.toFixed(1)}%`
                    : "N/A"}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
}