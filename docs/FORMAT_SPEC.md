# Oxide EDA Native File Format Specification v1.0

**Document Version:** 1.0.0  
**Specification Status:** Frozen (v1.0 Baseline)  
**Classification:** Core Standard  
**File Formats Covered:** `.snxsch`, `.snxpcb`, `.snxblk`, `.snxsym`, `.snxfp`, `.snxprj`  

---

## 1. Architectural Philosophy: TOML Envelope + TSV Bulk Data

Oxide EDA uses a dual-layer serialization model designed for:
1. **Deterministic Git Diffability:** Every primitive (wires, components, tracks, vias, pads) is a single line with fixed column positions. Changes to one track do not modify adjacent lines or indentation.
2. **Human Readability:** TOML envelopes provide clear top-level metadata, layer definitions, rule scopes, and environment configurations.
3. **High-Throughput Streaming Parse:** Bulk data blocks parse in $O(N)$ with zero JSON/XML nested tokenization overhead.
4. **Integer Coordinate Storage:** High-precision geometric coordinates are stored as exact integers (nanometers or microns) to prevent floating-point drift across platforms and compilers.

---

## 2. File Formats Overview

| Extension | Purpose | Container / Format |
|-----------|---------|--------------------|
| `.snxsch` | Schematic Sheet document | TOML Envelope + TSV tables (`components`, `wires`, `labels`, `junctions`) |
| `.snxpcb` | Printed Circuit Board layout | TOML Envelope + TSV tables (`tracks`, `vias`, `pads`, `footprints`) + TOML sub-tables (`zones`, `stackup`) |
| `.snxblk` | Reusable Schematic / Layout Snippet | TOML Envelope + embedded `.snxsch` / `.snxpcb` payload |
| `.snxsym` | Schematic Symbol definition | TOML Envelope + TSV pin/shape definitions |
| `.snxfp`  | PCB Footprint definition | TOML Envelope + TSV pad/shape definitions |
| `.snxprj` | Project Manifest & Workspace Settings | Pure TOML Manifest with sheet/PCB file lists and rule references |

---

## 3. Schematic File Format (`.snxsch`)

### 3.1 Format Identifier
Header line:
```toml
format = "snxsch/1"
uuid = "0192a8c0-0001-7000-8000-000000000001"
title = "Main Controller Sheet"
sheet_number = 1
total_sheets = 4
```

### 3.2 TSV Bulk Tables

#### `[components]`
Column layout:
`uuid` \t `reference` \t `value` \t `footprint` \t `pos_x_nm` \t `pos_y_nm` \t `rotation_deg` \t `mirror`

#### `[wires]`
Column layout:
`uuid` \t `start_x_nm` \t `start_y_nm` \t `end_x_nm` \t `end_y_nm` \t `net_id`

#### `[labels]`
Column layout:
`uuid` \t `text` \t `pos_x_nm` \t `pos_y_nm` \t `kind` \t `net_id` \t `orientation`

#### `[junctions]`
Column layout:
`uuid` \t `pos_x_nm` \t `pos_y_nm` \t `diameter_nm` \t `net_id`

---

## 4. PCB File Format (`.snxpcb`)

### 4.1 Format Identifier & Setup
```toml
format = "snxpcb/1"
uuid = "0192a8c0-0002-7000-8000-000000000001"

[setup]
grid_size_nm = 254000
trace_width_nm = 200000
clearance_nm = 150000
via_diameter_nm = 600000
via_drill_nm = 300000
```

### 4.2 Stackup Sub-Table
```toml
[[stackup.layers]]
id = 0
name = "Top Layer"
type = "signal"
thickness_nm = 35000
material = "copper"

[[stackup.layers]]
id = 1
name = "Dielectric 1"
type = "dielectric"
thickness_nm = 100000
material = "FR4"
er = 4.4
loss_tangent = 0.02
```

### 4.3 TSV Bulk Tables

#### `[tracks]`
Column layout:
`uuid` \t `start_x_nm` \t `start_y_nm` \t `end_x_nm` \t `end_y_nm` \t `width_nm` \t `layer` \t `net_id`

#### `[vias]`
Column layout:
`uuid` \t `net_id` \t `pos_x_nm` \t `pos_y_nm` \t `drill_nm` \t `diameter_nm` \t `layers` \t `via_type`

- `layers`: Comma-separated or hyphen-separated layer string (e.g. `TopCopper-BottomCopper`).
- `via_type`: `through`, `blind`, `buried`, `micro`.

#### `[pads]`
Column layout:
`uuid` \t `footprint_uuid` \t `number` \t `pos_x_nm` \t `pos_y_nm` \t `size_x_nm` \t `size_y_nm` \t `drill_nm` \t `shape` \t `layer` \t `net_id`

#### `[footprints]`
Column layout:
`uuid` \t `reference` \t `value` \t `footprint_id` \t `pos_x_nm` \t `pos_y_nm` \t `rotation_deg` \t `layer` \t `locked`

---

## 5. Constraint & Snippet Formats (`.snxblk` & Rules)

### 5.1 Block / Snippet Format (`.snxblk`)
```toml
format = "snxblk/1"
uuid = "0192a8c0-0010-7000-8000-000000000001"
name = "STM32F4_Decoupling_Array"
description = "Standard 4x 100nF + 1x 4.7uF decoupling cluster for STM32 VDD pins"
domain = "hierarchical_snippet"

[schematic]
# Embedded .snxsch payload or relative path

[layout]
# Embedded .snxpcb payload or relative path
```

---

## 6. Precision, Units & Round-Trip Invariants

1. **Nanometer Coordinate Standard**: `1 mm = 1,000,000 nm`.
2. **Zero-Loss Round-Trip**: Parsing a file and serializing it back must yield identical TSV and TOML content bit-for-bit (idempotent serialization).
3. **No Unanchored Primitives**: Every copper segment and via must carry a valid non-zero `net_id` (or `0` for unconnected).
4. **Deterministic Sorting**: TSV rows are sorted by UUID or primary spatial index upon save to guarantee stable Git diffs regardless of in-memory collection ordering.
