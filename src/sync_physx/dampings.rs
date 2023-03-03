use bevy::prelude::*;
use crate::{prelude::*, handles::PxRigidActorHandle};


#[derive(Component, Debug, Clone, Copy)]
pub struct PxDamping {
    pub linear: f32,
    pub angular: f32,
}

impl Default for PxDamping {
    fn default() -> Self {
        Self {
            linear: 0.05,
            angular: 0.1,
        }
    }
}



pub fn update_damping_system(
    mut physx: ResMut<PhysX>,
    query: Query<(&PxDamping, &PxRigidActorHandle), (Changed<PxDamping>, Without<PxStaticActor>)>,
) {

    unsafe {

        for (damping, handle) in query.iter() {

            let actor = *physx.handles.rigid_actors.get_mut(handle.0).unwrap();
    
            physx_sys::PxRigidBody_setLinearDamping_mut(actor as *mut physx_sys::PxRigidBody, damping.linear);
            physx_sys::PxRigidBody_setAngularDamping_mut(actor as *mut physx_sys::PxRigidBody, damping.angular);
    
        }

    }



}