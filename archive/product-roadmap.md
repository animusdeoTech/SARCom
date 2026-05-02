---
title: "Product roadmap (ARCHIVED)"
status: superseded
type: archive
tags: [archive, roadmap, superseded]
superseded_by: "TODO.md; ARCHITECTURE.md; decisions/"
archived_date: 2026-04-22
---

> **ARCHIVED — DO NOT USE AS CURRENT REFERENCE.**
>
> This document is the original calendar-based product roadmap written before the 2026-04-22 decision round. It is now out of date in several load-bearing ways:
>
> - Assumes **Leaflet web UI served by an `axum` HTTP server**. Superseded by [ADR-005](../decisions/ADR-005-map-and-ui.md) and [ADR-007](../decisions/ADR-007-touchscreen-primary-ui.md): the kiosk is a native Rust GUI (`egui` + `walkers`), no browser, no web server, no Leaflet.
> - Assumes a **server with HTTP API, heartbeats, `/health`, API keys**. Superseded by [ADR-008](../decisions/ADR-008-no-cloud-no-downlink.md): no server, no REST, no cloud, no phone app, pure uplink.
> - Assumes **gateway → server HTTP POST**. Superseded: the gateway writes directly to SQLite on the Pi and the UI reads from SQLite. See [ARCHITECTURE.md](../ARCHITECTURE.md).
> - Calendar ordering ("April", "Mei", ...) is aspirational and has slipped. The current backlog is [TODO.md](../TODO.md), which is ordered by blocking dependency, not by calendar.
>
> What is still useful below: the workstream decomposition (firmware / hardware / docs / power / regulatory / branding / licence), the ETSI regulatory call-out, and the reminder that CI, monitoring, and licence choice are real deliverables. Pull those forward into [TODO.md](../TODO.md) when they become the next thing to work on.
>
> The content below is preserved unchanged for historical context.

---

# LoRa Beacon — Product Development Roadmap

Alles wat draaiend moet zijn, van top tot teen. Niets uitgediept, alles aanwezig.

---

## Fasering

**v0 — Bureautest.** Drie nodes praten tegen elkaar op het bureau. Bewijs dat
de radioketen werkt. Geen GPS, geen server, geen internet. Puur bytes door
de lucht.

**v1 — Minimale volledige stack.** Alles draait end-to-end. Tag met GPS buiten,
relay forwardt, gateway parsed en POST naar server, web UI toont positie op
kaart. Dev omgeving ingericht, repo publiek, documentatie geschreven, branding
op orde, eerste community contacten gelegd. Niets diep, alles breed.

---

## Workstreams

### 1. Embedded firmware

| | v0 | v1 |
|---|---|---|
| Tag | Sentinel packet TX elke 5s, hardcoded tag_id | GPS fix → POSITION packet, 300s interval |
| Relay | RX → validate → TX, serial debug log | + dedup ring buffer (tag_id, seq_nr) |
| Gateway | RX → validate → parse → stdout | + HTTP POST naar server, heartbeat |
| Protocol crate | Frame format, CRC, serialize/deserialize | Sentinel validation, flag parsing |
| Toolchain | Rust + esp-hal + lora-phy geïnstalleerd, cargo check werkt | CI: cargo check voor alle targets |

**Deliverables v0:** drie nodes compileren en praten indoor.
**Deliverables v1:** tag buiten, echte coördinaten op server.

---

### 2. Hardware & fysieke montage

| | v0 | v1 |
|---|---|---|
| Tag | Heltec op bureau, USB-voeding, LoRa antenne | Idem + buiten met powerbank |
| Relay | Heltec op bureau, USB-voeding, LoRa antenne | Idem op vaste positie, USB-voeding |
| Gateway | RPi + Dragino HAT op bureau, Ethernet | Idem, vaste positie, stabiele voeding |
| Antennes | IPEX stubantennes op alle LoRa nodes | Idem (upgrade naar SMA in v2+) |
| Bekabeling | USB-C data kabels (niet charge-only!) | Idem |
| BOM | Besteld en ontvangen | Gedocumenteerd met leveranciers en prijzen |

**Deliverables v0:** alles op het bureau, aangesloten, werkend.
**Deliverables v1:** BOM document, foto's van de setup.

---

### 3. Netwerk & connectiviteit

| | v0 | v1 |
|---|---|---|
| LoRa config | Enkele frequentie (868.1 MHz), SF10, +14 dBm | Idem |
| Syncword | Getest dat SX1262 ↔ SX1276 elkaar horen | Gedocumenteerd |
| Gateway backhaul | Niet nodig | WiFi of Ethernet naar server |
| Server hosting | Niet nodig | Lokaal op laptop of RPi, of VPS |

---

### 4. Server & data platform

| | v0 | v1 |
|---|---|---|
| Server | Niet aanwezig | Rust (axum) + SQLite, draaiend |
| API | — | POST /api/v1/report, GET /api/v1/tags |
| Database | — | SQLite met reports tabel, dedup index |
| Simulatie | — | simulate_reports script voor testen zonder hardware |
| Logging | — | Inkomende reports loggen naar stdout |

