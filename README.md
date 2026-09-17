# supply_lines

A first-person city simulator in Rust, built on Bevy. Very early — right now it is a forest that runs itself: animals wander, get hungry, find fruit and eat it, and the day passes over them. The supply lines the name promises do not exist yet.

**Expected release: 2045**

## What actually runs today

- **Utility AI** — animals pick actions by scoring their own needs ([big-brain](https://github.com/zkat/big-brain)). Satiety drains on a timer; as it falls, the "find something to eat" score rises until it outweighs whatever else the animal was doing, and the eat action takes over.
- **Navmesh pathfinding** — hungry animals path to the nearest fruit over a generated navmesh (`vleue_navigator`) rather than walking through trees.
- **Vegetation** — plants grow and bear fruit on their own schedule; eating removes it.
- **Physics** — rigid bodies and collisions via Avian3D.
- **Time control** — adjustable simulation speed with a day/night light cycle.
- **RTS camera** and an egui inspector for poking at entities while it runs.

## Architecture

One Bevy plugin per concern, each owning its own components, systems and schedule registration: `needs`, `navigation`, `movement`, `vegetation`, `animal`, `age`, `ground`, `light`, `time_control`, `camera`, `hud`, `asset_loader`, `debug`. `main.rs` does nothing but compose them. Adding a behaviour means adding a plugin, not editing a god system.

## Build

```sh
cargo run
```

Requires a recent stable Rust toolchain (edition 2024). Dev builds use `opt-level = 1` for the workspace and `3` for dependencies — Bevy is unusably slow otherwise.

## Status

A sandbox for learning Bevy's ECS and utility AI, not a game yet. Expect the simulation to be deeper than the thing it is simulating.
