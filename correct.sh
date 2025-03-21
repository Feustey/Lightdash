#!/bin/bash

# Examiner la structure du fichier yields.rs et la corriger
cat > /tmp/fix_yields.awk << 'EOF'
BEGIN { 
    count = 0 
    fixed = 0
    in_yields_fn = 0
}

/pub fn yields\(\) -> Html \{/ { 
    in_yields_fn = 1
    count = 1  # Ouvrir le compteur pour la fonction yields
    print
    next
}

/^}$/ && in_yields_fn && count == 1 { 
    # On a trouvé l'accolade fermante correspondant à l'ouverture de la fonction
    print
    in_yields_fn = 0
    count = 0
    next
}

{
    # Compter les accolades ouvrantes et fermantes dans chaque ligne
    for (i = 1; i <= length($0); i++) {
        char = substr($0, i, 1)
        if (char == "{") count++
        if (char == "}") count--
    }
    
    print
    
    # Si on arrive à la fin avec un compteur déséquilibré, ajouter des accolades
    if ($0 ~ /^}$/ && in_yields_fn && count != 0) {
        for (i = 0; i < -count; i++) {
            print "    }"
            fixed++
        }
        count = 0
    }
}

END {
    # S'assurer que toutes les accolades sont correctement fermées à la fin du fichier
    if (in_yields_fn && count > 0) {
        for (i = 0; i < count; i++) {
            print "}"
            fixed++
        }
    }
    
    if (fixed > 0) {
        print "Fixed " fixed " missing closing braces" > "/dev/stderr"
    } else {
        print "No issues found" > "/dev/stderr"
    }
}
EOF

# Appliquer la correction au fichier yields.rs
awk -f /tmp/fix_yields.awk frontend/src/pages/yields.rs > frontend/src/pages/yields.rs.fixed
mv frontend/src/pages/yields.rs.fixed frontend/src/pages/yields.rs

echo "Le fichier yields.rs a été corrigé"