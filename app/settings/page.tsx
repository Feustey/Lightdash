"use client";

import React from "react";
import { Layout } from "@/components/layout";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Settings } from "lucide-react";

export default function SettingsPage() {
  return (
    <Layout>
      <div className="container mx-auto p-4">
        <h1 className="text-2xl font-bold mb-4">Paramètres</h1>
        <Card>
          <CardHeader>
            <div className="flex items-center">
              <Settings className="mr-2 h-5 w-5" />
              <CardTitle>Configuration</CardTitle>
            </div>
            <CardDescription>Personnalisez votre expérience Lightning</CardDescription>
          </CardHeader>
          <CardContent>
            <p>Contenu de la page Paramètres à venir...</p>
          </CardContent>
        </Card>
      </div>
    </Layout>
  );
} 