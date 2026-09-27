---
description: 'Top-level OKF summary: 14332 concepts across 3 domains and 1043 modules'
git_branch: master
git_repo: oxide-eda
okf_version: '0.2'
timestamp: '2026-09-27T20:22:35Z'
title: oxide-eda — Knowledge Summary
type: Index
---

# oxide-eda — Knowledge Summary

> OKF v0.2 bundle | 14,332 concepts | 3 domains | 1043 modules

## Stats

| Type | Count |
|------|-------|
| Function | 11,383 |
| Class | 1,608 |
| Module | 1,043 |
| Dependency | 277 |
| Table | 11 |
| Index | 10 |

| Language | Concepts |
|----------|----------|
| rust | 14,015 |
| manifest | 277 |
| sql | 25 |
| python | 15 |

## Domain Map

Use these links to navigate the bundle or prime an AI agent with focused context.

### [crates](crates/index.md) — 14,040 concepts

- [crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad](crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/index.md) (137 concepts) — Footprint-editor active-tab accessors and pad-property setters —
- [crates/oxide-app/src/keymap/profile](crates/oxide-app/src/keymap/profile/index.md) (102 concepts)
- [crates/oxide-library/src/adapters/library_set](crates/oxide-library/src/adapters/library_set/index.md) (88 concepts) — `LibrarySet` — a tiny resolver that composes any number of
- [crates/oxide-app/src/app/documents](crates/oxide-app/src/app/documents/index.md) (70 concepts)
- [crates/oxide-library/src/adapters/database](crates/oxide-library/src/adapters/database/index.md) (69 concepts) — `LibraryAdapter` over the HTTP API exposed by `oxide-library-server`.
- [crates/oxide-app/src/fonts/mod](crates/oxide-app/src/fonts/mod/index.md) (67 concepts) — Font management for Oxide.
- [crates/oxide-app/src/app/state/mod](crates/oxide-app/src/app/state/mod/index.md) (67 concepts)
- [crates/oxide-library/src/adapters/local_git/adapter](crates/oxide-library/src/adapters/local_git/adapter/index.md) (65 concepts) — `LibraryAdapter` trait implementation for `LocalGitAdapter`.
- *…and 1031 more modules*

### [installer](installer/index.md) — 2 concepts

- [installer/build-wordmark](installer/build-wordmark/index.md) (2 concepts) — Rasterize the Oxide wordmark SVGs into PNGs at 1x / 2x / 3x DPI tiers.

### [tools](tools/index.md) — 13 concepts

- [tools/build_icons](tools/build_icons/index.md) (5 concepts) — Pure-Python fallback for installer/build-icons.sh.
- [tools/sync_wordmark_from_black](tools/sync_wordmark_from_black/index.md) (4 concepts) — Mirror the <path id="text11"> 'd' attribute from oxide-logo-black.svg into
- [tools/regen_wordmark_path](tools/regen_wordmark_path/index.md) (4 concepts) — Regenerate the 'oxide' wordmark path in brand SVGs from Panton-Bold.ttf.

## Dependencies

> Full list at [`_dependencies/index.md`](/_dependencies/index.md) or `okf lookup --type Dependency`

| Ecosystem | Packages |
|----------|----------|
| cargo | 277 |

## Key Concepts

Highest-value concepts across all domains (Classes and Functions with rich descriptions).

