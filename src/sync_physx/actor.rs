use bevy::prelude::*;
use physx::{prelude::*, scene::Scene, traits::Class};

use crate::{PhysxRes, PxDynamicRigidBodyHandle, trans_to_physx};



// #[derive(Component)]
// pub struct StaticActor;


#[derive(Component)]
pub struct DynamicActor;


pub fn new_dyn_actor(
    mut commands: Commands,
    mut physx: ResMut<PhysxRes>,
    query: Query<(Entity, &Transform), Added<DynamicActor>>,
){ 

    for (e, trans) in query.iter() {

        let mut dyn_actor = physx.foundation.physics_mut().create_dynamic(&trans_to_physx(*trans), ()).unwrap();

        let handle = physx.handles.dynamic_actors.insert(dyn_actor.as_mut().as_mut_ptr());
        commands.entity(e).insert(PxDynamicRigidBodyHandle(handle));

        physx.scene.add_dynamic_actor(dyn_actor);

    }

}


