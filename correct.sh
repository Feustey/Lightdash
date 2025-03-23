#!/bin/bash

# Script pour corriger les erreurs dans le projet Yew/Rust
echo "Début des corrections..."

# Correction principale de l'erreur fold avec pattern incorrect dans chart.rs
echo "Correction de l'erreur 'expected a pattern, found a method call' dans chart.rs..."
# La ligne problématique est:
# let max_value = props.data.iter().fold(0.0f64, |a, (0.0f64, |a, (0.0f64, |a, (0.0, |a, &b| a.max(b))b| a.max(b))b| a.max(b))b| a.max(b));
# Cette ligne est très mal formée et contient plusieurs erreurs de syntaxe imbriquées
# La correction consiste à remplacer complètement par une version correcte:
sed -i '' 's/let max_value = props.data.iter().fold(0.0f64, |a, (0.0f64, |a, (0.0f64, |a, (0.0, |a, &b| a.max(b))b| a.max(b))b| a.max(b))b| a.max(b));/let max_value = props.data.iter().fold(0.0f64, |a, &b| a.max(b));/g' frontend/src/components/chart.rs

# Correction de l'erreur unexpected closing delimiter: `}` dans home.rs
echo "Correction de l'erreur 'unexpected closing delimiter: `}`' dans home.rs..."
# Supprimer l'accolade fermante en trop à la ligne 74
sed -i '' '74d' frontend/src/pages/home.rs

# 1. Correction des erreurs liées à Link<Route> dans navbar.rs
# Problème: La route doit implémenter Routable et le composant Link a besoin des paramètres corrects
echo "Correction des erreurs de Link<Route> dans navbar.rs..."
sed -i '' 's/<Link<Route>/<Link<AppRoute>/g' frontend/src/components/navbar.rs
sed -i '' 's/<\/Link<Route>>/<\/Link<AppRoute>>/g' frontend/src/components/navbar.rs

# 2. Correction de l'erreur JsCast dans chart.rs
echo "Correction de l'erreur JsCast dans chart.rs..."
# Ajouter l'import manquant au début du fichier
sed -i '' '1s/^/use wasm_bindgen::JsCast;\n/' frontend/src/components/chart.rs

# 3. Correction de l'erreur "can't call method `max` on ambiguous numeric type" dans chart.rs
echo "Correction de l'erreur de type float dans chart.rs..."
sed -i '' 's/(0.0, |a, &b| a.max(b))/(0.0f64, |a, &b| a.max(b))/g' frontend/src/components/chart.rs

# 4. Correction des erreurs de noms de champs dans NodeStats
echo "Correction des erreurs de noms de champs dans dashboard.rs..."
sed -i '' 's/stats.total_local_balance/stats.local_balance/g' frontend/src/pages/dashboard.rs
sed -i '' 's/stats.total_remote_balance/stats.remote_balance/g' frontend/src/pages/dashboard.rs

# 5. Correction des erreurs de propriété options sur ChartProps
echo "Correction des erreurs de propriété options sur ChartProps..."
# Modifier la structure ChartProps pour ajouter un champ options
sed -i '' '/pub title: String,/a\
    pub options: Option<serde_json::Value>,' frontend/src/components/mod.rs

# 6. Correction des erreurs dans alby.rs
echo "Correction des erreurs dans alby.rs..."
# Remplacer total_local_balance par local_balance
sed -i '' 's/total_local_balance: 0.0/local_balance: 0.0/g' frontend/src/pages/alby.rs
# Remplacer total_remote_balance par remote_balance
sed -i '' 's/total_remote_balance: 0.0/remote_balance: 0.0/g' frontend/src/pages/alby.rs
# Corriger le type de total_capacity
sed -i '' 's/total_capacity: 0.0/total_capacity: 0/g' frontend/src/pages/alby.rs
# Supprimer la ligne num_active_channels qui n'existe pas dans NodeStats
sed -i '' '/num_active_channels: 0,/d' frontend/src/pages/alby.rs
# Corriger le type de avg_channel_size
sed -i '' 's/avg_channel_size: 0.0/avg_channel_size: 0/g' frontend/src/pages/alby.rs

# 7. Correction des erreurs dans home.rs (champs dupliqués)
echo "Correction des champs dupliqués dans home.rs..."
# On ne garde que la première instance de chaque champ dupliqué
sed -i '' '12,15d' frontend/src/pages/home.rs
sed -i '' '18d' frontend/src/pages/home.rs

# 8. Correction des erreurs de type dans les filtres
echo "Correction des erreurs de type dans les filtres (recommendations.rs et channels.rs)..."
# Ajout des annotations de type dans recommendations.rs
sed -i '' 's/\.filter(|rec| {/\.filter(|rec: \&\&ChannelRecommendation| {/g' frontend/src/pages/recommendations.rs

# Ajout des annotations de type dans channels.rs
sed -i '' 's/\.filter(|channel| {/\.filter(|channel: \&\&Channel| {/g' frontend/src/pages/channels.rs

# 9. Correction des erreurs de Navbar et propriétés current_page
echo "Correction des erreurs de Navbar et propriétés current_page..."
# Ajout du champ current_page dans les propriétés Navbar
sed -i '' '/pub struct NavbarProps/a\
    pub current_page: AppRoute,' frontend/src/components/mod.rs

