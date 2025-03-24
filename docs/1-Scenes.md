# AAF Integration (With Bone Tracking Support)

- Control Vibrators, Constrictors, Inflators, Oscillators or Strokers based on the currently played AAF scene
- Animation-Pack specific triggers files can be provided in `F4SE\Plugins\Telekinesis2\Triggers\*.json` play a custom set of actions, funscript patterns or speed based on the currently played scen (stored)

## Stroker-Sync

- Sync linear strokers with the collision of meshes to resemble in-game movements
- Sync the speed of scalar devices with the movement speed or penetration depth of AAF animations
- Automatic tag detection based on player collisions
- Requires AAF 1.7 or higher, will not work with any AAF version that creates body doubles of the player

WARNING: This is an experimental feature to fetch vibration control directly from in-game meshes. Use at your own risk.
