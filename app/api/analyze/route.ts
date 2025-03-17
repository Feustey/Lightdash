import { NextResponse } from 'next/server';
import OpenAI from 'openai';

const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
});

// Recommandations par défaut en cas d'erreur de l'API
const DEFAULT_RECOMMENDATIONS = {
  analysis: `En raison d'une limitation temporaire de l'API, voici des recommandations générales pour votre nœud Lightning :

1. Maintenez une bonne liquidité :
   - Gardez un équilibre entre les fonds entrants et sortants
   - Maintenez une capacité suffisante pour gérer les transactions

2. Optimisez vos frais :
   - Ajustez vos frais en fonction de la concurrence
   - Surveillez les frais du marché pour rester compétitif

3. Gestion des canaux :
   - Maintenez des canaux actifs et bien dimensionnés
   - Fermez les canaux inactifs pour libérer des fonds

4. Sécurité :
   - Maintenez votre nœud à jour
   - Surveillez régulièrement l'activité de vos canaux

Ces recommandations sont basées sur les meilleures pratiques générales. Pour des conseils plus personnalisés, veuillez réessayer plus tard.`
};

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { prompt } = body;

    if (!prompt) {
      return NextResponse.json(
        { error: 'Le prompt est requis' },
        { status: 400 }
      );
    }

    try {
      const completion = await openai.chat.completions.create({
        model: "gpt-3.5-turbo",
        messages: [
          {
            role: "system",
            content: "Vous êtes un assistant spécialisé dans l'analyse des données du réseau Lightning."
          },
          {
            role: "user",
            content: prompt
          }
        ],
        temperature: 0.7,
        max_tokens: 500,
      });

      return NextResponse.json({
        analysis: completion.choices[0].message.content
      });
    } catch (openaiError: any) {
      console.error('Erreur OpenAI:', openaiError);
      
      // Vérifier si c'est une erreur de quota
      if (openaiError?.response?.status === 429) {
        return NextResponse.json(DEFAULT_RECOMMENDATIONS);
      }
      
      // Pour les autres erreurs OpenAI
      return NextResponse.json(DEFAULT_RECOMMENDATIONS);
    }
  } catch (error) {
    console.error('Erreur générale:', error);
    return NextResponse.json(DEFAULT_RECOMMENDATIONS);
  }
} 