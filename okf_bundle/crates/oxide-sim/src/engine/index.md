# engine

## Subdirectories

- [cascade](cascade/index.md)
- [lte_stepper](lte_stepper/index.md)
- [sparse_klu](sparse_klu/index.md)

## Modules

- [cascade](cascade.md) — Automated Four-Stage Convergence Recovery Cascade for Stiff Non-Linear Circuits.
- [lte_stepper](lte_stepper.md) — Variable-Order Integration (Gear BDF 1-6 / Trapezoidal) & Milne Local Truncation Error (LTE) Controller.
- [sparse_klu](sparse_klu.md) — Sparse Matrix Kernel with Approximate Minimum Degree (AMD) & Block Triangular Form (BTF).

## Classs

- [ConvergenceStage](ConvergenceStage.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [InProcessMnaSolver](InProcessMnaSolver.md) — Built-in in-process MNA Solver implementing analytical Newton-Raphson iterations,
- [MnaSolver](MnaSolver.md) — Core Modified Nodal Analysis (MNA) solver contract.
- [SimError](SimError.md) — [derive(Error, Debug, Clone)]
- [StepTelemetry](StepTelemetry.md) — [repr(C)]

## Functions

- [default](default.md)
- [default](default_1.md)
- [index](index.md) — [inline]
- [index](index_1.md) — [inline]
- [initialize](initialize.md)
- [initialize](initialize_1.md)
- [new](new.md)
- [new](new_1.md)
- [recover_convergence](recover_convergence.md)
- [recover_convergence](recover_convergence_1.md)
- [solve_linear](solve_linear.md) — Solves linear system A * x = b via Gaussian elimination with partial pivoting.
- [solve_linear](solve_linear_1.md) — Solves linear system A * x = b via Gaussian elimination with partial pivoting.
- [solve_step](solve_step.md)
- [solve_step](solve_step_1.md)
- [stamp_conductance](stamp_conductance.md)
- [stamp_conductance](stamp_conductance_1.md)
- [stamp_nonlinear_jacobian](stamp_nonlinear_jacobian.md)
- [stamp_nonlinear_jacobian](stamp_nonlinear_jacobian_1.md)
- [stamp_storage](stamp_storage.md)
- [stamp_storage](stamp_storage_1.md)
