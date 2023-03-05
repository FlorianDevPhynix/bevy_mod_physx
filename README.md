# bevy_mod_physx
Bevy plugin for NVIDIA PhysX using Bindings XXX.    
Current PhysX version: 5.1.3

### Disclaimer
This plugin is still in development, some core features are not yet implemented.  
Open for pull requests and issues.

## Usage
```toml
[dependencies]
bevy_mod_physx = "0.1"
```

## Example
```rust
use bevy::prelude::*;
use bevy_mod_physx::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysXPlugin)
        .add_startup_system(setup)
        .run();
}

pub fn setup(mut commands: Commands) {

    // Spawn a collision plane
    commands.spawn(PxPlane::default()); 

    // Spawn a dynamic rigid body
    commands.spawn(( 
        PxDynamicActor,
        PxCollider::Capsule { radius: 0.1, depth: 0.4},
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
    ));
}
```

## Features
* [x] Dynamic rigid bodies
* [x] Static rigid bodies + PxPlanes
* [x] Collision shapes (box, sphere, capsule)
* [x] Raycasts (only single)
* [x] PxMaterial (friction, restitution)
* [x] Some rigid body properties (mass properties, external forces, velocity, damping)
* [x] Articulation (multi body joint simulation)
* [x] Some change detection (mass properties, damping, material, external forces, articulation joint drive)

## Not yet implemented 
* [ ] Rigid body properties (gravity, com, locked axis, dominance, sleeping)
* [ ] Simple joints (fixed, revolute, prismatic, distance, spherical, ...)
* [ ] Removal detection (remove actor when entity or component is removed)
* [ ] Change detection (update physx when component is changed)
* [ ] Scene settings (gravity, ...)
* [ ] Collision shapes (convex, triangle mesh, heightfield)
* [ ] Collision filtering (collision groups, collision masks)
* [ ] Scene query (overlap, sweep)
* [ ] Debug renderer
* [ ] And more...

## Compatibility
Compatibility of `bevy_mod_physx` versions:

| `bevy_mod_physx`  | `bevy` |
| :--           | :--    |
| `0.1`         | `0.10` |

## License
Todo