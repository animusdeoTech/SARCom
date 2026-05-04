---
title: "Spike — SARCOM commercial / application viability"
status: open
type: spike
timebox: 1 day
---

# Spike: SARCOM commercial and application viability

## Why this spike exists

SARCOM is designed as a trail-SAR portfolio project. The architecture — offline, local-first, low-bandwidth last-known-position, no cloud/SIM/live tracking — happens to fit other domains too. Before spending engineering time on v1, it is worth spending one day asking whether this is worth productising, and if so, where.

The question is not "can we sell this." The question is:

> Is SARCOM a one-off SAR demonstrator, or does the core architecture have a credible edge in other domains where existing solutions are overkill, brittle, or economically annoying?

## Hypotheses

**H1 (viable):** There are 2–3 domains where SARCOM's offline/local/no-SIM-per-tag model solves a real gap that cloud/LTE/LoRaWAN platforms do not. Those domains can be named precisely.

**H0 (portfolio-only):** Every relevant domain is already adequately served by GPS/LTE trackers, LoRaWAN platforms, AIS/VTS, or farm-management systems. SARCOM stays a strong portfolio project and secondary domains are noted but not pursued.

Honest prior: **H0 is more likely for most domains.** The edge, if it exists, is narrow: places where the local-first architecture is a feature, not a constraint.

## Candidate domains

Evaluate these seven, no others:

1. Mountain / trail SAR *(the design domain — baseline)*
2. Marina / small-boat monitoring
3. Livestock and horses
4. Remote farm / field assets
5. Construction and forestry equipment
6. Outdoor events, camps, festivals
7. Nature parks and ranger operations

## What to research per domain

For each domain, answer four questions:

**1. What exists already?**
Name 2–3 concrete products or standards (AIS, VTS, LoRaWAN cattle collars, LTE-M boat monitors, etc.). Rough cost. Cloud/SIM dependency. Offline capability.

**2. Where do existing solutions fall short?**
Be specific: too expensive, requires SIM per asset, requires cloud account, fails where there is no LTE/WiFi, overkill for small sites, requires vendor platform, etc.

**3. Is SARCOM's architecture actually better there, or just different?**
"Different" is not enough. Better means: solves the gap from question 2 without introducing a worse problem.

**4. Who pays and is it plausible?**
Rough buyer profile. Willingness to pay for infra + devices vs. monthly subscription. One-off install vs. recurring.

## Scoring matrix

Rate each domain 1–5 on eight axes, then sum:

| Domain | Pain | Existing gap | SARCOM fit | Offline advantage | WTP | Deploy complexity | Competition | Portfolio value |
|--------|------|-------------|------------|------------------|-----|------------------|-------------|-----------------|
| Mountain SAR | | | | | | | | |
| Marina / boats | | | | | | | | |
| Livestock | | | | | | | | |
| Farm assets | | | | | | | | |
| Construction / forestry | | | | | | | | |
| Events / camps | | | | | | | | |
| Parks / rangers | | | | | | | | |

Top 2 domains by total score are the candidates to pursue.

## Acceptance criteria

Spike is done when you can write:

- For the top 2 domains: one paragraph each — what the existing solution is, what its gap is, why SARCOM is better (or not)
- A short list of product claims that are defensible
- A short list of claims to avoid
- A decision: pursue one of the domains / park as secondary / portfolio-only

## Claims likely safe to make

- Low-bandwidth last-known-position, not live tracking
- Local-first: works when cloud/LTE/WiFi are gone
- No SIM per tag
- Suitable for sparse periodic sightings and alarm states (SOS / geofence exit)
- One local gateway covers a site; relays extend coverage without infrastructure contracts

## Claims to avoid

- Replaces AIS or professional VTS/SAR systems
- Real-time tracking
- Precise LoRa-based localisation
- Works anywhere / global coverage
- Cheaper than everything
- No maintenance
- Commercial-ready (it is not, v1 is a garden demo)

## Fallback

If no domain clears a credible gap:

1. Keep SARCOM as portfolio / SAR mission-design project — that is a valid outcome
2. Note secondary domains in README as "architecturally compatible but not commercially prioritised"
3. Do not write a business plan until v1 garden demo exists and works

## Decision note template

```
Date:
Top domains: [list]
Gap per domain: [one line each]
SARCOM better because: [one line each, or "not better"]
Claims to use: [list]
Claims to avoid: [list]
Decision: pursue [domain] / park / portfolio-only
Rationale:
Next action:
```
