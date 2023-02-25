use bevy::prelude::*;
use physx::{prelude::*, scene::Scene, traits::Class};

use crate::{PhysxRes, PxDynamicRigidBodyHandle};



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


fn trans_to_physx(
    trans: Transform,
) -> PxTransform {
    let mut px_trans = PxTransform::default();

    *px_trans.translation_mut() = PxVec3::new(trans.translation.x, trans.translation.y, trans.translation.z);

    *px_trans.rotation_mut() = PxQuat::new(trans.rotation.x, trans.rotation.y, trans.rotation.z, trans.rotation.w);

    return px_trans;
}