# Cosmic Claude Usage

Un applet natif et léger pour le panneau COSMIC de Pop!_OS. Il vous permet de surveiller en temps réel votre consommation d'API Claude (Anthropic).

**Aperçu de l'applet :**
`Hebdo: 12%   Session: 4%`

---

## Architecture

Le projet est divisé en deux composants légers pour minimiser l'impact sur le système :

1. **Script de récupération (`fetch_usage.py`)** : Interroge l'API Anthropic toutes les 5 minutes en arrière-plan et sauvegarde les données en local.
2. **Applet COSMIC (`cosmic-claude-usage`)** : Écrit en Rust, il lit les données locales toutes les 30 secondes pour mettre à jour l'affichage dans le panneau.

---

## Prérequis

Assurez-vous d'avoir les éléments suivants installés sur votre système :

* **Clé API Anthropic** : [Générer une clé ici](https://console.anthropic.com/settings/keys)
* **Python** (≥ 3.8) : Déjà inclus dans Pop!_OS.
* **Rust & Cargo** :
```bash
curl https://sh.rustup.rs -sSf | sh
```

**Dépendances système (pour compiler `libcosmic`) :**

```bash
sudo apt install \
    libwayland-dev libxkbcommon-dev libinput-dev \
    libdbus-1-dev pkg-config libseat-dev \
    cmake libpipewire-0.3-dev
```

---

## Installation

**1. Cloner et installer :**

```bash
git clone https://github.com/PosTerrieur/Cosmic-Claude-Usage cosmic-claude-usage
cd cosmic-claude-usage
chmod +x install.sh
./install.sh

```

**2. Configuration :**
Ajoutez votre clé API dans le fichier généré `~/.config/claude-usage/config.env`

**3. Test de bon fonctionnement :**

```bash
# 1. Forcer la récupération des données
fetch-claude-usage

# 2. Vérifier la création du fichier de statistiques
cat ~/.local/share/claude-usage/stats.json

# 3. Lancer l'applet pour tester
cosmic-claude-usage
```

---

## Ajout au panneau COSMIC

1. Faites un **clic droit** sur votre panneau supérieur (ou inférieur) → **Edit Panel**
2. Cliquez sur **Add Applet**
3. Cherchez **Claude Usage** et glissez-le à l'emplacement de votre choix

---

## Configuration (`config.env`)

Fichier de configuration situé dans : `~/.config/claude-usage/config.env`

```env
ANTHROPIC_API_KEY=sk-ant-...
HEBDO_LIMIT=5000000     # Limite de tokens par semaine
SESSION_LIMIT=1000000   # Limite de tokens par session
```

***Quotas suggérés selon votre plan (à titre indicatif) :***

| Plan | Tokens/mois | `HEBDO_LIMIT` suggéré |
| --- | --- | --- |
| Gratuit | ~500 k | `125000` |
| Pro | ~5 M | `1250000` |

---

## Gestion de la Session

Une "session" commence lors de la première exécution du script de récupération après un redémarrage (stocké dans `~/.local/share/claude-usage/session_start.txt`).

Pour réinitialiser votre session manuellement à tout moment :

```bash
rm ~/.local/share/claude-usage/session_start.txt
fetch-claude-usage
```

---

## Désinstallation

Pour retirer complètement l'application et ses services de votre système :

```bash
systemctl --user disable --now claude-usage.timer
rm ~/.config/systemd/user/claude-usage.{service,timer}
rm ~/.local/bin/cosmic-claude-usage ~/.local/bin/fetch-claude-usage
rm ~/.local/share/applications/com.github.user.ClaudeUsage.desktop
rm -rf ~/.local/share/claude-usage ~/.local/share/cosmic-claude-usage
```
