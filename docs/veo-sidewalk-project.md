# Veo Sidewalk Detection — Project Documentation

> **Status:** Model pipeline complete. Hardware devices purchased and being tested in China. Annotation tool selection and calibration image collection in progress.
> **Last updated:** April 2026

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Problem Statement](#2-problem-statement)
3. [System Architecture](#3-system-architecture)
4. [What We Have Today](#4-what-we-have-today)
5. [End-to-End Project Flow](#5-end-to-end-project-flow)
6. [Dataset: CityScapes](#6-dataset-cityscapes)
7. [ML Model Strategy](#7-ml-model-strategy)
8. [Annotation Tool](#8-annotation-tool)
9. [Calibration Images](#9-calibration-images)
10. [Hardware](#10-hardware)
11. [Development Phases & Roadmap](#11-development-phases--roadmap)
12. [Open Questions & Decisions Pending](#12-open-questions--decisions-pending)
13. [Repo Structure](#13-repo-structure)

---

## 1. Project Overview

This project builds a **real-time sidewalk detection safety system** for Veo scooters. A camera mounted on the scooter continuously analyzes the road ahead. If the scooter is detected to be on a sidewalk, the system triggers a safety response — such as slowing down the vehicle or alerting the rider.

The system runs **entirely on the scooter** (edge inference — no cloud, no network dependency) using the Seeed reCamera's onboard NPU.

The underlying hardware SDK is [`recamera-rs`](../README.md) — a Rust SDK for the reCamera platform, maintained as an open-source project.

---

## 2. Problem Statement

Scooters riding on sidewalks are a safety hazard to pedestrians and create regulatory and liability issues. Current solutions rely on GPS geofencing, which:

- Has poor accuracy at the sidewalk vs. road boundary level (~2-5m GPS error)
- Cannot detect actual riding behavior in real time
- Does not work in areas without geofence coverage

**Vision-based detection** solves this at the pixel level: if the camera sees sidewalk beneath the scooter, the system acts — regardless of GPS accuracy or coverage.

---

## 3. System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Veo Scooter                          │
│                                                             │
│  ┌──────────────┐     ┌──────────────────────────────────┐ │
│  │  reCamera    │────▶│         Your Rust App            │ │
│  │  (SG2002)    │     │                                  │ │
│  │              │     │  1. Capture frame                │ │
│  │  Camera ─────┼────▶│  2. Run segmentation model      │ │
│  │  NPU    ─────┼────▶│  3. Count sidewalk pixels        │ │
│  │              │     │  4. If threshold exceeded →      │ │
│  └──────────────┘     │     trigger response             │ │
│                       └────────────┬─────────────────────┘ │
│                                    │ UART / RS-485          │
│                       ┌────────────▼─────────────────────┐ │
│                       │     Scooter Control System        │ │
│                       │  (slow down / alert / log)        │ │
│                       └───────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

**SDK layer** (`recamera-rs`) handles everything below the application:

```
Your App
  └── recamera-camera    → grab frames from camera
  └── recamera-infer     → run .cvimodel on NPU
  └── recamera-uart      → send signal to scooter hardware
  └── recamera-config    → load settings from file
  └── recamera-logging   → log events for safety audits
  └── recamera-storage   → save flagged frames to disk
```

---

## 4. What We Have Today

### Done
| Component | Status | Notes |
|---|---|---|
| `recamera-rs` SDK | Complete | All 11 crates implemented, 47 unit tests pass |
| Camera capture API | Complete | `recamera-camera` — frame capture from CVI pipeline |
| NPU inference API | Complete | `recamera-infer` — loads and runs `.cvimodel` files |
| UART / RS-485 API | Complete | `recamera-uart`, `recamera-rs485` — serial comms |
| Config loading | Complete | `recamera-config` — TOML file parsing |
| Logging | Complete | `recamera-logging` — structured tracing |
| Storage | Complete | `recamera-storage` — save frames to disk |
| Dataset identified | Complete | CityScapes — has labeled `sidewalk` class |
| Model selected | Complete | BiSeNetV2 confirmed as first model to try |
| Model pipeline | Complete | PyTorch → ONNX → `.cvimodel` pipeline working |
| Cross-compilation | Complete | Rust cross-compile to SG2002 unblocked |
| Calibration images | In progress | 88/~100 collected by CTO |
| reCamera devices | Purchased | 2 devices in China, being tested by embedded team |

### Not Yet Done
| Component | Status | Notes |
|---|---|---|
| Hardware-validated SDK | In progress | Being tested in China with embedded team |
| Annotation tool | In progress | CVAT identified as top choice, to be confirmed |
| Annotated training data | Pending | Starts after annotation tool is finalized |
| Application code | Pending | Decision logic not yet written |
| Scooter integration | Pending | UART signal format TBD with hardware team |
| US sidewalk fine-tuning data | Pending | CityScapes is European — need local data |

---

## 5. End-to-End Project Flow

### Full pipeline from camera to scooter response

```
Camera captures frame (1280×720, JPEG/RGB)
    │
    ▼
Segmentation model (BiSeNetV2) runs on NPU
    │  Input:  raw frame pixels
    │  Output: per-pixel class label tensor
    ▼
Post-processing (your app)
    │  Collapse CityScapes classes → 2: sidewalk / not-sidewalk
    │  Count sidewalk pixels as % of frame
    │  Apply threshold (e.g. >15% of bottom half = on sidewalk)
    ▼
Decision logic
    │  Below threshold → continue normal operation
    │  Above threshold → trigger response
    ▼
Response (TBD)
    ├── Option A: Send speed reduction command via UART
    ├── Option B: Trigger onboard alert / buzzer
    └── Option C: Log event + GPS stamp for fleet review
```

### Model pipeline (offline, one-time setup)

```
CityScapes dataset
    │
    ▼
Train BiSeNetV2 segmentation model
(simplified to 2 classes: sidewalk / not-sidewalk)
    │
    ▼
Export to ONNX
    │
    ▼
Collect ~100 calibration images (unannotated, from real scooter angle)
    │
    ▼
Convert ONNX → INT8 .cvimodel using Sophgo Model Deploy Tool
(calibration images used here to determine quantization ranges)
    │
    ▼
Deploy to reCamera at /userdata/models/sidewalk.cvimodel
    │
    ▼
recamera-infer loads and runs it at runtime
```

---

## 6. Dataset: CityScapes

**Source:** [cityscapes-dataset.com](https://www.cityscapes-dataset.com) — free with registration

**Reference:** [CVAT blog — Top Datasets for Semantic Segmentation](https://www.cvat.ai/resources/blog/top-datasets-semantic-segmentation)

### What it is

A large-scale benchmark dataset for semantic understanding of urban street scenes. Recorded from a camera mounted on a car across 50 cities in Germany and Switzerland. Every pixel in every image is labeled with a semantic class.

### Stats

| Property | Value |
|---|---|
| Total frames | 25,000 |
| Fine annotations (pixel-precise) | 5,000 |
| Coarse annotations | 20,000 |
| Image resolution | 2048 × 1024 |
| Number of classes | 30+ |
| Recording locations | 50 cities across Germany/Switzerland |

### Classes relevant to this project

```
Flat surfaces
  ├── road       ← where scooter should be
  └── sidewalk   ← where scooter should NOT be  ← our target class
```

Sidewalk makes up approximately **5% of the dataset** by pixel area.

### Why it's a good starting point

- Has a dedicated `sidewalk` class with pixel-level annotations
- Designed for moving-vehicle camera perspectives (similar to scooter-mounted camera)
- Widely used — many pretrained models available that already understand these classes
- State-of-the-art models achieve >86% mean IoU on this benchmark

### Known limitations for our use case

| Limitation | Impact | Mitigation |
|---|---|---|
| European city footage | US sidewalks look different (materials, curb cuts, markings) | Fine-tune on US footage once device arrives |
| Car-height camera angle | Scooter camera is lower, different perspective | Collect calibration frames from mounted camera |
| 5% sidewalk class imbalance | Model may underweight sidewalk pixels during training | Use class-weighted loss during training |
| Weather diversity limited | Real scooter conditions (night, rain, glare) may not be covered | Augment training data or collect edge-case footage |

### Plan for this dataset

1. Download CityScapes fine annotations (5,000 images)
2. Remap all 30+ classes to 2: `sidewalk=1`, `everything_else=0`
3. Train BiSeNetV2 on remapped data
4. Evaluate on held-out CityScapes val set → establish baseline IoU
5. Once reCamera arrives: collect real scooter footage, label sidewalk, fine-tune

---

## 7. ML Model Strategy

### Model decision

| Model | Status | Reason |
|---|---|---|
| PP-LiteSeg | Rejected | Unstable ONNX conversion — breaks during PyTorch → ONNX export |
| **BiSeNetV2** | **Selected** | Reliable ONNX conversion, fast, designed for real-time street segmentation |
| YOLOv8-seg | Deprioritized | More complex than needed for 2-class problem |
| DeepLabV3+ | Deprioritized | Slower ONNX ecosystem |

**BiSeNetV2** is a real-time semantic segmentation model designed specifically for street scenes. It uses a bilateral segmentation network — one branch captures spatial detail, the other captures context — making it fast and accurate for road/sidewalk scenes.

### Model conversion path

The SG2002 NPU requires `.cvimodel` INT8 quantized format. The conversion chain:

```
PyTorch BiSeNetV2 model
    → torch.onnx.export()
    → model.onnx
    → Sophgo Model Deploy Tool
        (run_calibration.py with ~100 calibration images)
        (model_deploy.py → INT8 quantization)
    → sidewalk.cvimodel
```

Sophgo's toolchain: [github.com/sophgo/tpu-mlir](https://github.com/sophgo/tpu-mlir)

**Status:** Pipeline is working end-to-end. Pending real device test.

### Output interpretation

`recamera-infer` returns raw tensors — your application is responsible for post-processing:

```rust
let output = model.run(&frame.data)?;
match output {
    Output::Raw(tensors) => {
        // tensors[0] shape: [1, H, W] or [1, 2, H, W] depending on model
        // Count pixels where argmax == sidewalk_class_id
        // Compute as % of frame → compare to threshold
    }
    _ => {}
}
```

---

## 8. Annotation Tool

### Why we need one

CityScapes gives us a baseline model. But to improve accuracy for US streets and the scooter camera angle, we need to label our own real footage and retrain. The annotation tool is how we draw "this pixel is sidewalk" on hundreds of real frames.

### Decision: CVAT

**CVAT** ([cvat.ai](https://cvat.ai)) is the recommended tool.

| Reason | Detail |
|---|---|
| Native CityScapes format | Import and export directly — no conversion needed |
| Free cloud tier | Start immediately, no credit card |
| AI-assisted annotation | SAM 3 integration — click once, it auto-segments sidewalk |
| Video support | Annotate frames from scooter footage directly |
| Multi-user | Team can collaborate on labeling |
| Self-hostable | Can run on own infrastructure if needed |

### Alternatives evaluated

| Tool | CityScapes Support | Free | AI Assist | Verdict |
|---|---|---|---|---|
| CVAT | Native | Yes | SAM 3 | **Recommended** |
| Label Studio | No (needs conversion) | Yes (self-hosted) | SAM 2 | Good budget backup |
| V7 Darwin | No | No ($150+/mo) | SAM 3 | Best UX but costly |
| Roboflow | Unclear | Limited | Yes | Good for non-technical teams |
| Scale AI | Yes | No (enterprise) | Yes (automotive AI) | Overkill for small team |

**Status:** To be confirmed and shared in team Slack channel by end of this week.

---

## 9. Calibration Images

### What they are and why we need them

The SG2002 NPU runs models in **INT8** (compressed integers) instead of floating point. When compressing the model, the Sophgo quantization tool needs to see real input images to figure out the correct number ranges. Without this, the compressed model produces garbage outputs.

Calibration images are **~100 unannotated photos** that represent what the camera will actually see in production. No labeling needed — the tool just looks at pixel values.

### Requirements

| Property | Requirement |
|---|---|
| Count | ~100 images |
| Annotations needed | None |
| Resolution | 1280×720 (match reCamera output) |
| Format | JPEG |
| Camera angle | Scooter handlebar height, slightly downward, road-facing |
| Coverage | ~50 daytime, ~50 nighttime |
| Content | Mix of sidewalk, road, curb transitions, varied lighting |

### Status

| Source | Count | Status |
|---|---|---|
| CTO collected | 88 | Done |
| Shruti to collect | ~12 remaining | In progress — using phone at scooter height |

### Collection guidance (phone-based)

Since the reCamera is not yet available, use a phone and match these settings:
- Shoot in landscape at 1280×720 (or resize after)
- Hold at ~handlebar height, pointing slightly downward
- Capture real streets: sidewalk visible, curb cuts, road
- Cover daytime and nighttime conditions

---

## 10. Hardware

### Seeed reCamera

- **SoC:** Sophgo SG2002
- **NPU:** 1 TOPS (INT8)
- **Camera:** Integrated, configurable resolution and FPS
- **Communication:** UART, RS-485 (for scooter integration)
- **OS:** Linux
- **Vendor libs:** `libsys.so`, `libvi.so`, `libvpss.so`, `libvenc.so`, `libcviruntime.so` — loaded at runtime by `recamera-rs`

**Device status:** 2 units purchased, currently in China being tested by CTO and embedded team.

### Cross-compilation

Rust cross-compilation to SG2002 was a blocker — the vendor toolchain is old and didn't support modern Rust. This has been resolved. However, device-side runtime errors are still possible and will be discovered during testing in China.

Fallback: if runtime issues cannot be resolved, the application will be rewritten in C/C++ directly against Sophgo's C SDK. The overall project architecture and ML work remains unchanged regardless.

### Scooter integration

The reCamera will communicate with Veo's scooter control system over UART or RS-485. Exact signal protocol TBD — requires coordination with Veo's hardware/firmware team.

---

## 11. Development Phases & Roadmap

### Phase 1 — Pre-hardware
- [x] Understand `recamera-rs` SDK
- [x] Identify dataset (CityScapes)
- [x] Select model (BiSeNetV2)
- [x] Build model conversion pipeline (ONNX → `.cvimodel`)
- [x] Resolve Rust cross-compilation
- [ ] Finalize annotation tool (CVAT) — share in Slack this week
- [ ] Complete calibration image collection (~12 remaining)

### Phase 2 — Hardware testing (in progress, China)
- [ ] Validate SDK runs on real reCamera device
- [ ] Run end-to-end: capture frame → run model → read output
- [ ] Measure inference latency (target: <100ms per frame)
- [ ] Confirm Rust runtime works (or fall back to C/C++)

### Phase 3 — Training & annotation (starts after CTO returns)
- [ ] Set up CVAT with CityScapes data
- [ ] Train BiSeNetV2 baseline on CityScapes (2-class)
- [ ] Evaluate baseline IoU on val set
- [ ] Collect real US scooter footage
- [ ] Annotate with CVAT → export CityScapes format
- [ ] Fine-tune model on real data

### Phase 4 — Integration
- [ ] Wire detection output to scooter control (UART signal)
- [ ] Define and implement response behavior (slow down / alert / log)
- [ ] Tune detection threshold to minimize false positives
- [ ] Write application code

### Phase 5 — Production
- [ ] Field testing on real scooters
- [ ] Safety review and edge case testing
- [ ] Performance and power profiling
- [ ] Deployment packaging and OTA update strategy

---

## 12. Open Questions & Decisions Pending

| Question | Owner | Priority |
|---|---|---|
| Does Rust runtime work on real reCamera device? | CTO / embedded team (China) | High |
| What is the UART signal format for scooter speed control? | Hardware/firmware team | High |
| What is the acceptable false positive rate? | Product/Safety | High |
| Camera mounting angle and position on scooter | Hardware team | High |
| What happens if camera/model fails? Fail-safe behavior? | Safety | High |
| Target inference latency budget? | Systems | Medium |
| Do we need night/low-light performance? | Product | Medium |
| US-specific fine-tuning data — how to collect and label? | ML | Medium |
| Single scooter pilot or fleet rollout plan? | Product | Low |

---

## 13. Repo Structure

```
recamera-rs/
├── README.md                        # SDK documentation (recamera-rs)
├── docs/
│   └── veo-sidewalk-project.md      # This file — Veo application documentation
├── Cargo.toml                       # Workspace manifest
└── crates/
    ├── recamera/                    # Facade crate
    ├── recamera-core/               # Shared types and errors
    ├── recamera-camera/             # Camera capture
    ├── recamera-infer/              # NPU inference
    ├── recamera-cvi-sys/            # FFI bindings + runtime loader
    ├── recamera-uart/               # UART serial
    ├── recamera-rs485/              # RS-485
    ├── recamera-storage/            # File/image storage
    ├── recamera-config/             # TOML config loading
    ├── recamera-logging/            # Structured logging
    └── recamera-system/             # Device info, LED, system utils
```

---

> This document is a living record. Update it as decisions are made, hardware is tested, and phases complete.
