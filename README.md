# bevy_physx
Bevy plugin for NVIDIA PhysX.    
Current PhysX version: 5.1.3

### Disclaimer
This plugin is still in development, some core features are not yet implemented.  
Open for pull requests and issues.

## Usage
```toml
[dependencies]
bevy_physx = "0.1"
```

## Example
```rust
use bevy::prelude::*;
use bevy_physx::prelude::*;

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
        PxCollider::Box { half_extents: Vec3::new(0.5, 0.5, 0.5) },
        Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
    ));
}
```

## Features
* [x] Dynamic rigid bodies
* [x] Static rigid bodies + PxPlanes
* [x] Collision shapes (box, sphere, capsule)
* [x] Raycasts (only single)
* [x] PxMaterial (friction, restitution)
* [x] Articulation (multi body joint simulation)
* [x] External forces
* [x] Velocity

## Not yet implemented 
* [ ] Rigid body properties (mass, dencity, linear damping, angular damping, com, ...)
* [ ] Simple joints (fixed, revolute, prismatic, distance, spherical, ...)
* [ ] Removal detection (remove actor when entity or component is removed)
* [ ] Change detection (update physx when component is changed)
* [ ] Collision shapes (convex, triangle mesh, heightfield)
* [ ] Collision filtering (collision groups, collision masks)
* [ ] Scene query (overlap, sweep)
* [ ] Debug renderer
* [ ] And more...

## Compatibility
Compatibility of `bevy_physx` versions:

| `bevy_physx`  | `bevy` |
| :--           | :--    |
| `0.1`         | `0.10` |

## License
Todo