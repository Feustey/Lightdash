"use client";

import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from "recharts";
import { Layout } from "@/components/layout";
import { Card } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Activity, Bolt, MessageCircle, Settings } from "lucide-react";

const mockData = [
  { name: "00:00", revenue: 400, volume: 2400 },
  { name: "04:00", revenue: 300, volume: 1398 },
  { name: "08:00", revenue: 200, volume: 9800 },
  { name: "12:00", revenue: 278, volume: 3908 },
  { name: "16:00", revenue: 189, volume: 4800 },
  { name: "20:00", revenue: 239, volume: 3800 },
];

export default function Home() {
  return (
    <Layout>
      <div className="flex-1 space-y-4 p-4 md:p-8 pt-6">
        <div className="flex items-center justify-between space-y-2">
          <h2 className="text-3xl font-bold tracking-tight">Dashboard</h2>
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
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
              <Card className="p-6">
                <h3 className="text-sm font-medium">Total Revenue (24h)</h3>
                <div className="mt-2 text-2xl font-bold">₿ 0.00123</div>
                <p className="text-xs text-muted-foreground">+20.1% from last 24h</p>
              </Card>
              <Card className="p-6">
                <h3 className="text-sm font-medium">Channel Capacity</h3>
                <div className="mt-2 text-2xl font-bold">₿ 1.234</div>
                <p className="text-xs text-muted-foreground">32 active channels</p>
              </Card>
              <Card className="p-6">
                <h3 className="text-sm font-medium">Forward Success Rate</h3>
                <div className="mt-2 text-2xl font-bold">98.2%</div>
                <p className="text-xs text-muted-foreground">Last 1000 attempts</p>
              </Card>
              <Card className="p-6">
                <h3 className="text-sm font-medium">Avg Fee Rate</h3>
                <div className="mt-2 text-2xl font-bold">324 ppm</div>
                <p className="text-xs text-muted-foreground">Across all channels</p>
              </Card>
            </div>
            <div className="grid gap-4 md:grid-cols-2">
              <Card className="p-6">
                <h3 className="text-sm font-medium mb-4">Revenue vs Volume</h3>
                <LineChart width={500} height={300} data={mockData}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="name" />
                  <YAxis yAxisId="left" />
                  <YAxis yAxisId="right" orientation="right" />
                  <Tooltip />
                  <Legend />
                  <Line yAxisId="left" type="monotone" dataKey="revenue" stroke="#8884d8" />
                  <Line yAxisId="right" type="monotone" dataKey="volume" stroke="#82ca9d" />
                </LineChart>
              </Card>
              <Card className="p-6">
                <h3 className="text-sm font-medium mb-4">Channel Health</h3>
                <div className="space-y-8">
                  <div className="space-y-2">
                    <div className="flex items-center">
                      <span className="text-sm font-medium">Channel Uptime</span>
                      <span className="ml-auto text-sm">99.9%</span>
                    </div>
                    <div className="h-2 bg-secondary rounded-full">
                      <div className="h-2 bg-primary rounded-full" style={{ width: "99.9%" }} />
                    </div>
                  </div>
                  <div className="space-y-2">
                    <div className="flex items-center">
                      <span className="text-sm font-medium">Balanced Liquidity</span>
                      <span className="ml-auto text-sm">76%</span>
                    </div>
                    <div className="h-2 bg-secondary rounded-full">
                      <div className="h-2 bg-primary rounded-full" style={{ width: "76%" }} />
                    </div>
                  </div>
                </div>
              </Card>
            </div>
          </TabsContent>
        </Tabs>
      </div>
    </Layout>
  );
}