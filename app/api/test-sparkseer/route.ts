import { NextResponse } from 'next/server';
import { fetchSparkSeerNodeStats } from '@/app/utils/sparkseer';

export async function GET() {
  try {
    const testPubkey = '02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b';
    console.log('Test de l\'API SparkSeer pour le nœud:', testPubkey);
    
    const data = await fetchSparkSeerNodeStats(testPubkey);
    
    if (!data) {
      console.error('Aucune donnée reçue de l\'API SparkSeer');
      return NextResponse.json({ 
        success: false, 
        error: 'Aucune donnée reçue de l\'API SparkSeer' 
      }, { status: 500 });
    }

    return NextResponse.json({ 
      success: true, 
      data,
      timestamp: new Date().toISOString()
    });
  } catch (error) {
    console.error('Erreur lors du test de l\'API SparkSeer:', error);
    return NextResponse.json({ 
      success: false, 
      error: error instanceof Error ? error.message : 'Erreur inconnue' 
    }, { status: 500 });
  }
} 