# Claude Context — Veo Sidewalk Detection Project

Read this before doing anything. Full project documentation is at [docs/veo-sidewalk-project.md](docs/veo-sidewalk-project.md).

## What This Repo Is

`recamera-rs` is a Rust SDK for the Seeed reCamera (SG2002 SoC). Shruti is using it to build a **real-time sidewalk detection safety system for Veo scooters**. The camera detects if a scooter is on a sidewalk and triggers a response (slow down / alert).

## Current Status (April 2026)

- Model pipeline complete: BiSeNetV2 → ONNX → `.cvimodel`
- 2 reCamera devices purchased, being tested in China by CTO + embedded team
- Rust cross-compilation unblocked (C/C++ fallback still possible if device runtime fails)
- Annotation tool being finalized (CVAT is top recommendation)
- ~88/100 calibration images collected

## Key Decisions Made

| Decision | Choice | Reason |
|---|---|---|
| Segmentation model | BiSeNetV2 | PP-LiteSeg rejected — unstable ONNX conversion |
| Training dataset | CityScapes | Has native `sidewalk` class, pixel-level annotations |
| Annotation tool | CVAT (pending confirm) | Native CityScapes format, free, SAM 3 AI assist |
| Language | Rust (via recamera-rs) | Cross-compile resolved; C/C++ fallback if device fails |
| Camera resolution | 1280×720 | reCamera default |

## What Shruti Is Working On

1. Collecting remaining calibration images (~phone, scooter handlebar height, 1280×720, day + night)
2. Posting CVAT annotation tool recommendation in team Slack
3. Waiting on CTO to return from China before in-person annotation discussion

## Codebase Layout

- `crates/` — 11 Rust crates (SDK layer, do not modify unless fixing SDK bugs)
- `docs/veo-sidewalk-project.md` — full living project doc, update this as things change

## Things That Are NOT Done Yet

- Application code (the actual Rust app that runs on the scooter)
- Annotated training data
- Model trained on real US footage
- Scooter UART signal protocol defined
- Hardware-validated SDK (pending China device test)

## How to Help

- Keep `docs/veo-sidewalk-project.md` up to date as decisions are made
- When hardware test results come in from China, update the roadmap
- When asked to write application code, refer to the SDK APIs in `crates/recamera-camera`, `crates/recamera-infer`, `crates/recamera-uart`
