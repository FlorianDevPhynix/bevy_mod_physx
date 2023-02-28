use bevy::prelude::*;
use physx::{prelude::*, scene::Scene, traits::Class};

use crate::{PhysXRes, trans_to_physx, PxRigidActorHandle};



//dynamic
#[derive(Component)]
pub struct DynamicActor;


pub fn new_dyn_actor(
    mut commands: Commands,
    mut physx: ResMut<PhysXRes>,
    query: Query<(Entity, &Transform), Added<DynamicActor>>,
){ 

    for (e, trans) in query.iter() {

        let mut dyn_actor = physx.foundation.physics_mut().create_dynamic(&trans_to_physx(*trans), ()).unwrap();

        let handle = physx.handles.rigid_actors.insert(dyn_actor.as_mut().as_mut_ptr());
        commands.entity(e).insert(PxRigidActorHandle(handle));

        physx.scene.add_dynamic_actor(dyn_actor);

    }

}



//static
#[derive(Component)]
pub struct StaticActor;


pub fn new_static_actor(
    mut commands: Commands,
    mut physx: ResMut<PhysXRes>,
    query: Query<(Entity, &Transform), Added<StaticActor>>,
){ 

    for (e, trans) in query.iter() {

        let mut static_actor = physx.foundation.physics_mut().create_static(trans_to_physx(*trans), ()).unwrap();

        let handle = physx.handles.rigid_actors.insert(static_actor.as_mut().as_mut_ptr());
        commands.entity(e).insert(PxRigidActorHandle(handle));

        physx.scene.add_static_actor(static_actor);

    }

}