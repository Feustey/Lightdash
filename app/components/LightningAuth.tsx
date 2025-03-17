import { useState, useEffect } from 'react';
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Bolt, CheckCircle, AlertCircle, TestTube } from "lucide-react";
import { createPaymentRequest, checkPaymentStatus, verifyPayment, PREMIUM_AMOUNT } from "../utils/lightning";
import { isAlbyInstalled, requestAlbyAccess, sendPaymentWithAlby } from "../utils/alby";

interface LightningAuthProps {
  onAuth: (success: boolean, pubkey: string) => void;
  onPremiumAccess: (success: boolean) => void;
}

export function LightningAuth({ onAuth, onPremiumAccess }: LightningAuthProps) {
  const [loading, setLoading] = useState(false);
  const [paid, setPaid] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [hasAlby, setHasAlby] = useState(false);

  useEffect(() => {
    setHasAlby(isAlbyInstalled());
  }, []);

  const handleLightningLogin = async () => {
    setLoading(true);
    setError(null);
    try {
      if (!hasAlby) {
        throw new Error("L'extension Alby n'est pas installée");
      }
      await requestAlbyAccess();
      const mockPubkey = "02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b";
      onAuth(true, mockPubkey);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Erreur lors de l'authentification Lightning");
    } finally {
      setLoading(false);
    }
  };

  const handlePayment = async () => {
    setLoading(true);
    setError(null);
    try {
      if (!hasAlby) {
        throw new Error("L'extension Alby n'est pas installée");
      }

      const invoice = await createPaymentRequest(PREMIUM_AMOUNT);
      await sendPaymentWithAlby(invoice);
      
      const status = await checkPaymentStatus(invoice);
      if (status === 'completed') {
        const verified = await verifyPayment(invoice);
        if (verified) {
          setPaid(true);
          onPremiumAccess(true);
        }
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Erreur lors du paiement");
    } finally {
      setLoading(false);
    }
  };

  const simulatePayment = async () => {
    setLoading(true);
    setError(null);
    try {
      // Simuler un délai pour le paiement
      await new Promise(resolve => setTimeout(resolve, 1500));
      setPaid(true);
      onPremiumAccess(true);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Erreur lors de la simulation du paiement");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-4">
      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Connexion avec Lightning</h2>
        <p className="text-sm text-muted-foreground mb-4">
          Connectez-vous avec votre nœud Lightning pour accéder à toutes les fonctionnalités.
        </p>
        {!hasAlby && (
          <div className="bg-yellow-50 dark:bg-yellow-900/20 p-4 rounded-lg mb-4">
            <div className="flex items-center text-yellow-800 dark:text-yellow-200">
              <AlertCircle className="h-5 w-5 mr-2" />
              <p className="text-sm">
                L'extension Alby est requise pour utiliser cette fonctionnalité.
                <a
                  href="https://getalby.com"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="ml-1 underline"
                >
                  Installer Alby
                </a>
              </p>
            </div>
          </div>
        )}
        <Button
          onClick={handleLightningLogin}
          disabled={loading || !hasAlby}
          className="w-full"
        >
          <Bolt className="mr-2 h-4 w-4" />
          {loading ? "Connexion en cours..." : "Se connecter avec Lightning"}
        </Button>
      </Card>

      <Card className="p-6">
        <h2 className="text-xl font-bold mb-4">Accès Premium</h2>
        <div className="space-y-4">
          <div className="bg-muted p-4 rounded-lg">
            <h3 className="font-medium mb-2">Recommandations d'optimisation</h3>
            <ul className="text-sm space-y-2">
              <li className="flex items-center">
                <CheckCircle className="h-4 w-4 mr-2 text-green-500" />
                Analyse détaillée de votre nœud
              </li>
              <li className="flex items-center">
                <CheckCircle className="h-4 w-4 mr-2 text-green-500" />
                Recommandations personnalisées
              </li>
              <li className="flex items-center">
                <CheckCircle className="h-4 w-4 mr-2 text-green-500" />
                Stratégies d'optimisation des revenus
              </li>
              <li className="flex items-center">
                <CheckCircle className="h-4 w-4 mr-2 text-green-500" />
                Suivi des performances
              </li>
            </ul>
          </div>
          
          <div className="border-t pt-4">
            <p className="text-sm text-muted-foreground mb-2">
              Prix d'accès : {PREMIUM_AMOUNT.toLocaleString()} sats
            </p>

            <div className="space-y-2">
              <Button
                onClick={handlePayment}
                disabled={loading || paid || !hasAlby}
                className="w-full"
              >
                {paid ? "Accès Premium Activé" : "Payer avec Alby"}
              </Button>

              <Button
                onClick={simulatePayment}
                disabled={loading || paid}
                variant="outline"
                className="w-full"
              >
                <TestTube className="mr-2 h-4 w-4" />
                {loading ? "Simulation en cours..." : "Simuler un paiement réussi"}
              </Button>
            </div>
          </div>
        </div>
      </Card>

      {error && (
        <div className="text-red-500 text-sm mt-2">
          {error}
        </div>
      )}
    </div>
  );
} 