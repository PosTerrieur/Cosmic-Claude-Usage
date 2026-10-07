#!/usr/bin/env bash
# install.sh – Installe l'applet COSMIC Claude Usage
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOCAL_BIN="$HOME/.local/bin"
LOCAL_SHARE="$HOME/.local/share"
CONFIG_DIR="$HOME/.config/claude-usage"
SYSTEMD_DIR="$HOME/.config/systemd/user"

echo "=== Installation de cosmic-claude-usage ==="

# ── 1. Build Rust ─────────────────────────────────────────────────────────────
echo ""
echo "[1/5] Compilation de l'applet Rust..."
echo "      (première compilation : peut prendre quelques minutes)"
cd "$SCRIPT_DIR"
cargo build --release

# ── 2. Copie du binaire ───────────────────────────────────────────────────────
echo ""
echo "[2/5] Installation du binaire..."
mkdir -p "$LOCAL_BIN"
cp target/release/cosmic-claude-usage "$LOCAL_BIN/"
echo "      → $LOCAL_BIN/cosmic-claude-usage"

# ── 3. Script Python + config ─────────────────────────────────────────────────
echo ""
echo "[3/5] Installation du script de mise à jour..."
mkdir -p "$LOCAL_SHARE/cosmic-claude-usage"
cp fetch_usage.py "$LOCAL_SHARE/cosmic-claude-usage/"
chmod +x "$LOCAL_SHARE/cosmic-claude-usage/fetch_usage.py"

# Wrapper dans ~/.local/bin
cat > "$LOCAL_BIN/fetch-claude-usage" << 'EOF'
#!/usr/bin/env bash
exec python3 "$HOME/.local/share/cosmic-claude-usage/fetch_usage.py" "$@"
EOF
chmod +x "$LOCAL_BIN/fetch-claude-usage"

# Config template
mkdir -p "$CONFIG_DIR"
if [ ! -f "$CONFIG_DIR/config.env" ]; then
    cp "$SCRIPT_DIR/config.env.template" "$CONFIG_DIR/config.env"
    echo ""
    echo "  !! Remplissez votre clé API dans : $CONFIG_DIR/config.env"
fi

# ── 4. Fichier .desktop COSMIC ────────────────────────────────────────────────
echo ""
echo "[4/5] Enregistrement de l'applet COSMIC..."
mkdir -p "$LOCAL_SHARE/applications"
cp "$SCRIPT_DIR/com.github.user.ClaudeUsage.desktop" \
   "$LOCAL_SHARE/applications/"
echo "      → $LOCAL_SHARE/applications/com.github.user.ClaudeUsage.desktop"

# ── 5. Systemd user timer ─────────────────────────────────────────────────────
echo ""
echo "[5/5] Installation et activation du timer systemd..."
mkdir -p "$SYSTEMD_DIR"
cp "$SCRIPT_DIR/systemd/claude-usage.service" "$SYSTEMD_DIR/"
cp "$SCRIPT_DIR/systemd/claude-usage.timer"   "$SYSTEMD_DIR/"
systemctl --user daemon-reload
systemctl --user enable --now claude-usage.timer
echo "      → Timer actif (vérifiez : systemctl --user status claude-usage.timer)"

# ── Résumé ────────────────────────────────────────────────────────────────────
echo ""
echo "=== Installation terminée ==="
echo ""
echo "Étapes suivantes :"
echo "  1. Éditez  $CONFIG_DIR/config.env  et renseignez ANTHROPIC_API_KEY"
echo "  2. Lancez un premier refresh :  fetch-claude-usage"
echo "  3. Dans COSMIC Settings → Panel → Applets, ajoutez 'Claude Usage'"
echo ""
echo "Le fichier de stats est : ~/.local/share/claude-usage/stats.json"
echo "Vous pouvez l'éditer manuellement pour tester l'affichage."
