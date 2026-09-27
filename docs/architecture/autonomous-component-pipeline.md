# Autonomous Component Discovery, Generative Ingest, Parametric Synthesis & Simulation Binding Architecture

**Subsystems:** `oxide-library`, `oxide-bake`, `oxide-sim`, `oxide-ai`  
**Classification:** Core System Architecture & Verification Protocol  
**Author:** Office of the Chief Technology Officer, Lead Systems Architect & Senior Principal QA

---

## 1. Executive Summary & Architectural Mission

In electronic systems design, component sourcing, library drafting, and simulation modeling account for up to 40% of engineering time. Traditional EDA tools force engineers to manually redraw schematic symbols, calculate pad dimensions, convert mechanical tolerances into PCB footprints, and scour vendor sites for unverified SPICE models.

`oxide-eda` implements an end-to-end, closed-loop **Autonomous Component Pipeline**. Given an arbitrary manufacturer part number (MPN), generic description, or vendor URL, the system autonomously:
1. **Harvests** parametric specifications, distributor telemetry (stock, price), mechanical package drawings, and simulation models from worldwide distributors and web repositories.
2. **Parses** unstructured PDF datasheets using deterministic table extractors and multi-modal neural parsers.
3. **Synthesizes** standardized, electrically verified schematic symbols partitioned into logical multi-gate units with strict 100-mil grid alignment.
4. **Generates** mathematically rigorous, IPC-7351C-compliant footprints (Density Levels A, B, C) and procedural 3D boundary-representation (B-Rep) STEP/mesh models.
5. **Ingests & Binds** native SPICE/IBIS macromodels with automatic pin-to-node mapping, enabling immediate in-process simulation within `oxide-sim`.
6. **Enforces** strict anti-hallucination Senior QA verification gates (`GATE-PIN-01` to `GATE-SOA-05`) before atomic commit to `.snxlib`.

```
                                  [ User Query: MPN / Keyword ]
                                                │
                                                ▼
                             [ Multi-Tier Harvester & Scraper ]
                   (DigiKey, Mouser, LCSC, Octopart, Manufacturer Portal)
                                                │
                  ┌─────────────────────────────┴─────────────────────────────┐
                  ▼                                                           ▼
        [ Structured Metadata ]                                     [ Unstructured Assets ]
     (Specs, Pin Tables, Packaging)                              (PDF Datasheets, STEP, SPICE)
                  │                                                           │
                  ▼                                                           ▼
    [ Entity Normalization Engine ]                             [ Datasheet Intelligence (OCR/LLM) ]
                  │                                             (Pin Matrices, Dimension Limits)
                  ├─────────────────────────────┬─────────────────────────────┘
                  │                             │
                  ▼                             ▼
   [ Procedural Symbol Generator ]   [ IPC-7351C Pad Synthesizer ]
    * Pin Electrical Direction        * Solder Fillet Math (Toe/Heel)
    * Multi-Gate Splitting            * Courtyard & Silk Keepouts
    * IEC/IEEE Standard Shapes        * STEP/GLB 3D B-Rep Anchor
                  │                             │
                  └──────────────┬──────────────┘
                                 ▼
                 [ SPICE / IBIS Harvester & Bridge ]
                 * .subckt / .model Ingestion & Sanitization
                 * IBIS V/I & V/t Table-to-Macromodel Synthesis
                 * Exact Pin-to-Subcircuit Node Mapping
                                 │
                                 ▼
                 [ Senior QA Verification Sandbox ]
                 * Topological ERC & Dimension Sanity Verification
                 * Isolated DC & Transient Simulation Sanity Solve
                                 │
                                 ▼
            [ Atomic Commit to .snxlib & Project Database ]
```

---

## 2. Harvester Topology & Cascade Architecture (`oxide-library`)

The harvesting tier combines structured API connectors, automated distributor scraping, and fall-through pipelines:

```
[ Tier 1: Local & Global Workspace Cache ]
  Query embedded SQLite local cache & central corporate Git library
             │ Miss
             ▼
[ Tier 2: Authenticated Distributor REST APIs ]
  Query DigiKey, Mouser, Element14, and LCSC APIs via OAuth tokens
             │ Miss / Partial Metadata
             ▼
[ Tier 3: Open EDA Hub Bridges ]
  Query Nexar/Octopart, SnapMagic, Ultra Librarian, and SamacSys endpoints
             │ Miss / Missing Datasheet or Models
             ▼
[ Tier 4: Direct Manufacturer Web Crawling & Scraping Engine ]
  Headless browser crawl of vendor portals (TI, ADI, ST, Infineon, NXP, Microchip)
             │ Miss
             ▼
[ Tier 5: Generative Synthesis Fallback ]
  Synthesize from user-provided PDF upload or minimal parametric prompt
```

### Rate-Limiting & Jitter Mathematics

