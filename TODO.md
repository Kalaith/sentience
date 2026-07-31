# TODO — Sentience

## Testing

- Stealth tests for line-of-sight acquisition, alert escalation, cloak timing, crouch/jump movement, and dismantle boundaries — `route_tests.rs` only proves each level is passable.
- Migration tests for campaign progress when level data, morality flags, or route upgrades change; `migrate_save_value` has no coverage.

## Architecture

- Split morality-route branching from movement so the Helpful and Gremlin progression trees can be exercised independently.
