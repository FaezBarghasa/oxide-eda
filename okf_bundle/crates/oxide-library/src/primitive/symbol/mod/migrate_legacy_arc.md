---
okf_version: "0.2"
type: Function
title: migrate_legacy_arc
description: "Load-time self-heal for two pre-normalization `Arc`-authoring bugs."
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/migrate_legacy_arc
language: rust
---

# migrate_legacy_arc

Load-time self-heal for two pre-normalization `Arc`-authoring bugs.

## Signature

```rust
fn migrate_legacy_arc(kind: SymbolGraphicKind) -> SymbolGraphicKind
```

## Docstring

Load-time self-heal for two pre-normalization `Arc`-authoring bugs.
Non-`Arc` kinds pass through untouched.

1. **Full-turn arcs vanish.** A raw span (`end_deg - start_deg`)
that's an exact, nonzero multiple of 360° — legitimate legacy
full-circle authoring (`0 -> 360`), or a placement/rotation
drift that happened to land on exactly one full turn — computes
a zero CCW-wraparound sweep and draws nothing. A 360° arc IS a
circle, so it's converted to one: [`SymbolGraphicKind::Circle`]
with the same `center`/`radius`. Checked, and applied, BEFORE
rule 2 below: an exact-360° span also happens to satisfy rule
2's discriminator (e.g. `30 -> -330`), and running it through
the swap-and-`rem_euclid` there would collapse it to
`start == end` — the exact invisible-point degenerate this
conversion exists to avoid — instead of the correct circle.

2. **Legacy CW-signed arcs render as their complement.** Before
this codebase's CPU canvas draw path adopted the CCW-wraparound
sweep convention (matching what hit-test / the GPU shader
always assumed), the Place Arc tool could commit a clockwise
drag's raw, unswapped pair — e.g. `start: 30, end: -60` — which
the old (signed-sweep) CPU draw rendered as the short 90° arc
the user actually saw and clicked to place. Reading that same
stored pair under the CCW-wraparound convention sweeps the
270° complement instead. The discriminator distinguishes this
from a pre-existing, INTENTIONALLY wrapped pair (which
`rotation.rs`'s Arc rotate transform has always been able to
produce, e.g. rotating a 0°-crossing arc): a negative or
unwrapped-past-a-full-turn endpoint can only come from the
placement tool's raw drag delta, never from `rotation.rs`,
which always normalizes both endpoints into `[0, 360)`. A
wrapped pair with BOTH endpoints already in `[0, 360)` is left
unchanged — that's the rotation-produced, already-correct
wraparound form.

## Source
Lines 623–652 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
| calls | [normalize_arc_endpoints_deg](/crates/oxide-library/src/primitive/symbol/mod/normalize_arc_endpoints_deg.md) |
| called_by | [from_toml_str](/crates/oxide-library/src/primitive/symbol/mod/from_toml_str.md) |
