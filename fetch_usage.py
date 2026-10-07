#!/usr/bin/env python3
import json, os, sys
from datetime import datetime, timezone
from pathlib import Path
import urllib.request, urllib.error

CONFIG_FILE = Path.home() / ".config" / "claude-usage" / "config.env"
STATS_DIR   = Path.home() / ".local" / "share" / "claude-usage"
STATS_FILE  = STATS_DIR / "stats.json"
RAW_FILE    = STATS_DIR / "last_raw.json"

if CONFIG_FILE.exists():
    for line in CONFIG_FILE.read_text().splitlines():
        line = line.strip()
        if line and not line.startswith("#") and "=" in line:
            k, _, v = line.partition("=")
            os.environ.setdefault(k.strip(), v.strip())

SESSION_KEY = os.environ.get("SESSION_KEY", "")

HEADERS = {
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
    "Accept":     "application/json",
    "Referer":    "https://claude.ai/",
    "Cookie":     f"sessionKey={SESSION_KEY}",
}

def claude_get(path: str):
    req = urllib.request.Request(f"https://claude.ai{path}", headers=HEADERS)
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            return json.loads(resp.read().decode())
    except urllib.error.HTTPError as e:
        body = e.read().decode(errors="replace")
        raise RuntimeError(f"HTTP {e.code} → {body[:300]}") from e

def fetch_org_id() -> str:
    orgs = claude_get("/api/organizations")
    if not orgs:
        raise RuntimeError("Aucune organisation retournée")
    return orgs[0]["uuid"]

def fetch_usage(org_id: str) -> dict:
    return claude_get(f"/api/organizations/{org_id}/usage")

def parse_usage(raw: dict) -> dict:
    five_hour = raw.get("five_hour") or {}
    seven_day = raw.get("seven_day") or {}
    return {
        "session_pct":   float(five_hour.get("utilization") or 0.0),
        "hebdo_pct":     float(seven_day.get("utilization") or 0.0),
        "reset_session": five_hour.get("resets_at") or "",
        "reset_hebdo":   seven_day.get("resets_at") or "",
    }

def main():
    if not SESSION_KEY:
        print("ERREUR : SESSION_KEY non définie", file=sys.stderr)
        sys.exit(1)

    STATS_DIR.mkdir(parents=True, exist_ok=True)

    try:
        org_id    = fetch_org_id()
        raw_usage = fetch_usage(org_id)
    except RuntimeError as e:
        print(f"[fetch_usage] Erreur : {e}", file=sys.stderr)
        sys.exit(1)

    RAW_FILE.write_text(json.dumps(raw_usage, indent=2))
    parsed = parse_usage(raw_usage)

    stats = {
        "hebdo_pct":     round(parsed["hebdo_pct"],   1),
        "session_pct":   round(parsed["session_pct"], 1),
        "reset_hebdo":   parsed["reset_hebdo"],
        "reset_session": parsed["reset_session"],
        "last_updated":  datetime.now(tz=timezone.utc).isoformat(),
    }
    STATS_FILE.write_text(json.dumps(stats, indent=2))
    print(f"[fetch_usage] Hebdo: {stats['hebdo_pct']}%  Session (5h): {stats['session_pct']}%")

if __name__ == "__main__":
    main()