# Modification dans channels.rs pour utiliser AppRoute au lieu de Route
sed -i '' 's/<Navbar current_page={Route::Channels} \/>/<Navbar current_page={AppRoute::Channels} \/>/g' frontend/src/pages/channels.rs

# 10. Correction des erreurs de SearchInputProps
echo "Correction des propriétés SearchInputProps..."
# Ajout du champ value dans SearchInputProps
sed -i '' '/pub struct SearchInputProps/a\
    pub value: String,' frontend/src/components/mod.rs

# 11. Implémentation de Default pour NodeStats et Action
echo "Implémentation de Default pour NodeStats et Action..."
# Ajout de l'implémentation de Default pour NodeStats
cat >> frontend/src/types.rs << 'EOF'

impl Default for NodeStats {
    fn default() -> Self {
        Self {
            alias: String::new(),
            pubkey: String::new(),
            local_balance: 0.0,
            remote_balance: 0.0,
            total_capacity: 0,
            avg_channel_size: 0,
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Self {
            id: String::new(),
            action_type: String::new(),
            node_alias: String::new(),
            description: String::new(),
            amount: 0.0,
            status: String::new(),
            created_at: String::new(),
        }
    }
}
EOF

# 12. Correction des erreurs de retour dans les fonctions spawn_local
echo "Correction des erreurs de retour dans les fonctions spawn_local..."
# Correction dans les fonctions spawn_local pour gérer les erreurs
sed -i '' '/spawn_local(async move {/,/});/s/stats\.unwrap_or_else(|_| NodeStats::default())/let _ = stats.unwrap_or_else(|_| NodeStats::default()); ()/g' frontend/src/services/mod.rs
sed -i '' '/spawn_local(async move {/,/});/s/channels\.unwrap_or_else(|_| vec!\[\])/let _ = channels.unwrap_or_else(|_| vec!\[\]); ()/g' frontend/src/services/mod.rs
sed -i '' '/spawn_local(async move {/,/});/s/actions\.unwrap_or_else(|_| vec!\[\])/let _ = actions.unwrap_or_else(|_| vec!\[\]); ()/g' frontend/src/services/mod.rs
sed -i '' '/spawn_local(async move {/,/});/s/action\.unwrap_or_else(|_| Action::default())/let _ = action.unwrap_or_else(|_| Action::default()); ()/g' frontend/src/services/mod.rs
sed -i '' '/spawn_local(async move {/,/});/s/recommendations\.unwrap_or_else(|_| vec!\[\])/let _ = recommendations.unwrap_or_else(|_| vec!\[\]); ()/g' frontend/src/services/mod.rs

# 13. Correction des erreurs de unwrap_or_else sur des valeurs qui ne sont pas des Result/Option
echo "Correction des erreurs de unwrap_or_else..."
# Remplacer les unwrap_or_else inappropriés par des approches alternatives
sed -i '' 's/actions\.unwrap_or_else(|_| vec!\[\])/actions/g' frontend/src/services/mod.rs
sed -i '' 's/stats\.unwrap_or_else(|_| NodeStats::default())/stats/g' frontend/src/services/mod.rs
sed -i '' 's/channels\.unwrap_or_else(|_| vec!\[\])/channels/g' frontend/src/services/mod.rs
sed -i '' 's/recommendations\.unwrap_or_else(|_| vec!\[\])/recommendations/g' frontend/src/services/mod.rs
sed -i '' 's/action\.unwrap_or_else(|_| Action::default())/action/g' frontend/src/services/mod.rs

# 14. Création d'un fichier routes.rs avec l'enum AppRoute (Routable)
echo "Création d'un fichier routes.rs avec l'enum AppRoute..."
if ! grep -q "pub enum AppRoute" frontend/src/routes.rs 2>/dev/null; then
    cat > frontend/src/routes.rs << 'EOF'
use yew_router::Routable;

#[derive(Clone, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/channels")]
    Channels,
    #[at("/transactions")]
    Transactions,
    #[at("/yields")]
    Yields,
    #[at("/recommendations")]
    Recommendations,
    #[at("/about")]
    About,
    #[at("/alby")]
    Alby,
    #[not_found]
    NotFound,
}
EOF
    
    # Ajouter l'import dans main.rs
    sed -i '' '1s/^/mod routes;\nuse routes::AppRoute;\n/' frontend/src/main.rs
    
    # Ajouter l'import dans navbar.rs
    sed -i '' '1s/^/use crate::routes::AppRoute;\n/' frontend/src/components/navbar.rs
fi

# 15. Correction du Switch dans lib.rs
echo "Correction du Switch dans lib.rs..."
sed -i '' 's/<Switch::<Route> render={switch} \/>/<Switch<AppRoute> render={switch} \/>/g' frontend/src/lib.rs
# Ajouter l'import nécessaire
sed -i '' '1s/^/use crate::routes::AppRoute;\n/' frontend/src/lib.rs

# 16. Correction des problèmes avec l'enum Route dans types.rs
echo "Correction des problèmes avec l'enum Route dans types.rs..."
# Mise à jour de l'enum Route pour s'assurer que tous les cas sont traités
sed -i '' 's/#\[derive(Debug, Clone, PartialEq, Deserialize, Serialize)\]/#\[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Copy)\]/' frontend/src/types.rs

echo "Corrections terminées!"
echo "N'oubliez pas de vérifier les fichiers modifiés manuellement car certaines erreurs peuvent nécessiter une intervention plus détaillée."
echo "Pour rendre ce script exécutable, utilisez: chmod +x correct.sh"