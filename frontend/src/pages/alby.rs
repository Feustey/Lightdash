use yew::prelude::*;

#[function_component(AlbyPage)]
pub fn alby_page() -> Html {
    html! {
        <div class="alby-container">
            <h1 class="alby-title">{"Alby Hub : Un Guide pour Configurer un Nœud Lightning avec Facilité"}</h1>
            
            <div class="alby-intro">
                <p>{"Le réseau Lightning est une solution de second layer qui permet d'effectuer des transactions Bitcoin rapides et peu coûteuses. Pour tirer pleinement parti de cette technologie, il est essentiel de configurer un nœud Lightning. Alby Hub se présente comme une solution accessible pour ceux qui souhaitent s'impliquer dans ce réseau sans avoir à maîtriser des compétences techniques approfondies. Ce guide vous explique comment utiliser Alby Hub pour configurer et gérer votre propre nœud Lightning."}</p>
            </div>

            <section class="alby-section">
                <h2>{"Qu'est-ce qu'Alby Hub ?"}</h2>
                <p>{"Alby Hub est une plateforme conçue pour simplifier la configuration et la gestion d'un nœud Lightning. Elle s'adresse aussi bien aux débutants qu'aux utilisateurs expérimentés, en offrant une interface intuitive et des outils automatisés. Grâce à Alby Hub, vous pouvez déployer un nœud Lightning en quelques étapes simples, sans avoir à manipuler des lignes de code complexes ou à configurer manuellement des paramètres techniques."}</p>
            </section>

            <section class="alby-section">
                <h2>{"Pourquoi Utiliser Alby Hub ?"}</h2>
                <ul class="alby-features">
                    <li>{"Accessibilité : Alby Hub rend la technologie Lightning accessible à tous, même à ceux qui n'ont pas de connaissances techniques approfondies."}</li>
                    <li>{"Automatisation : La plateforme automatise de nombreuses tâches, comme l'installation du logiciel, la configuration des ports et la gestion des canaux de paiement."}</li>
                    <li>{"Sécurité : Alby Hub intègre des fonctionnalités de sécurité pour protéger vos fonds et vos données."}</li>
                    <li>{"Interopérabilité : Le nœud créé via Alby Hub est compatible avec d'autres applications et services Lightning, ce qui facilite son intégration dans votre écosystème Bitcoin."}</li>
                </ul>
            </section>

            <section class="alby-section">
                <h2>{"Étapes pour Configurer un Nœud Lightning avec Alby Hub"}</h2>
                <div class="alby-steps">
                    <div class="step">
                        <h3>{"1. Créez un Compte Alby"}</h3>
                        <p>{"La première étape consiste à créer un compte sur la plateforme Alby. Rendez-vous sur le site officiel d'Alby Hub et suivez les instructions pour vous inscrire. Une fois votre compte activé, vous aurez accès à un tableau de bord personnalisé."}</p>
                    </div>
                    <div class="step">
                        <h3>{"2. Configurez Votre Nœud"}</h3>
                        <p>{"Dans votre tableau de bord, sélectionnez l'option pour créer un nouveau nœud Lightning. Alby Hub vous guidera à travers les étapes nécessaires, notamment :"}</p>
                        <ul>
                            <li>{"Le choix du matériel (vous pouvez utiliser un ordinateur personnel ou un appareil dédié comme un Raspberry Pi)."}</li>
                            <li>{"L'installation du logiciel nécessaire (Alby Hub gère cette étape automatiquement)."}</li>
                            <li>{"La configuration des paramètres réseau pour assurer la connectivité."}</li>
                        </ul>
                    </div>
                    <div class="step">
                        <h3>{"3. Financez Votre Nœud"}</h3>
                        <p>{"Une fois votre nœud configuré, vous devez le financer en Bitcoin. Alby Hub vous fournira une adresse Bitcoin sur laquelle vous pourrez envoyer des fonds. Ces fonds seront utilisés pour ouvrir des canaux de paiement sur le réseau Lightning."}</p>
                    </div>
                    <div class="step">
                        <h3>{"4. Ouvrez des Canaux de Paiement"}</h3>
                        <p>{"Les canaux de paiement sont essentiels pour effectuer des transactions sur le réseau Lightning. Alby Hub simplifie cette étape en vous permettant de connecter votre nœud à d'autres nœuds populaires. Vous pouvez également configurer des canaux privés pour des transactions spécifiques."}</p>
                    </div>
                    <div class="step">
                        <h3>{"5. Gérez Votre Nœud"}</h3>
                        <p>{"Alby Hub propose des outils de gestion pour surveiller l'état de votre nœud, vérifier les transactions et optimiser les canaux de paiement. Vous pouvez également recevoir des notifications en cas de problème ou de mise à jour nécessaire."}</p>
                    </div>
                </div>
            </section>

            <section class="alby-section">
                <h2>{"Avantages d'Utiliser Alby Hub"}</h2>
                <div class="alby-advantages">
                    <div class="advantage">
                        <h3>{"Gain de Temps"}</h3>
                        <p>{"L'automatisation des processus techniques vous permet de gagner du temps et de vous concentrer sur l'utilisation du réseau Lightning."}</p>
                    </div>
                    <div class="advantage">
                        <h3>{"Support Technique"}</h3>
                        <p>{"Alby Hub offre un support technique pour vous aider en cas de difficulté."}</p>
                    </div>
                    <div class="advantage">
                        <h3>{"Évolutivité"}</h3>
                        <p>{"Vous pouvez facilement mettre à niveau votre nœud ou ajouter de nouvelles fonctionnalités grâce à l'interface intuitive."}</p>
                    </div>
                </div>
            </section>

            <section class="alby-section">
                <h2>{"Conseils pour Optimiser Votre Expérience"}</h2>
                <div class="alby-tips">
                    <div class="tip">
                        <h3>{"Choisissez un Matériel Adapté"}</h3>
                        <p>{"Bien qu'Alby Hub soit compatible avec de nombreux appareils, l'utilisation d'un matériel dédié comme un Raspberry Pi peut améliorer les performances de votre nœud."}</p>
                    </div>
                    <div class="tip">
                        <h3>{"Sécurisez Vos Fonds"}</h3>
                        <p>{"Utilisez des méthodes de sauvegarde robustes pour protéger vos clés privées et vos fonds."}</p>
                    </div>
                    <div class="tip">
                        <h3>{"Restez Informé"}</h3>
                        <p>{"Le réseau Lightning évolue rapidement. Suivez les mises à jour d'Alby Hub et les nouvelles fonctionnalités pour tirer le meilleur parti de votre nœud."}</p>
                    </div>
                </div>
            </section>

            <section class="alby-section">
                <h2>{"Conclusion"}</h2>
                <p>{"Alby Hub est une solution idéale pour ceux qui souhaitent participer au réseau Lightning sans se heurter à des obstacles techniques. Grâce à son interface conviviale et ses fonctionnalités automatisées, configurer et gérer un nœud Lightning devient accessible à tous. Que vous soyez un utilisateur débutant ou expérimenté, Alby Hub vous offre les outils nécessaires pour explorer pleinement le potentiel du réseau Lightning et contribuer à l'écosystème Bitcoin."}</p>
            </section>
        </div>
    }
} 