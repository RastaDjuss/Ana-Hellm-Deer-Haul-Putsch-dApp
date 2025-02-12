# Ana-Chain

Ana-Chain est une application fullstack sociale utilisant Solana comme backend pour une gouvernance décentralisée.

---

### **Setup Commands Using `pnpm`**

Une fois le dépôt cloné, vous pouvez utiliser les commandes `pnpm` pour simplifier le développement et la gestion des tâches de construction. `pnpm` est un gestionnaire de paquets rapide et efficace, idéal pour gérer les dépendances d'un projet complexe.

Voici la liste des commandes clés :

---

### **Commandes globales**

#### **`pnpm install`**
- Installe toutes les dépendances requises pour le projet.
- Cela couvre à la fois les dépendances frontend (`web`) et backend (`anchor`), préparant ainsi tout le projet en une seule commande.
- **Note :** Ce processus peut prendre du temps, car il installe toutes les dépendances nécessaires pour une application fullstack. Vous pouvez suivre l'avancement directement dans votre terminal.

#### **`pnpm dev`**
- Démarre le serveur de développement pour l'application frontend React avec un rechargement à chaud.
- Idéal pour un développement rapide et un aperçu en live.

#### **`pnpm build`**
- Compile et construit l'application frontend en mode production. Les fichiers finaux sont placés dans le répertoire `out`.

#### **`pnpm start`**
- Démarre la version de production de l'application frontend, parfaite pour la vérification locale avant déploiement.

---

### **Commandes liées à Anchor**

#### **`pnpm anchor`**
- Permet d'exécuter toute commande Anchor directement depuis la racine. Equivalent à naviguer dans le répertoire `anchor` et à exécuter `anchor`.

Exemple :
```bash
pnpm anchor build
```

#### **`pnpm anchor:build`**
- Compile le programme Solana dans le répertoire `anchor` en utilisant `anchor build`.
- Utile si vous souhaitez construire votre programme manuellement sans exécuter tout le flux de développement.

#### **`pnpm anchor-localnet`**
- Démarre un validateur Solana local, déploie le programme Anchor sur ce validateur local.
- Cette commande est cruciale pour tester les programmes localement sans dépendre d'un environnement externe.

#### **`pnpm anchor-test`**
- Exécute les tests pour le programme Solana à l'aide du framework de test fourni par Anchor.

#### **`pnpm anchor-deploy`**
- Déploie le programme en cours sur la blockchain cible configurée dans le fichier `Anchor.toml`.

---

### **Commandes diverses**

#### **`pnpm lint-init`**
- Initialise ESLint pour le projet.

#### **`pnpm lint`**
- Exécute ESLint et corrige les problèmes de style dans les fichiers `.js`, `.ts` et `.tsx`.

---

### **Installation des dépendances spécifiques**

#### **Frontend (governance-ui)**
Pour mettre en place les dépendances du frontend situé dans le répertoire `governance-ui`, utilisez la commande suivante :

```bash
pnpm gov-ui-install
```

Cette commande navigue dans le répertoire `governance-ui` et installe toutes les dépendances nécessaires avec `pnpm install`.

---

### **Instructions rapides**

#### Installer les dépendances :
```bash
pnpm install
```

#### Démarrer le projet en mode développement :
```bash
pnpm dev
```

#### Compiler le programme Anchor :
```bash
pnpm anchor:build
```

#### Démarrer un réseau local pour Solana :
```bash
pnpm anchor-localnet
```

---

## **Applications disponibles**

### **anchor**
- Il s'agit d'un programme Solana écrit en Rust en utilisant le framework Anchor.
- Vous pouvez exécuter toutes les commandes Anchor standard depuis le répertoire `anchor` ou en préfixant vos commandes avec `pnpm`.

Exemple :
```bash
pnpm anchor build
```