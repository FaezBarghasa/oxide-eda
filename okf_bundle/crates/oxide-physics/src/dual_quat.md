---
okf_version: "0.2"
type: Module
title: dual_quat
description: Unit Dual-Quaternion Forward Kinematic Transformation Chains for Rigid-Flex Fold Animation.
resource: crates/oxide-physics/src/dual_quat.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:36:07Z"
concept_id: crates/oxide-physics/src/dual_quat
language: rust
---

# dual_quat

Unit Dual-Quaternion Forward Kinematic Transformation Chains for Rigid-Flex Fold Animation.

## Docstring

Unit Dual-Quaternion Forward Kinematic Transformation Chains for Rigid-Flex Fold Animation.

Conforms to Master Technical Directive Horizon III (§4, Task 3.2):
- Dual-quaternion algebra $\hat{\mathbf{q}} = \mathbf{q}_r + \epsilon \mathbf{q}_d$ ($\epsilon^2 = 0$).
- Eliminates gimbal lock and numerical singularities across $0^\circ \to 180^\circ$ continuous folds.
- Kinematic chain propagation across arbitrary polygonal board domains $\Omega_k$.

## Relationships

| Type | Target |
|------|--------|
| related | [Quaternion](/crates/oxide-physics/src/dual_quat/Quaternion.md) |
| related | [new](/crates/oxide-physics/src/dual_quat/new.md) |
| related | [from_axis_angle](/crates/oxide-physics/src/dual_quat/from_axis_angle.md) |
| related | [multiply](/crates/oxide-physics/src/dual_quat/multiply.md) |
| related | [conjugate](/crates/oxide-physics/src/dual_quat/conjugate.md) |
| related | [new](/crates/oxide-physics/src/dual_quat/new.md) |
| related | [from_axis_angle](/crates/oxide-physics/src/dual_quat/from_axis_angle.md) |
| related | [multiply](/crates/oxide-physics/src/dual_quat/multiply.md) |
| related | [conjugate](/crates/oxide-physics/src/dual_quat/conjugate.md) |
| related | [DualQuaternion](/crates/oxide-physics/src/dual_quat/DualQuaternion.md) |
| related | [from_rotation_translation](/crates/oxide-physics/src/dual_quat/from_rotation_translation.md) |
| related | [multiply](/crates/oxide-physics/src/dual_quat/multiply.md) |
| related | [transform_point](/crates/oxide-physics/src/dual_quat/transform_point.md) |
| related | [from_rotation_translation](/crates/oxide-physics/src/dual_quat/from_rotation_translation.md) |
| related | [multiply](/crates/oxide-physics/src/dual_quat/multiply.md) |
| related | [transform_point](/crates/oxide-physics/src/dual_quat/transform_point.md) |
| related | [test_dual_quaternion_translation_and_rotation](/crates/oxide-physics/src/dual_quat/test_dual_quaternion_translation_and_rotation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