Connections employ token bucket rate-limiting algorithms with randomized jitter:
$$t_{\text{backoff}} = \min\left(t_{\text{max}}, t_{\text{base}} \cdot 2^{\text{retry}}\right) \pm \Delta t_{\text{jitter}}$$

Payloads are content-hashed via SHA-256 for network and memory deduplication.

---

## 3. Procedural Schematic Symbol Synthesizer (`oxide-library`)

Following IEC 60617 and IEEE 315 standards:

### 3.1 Multi-Gate Partitioning
- **Homogeneous Arrays:** Dual/Quad Op-Amps, Hex Inverters, Octal Buffers $\to$ Partitioned into $K$ functional units (`part_number = 1..=K`) plus shared Power/Ground (`part_number = 0`).
- **Heterogeneous ICs:** Microcontrollers, FPGAs, and SoCs partitioned by IO Banks, Power Rails, Analog Subsystems, and Core Control.

### 3.2 Energy-Minimizing Pin Arrangement & Grid Snapping
- **Left:** Inputs, Enables, Clocks, Reference inputs.
- **Right:** Outputs, Inverted outputs, Status lines.
- **Top:** Positive supply rails ($V_{\text{DD}}, V_{\text{CC}}, 3\text{V}3$).
- **Bottom:** Return paths, grounds ($V_{\text{SS}}, \text{GND}$), exposed thermal pads.
- **Grid Quantum:** Pin tip coordinates snap strictly to $100\,\text{mil}$ ($2.54\,\text{mm}$).
- **Active-Low:** Overbar or inverted bubble `Dot` notation.

---

## 4. Mathematical IPC-7351C Parametric Footprint & 3D B-Rep Generator (`oxide-bake`)

### 4.1 Solder Fillet Mathematics

Pad dimensions are synthesized across density levels:
* **Density Level A (Most / Prototyping):** Robust vibration resistance, military/aerospace.
* **Density Level B (Nominal / Commercial):** Standard commercial assembly yields.
* **Density Level C (Least / Handheld):** Ultra-dense mobile and handheld layouts.

$$Z_{\text{max}} = L_{\text{min}} + 2 J_T + \sqrt{C_L^2 + F^2 + P^2}$$
$$G_{\text{min}} = S_{\text{max}} - 2 J_H - \sqrt{C_S^2 + F^2 + P^2}$$
$$\text{Pad Length } X = \frac{Z_{\text{max}} - G_{\text{min}}}{2}$$
$$\text{Pad Width } Y = b_{\text{max}} + 2 J_S + \sqrt{C_W^2 + F^2 + P^2}$$
$$\text{Center-to-Center Distance } C = \frac{Z_{\text{max}} + G_{\text{min}}}{2}$$

### 4.2 Mask & Paste Formulations
- **Solder Mask Clearance:** $+0.05\,\text{mm}$ radial expansion.
- **Thermal Pad Window-Panning:** Thermal pads $>2.0 \times 2.0\,\text{mm}$ are automatically partitioned into an $M \times N$ matrix of paste apertures with $0.25\,\text{mm}$ solder dams, achieving $60\% \text{ to } 70\%$ paste coverage.
- **Courtyard Excess:** $+0.50\,\text{mm}$ (Level A), $+0.25\,\text{mm}$ (Level B), $+0.12\,\text{mm}$ (Level C).

### 4.3 Procedural 3D B-Rep Synthesis
- Base body extruded to seated height $A$.
- Lead formation geometry and Pin 1 orientation chamfer notch.
- Geometric centroid snapped to footprint origin $(x=0, y=0, z=0)$.

---

## 5. Senior QA Verification Protocol & Anti-Hallucination Gates

```
+----------------------------------------------------------------------------------------------------+
|                             COMPONENT ACCEPTANCE GATES & SPECIFICATIONS                            |
+----------------+---------------------+-------------------------------------------------------------+
| Gate ID        | Target Subsystem    | Acceptance Criteria                                         |
+----------------+---------------------+-------------------------------------------------------------+
| GATE-PIN-01    | Symbol Synthesizer  | 100% bijective mapping between schematic symbol pins and   |
|                |                     | footprint pads. Zero duplicate pin IDs or missing numbers.  |
| GATE-IPC-02    | Footprint Generator | Pad dimensions comply with IPC-7351C formulas within        |
|                |                     | ε <= 0.001 mm. Minimum clearance between copper >= 0.1 mm. |
| GATE-STEP-03   | 3D Importer/Mesh    | STEP model bounding box coplanar with pads. Pin 1 center    |
|                |                     | aligns with footprint Pad 1 within <= 0.02 mm error margin. |
| GATE-SIM-04    | oxide-sim Validator | Ingested or synthesized SPICE model successfully converges  |
|                |                     | in a standalone testbench circuit without timestep errors.  |
| GATE-SOA-05    | Reliability Suite   | Safe Operating Area limits parsed and populated with at     |
|                |                     | least voltage and power limits for active semiconductor ICs.|
+----------------+---------------------+-------------------------------------------------------------+
```