**Deliverables v1:** server draait, accepteert reports, slaat op, dedupliceert.

---

### 5. Web UI

| | v0 | v1 |
|---|---|---|
| Kaart | Niet aanwezig | Leaflet.js, tag marker, gateway marker |
| Audit log | — | Tabel met recente reports (tag, seq, tijd, RSSI) |
| Auto-refresh | — | Elke 10s, geen WebSocket |
| Track history | — | Lijn op de kaart van recente posities |
| Hosting | — | Served door de axum server zelf |

**Deliverables v1:** open browser, zie tag bewegen op de kaart.

---

### 6. Debugging & testing flow

| | v0 | v1 |
|---|---|---|
| Serial debug | Tag en relay via USB serial monitor | Idem |
| Gateway debug | stdout op RPi | + structured logging |
| Twee-hop bewijs | Relay uit → geen packets, relay aan → packets | Gedocumenteerd met screenshots |
| Packet inspector | Handmatig hex lezen | Script dat raw hex decoded |
| Test protocol | Ad hoc | Geschreven checklist per fase |
| CI | Geen | cargo check voor protocol crate |

**Deliverables v0:** twee-hop bewijs met screenshots.
**Deliverables v1:** test checklist, packet decoder script.

---

### 7. Veldoperatie & deployment

| | v0 | v1 |
|---|---|---|
| Locatie | Bureau | Buiten: tuin, park, of open veld |
| Setup procedure | Ad hoc | Geschreven: "zo zet je een veldtest op" |
| GPS cold start | Niet relevant | Gedocumenteerd: verwachte wachttijd, gedrag |
| Bereik test | Niet relevant | Eerste meting: tag → relay → gateway afstand |
| Weer/omgeving | Indoor | Droog weer, geen enclosure nodig |

**Deliverables v1:** eerste veldtest report met afstanden en RSSI waarden.

---

### 8. Beveiliging

| | v0 | v1 |
|---|---|---|
| LoRa | Geen | Geen (bewust: v1 scope) |
| Server API | — | API key in header (hardcoded, niet fancy) |
| HTTPS | — | Optioneel: self-signed cert of plain HTTP lokaal |
| Documentatie | — | "Security: niet aanwezig in v1, plan voor v2" |

---

### 9. Monitoring & ops

| | v0 | v1 |
|---|---|---|
| Gateway heartbeat | Niet aanwezig | Elke 60s naar server |
| Server health | — | /health endpoint |
| "Is het systeem aan?" | Kijk of serial output loopt | Dashboard toont laatste heartbeat |
| Alerting | — | Niet in v1 (v3: Telegram alerts) |

---

### 10. Dev omgeving

| | v0 | v1 |
|---|---|---|
| Repo | Lokaal, private | GitHub, publiek, met README + LICENSE |
| Repo structuur | Monorepo met crates | Gedocumenteerd in README |
| CLAUDE.md | Niet nodig | In repo root, beschrijft architectuur voor Claude Code |
| Rust toolchain | Nightly + esp targets geïnstalleerd | Gedocumenteerd in CONTRIBUTING.md of README |
| Editor | Wat je wil | Configuratie gedocumenteerd |
| Claude Code | Optioneel | Geïnstalleerd, CLAUDE.md geschreven |
| CI | Geen | GitHub Actions: cargo check, cargo clippy |

**Deliverables v1:** iemand anders kan de repo clonen en builden.

---

### 11. Documentatie

| Document | v0 | v1 |
|---|---|---|
| ARCHITECTURE.md | Bestaand (v5) | Bijgewerkt naar Rust stack |
| README.md | Niet nodig | Project overzicht, build instructies, status |
| v0-setup.md | Bestaand | Bijgewerkt naar Rust stack |
| Hardware BOM | In v0-setup.md | Eigen document met leveranciers |
| Protocol spec | In ARCHITECTURE.md | Eigen document of crate-level docs |
| Veldtest report | — | Template + eerste ingevulde versie |
| Test checklist | — | Per fase: wat testen, wat verwachten |
| CLAUDE.md | — | In repo root |
| CONTRIBUTING.md | — | Build instructies, code conventions |
| Dragino pin mapping | In v0-setup.md | Geverifieerd met BCM nummers |
| Presence kickstart | Dit document | Bijgewerkt met voortgang |

---

### 12. Knowledge base & notities

| | v0 | v1 |
|---|---|---|
| Obsidian vault | Optioneel | Opgezet met project structuur |
| Datasheet notities | Niet nodig | SX1262, SX1276, UC6580, ESP32-S3 |
| Decision log | In hoofd | Markdown file: waarom Rust, waarom geen Zephyr, etc. |
| Claude Project | Dit project | Bijgehouden met actuele docs |

---

### 13. Branding & marketing

