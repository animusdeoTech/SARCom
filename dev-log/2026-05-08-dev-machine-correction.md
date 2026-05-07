---
title: "Dev machine correction — Windows desktop, not a 'laptop'"
date: 2026-05-08
type: dev-log
session-trigger: "Pieter pushed back: 'I almost never work on a laptop, I work on my Windows home PC' — entropy fix"
---

# Dev machine correction

## What changed

Doc-set drift had accumulated 9 references to the dev machine framed as a "laptop" / "dev laptop" / "developer laptop". Pieter's actual dev machine is a **Windows home PC (desktop)** — he almost never uses a laptop for SARCOM work. Every Claude session reading those docs was absorbing the wrong assumption and propagating it into new prose.

This entry retires the framing across the doc set in a single commit. The 9 lines fixed:

| File:line | Old wording (retired) | New wording |
|---|---|---|
| `decisions/ADR-001-firmware-language.md:48` | "developer laptop compiles ESP32-S3 binaries…" | "developer workstation compiles ESP32-S3 binaries…" |
| `decisions/ADR-004-gateway-platform.md:37` | "Cross-compile from the dev laptop." | "Cross-compile from the dev workstation (Windows host)." |
| `ARCHITECTURE.md:644` | "kiosk spike on a laptop first, then on the Pi" | "kiosk spike on the dev workstation first, then on the Pi" |
| `TODO.md:73` | "rendering a local `.pmtiles` archive … on a laptop first, then on the Pi" | "… on the dev workstation first, then on the Pi" |
| `bom.md:77` | "Most laptops have one — only order if yours doesn't." | "Order one if your dev workstation does not have a built-in SD slot." |
| `bom.md:103` | "the laptop's manually-set system clock" | "the dev workstation's manually-set system clock" |
| `dev-log/2026-05-07-handheld-pivot-doc-audit-close.md:128` | quoted bom.md L99 wording with "the laptop's manually-set system clock" | quote updated to match the new bom.md L103 wording |
| `spikes/tak-cot-integration-spike.md:95` | "from a Rust binary on a Pi (or even Pieter's laptop)" | "… (or even Pieter's dev workstation)" |
| `spikes/tak-cot-integration-spike.md:145` | "or a Docker container on a laptop" | "or a Docker container on the dev workstation OR a second Pi" |

## What did NOT change

Third-party "phone/laptop" mentions stay untouched. They describe **someone else's** device — staff phone or service-engineer device, not Pieter's dev machine — and the BLE-commissioning / ADR-007 hut-staff framing depends on the existing wording. Specifically preserved as-is:

- `decisions/ADR-006-relay-has-gnss.md:32` — "Service engineer stands next to the pole with a phone/laptop, connects over BLE …"
- `decisions/ADR-007-touchscreen-primary-ui.md:57` — "Web app on staff's phone or laptop." (rejected alternative)
- `dev-log/2026-05-06-doc-contradictions-and-blockers-audit.md:35, :56` — quotes the BLE phone/laptop-as-peer formulation
- `spikes/ble-commissioning-scope-spike.md:17, :29, :46` — phone/laptop excluded from v1 BLE topology
- `spikes/ble-gateway-ui-flow-spike.md:24, :245` — handheld kiosk is the only UI; no phone, no third-party device
- `archive/product-roadmap.md` — archived; not maintained
- `decisions/ADR-004-gateway-platform.md:49` — "A laptop as gateway." (rejected alternative for gateway form factor; describes a hypothetical gateway shape, not Pieter's dev machine)
- `spikes/tak-cot-integration-spike.md:70` — "easier to spin up on a Pi or [generic-host] for local testing" — generic test-rig framing, ambiguous between Pieter's machine and any other host. Left untouched (not enumerated for this sweep).
- `TODO.md:156` — "Per-developer local runs of the system model on a [generic-host]" — generic developer framing in the deferred-v2 software-sim section, not Pieter-specific. Left untouched (not enumerated for this sweep).
- `TODO.md:173` — "phone/laptop-as-BLE-peer formulation" — third-party BLE topology reference.

## Rule for future sessions

CLAUDE.md "Tone and working style (Pieter)" section now pins the dev machine as Windows home PC (desktop). Do not re-open the question. Do not introduce new prose that frames Pieter's dev machine as a portable. Cross-compile / kiosk spike / espflash / Yocto-cross / `cargo check` workflow runs on a Windows desktop; that is the single source of truth.

If a future doc legitimately needs to describe a portable device (a third-party staff or operator device, a rejected-alternative gateway shape, a generic developer running the sim), name it explicitly as third-party / generic and do not let the framing leak back into Pieter's dev environment.
