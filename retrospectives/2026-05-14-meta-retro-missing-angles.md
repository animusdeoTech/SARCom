---
title: "Meta-retro — wat de 2026-05-14 retros NIET vingen"
date: 2026-05-14
type: meta-retrospective
scope: gap-analysis-of-the-2026-05-14-retro-set
source-sessions:
  - dev-log/2026-05-13-gateway-v1-cad-session-risks.md
  - dev-log/2026-05-14-c1-depth-stackup-arithmetic.md
  - dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md
  - dev-log/2026-05-14-anker-dims-and-gate-propagation.md
  - dev-log/2026-05-14-cad-day-retrospective.md
  - retrospectives/2026-05-14-design-decisions.md
  - docs/cad-skill-reference.md
---

# Meta-retro — wat de 2026-05-14 retros NIET vingen

De bestaande retro-set (`cad-day-retrospective.md` + `design-decisions.md`) is grondig op failure-cataloging en op decision-framing met trade-offs. Wat ze niet expliciet vangen: wat **goed** ging en herhaalbaar is; hoe de drie AI-instances tegen elkaar werkten; welke source-of-truth ordening impliciet door alles heen liep; wat élke open carry-over concreet zou unblocken; of de cardboard mockup ooit gebouwd is; welke patronen in de tooling stack rendabel of fragiel bleken; en welke v2-wensen op tafel kwamen met hun expliciete v1-afwijzing redenen. Dit document vult die zeven gaten in.

## §1 — Positive-pattern catalogus

Wat over de hele dag werkte en herhaalbaar is. De bestaande retros documenteren elk patroon impliciet maar markeren ze niet als "doe dit opnieuw."

**Pattern 1 — HALT-and-ask voor ADR-relevante beslissingen.** Toen pogo-drop het CoT/TAK gate predikat ondermijnde, stopte de session expliciet en stelde Pieter de ADR-016 vraag met 4 expliciete opties (a/b/c/d). Pieter koos (b) "WiFi + manual opt-in", schriftelijk vastgelegd. Bron: `dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md` §"HALT — item 2: ADR-016 gate question". Zonder dit patroon: AI zou stilletjes een gate-formulering hebben geherframed (waarschijnlijk "WiFi + battery-state + opt-in" als compromis-achtig alternatief), Pieter zou geen ownership van de beslissing hebben, en de ADR-amendment-trail zou ambigu zijn. Toepassen wanneer: een opgemerkt issue raakt aan een pending ADR of een Accepted ADR.

**Pattern 2 — Audit-filter tabel met vier verdicten.** Autodesk Assistant produceerde 5 "conflicts" + diverse findings. Vier-verdicten classificatie (REAL / STALE / HALLUCINATION / VERIFIED-OK) sorteerde de output binnen ~15 minuten met cited evidence per verdict. Bron: `dev-log/2026-05-14-anker-dims-and-gate-propagation.md` §"Post-Autodesk-Assistant-audit pass (filter then act)". Zonder dit patroon: de 5 findings zouden waarschijnlijk allemaal als action-items behandeld zijn — wat had geresulteerd in heropening van de pogo (C5), shrinking front_depth naar het verkeerde getal (C1/P1.1), en geometrie-changes op een non-issue (C3 boss "asymmetrie"). Toepassen wanneer: enige bot-gegenereerde audit of review-output.

**Pattern 3 — Per-row stack-up arithmetic met expliciete `Source` kolom.** De C1 depth dev-log dwong elke laag in de stack-up te zijn ofwel cited (spike-close path:line of datasheet URL) of expliciet als **HAND-WAVE** gelabeld. Resultaat: 45-55 mm spec werd zichtbaar als hand-wave; 85-100 mm reality werd verdedigbaar gecorrigeerd. Bron: `dev-log/2026-05-14-c1-depth-stackup-arithmetic.md` §Method + §"Front compartment Z-stack" + §"Rear compartment Z-stack". Zonder dit patroon: de spec was als 45-55 mm gebleven, en de eerste fysieke print was op verkeerde diepte gegaan (of had geen ruimte voor de HAT). Toepassen wanneer: een spike-close een depth/footprint/volume cijfer geeft zonder verifieerbare math.

