use bevy::prelude::*;

use physx::prelude::*;

use crate::{PhysXRes, PxRigidDynamic, PxRigidActorHandle, DynamicActor};





//positions
pub fn px_sync_transforms(   
    physx: Res<PhysXRes>,
    mut query: Query<(&PxRigidActorHandle, &mut Transform), Or<(With<DynamicActor>, With<crate::sync_physx::articulation::ArticulationLink>)>>,
    // mut query_static_transforms: Query<(&PhysXStaticRigidBodyHandle, &mut Transform)>,
){
    
    //dyn Actors
    query.par_iter_mut().for_each_mut(|(handle, mut transform)| {

        let actor = physx.handles.rigid_actors.get(handle.0)
            .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

        let position = Vec3::new(actor.get_global_position().x(), actor.get_global_position().y(), actor.get_global_position().z());

        let global_roation = actor.get_global_rotation();
        let rotation = Quat::from_xyzw(global_roation.x(), global_roation.y(), global_roation.z(), global_roation.w());

        *transform = Transform::from_translation(position).with_rotation(rotation);
        
    });


    // //link
    // query_link_transforms.par_iter_mut().for_each_mut(|(handle, mut transform)| {

    //     let actor = physx.handles.articulation_links.get(handle.0)
    //         .and_then(|actor| unsafe { (*actor as *mut PxArticulationLink).as_mut() }).unwrap();

    //     let position = Vec3::new(actor.get_global_position().x(), actor.get_global_position().y(), actor.get_global_position().z());

    //     let global_roation = actor.get_global_rotation();
    //     let rotation = Quat::from_xyzw(global_roation.x(), global_roation.y(), global_roation.z(), global_roation.w());

    //     *transform = Transform::from_translation(position).with_rotation(rotation);

    // });



    //static Actors
    // for (handle, mut transform) in query_static_transforms.iter_mut() {

    //     let actor = physx.handles.dynamic_actor.get(handle.0)
    //         .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

    //     let position = Vec3::new(actor.get_global_position().x(), actor.get_global_position().y(), actor.get_global_position().z());

    //     let global_roation = actor.get_global_rotation();
    //     let rotation = Quat::from_xyzw(global_roation.x(), global_roation.y(), global_roation.z(), global_roation.w());

    //     *transform = Transform::from_translation(position).with_rotation(rotation);
    // }

}