| Concept | Type | Module | Description |
|---------|------|--------|-------------|
| [theoretical_bpsk_awgn](/crates/oxide-rf/src/ber/theoretical_bpsk_awgn.md) | Function | `crates/oxide-rf/src` | Theoretical BPSK/QPSK BER in AWGN: $P_b = \frac{1}{2} \text{… |
| [theoretical_bpsk_awgn](/crates/oxide-rf/src/ber/theoretical_bpsk_awgn_1.md) | Function | `crates/oxide-rf/src` | Theoretical BPSK/QPSK BER in AWGN: $P_b = \frac{1}{2} \text{… |
| [compute_synchronized_timestep](/crates/oxide-cosim/src/mixed_signal/compute_synchronized_timestep.md) | Function | `crates/oxide-cosim/src` | Determines the next synchronized continuous timestep $h_{\te… |
| [compute_synchronized_timestep](/crates/oxide-cosim/src/mixed_signal/compute_synchronized_timestep_1.md) | Function | `crates/oxide-cosim/src` | Determines the next synchronized continuous timestep $h_{\te… |
| [evaluate_trajectory](/crates/oxide-cosim/src/mixed_signal/evaluate_trajectory.md) | Function | `crates/oxide-cosim/src` | Evaluates continuous voltage trajectory (t0, v0) -> (t1, v1)… |
| [evaluate_trajectory](/crates/oxide-cosim/src/mixed_signal/evaluate_trajectory_1.md) | Function | `crates/oxide-cosim/src` | Evaluates continuous voltage trajectory (t0, v0) -> (t1, v1)… |
| [compute_target_impedance](/crates/oxide-rf/src/pdn/compute_target_impedance.md) | Function | `crates/oxide-rf/src` | Computes $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Del… |
| [compute_target_impedance](/crates/oxide-rf/src/pdn/compute_target_impedance_1.md) | Function | `crates/oxide-rf/src` | Computes $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Del… |
| [multiply](/crates/oxide-physics/src/dual_quat/multiply_2.md) | Function | `crates/oxide-physics/src` | Composes dual quaternions: $\hat{\mathbf{q}}_{\text{combined… |
| [multiply](/crates/oxide-physics/src/dual_quat/multiply_3.md) | Function | `crates/oxide-physics/src` | Composes dual quaternions: $\hat{\mathbf{q}}_{\text{combined… |
| [verify_compliance](/crates/oxide-rf/src/pdn/verify_compliance.md) | Function | `crates/oxide-rf/src` | Verifies whether the PDN impedance profile satisfies $Z(f) \… |
| [verify_compliance](/crates/oxide-rf/src/pdn/verify_compliance_1.md) | Function | `crates/oxide-rf/src` | Verifies whether the PDN impedance profile satisfies $Z(f) \… |
| [RoutingEngine](/crates/oxide-router/src/lib/RoutingEngine.md) | Class | `crates/oxide-router/src` | Main routing engine coordinating topological autorouting, in… |
| [solve_microstrip](/crates/oxide-physics/src/bem_solver/solve_microstrip.md) | Function | `crates/oxide-physics/src` | Evaluates transmission line parameters using closed-form ana… |
| [solve_microstrip](/crates/oxide-physics/src/bem_solver/solve_microstrip_1.md) | Function | `crates/oxide-physics/src` | Evaluates transmission line parameters using closed-form ana… |
| [copy_room_format](/crates/oxide-engine/src/room/copy_room_format.md) | Function | `crates/oxide-engine/src` | Copies the relative placement, orientation, routing traces, … |
| [copy_room_format](/crates/oxide-engine/src/room/copy_room_format_1.md) | Function | `crates/oxide-engine/src` | Copies the relative placement, orientation, routing traces, … |
| [strip_channel_suffix](/crates/oxide-engine/src/room/strip_channel_suffix.md) | Function | `crates/oxide-engine/src` | Helper function to strip channel numbers/suffixes (e.g. "R1_… |
| [TopologicalRouter](/crates/oxide-router/src/interactive/mod/TopologicalRouter.md) | Class | `crates/oxide-router/src` | Topological router contract for continuous Constrained Delau… |
| [search](/crates/oxide-library/src/scraper/search.md) | Function | `crates/oxide-library/src` | Searches online component distributors for the given query/M… |

## Usage with OpenCode

```bash
# Prime full context
RUN cat ./okf_bundle/SUMMARY.md

# Prime specific domain
RUN cat ./okf_bundle/crates/index.md

# Find a concept
RUN find ./okf_bundle -name '<ConceptName>.md' | xargs cat
```
