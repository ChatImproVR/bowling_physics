# Bowling Physics Game

## Build Steps

`cargo run --release --bin bowling`

should work to run the main bowling simulation, since we use a custom fork of ncollide, and kiss3d to update nalgebra to 0.31, rapier3d doesn't need updating since it uses nalgebra 0.31 by default.

## Bowling game

Controls are left click to move view, right click to rotate, scroll to zoom, and space to go forward a step in the simulation. (Subject to change, next iteration might have a UI widget to control the scene.)