# supply_lines

A first-person city simulator in Rust, built on Bevy. Very early — right now it is a forest that runs itself: animals wander, get hungry, find fruit and eat it, and the day passes over them. The supply lines the name promises do not exist yet.

**Expected release: 2045**

## What actually runs today

- **Utility AI** — animals pick actions by scoring their own needs ([big-brain](https://github.com/zkat/big-brain)). Satiety drains on a timer; as it falls, the "find something to eat" score rises until it outweighs whatever else the animal was doing, and the eat action takes over.
- **Navmesh pathfinding** — hungry animals path to the nearest fruit over a generated navmesh (`vleue_navigator`) rather than walking through trees.
- **A forest that reproduces** — mature trees have a daily chance to drop fruit. Fruit nothing eats eventually rots where it lies, and may take root as a new tree if there is clearance around it. Trees grow from sapling to full size over their first days and die of old age.
- **Physics** — rigid bodies and collisions via Avian3D.
- **Time control** — adjustable simulation speed with a day/night light cycle.
- **RTS camera** and an egui inspector for poking at entities while it runs.

## Forest lifecycle

Current tuning: trees reach maturity after 5 simulated days and live 300. A mature tree has a 10% chance each day of dropping 1–3 fruit. Fruit left uneaten rots after 30 days, and when it does there is a 20% chance a tree sprouts in its place — but only if no other tree is within 2.8 units, so the forest thins itself out instead of collapsing into a thicket.

Eating, rotting, sprouting and crowding all hang off the same handful of numbers, so nudging any one of them ripples through the others. That interlock is the point rather than a side effect: the thesis behind the game is that complex systems respond to influence in ways that are hard to predict and worth reasoning about, and that the interesting play lives in strategizing around those consequences.

## Architecture

One Bevy plugin per concern, each owning its own components, systems and schedule registration: `needs`, `navigation`, `movement`, `vegetation`, `animal`, `age`, `ground`, `light`, `time_control`, `camera`, `hud`, `asset_loader`, `debug`. `main.rs` does nothing but compose them. Adding a behaviour means adding a plugin, not editing a god system.

## Build

```sh
cargo run
```

Requires a recent stable Rust toolchain (edition 2024). Dev builds use `opt-level = 1` for the workspace and `3` for dependencies — Bevy is unusably slow otherwise.

## Planned

Seed dispersal moves to the animals. Today a fruit can only become a tree by rotting where it fell, which means the forest reseeds itself in place. The intent is for animals to carry seeds and deposit them as they forage — landing in fertile conditions with a much better chance of taking root than a fruit sitting on the ground. Where the forest spreads then depends on where the animals go, and the two systems start feeding each other.

## Status

A sandbox for learning Bevy's ECS and utility AI, not a game yet. Expect the simulation to be deeper than the thing it is simulating.
