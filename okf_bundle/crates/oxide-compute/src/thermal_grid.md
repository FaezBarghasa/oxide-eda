---
okf_version: "0.2"
type: Module
title: thermal_grid
description: 3D Transient Electro-Thermal Heat Diffusion Finite Difference Grid.
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid
language: rust
---

# thermal_grid

3D Transient Electro-Thermal Heat Diffusion Finite Difference Grid.

## Docstring

3D Transient Electro-Thermal Heat Diffusion Finite Difference Grid.

Conforms to Master Technical Directive Horizon VI (§7, Task 6.2):
- Discretizes PCB volume (FR-4 substrate, copper planes, thermal vias) into 3D grid.
- Solves Fourier's heat diffusion equation:
$\rho c_p \frac{\partial T}{\partial t} = \nabla \cdot (k \nabla T) + Q(\vec{r}, t)$
- Bidirectional Joule heat dissipation feedback loop with temperature-dependent resistivity.

## Relationships

| Type | Target |
|------|--------|
| related | [GridThermalMaterial](/crates/oxide-compute/src/thermal_grid/GridThermalMaterial.md) |
| related | [conductivity_w_per_mk](/crates/oxide-compute/src/thermal_grid/conductivity_w_per_mk.md) |
| related | [volumetric_heat_capacity](/crates/oxide-compute/src/thermal_grid/volumetric_heat_capacity.md) |
| related | [conductivity_w_per_mk](/crates/oxide-compute/src/thermal_grid/conductivity_w_per_mk.md) |
| related | [volumetric_heat_capacity](/crates/oxide-compute/src/thermal_grid/volumetric_heat_capacity.md) |
| related | [ThermalGrid3D](/crates/oxide-compute/src/thermal_grid/ThermalGrid3D.md) |
| related | [new](/crates/oxide-compute/src/thermal_grid/new.md) |
| related | [index](/crates/oxide-compute/src/thermal_grid/index.md) |
| related | [set_material](/crates/oxide-compute/src/thermal_grid/set_material.md) |
| related | [inject_joule_heat](/crates/oxide-compute/src/thermal_grid/inject_joule_heat.md) |
| related | [clear_heat_sources](/crates/oxide-compute/src/thermal_grid/clear_heat_sources.md) |
| related | [step_transient](/crates/oxide-compute/src/thermal_grid/step_transient.md) |
| related | [max_temperature_celsius](/crates/oxide-compute/src/thermal_grid/max_temperature_celsius.md) |
| related | [temperature_celsius](/crates/oxide-compute/src/thermal_grid/temperature_celsius.md) |
| related | [new](/crates/oxide-compute/src/thermal_grid/new.md) |
| related | [index](/crates/oxide-compute/src/thermal_grid/index.md) |
| related | [set_material](/crates/oxide-compute/src/thermal_grid/set_material.md) |
| related | [inject_joule_heat](/crates/oxide-compute/src/thermal_grid/inject_joule_heat.md) |
| related | [clear_heat_sources](/crates/oxide-compute/src/thermal_grid/clear_heat_sources.md) |
| related | [step_transient](/crates/oxide-compute/src/thermal_grid/step_transient.md) |
| related | [max_temperature_celsius](/crates/oxide-compute/src/thermal_grid/max_temperature_celsius.md) |
| related | [temperature_celsius](/crates/oxide-compute/src/thermal_grid/temperature_celsius.md) |
| related | [test_3d_thermal_joule_heating](/crates/oxide-compute/src/thermal_grid/test_3d_thermal_joule_heating.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
