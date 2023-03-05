use bevy::prelude::*;
use physx::prelude::*;

use crate::{PhysX, PxRigidDynamic, PxRigidActorHandle, PxDynamicActor, prelude::articulations::PxArticulationLink};




//positions
pub fn sync_transforms(   
    physx: Res<PhysX>,
    mut query: Query<(&PxRigidActorHandle, &mut Transform), Or<(With<PxDynamicActor>, With<PxArticulationLink>)>>,
){
    
    query.par_iter_mut().for_each_mut(|(handle, mut transform)| {

        let actor = physx.handles.rigid_actors.get(handle.0)
            .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

        let position = Vec3::new(actor.get_global_position().x(), actor.get_global_position().y(), actor.get_global_position().z());

        let global_roation = actor.get_global_rotation();
        let rotation = Quat::from_xyzw(global_roation.x(), global_roation.y(), global_roation.z(), global_roation.w());

        *transform = Transform::from_translation(position).with_rotation(rotation);
        
    });

}



//only active actors
// pub fn px_sync_transforms(
//     mut commands: Commands,
//     mut physx: ResMut<PhysX>,
//     mut trans_q: Query<(Entity, &mut Transform)>,
// ) {
//     unsafe {

//         let map = physx.actor_to_entity.clone();

        
//         for active_actor in physx.scene.get_active_actors().iter_mut() {

//             //active actor to ref mut actor
//             let actor =  (*active_actor).as_mut_ptr() as *mut physx_sys::PxRigidActor;

//             //get entity from actor
//             let entity = *map.get(&actor).unwrap();

//             //set transform
//             let position = Vec3::new(active_actor.get_global_position().x(), active_actor.get_global_position().y(), active_actor.get_global_position().z());

//             let global_roation = active_actor.get_global_rotation();
//             let rotation = Quat::from_xyzw(global_roation.x(), global_roation.y(), global_roation.z(), global_roation.w());

//             let transform = Transform::from_translation(position).with_rotation(rotation);
//             *trans_q.get_mut(entity).unwrap().1 = transform;
//         }
//     }

// }