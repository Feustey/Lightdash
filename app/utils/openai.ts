import OpenAI from 'openai';

// Suppression de l'initialisation du client OpenAI côté client
// qui cause l'erreur de sécurité

export async function generateResponse(prompt: string) {
  try {
    const response = await fetch('/api/analyze', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ prompt }),
    });

    if (!response.ok) {
      throw new Error('Erreur lors de l\'appel à l\'API');
    }

    const data = await response.json();
    return data.analysis;
  } catch (error) {
    console.error('Erreur lors de l\'appel à l\'API:', error);
    throw new Error('Erreur lors de la génération de la réponse');
  }
} 