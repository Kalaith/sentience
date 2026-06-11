# Sentience

Sentience is a Rust + Macroquad 2D puzzle-platformer / stealth game built on the shared `macroquad-toolkit`.

The player is a service robot aboard a locked-down deep-space research vessel. Every major system interaction offers a moral fork: saving the crew keeps humans alive, organized, and dangerous, while villainous sabotage makes traversal easier by removing or weakening the threat.

## Implemented Design

- Eight-level campaign across Cargo, Habitation, Engineering, and Core sectors.
- Morality-driven difficulty: Savior choices create clean, well-lit, heavily patrolled spaces; Villain choices create smoke, darkness, wreckage, disabled guards, and easier traversal.
- 2D movement with left/right travel, jumping, crouching, and crate pushing.
- System interactions through consoles and the final AI Core decision.
- No player weapons. Violence is represented only through environmental choices.
- Line-of-sight stealth with instant dismantling when active humans or turrets acquire the player.
- Level 7 morality state check and Level 8 final ending choice.
- Toolkit save/load/delete support with versioned save migration.

## Controls

```text
A / D or Left / Right  Move
W / Up / Space         Jump
S / Down               Crouch
E / Enter              Interface
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