**Pattern 4 — Iterate-collect-delete als recovery van Fusion API InternalValidationError.** Bij feature-deletes binnen een `for f in comp.features:` loop crashte Fusion bij access van een al-cascade-deleted feature. Recovery: eerst `to_delete = [f for f in comp.features if matches(f)]` (materializeer de list), daarna `for f in to_delete: f.deleteMe()`. Bron: `dev-log/2026-05-14-cad-day-retrospective.md` §F10. Zonder dit patroon: Fusion zou herhaalbaar crashen tijdens cleanup operaties, met design mogelijk in mid-edit state. Toepassen wanneer: enige Fusion API loop die features of bodies muteert.

**Pattern 5 — Spike-close supersession layering.** Bij elke spike-close amendment werd één dated section bovenaan toegevoegd (`## 2026-05-14 partial supersession — ...`) plus inline `[SUPERSEDED 2026-05-14]` / `[CORRECTED 2026-05-14]` markers in §Closed en §Decision. Historische tekst bleef leesbaar, current state werd legible. Bron: power-arch + enclosure spike-close amendments + retrospective §"Structural observation". Zonder dit patroon: rewrite-in-place zou de history-trail vernietigen — readers in 2027 zouden geen idee hebben wat veranderd is of waarom. Toepassen wanneer: een Accepted spike-close inhoudelijk wordt aangepast.

**Pattern 6 — Forensic timeline rollback voor volume anomalies.** Heat-spreader pocket cut verwijderde 11,939 mm³ ipv ideal 7,200. Timeline rollback via `design.timeline.markerPosition` capture'de body volume bij elke feature-step; face-level diff tussen pre-cut en post-cut state isoleerde de bron (4 cavity-interior sliver faces). Bron: `dev-log/2026-05-14-anker-dims-and-gate-propagation.md` §"End-of-day 2026-05-14 — Heat-spreader pocket volume delta — forensic verdict". Zonder dit patroon: het volume-overschot was hand-waved als "fillet artifacts" zonder bewijs; de pocket had verkeerd kunnen zijn zonder dat we het wisten. Toepassen wanneer: een feature reports een volume-delta die niet matcht met geometric ideal.

**Pattern 7 — Memory commits voor caught flips.** Toen Pieter tweemaal de passive-cooling correctie moest doen, werd een memory file (`feedback_cooling_is_passive.md`) opgeslagen die de regel + reden + how-to-apply codificeerde. Zelfde voor "prototype bulk is fine". Bron: chat-thread observatie + de memory bestanden zelf. Zonder dit patroon: zelfde flip zou een derde keer gebeuren in de volgende sessie. Toepassen wanneer: een AI-side mistake binnen één sessie herhaalt na expliciete correctie.

## §2 — AI-collaboration retro

Drie AI-instances werkten op 2026-05-14 aan het gateway-v1 design. Hun rollen, betrouwbaarheidsprofielen, en de impliciete hiërarchie die ontstond.

