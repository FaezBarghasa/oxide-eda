# mod

## Classs

- [Body3D](Body3D.md) — Embedded 3D body description. Lives on [`Footprint`] so two MPNs that share
- [BodyShape](BodyShape.md) — 3D body shape — drives the procedural render (no STEP required).
- [ComponentType](ComponentType.md) — v0.21 — Altium-parity component type. Drives whether the part
- [Footprint](Footprint.md) — Reusable PCB primitive. Bound by `Component::footprint_ref`.
- [FootprintFile](FootprintFile.md) — Multi-footprint container for `.snxfpt` files — Altium PCB
- [FootprintFileWire](FootprintFileWire.md) — On-disk wire shape. Mirrors [`FootprintFile`] but each
- [FootprintWire](FootprintWire.md) — [derive(Serialize, Deserialize)]
- [FpCutout](FpCutout.md) — Board cutout — subtracts from the PCB outline (mounting hole, slot,
- [FpGraphic](FpGraphic.md) — One footprint silkscreen / fab graphic.
- [FpGraphicKind](FpGraphicKind.md) — Footprint graphic kinds — silkscreen / fab outline primitives.
- [FpKeepout](FpKeepout.md) — DRC keepout zone. v0.14 stores the polygon + layer + forbid kind.
- [FpMaskOpening](FpMaskOpening.md) — Solder-mask opening (cutout) — copper without solder mask covering.
- [FpPasteAperture](FpPasteAperture.md) — Standalone paste aperture — a paste opening separate from any pad
- [FpPour](FpPour.md) — Copper pour / region. Fill generation lives in v0.15 — v0.14 stores
- [FpVScore](FpVScore.md) — V-score panelisation hint — straight-line score on the PCB surface.
- [KeepoutForbid](KeepoutForbid.md) — What a keepout zone forbids. DRC enforcement lands in v0.15.
- [NetRef](NetRef.md) — Net reference — string for now, will become a UUID once nets are
- [PourFillType](PourFillType.md) — Pour fill mode — drives the polygon raster fill at PCB-render time.
- [StepAttachment](StepAttachment.md) — Optional STEP/WRL attachment for mech-CAD export. Content-hashed so two
- [ThermalReliefStyle](ThermalReliefStyle.md) — Thermal-relief connection style for pads inside a pour. v0.14
- [VScoreSide](VScoreSide.md) — Which side of the PCB the V-score is cut into. v0.15.

## Functions

- [default](default.md)
- [default](default_1.md)
- [default_footprint_format](default_footprint_format.md)
- [default_footprint_version](default_footprint_version.md)
- [default_schema_v2](default_schema_v2.md)
- [default_true](default_true.md)
- [empty](empty.md) — Empty footprint with no pads — what the New Component flow seeds.
- [empty](empty_1.md) — Empty footprint with no pads — what the New Component flow seeds.
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [from_bytes](from_bytes.md) — Decode bytes as UTF-8 and parse via [`FootprintFile::from_toml_str`].
- [from_bytes](from_bytes_1.md) — Decode bytes as UTF-8 and parse via [`FootprintFile::from_toml_str`].
- [from_footprint](from_footprint.md) — Build a new container holding a single footprint — what the
- [from_footprint](from_footprint_1.md) — Build a new container holding a single footprint — what the
- [from_toml_str](from_toml_str.md) — Parse the TOML+TSV wire format. The format-token check pins us
- [from_toml_str](from_toml_str_1.md) — Parse the TOML+TSV wire format. The format-token check pins us
- [get_footprint](get_footprint.md) — Locate a footprint by UUID within this file.
- [get_footprint](get_footprint_1.md) — Locate a footprint by UUID within this file.
- [get_footprint_mut](get_footprint_mut.md)
- [get_footprint_mut](get_footprint_mut_1.md)
- [is_default](is_default.md)
- [is_default](is_default_1.md)
- [label](label.md)
- [label](label_1.md)
- [named](named.md)
- [named](named_1.md)
- [to_toml_string](to_toml_string.md) — Serialise to canonical TOML+TSV. Pad lists become
- [to_toml_string](to_toml_string_1.md) — Serialise to canonical TOML+TSV. Pad lists become