| | v0 | v1 |
|---|---|---|
| Projectnaam | Niet nodig | Gekozen, gereserveerd (GitHub org + domein) |
| Logo | Niet nodig | SVG, werkt op alle formaten |
| Businesskaart | — | 100 stuks gedrukt |
| LinkedIn | — | Headline bijgewerkt, eerste post |
| Hackaday.io | — | Project page met beschrijving + foto's |
| Website | — | GitHub Pages of niets (README is genoeg) |

---

### 14. Community & contactpunten

| | v0 | v1 |
|---|---|---|
| TTN community | Niet nodig | Geïntroduceerd op Brussels/NL forum |
| Reddit | — | Architectuur gepost op r/lora |
| Events | — | Things Conference ticket (sept) |
| Open source bijdrage | — | esp-hal BSP PR of RadioLib issue |

---

### 15. "Hoe gebruik ik AI" documentatie

| | v0 | v1 |
|---|---|---|
| CLAUDE.md | — | In repo, beschrijft crate architectuur |
| Claude Project | Actief | Docs actueel gehouden |
| Claude Code workflow | — | Gedocumenteerd: wanneer Code vs chat vs Research |
| Prompt patronen | — | Notities over wat goed werkt voor embedded Rust |

---

### 16. Regulatory & compliance

| | v0 | v1 |
|---|---|---|
| Duty cycle | Niet relevant (indoor) | Berekend en gedocumenteerd per mode |
| ETSI EN 300 220 | Gelezen | Relevante limieten in ARCHITECTURE.md |
| Frequentieplan | 868.1 MHz hardcoded | Dual-channel plan gedocumenteerd |
| ERP berekening | Niet nodig | TX power + antenne gain = ERP, gedocumenteerd |

---

### 17. Power & batterij

| | v0 | v1 |
|---|---|---|
| Tag | USB-voeding | USB powerbank, meting van runtime |
| Relay | USB-voeding | Idem |
| Gateway | Netvoeding via RPi | Idem |
| Deep sleep | Niet geïmplementeerd | Niet in v1 (v1 = altijd aan, Phase 3) |
| Stroommeting | — | Eerste meting met multimeter, gedocumenteerd |

---

### 18. Licentie & IP

| | v0 | v1 |
|---|---|---|
| Code licentie | Niet relevant | Gekozen (MIT? Apache-2.0? dual?) in repo |
| Hardware design | — | Open source of niet — beslissing gedocumenteerd |

---

## Wat je miste

Bovenstaande voegt toe wat niet in je oorspronkelijke lijst zat:

- **Beveiliging** (§8) — zelfs minimaal: API key op de server
- **Monitoring** (§9) — hoe weet je dat het systeem draait?
- **Regulatory** (§16) — duty cycle tracking, ERP berekeningen
- **Power** (§17) — eerste stroommeting, runtime schatting
- **Licentie** (§18) — keuze maken vóór repo publiek gaat
- **CI/CD** (in §10) — minimaal cargo check in GitHub Actions
- **Veldtest report template** (in §7, §11) — gestructureerde output van je eerste buitentest
- **Syncword verificatie** (in §3) — SX1262 ↔ SX1276, moet getest en gedocumenteerd

---

## Volgorde

Niet alles parallel. Ruwweg:

```
April    ██ Dev omgeving (repo, toolchain, CLAUDE.md)
         ██ Hardware bestellen/ontvangen
         ██ Naam + logo + GitHub org

Mei      ██ v0: embedded firmware (tag → relay → gateway op bureau)
         ██ Twee-hop bewijs
         ██ Eerste LinkedIn post
         ██ Businesskaarten

Juni     ██ v1 embedded: GPS integratie
         ██ v1 server: axum + SQLite + API
         ██ v1 web UI: Leaflet kaart
         ██ Hackaday.io project page
         ██ TTN forum introductie

Juli     ██ Gateway → server koppeling
         ██ Eerste veldtest buiten
         ██ Veldtest report
         ██ Documentatie ronde (README, BOM, test checklist)

Aug      ██ Stabilisatie en bugfixes
         ██ Monitoring (heartbeat, /health)
         ██ CI pipeline
         ██ Website als dat nuttig lijkt
         ██ Licentie beslissing

Sept     ██ The Things Conference (22-23 sept, Amsterdam)
         ██ Alles gedocumenteerd en presenteerbaar
```

---

## Succescriteria v1

v1 is klaar wanneer:

1. Tag buiten verstuurt echte GPS-coördinaten via LoRa
2. Relay forwardt het packet naar de gateway
3. Gateway parsed het packet en POST naar server
4. Server slaat op en dedupliceert
5. Web UI toont tag positie op Leaflet kaart met track history
6. Gateway heartbeat zichtbaar op dashboard
7. Repo is publiek met README, LICENSE, build instructies
8. Minimaal één veldtest gedocumenteerd met afstanden en RSSI
9. Branding aanwezig: naam, logo, businesskaart, LinkedIn
10. Eerste community contact gelegd (TTN forum of Reddit post)
