# mixed_signal

## Classs

- [AtoDGateway](AtoDGateway.md) — Analog-to-Digital (AtoD) Gateway Boundary Bridge.
- [DtoAGateway](DtoAGateway.md) — Digital-to-Analog (DtoA) Gateway Boundary Bridge.
- [LockstepSynchronizer](LockstepSynchronizer.md) — Lockstep Synchronizer coupling the continuous MNA solver with the discrete event queue.
- [Logic12State](Logic12State.md) — 12-State Logic Taxonomy conforming to Directive §3.6.
- [LogicEvent](LogicEvent.md) — Discrete Mixed-Signal Event in the event queue.
- [LogicEventQueue](LogicEventQueue.md) — Priority event queue for discrete logic scheduling.

## Functions

- [advance_to](advance_to.md) — Advances lockstep time to `target_time_s` and drains scheduled discrete events.
- [advance_to](advance_to_1.md) — Advances lockstep time to `target_time_s` and drains scheduled discrete events.
- [clear](clear.md)
- [clear](clear_1.md)
- [cmp](cmp.md)
- [cmp](cmp_1.md)
- [compute_synchronized_timestep](compute_synchronized_timestep.md) — Determines the next synchronized continuous timestep $h_{\text{sync}}$, truncating if a discrete event precedes $t + h$.
- [compute_synchronized_timestep](compute_synchronized_timestep_1.md) — Determines the next synchronized continuous timestep $h_{\text{sync}}$, truncating if a discrete event precedes $t + h$.
- [eq](eq.md)
- [eq](eq_1.md)
- [evaluate_trajectory](evaluate_trajectory.md) — Evaluates continuous voltage trajectory (t0, v0) -> (t1, v1) and calculates exact crossing event if threshold exceeded.
- [evaluate_trajectory](evaluate_trajectory_1.md) — Evaluates continuous voltage trajectory (t0, v0) -> (t1, v1) and calculates exact crossing event if threshold exceeded.
- [is_empty](is_empty.md)
- [is_empty](is_empty_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [new](new_6.md)
- [new](new_7.md)
- [next_event_time](next_event_time.md)
- [next_event_time](next_event_time_1.md)
- [partial_cmp](partial_cmp.md)
- [partial_cmp](partial_cmp_1.md)
- [pop_before_or_at](pop_before_or_at.md)
- [pop_before_or_at](pop_before_or_at_1.md)
- [push](push.md)
- [push](push_1.md)
- [resolve](resolve.md) — Resolves driver contention using standard wired logic resolution.
- [resolve](resolve_1.md) — Resolves driver contention using standard wired logic resolution.
- [sample_voltage](sample_voltage.md) — Evaluates the continuous instantaneous output voltage at time `t`.
- [sample_voltage](sample_voltage_1.md) — Evaluates the continuous instantaneous output voltage at time `t`.
- [test_atod_gateway_threshold_interpolation](test_atod_gateway_threshold_interpolation.md) — [test]
- [test_dtoa_gateway_exponential_ramp](test_dtoa_gateway_exponential_ramp.md) — [test]
- [test_lockstep_timestep_truncation](test_lockstep_timestep_truncation.md) — [test]
- [test_logic_12_state_resolution](test_logic_12_state_resolution.md) — [test]
- [to_binary](to_binary.md) — Converts a 12-state value to standard binary logic if resolvable.
- [to_binary](to_binary_1.md) — Converts a 12-state value to standard binary logic if resolvable.
- [update_state](update_state.md) — Schedules a discrete logic state update and initiates an exponential transition ramp.
- [update_state](update_state_1.md) — Schedules a discrete logic state update and initiates an exponential transition ramp.
