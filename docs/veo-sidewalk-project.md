# Veo Sidewalk Detection — Project Documentation

> **Status:** Pre-hardware. SDK layer complete. Model selection and app development in progress.
> **Last updated:** March 2026

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Problem Statement](#2-problem-statement)
3. [System Architecture](#3-system-architecture)
4. [What We Have Today](#4-what-we-have-today)
5. [End-to-End Project Flow](#5-end-to-end-project-flow)
6. [Dataset: CityScapes](#6-dataset-cityscapes)
7. [ML Model Strategy](#7-ml-model-strategy)
8. [Hardware](#8-hardware)
9. [Development Phases & Roadmap](#9-development-phases--roadmap)
10. [Open Questions & Decisions Pending](#10-open-questions--decisions-pending)
11. [Repo Structure](#11-repo-structure)

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

### Not Yet Done
| Component | Status | Notes |
|---|---|---|
| Physical reCamera device | Pending | Awaiting delivery from supplier |
| Hardware-validated SDK | Pending | SDK is untested on real device |
| Sidewalk segmentation model | Pending | Training/selection not started |
| `.cvimodel` conversion | Pending | Requires Sophgo offline toolchain |
| Application code | Pending | Decision logic not yet written |
| Scooter integration | Pending | UART signal format TBD with hardware team |
| US sidewalk fine-tuning data | Pending | CityScapes is European — may need local data |

---

## 5. End-to-End Project Flow

### Full pipeline from camera to scooter response

```
Camera captures frame (1280×720 or higher, JPEG/RGB)
    │
    ▼
Segmentation model runs on NPU
    │  Input:  raw frame pixels
    │  Output: per-pixel class label tensor
    ▼
Post-processing (your app)
    │  Collapse 30 CityScapes classes → 2: sidewalk / not-sidewalk
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
Train segmentation model
(YOLOv8-seg or DeepLabV3, simplified to 2 classes)
    │
    ▼
Export to ONNX
    │
    ▼
Convert to .cvimodel using Sophgo Model Deploy Tool
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
3. Train a lightweight segmentation model (2-class is much simpler than 30-class)
4. Evaluate on held-out CityScapes val set → establish baseline IoU
5. Once reCamera arrives: collect 200-500 frames from actual scooter, label sidewalk, fine-tune

---

## 7. ML Model Strategy

### Model candidates

| Model | Pros | Cons |
|---|---|---|
| YOLOv8-seg | Fast, ONNX export built-in, good community support | More complex than needed for 2-class problem |
| DeepLabV3+ (MobileNetV3 backbone) | Lightweight, good for edge, strong on CityScapes | Slower ONNX ecosystem |
| SegFormer-B0 | State of the art, small variant available | Transformer — may be heavy for SG2002 NPU |
| Custom lightweight U-Net | Smallest possible, tuned for 2 classes | Needs training from scratch |

**Current recommendation:** Start with YOLOv8-seg (pretrained on CityScapes or COCO) — fastest path to a working `.cvimodel`.

### Model conversion path

The SG2002 NPU requires `.cvimodel` format. The conversion chain:

```
PyTorch model
    → torch.onnx.export()
    → model.onnx
    → Sophgo Model Deploy Tool (run_calibration.py + model_deploy.py)
    → model.cvimodel
```

Sophgo's toolchain documentation: [github.com/sophgo/tpu-mlir](https://github.com/sophgo/tpu-mlir)

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

## 8. Hardware

### Seeed reCamera

- **SoC:** Sophgo SG2002
- **NPU:** 1 TOPS (INT8)
- **Camera:** Integrated, configurable resolution and FPS
- **Communication:** UART, RS-485 (for scooter integration)
- **OS:** Linux
- **Vendor libs:** `libsys.so`, `libvi.so`, `libvpss.so`, `libvenc.so`, `libcviruntime.so` — loaded at runtime by `recamera-rs`

**Device status:** Awaiting delivery from supplier. SDK was built to work without the device present (builds compile on any host; vendor libs only needed at runtime on device).

### Scooter integration

The reCamera will communicate with Veo's scooter control system over UART or RS-485. Exact signal protocol TBD — requires coordination with Veo's hardware/firmware team.

---

## 9. Development Phases & Roadmap

### Phase 1 — Pre-hardware (now)
- [x] Understand `recamera-rs` SDK
- [x] Identify dataset (CityScapes)
- [ ] Write application skeleton (compiles, wires up SDK calls, won't run without device)
- [ ] Write and unit test post-processing logic with mock frame data
- [ ] Define config file schema (`scooter.toml`)
- [ ] Research model conversion toolchain (Sophgo tpu-mlir)
- [ ] Download CityScapes, explore class distribution

### Phase 2 — Model
- [ ] Train baseline 2-class segmentation model on CityScapes
- [ ] Evaluate on CityScapes val set (target: >70% sidewalk IoU)
- [ ] Convert to ONNX
- [ ] Convert ONNX → `.cvimodel` using Sophgo toolchain
- [ ] Verify model loads via `recamera-infer` (can mock-test with dummy tensors)

### Phase 3 — Hardware arrives
- [ ] Mount reCamera on scooter (or test rig)
- [ ] Validate SDK works on real device — shake out any FFI/library loading bugs
- [ ] Run end-to-end pipeline: capture → infer → log output
- [ ] Measure latency (target: <100ms per frame)
- [ ] Collect real scooter footage for fine-tuning

### Phase 4 — Integration
- [ ] Wire detection output to scooter control (UART signal)
- [ ] Define and implement response behavior (slow down / alert / log)
- [ ] Fine-tune model on real scooter footage
- [ ] Tune detection threshold to minimize false positives

### Phase 5 — Production
- [ ] Field testing on real scooters
- [ ] Safety review and edge case testing
- [ ] Performance and power profiling
- [ ] Deployment packaging and OTA update strategy

---

## 10. Open Questions & Decisions Pending

| Question | Owner | Priority |
|---|---|---|
| What is the UART signal format for scooter speed control? | Hardware/firmware team | High |
| What is the acceptable false positive rate? (sidewalk detected when not on sidewalk) | Product/Safety | High |
| Camera mounting angle and position on scooter | Hardware team | High |
| Do we need night/low-light performance? | Product | Medium |
| US-specific fine-tuning data — how to collect and label? | ML | Medium |
| Target inference latency budget? | Systems | Medium |
| What happens if the camera/model fails? Fail-safe behavior? | Safety | High |
| Single scooter pilot or fleet rollout plan? | Product | Low |

---

## 11. Repo Structure

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

> This document is a living record. Update it as decisions are made, hardware arrives, and phases complete.
