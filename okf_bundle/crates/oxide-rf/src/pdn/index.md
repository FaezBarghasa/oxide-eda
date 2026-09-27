# pdn

## Classs

- [DecapModel](DecapModel.md) — Decoupling Capacitor Equivalent Circuit Model (RLC).
- [PdnSolver](PdnSolver.md) — Power Delivery Network (PDN) Impedance Field Solver.
- [PdnTargetSpec](PdnTargetSpec.md) — Target Impedance Specification for a Power Rail.
- [PowerPlaneCavity](PowerPlaneCavity.md) — Planar Cavity Plane Geometry.

## Functions

- [compute_target_impedance](compute_target_impedance.md) — Computes $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Delta V_{\text{ripple}}}{I_{\text{transient}}}$ in Ohms.
- [compute_target_impedance](compute_target_impedance_1.md) — Computes $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Delta V_{\text{ripple}}}{I_{\text{transient}}}$ in Ohms.
- [evaluate_impedance_profile](evaluate_impedance_profile.md) — Evaluates total PDN impedance profile $Z(f)$ across frequency grid combining plane and decaps.
- [evaluate_impedance_profile](evaluate_impedance_profile_1.md) — Evaluates total PDN impedance profile $Z(f)$ across frequency grid combining plane and decaps.
- [impedance_at](impedance_at.md) — Calculates impedance magnitude at frequency $f$ for $N$ parallel capacitors.
- [impedance_at](impedance_at_1.md) — Calculates impedance magnitude at frequency $f$ for $N$ parallel capacitors.
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [static_capacitance](static_capacitance.md) — Computes static plane capacitance $C_{\text{plane}} = \varepsilon_0 \varepsilon_r \frac{A}{d}$.
- [static_capacitance](static_capacitance_1.md) — Computes static plane capacitance $C_{\text{plane}} = \varepsilon_0 \varepsilon_r \frac{A}{d}$.
- [test_pdn_impedance_profile_evaluation](test_pdn_impedance_profile_evaluation.md) — [test]
- [test_pdn_target_impedance_calculation](test_pdn_target_impedance_calculation.md) — [test]
- [verify_compliance](verify_compliance.md) — Verifies whether the PDN impedance profile satisfies $Z(f) \le Z_{\text{target}}$ across all test frequencies.
- [verify_compliance](verify_compliance_1.md) — Verifies whether the PDN impedance profile satisfies $Z(f) \le Z_{\text{target}}$ across all test frequencies.
