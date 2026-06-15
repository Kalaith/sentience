# Sentience

Sentience is a Rust + Macroquad 2D puzzle-platformer / stealth game built on the shared `macroquad-toolkit`.

The player is a service robot aboard a locked-down deep-space research vessel. Every major system interaction offers a moral fork: saving the crew keeps humans alive, organized, and dangerous, while villainous sabotage makes traversal easier by removing or weakening the threat.

## Implemented Design

- Twenty-level campaign across ship sectors from Cargo through the Core.
- Morality-driven difficulty: Savior choices create clean, well-lit, heavily patrolled spaces; Villain choices create smoke, darkness, wreckage, disabled guards, and easier traversal.
- Route upgrades: peaceful play improves stealth, movement, jumping, and phase-cloak; evil play improves destructive rupture pulse range, damage, and cooldown.
- 2D movement with left/right travel, jumping, crouching, and crate pushing.
- System interactions through consoles and a routed final confrontation.
- Peaceful-to-humans campaigns fight Central Command AI. Evil campaigns fight the ship captain.
- Both endings destroy the ship.
- Line-of-sight stealth with instant dismantling when active humans or turrets acquire the player.
- Toolkit save/load support with versioned save migration.

## Controls

```text
A / D or Left / Right  Move
W / Up / Space         Jump
S / Down               Crouch
E / Enter              Interface
F                      Route ability
R                      Retry after dismantling
Mouse                  Click UI choices and save controls
P / O                  Save / load shortcut
```

## Run

```powershell
cargo run --manifest-path Cargo.toml
```

## Test

```powershell
cargo test --manifest-path Cargo.toml
```

## Publish Validation

Use the project validation path:

```powershell
.\publish.ps1
```
# Practical Future Improvements

- Add replayable stealth tests for line-of-sight, alert escalation, cloak timing, crouch/jump movement, and dismantle boundaries.
- Split morality-route branching from movement so savior and villain progression trees can be tested independently.
- Add level fixtures for each sector that validate console choices, patrol layouts, upgrades, and final confrontation routing.
- Add migration tests for campaign progress when level data, morality flags, or route upgrades change.