**Cowork (Claude in chat-thread).** Strategic synthesis layer: redeneerde over decisions, schreef de retros, framed trade-offs, coördineerde cross-doc propagation. Got reliably right: cross-doc consistency (Anker dim propagation in één pass, gate-language propagation in 5 files + extra 3 in follow-up), audit-filter judgment with cited evidence, narrative coherence van de drie dev-logs. Got wrong: tweemaal passive→active cooling implicit flip (gecorrigeerd door Pieter, gevangen door memory commit); param-count tracking discipline (claimed 22, was 23 — `cad-day-retrospective.md` §F13); aanvankelijke audit-filter onderschatting (Pieter's review-pass vond 3 extra files met stale gate-language die de eerste pass miste).

**Claude Code CLI (via `fusion_execute` MCP).** Tool-execution layer: alle CAD edits, alle parameter mutations, alle body extrudes/cuts/shells, alle timeline rollbacks voor forensic, alle live geometry queries. Got reliably right: live design state queries (component/body/feature/sketch inventory), parametric expression handling (`-rear_depth` als translation, `outer_w / 2 - 1.5 mm` als offset), timeline rollback voor pre/post state vergelijking, parametric Move-feature setup, face-level diff via signature comparison. Got wrong: F2 cross-component extrude attempts (eerste poging crashte met `InternalValidationError: bSet`); F4 enum guess (mapped Operation=1 als JoinFeatureOperation, was eigenlijk CutFeatureOperation); F5 `addByTwoPoints` line-coïncidentie miss (rectangle profile failde to form); F7 sketch-X→world-Y assumption op offset YZ plane (mapped to world Z in werkelijkheid). Alle 4 gefixt binnen dezelfde sessie via API-inspectie of geometry re-verification.

**Autodesk Assistant (Fusion 360 in-app LLM).** Audit/review layer: geen schrijfacties, alleen kritisch lezen van design state + spec docs. Got reliably right: 2 van 5 findings (C2 rear-shell undersize / 1.5 mm step, C4 heat-spreader-pocket sketch zonder extrude). Beide bleken via live verification echt. Got wrong: 4 STALE findings (C1 depth-spec las 45-55 mm pre-correction; C5 pogo bore "missing" — pogo retired diezelfde dag; P1.1 stack math conflated `display_glass_offset` met depth contributor; Item-9 bank width 62 mm gebruikte pre-correction Anker spec); 1 HALLUCINATION (C3 door bosses asymmetric — mat vanaf sketch origin 0,0 ipv door geometric center -20,0; bosses zijn beide 11.5 mm van door-center, dus symmetrisch). Bron: `dev-log/2026-05-14-anker-dims-and-gate-propagation.md` §"Post-Autodesk-Assistant-audit pass".

**Menselijke interventie (Pieter) was vereist op:** (a) de passive-cooling miscommunicatie — Cowork mappe niet automatisch het 05-13 dev-log's misleidende `active-cooler-stack-equivalent` phrasing terug naar de spike-close's expliciete passive commitment; (b) de ADR-016 gate-keuze — vier opties op tafel, Pieter koos (b); (c) audit-filter judgment calls — Cowork classificeerde de 5 findings maar Pieter verifieerde elk verdict in de review; (d) drie extra files met stale gate-language die de eerste propagation-pass miste — Pieter ving die in de review. Patroon: AI's vingen geometrische en numerieke issues; Pieter ving narrative/contradiction issues en architectuur-keuzes.

**Impliciete role hiërarchie die ontstond:**

```
Pieter (architectuur + ADR-keuzes + narrative integrity check)
  ↓
Cowork (synthese + cross-doc propagation + retro/narrative)
  ↓
Claude Code CLI via fusion_execute (live CAD edits + forensic queries)
  ↓
Autodesk Assistant (raw audit output, NIET authoritative — filter required)
```

Geen van de drie AI's was authoritative voor design-intent disputes. Alle drie waren handig voor verschillende lagen van werk. De fout-modus die het meest tijd kostte was wanneer Cowork audit-bot output behandelde als input zonder filter (eerste poging op de Autodesk audit) — gecorrigeerd door explicit verdict-table workflow.

## §3 — Source-of-truth hierarchie

Eén-regel declaratie van de ordening die impliciet door de hele dag liep:

```
Live Fusion state (via fusion_execute) >
  spike-close §Decision post-amendments >
    §Closed verdict >
      spike-close prose >
        dev-logs >
          audit-bot output
```

Per adjacent paar, één concreet voorbeeld waar de lagere de hogere verloor op 2026-05-14:

**Live Fusion state > spike-close §Decision post-amendments.** Toen het Anker-dim correction passeerde, was de `rear_depth = 40 mm` user parameter in Fusion live aanpasbaar; de spike-close §Decision tekst was statisch (een Edit operatie). De parametric live state is de finale waarheid voor wat geëxtrudeerd wordt — de spike-close tekst is design intent dat naar de live state propageert, niet omgekeerd. Toen de pre-correction spike-close 154×62×30 mm cited met `rear_depth = 35 mm` in Fusion, was Fusion's getal de werkelijkheid; spike-close tekst was de hand-wave. Correctie pad: update spike-close om Fusion (post Anker verification) te matchen, niet omgekeerd.

**§Decision post-amendments > §Closed verdict.** In de enclosure spike-close zegt §Closed verdict (van 2026-05-08) "magnetic-pogo charging (no panel-mount USB-C)". De §Decision note heeft een 2026-05-14 amendment block bovenaan met `[SUPERSEDED 2026-05-14 — no in-shell charging in v1]`. Voor elke reader na 2026-05-14: amendment wint van de oorspronkelijke verdict. §Closed text blijft historisch leesbaar maar is niet meer current.

**§Closed verdict > spike-close prose.** Spike-close §Closed zegt expliciet "NO active cooler. NO fan. NO vent for cooling." Spike-close prose elders en de 05-13 dev-log gebruikten "active-cooler-stack-equivalent" als height-envelope vergelijking — niet als design statement, maar lezers misinterpreteerden. §Closed verdict wint: passive. Prose phrasing dat conflictueert is sloppy phrasing, niet een tegenstrijdige beslissing.

**Spike-close prose > dev-logs.** 2026-05-13 dev-log §C1 zegt "Pi 5 + active-cooler-stack-equivalent + ..." — direct conflict met de spike-close prose én verdict. Dev-log verloor: werd 2026-05-14 morning amend om expliciet passive heat-spreader stack te benoemen. Dev-logs zijn temporele sessie-records van AI-reasoning; spike-closes zijn formele commitments. Wanneer ze conflicteren over een Accepted decision, het dev-log heeft de fout.

**Dev-logs > audit-bot output.** Autodesk Assistant audit claimde "total depth 100 mm is 45 mm over spec (45-55 mm)". Maar de spec was 's morgens al gecorrigeerd naar 85-100 mm in de c1-depth-stackup dev-log + enclosure spike amendment. Dev-log (en de spike-close amendment dat het droeg) wint: audit was STALE. Audit-bot output zonder dev-log/spike awareness is altijd kandidaat-input, nooit authoritative.

Anti-pattern: audit-bot output behandelen als level-equal aan de andere bronnen. Dat is wat de eerste poging op de Autodesk audit zou zijn geweest zonder de explicit filter-workflow.

## §4 — Decision residue map: wat unblockt elke carry-over?

Voor elke open carry-over in TODO.md §"Carry-over voor volgende CAD sessie (2026-05-15+)": de blocker, wat het concreet ANSWERABLE maakt, en de geschatte cost.

| # | Carry-over | Wat unblockt | Cost |
|---|---|---|---|
| 1 | **Orientation X vs Y** (Pi/HAT facing display) | (a) Pi Touch Display 2 fysiek in handen + standoff direct gemeten met digital caliper (cost: ~1 dag wachten op delivery + ~€90 voor het display als nog niet besteld); OR (b) Raspberry Pi support reply op vraag "what is the standoff height between the TD2 back and a Pi-mounted via the corner standoffs?" (cost: variabele wait, mogelijk geen antwoord); OR (c) cardboard mockup met beide orientations (cost: ~30 min, low precision); OR (d) Pieter maakt tentatieve keuze + accept de cascading slack/squeeze (cost: 0; pinst #2-#3 cascadeert immediately) | Hardware-route: 1 dag; mockup-route: 30 min; tentatieve keuze: 0 |
| 2 | **Front-depth squeeze** (-5 mm tekort bij Orientation X) | Cascadeert direct uit #1: Orientation X = front_depth moet groeien van 60 → ~65 mm; Orientation Y = HAT relocates side-mount, depth stack veranderd fundamentally. Geen onafhankelijk werk vóór #1. | Nil (downstream van #1) |
| 3 | **Rear-shell / front-shell X-asymmetry** (1.5 mm step) | Inspecteer welke sketches de outer envelopes definiëren in beide shells; cross-ref met de gasket-groove offset conventie. Drie interpretaties (intentioneel tongue-and-groove / accidenteel sketch-source / unknown-maar-werkt) elimineren door evidence: was de `rear-outer-envelope` profile[2] (20,038 mm²) bewust gekozen vs profile[0]? Check sketch dimensioning. | ~30-45 min review |
| 4 | **Rear compartment slack** (30 mm X-axis) | Architectuur-keuze: drie opties (krimpen device-footprint naar 150×120; laten voor cable-routing marge; hergebruiken voor stowage zoals magnet-holster/silica-gel/spare-antenna). Geen technisch werk; alleen Pieter beslissing in context van prototyping speed vs final-print depth budget. | 0 (decision); implementation cost varieert per optie |
| 5 | **Sketch origin hygiëne op `door-profile`** | Translate sketch zodanig dat door geometric center op (0, 0) ligt; verifieer dat dependent features (gasket groove offsets, anchor dims, Move feature) blijven correct resolven. Parallel uitvoerbaar zonder Orientation beslissing — door-area is in X+Y plane, niet afhankelijk van Pi orientation. | ~15-30 min CLI werk |
| 6 | **Heat-spreader pocket volume delta** (open BREP hypothese) | Diepe investigation: maak test-cuts met variërende profile shapes/depths op een dummy shell-feature body; vergelijk volume reductie patronen. Doel: verifieer of de Z=-20, Z=-18.5, Z=0 sliver-disappearance pattern reproduceerbaar is bij andere cut+shell combinaties. Low priority — pocket geometry is functioneel correct. | ~1-2 uur forensic; alleen waarmaken als pocket-related issues opduiken in latere features |

Cascade-volgorde voor maximum efficiency: #1 → #2 valt vanzelf → #3 parallel met #5 (CLI-doable, geen Pieter-input) → #4 architectuur-sessie (Pieter, ander moment) → #6 als laatste, alleen-bij-symptomen.

## §5 — Cardboard mockup status

**Status: NIET gebouwd.** Voorgesteld in twee dev-logs, niet uitgevoerd op 2026-05-14, reden niet expliciet vastgelegd.

Eerste suggestie: `dev-log/2026-05-13-gateway-v1-cad-session-risks.md` §"Substantive design decisions still open" §C1 — "Decision deferred to a rested session, ideally after a cardboard mockup of 180 × 120 × 75 mm is taped together to feel ergonomically." Tweede suggestie (geüpdate dim): `dev-log/2026-05-14-c1-depth-stackup-arithmetic.md` §"Provisional implications" — "A cardboard mockup is still useful, but at 180 × 120 × ~90 mm rather than 180 × 120 × 75 mm" + §"Next session pickup" — "Cardboard mockup at 180 × 120 × ~90 mm when convenient — useful but no longer in the critical path."

Wat er gebeurde in plaats daarvan: de dag pivoteerde naar pogo-drop + Anker-dim correction + door rebuild + front-shell-solid relocation + EOD forensic. De ergonomische check voor depth was niet meer critical-path zodra 100 mm geaccept werd op basis van "doosje" framing + design-decisions retro §"Patroon dat door alles loopt" (de moeilijkere maar eerlijke beslissing nemen, inclusief "het is dik").

Wat we missen door niet te bouwen: ergonomische valideren of 180×120×100 mm fysiek prettig draagbaar is voor SAR field operators. Het paper-justified depth is verdedigbaar; de hand-feel is ongetest. Risico: eerste fysieke print prototype voelt onergonomisch, vereist redesign.

Aanbeveling: mockup voor de volgende CAD sessie als low-effort ergonomic sanity check. 30 minuten met karton/kleeflint vóór de eerste print is goedkoop tegenover een afwijzing-na-print iteration. Honest entry voor de retro: suggestion was deferred without explicit decision moment; should be picked up before v1 print commit.

## §6 — Tooling stack stability lessons

Wat over de hele tooling stack werkte, brak, en welke fallback bestond. Eén regel per item.

| Tool | Status | Wat brak / werkte | Fallback |
|---|---|---|---|
| `ndoo/fusion360-mcp-bridge` | Alpha (single commit, ~7 weeks old per 05-13 dev-log) | Algemeen bruikbaar als geheel; specifieke tools fragiel | Geen — er is geen tweede MCP bridge voor Fusion. Autodesk's officiële MCP is documented fallback bij paid subscription |
| `mcp__fusion360__fusion_screenshot` | **BROKEN** op Fusion 2702.x | "Viewport.saveAsImageFileWithOptions() takes 2 positional arguments but 5 were given" — bridge-wrapper kapot tegen huidige Fusion API | `app.activeViewport.saveAsImageFile(path, w, h)` direct via `fusion_execute` — werkt reliable, alle 11 screenshots vandaag via deze fallback |
| `mcp__fusion360__fusion_execute` | **STABIEL** | Alle CAD edits, queries, timeline rollback, forensic — geen failure modes vandaag | n/a — dit is de primary interface |
| `Edit` / `Read` / `Grep` / `Glob` tools | Stabiel | Alle doc edits + cross-doc propagation verlopen via Edit; grep voor stale value detection werkte | n/a |
| `WebFetch` voor vendor PDFs | Werkte na redirect-resolve | Direct PDF fetch van datasheets gaf de echte Pi TD2 dim (15 mm); redirect handling vereist tweede fetch met de geredirecte URL | `WebSearch` als laatste resort maar zie row hieronder |
| `WebSearch` voor mechanical dimensions | **UNRELIABLE** | Returned 8.55 mm voor Pi TD2 thickness; correct is 15 mm. Eerste-pass info is search-snippet-summary van mogelijk verouderde of foute sources | Always cross-check tegen vendor PDF direct via WebFetch |
| Cowork (chat Claude) working memory | Drift risk | Tweemaal passive→active cooling flip ondanks Pieter's prior corrections (zelfde dag); param-count tracking ontspoorde over multi-step edits | Memory commits voor caught flips (`feedback_cooling_is_passive.md`, `feedback_prototype_bulk_is_fine.md`); live-query Fusion voor param totals niet uit hoofd rekenen |
| Autodesk Assistant (in-app LLM audit) | Nuttig als input, NIET als output | 2 van 5 findings correct; 4 STALE (pre-correction spec readings) + 1 HALLUCINATION (boss asymmetry misread) | Filter workflow met 4-verdict tabel altijd vereist; nooit direct actie op audit output zonder verification |
| Git | Stabiel | 2 commits voor 2026-05-14 werk landed clean; geen merge conflicts; warning op CRLF (cosmetic) | `.gitattributes` config zou CRLF warnings stoppen (deferred, cosmetic) |
| Memory system (`MEMORY.md` + individual files) | Stabiel | 2 nieuwe memories opgeslagen vandaag (cooling-is-passive, prototype-bulk-is-fine) — beide direct nuttig | n/a |

**Patroon:** de stable layers (`fusion_execute`, `Edit/Read/Grep`, `git`, memory) zijn de meest gebruikt; de fragile layers (`fusion_screenshot`, `WebSearch`, Autodesk Assistant) zijn de meest specifiek nuttige maar vereisen workaround/filter. **Skill-relevant inzicht:** future Claude moet de fragility-niveau weten per tool en automatisch de fallback inzetten zonder Pieter te vragen.

## §7 — v2 wishlist preservatie

Features die tijdens 2026-05-14 (of eerder) expliciet naar v2 verwezen werden, met de v1-rejection reden bewaard. Doel: als toekomstige sessies deze re-propose-en, is de context wat er gewogen werd al opgeschreven — geen re-litigatie van settled trade-offs.

1. **Custom PCB substrate (Pi 5 + Dragino HAT vervangen door ontworpen board)**
   - V1 rejection: 4-6 maanden bring-up time, schematic/layout/spin risico, "first spin werkt zelden", drop-in components save calendar time. Botst met `CLAUDE.md` "quality > speed, hates fastest-time-to-market shortcuts" + "physical plug-and-play".
   - Bron: `retrospectives/2026-05-14-design-decisions.md` §1 "Substrate NIET pivoten naar custom PCB"
   - V2 angle: wanneer iteration speed minder kritiek is, custom PCB kan 10-15 mm depth besparen door de electronica-laag in te krimpen. Display (15 mm) en Anker (31.4 mm) blijven dominant — netto ~10-15 mm dunner is haalbaar, niet meer.

2. **Magnetic-pogo charging (in-shell)**
   - V1 rejection: lifelong IP65 sealing surface (pogo-pins zijn een continue leak-risk), daily gasket cycling, BOM accessories (magnetic cable + spare + magnet array), complicates signal contract (POWER_GOOD signal afhankelijk).
   - Bron: `dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md` + `retrospectives/2026-05-14-design-decisions.md` §2
   - V2 angle: alleen heropenen als user-research toont dat mid-incident charging kritiek is (huidige SARCOM use case is shift-change swap). Bij heropening: pick een vendor-class met geverifieerde current rating (≥5A voor Pi 5 peak); document magnetic-pogo lifespan in cycles vs the daily-swap alternatief.

3. **Steam Deck-style side buttons**
   - V1 rejection: niet in scope; touchscreen blijft enige UI per ADR-007; geen geometrische reservatie voor v1 (basic shell extrudes vermijden alleen thermische massa / boss-clusters op die zijden, passive constraint).
   - Bron: chat-thread + `dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md` §"Steam Deck-style side buttons"
   - V2 angle: vereist ADR-007 amendment (touchscreen-only UI) + side-edge geometric reservation + nieuwe bulkhead inventory entries (2-4 tactile buttons IP65 sealed). Niet meer compatible met current basic-shell geometry zonder rework.

4. **5" portrait variant (alternatief naast 7" landscape v1)**
   - V1 rejection: één variant per prototype generation; 7" landscape gekozen voor de chosen substrate + display class. Portrait variant geparked als v2 follow-up spike.
   - Bron: enclosure spike-close §Closed
   - V2 angle: kleinere form factor voor andere deployment scenarios (pocket-carry SAR operators vs hut-carry); requires parametric dual-front design die v1 expliciet NIET doet.

5. **External GPS SMA bulkhead**
   - V1 rejection: L80-M39 patch antenna (15×15×4 mm op de Dragino HAT) vuurt door 3 mm ASA shell; ADR-011 maakt GPS opportunistic (DS3231 RTC primair), dus "no GPS antenna" is geen blocker.
   - Bron: ADR-011 + enclosure spike-close §"GPS SMA" + retrospective glossary
   - V2 angle: alleen heropenen als field-test toont dat patch antenna geen sky-view kan krijgen door 3 mm ASA shell. Implementatie: één extra threaded SMA-female bulkhead + IPEX1.0→SMA pigtail van de HAT.

6. **Phone-friendly read-only map view (HTTP server op gateway)**
   - V1 rejection: partially superseded door ADR-016 outbound CoT/TAK export (phones met ATAK/iTAK/WinTAK zien SARCOM tag positions via die path). Volledige HTTP server schendt ADR-008 inbound-network-surface clause.
   - Bron: `TODO.md` §Deferred (v2+)
   - V2 angle: native phone-app of in-house phone-friendly HTTP UI als CoT/TAK export insufficient blijkt voor SAR operators' workflow. Vereist ADR-008 amendment + nieuwe security context.

7. **Cloud sync van tag_reports** (Postgres backend)
   - V1 rejection: ADR-008 "no cloud, no inbound network" — fundamental architectural commitment. SQLite local-first is de v1 contract.
   - Bron: TODO.md §Deferred
   - V2 angle: alleen heropenen na expliciete ADR-008 amendment of fundamentele scope-shift (e.g., SARCOM wordt commercieel product met multi-site coverage).

Niet alle v2 wishes zijn architectureel even open. De pogo-drop en de custom-PCB rejection cite expliciet de project values (`CLAUDE.md`); herroeping vereist een waarde-revisie. De external GPS bulkhead en 5" portrait variant zijn pragmatic deferrals — als evidence opduikt dat ze nodig zijn, ze openen zonder filosofische strijd. De cloud-sync en HTTP server zijn fundamenteel ADR-blocked en heropenen vereist een nieuwe ADR.

---

## Time-tracking

Niet bijgehouden vandaag. Pieter heeft geen tijdslog van de werkdag; ruwe inschatting op basis van de drie session brieven (morning → midday → afternoon → EOD wrap-up) is dat de totale CAD/docs werk meer dan 6 uur was, maar precieze breakdown is niet gevangen. Zou nuttig zijn voor toekomstige planning ("hoeveel tijd kost een depth-correction propagation pass" etc.) maar geen retroactive fabrication.

## Cross-refs

- `dev-log/2026-05-13-gateway-v1-cad-session-risks.md`
- `dev-log/2026-05-14-c1-depth-stackup-arithmetic.md`
- `dev-log/2026-05-14-pogo-drop-and-shell-extrudes.md`
- `dev-log/2026-05-14-anker-dims-and-gate-propagation.md`
- `dev-log/2026-05-14-cad-day-retrospective.md`
- `retrospectives/2026-05-14-design-decisions.md`
- `docs/cad-skill-reference.md` (2026-05-14 state; nog niet prescriptive geherstructureerd)
- `TODO.md` §"Carry-over voor volgende CAD sessie (2026-05-15+)"
