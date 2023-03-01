use bevy::prelude::*;

use physx::prelude::*;

use crate::{PhysXRes, PxRigidDynamic, PxRigidActorHandle, PxDynamicActor};
use crate::sync_physx::articulations::PxArticulationLink;




//positions
pub fn px_sync_transforms(   
    physx: Res<PhysXRes>,
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
