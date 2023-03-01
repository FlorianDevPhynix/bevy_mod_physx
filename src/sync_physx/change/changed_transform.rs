use bevy::prelude::*;

use physx::traits::Class;

use crate::helpers::trans_to_physx;
use crate::{PhysXRes, PxRigidActorHandle, PxPlane};





//positions
pub fn set_changed_transform(//dosent work yet
    physx: Res<PhysXRes>,
    mut query: Query<(&PxRigidActorHandle, &Transform), (Changed<Transform>, Without<PxPlane>)>,
){
    
    for (handle, trans) in query.iter_mut() {

        unsafe {

            let actor = *physx.handles.rigid_actors.get(handle.0).unwrap();

            physx_sys::PxRigidActor_setGlobalPose_mut(actor, trans_to_physx(*trans).as_ptr(), true);
        }

    }


}