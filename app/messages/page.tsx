"use client";

import React from "react";
import { Layout } from "@/components/layout";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { MessageCircle } from "lucide-react";

export default function MessagesPage() {
  return (
    <Layout>
      <div className="container mx-auto p-4">
        <h1 className="text-2xl font-bold mb-4">Messages</h1>
        <Card>
          <CardHeader>
            <div className="flex items-center">
              <MessageCircle className="mr-2 h-5 w-5" />
              <CardTitle>Centre de Messages</CardTitle>
            </div>
            <CardDescription>Communiquez avec d'autres nœuds Lightning</CardDescription>
          </CardHeader>
          <CardContent>
            <p>Contenu de la page Messages à venir...</p>
          </CardContent>
        </Card>
      </div>
    </Layout>
  );
} 