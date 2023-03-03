
use bevy::prelude::*;


use physx::traits::Class;

use crate::{PxStaticActor};
use crate::{helpers::physx_vec3, PhysX, PxRigidActorHandle};



//forces
#[derive(Component, Default)]
pub struct PxExternalForce {
    pub force: Vec3,
    pub torque: Vec3,
}

#[allow(dead_code)] //todo: transform.translation should be center of mass
impl PxExternalForce {

    pub fn add_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn add_torque(&mut self, torque: Vec3) {
        self.torque += torque;
    }

    pub fn add_force_at_pos(&mut self, force: Vec3, pos: Vec3, transform: &Transform) {

        let torque = (pos - transform.translation).cross(force);

        self.add_force(force);
        self.add_torque(torque);
    }

    pub fn add_force_at_local_pos(&mut self, force: Vec3, local_pos: Vec3, transform: &Transform) {

        let global_force_pos = transform.transform_point(local_pos);

        self.add_force_at_pos(force, global_force_pos, transform);
    }

    pub fn add_local_force_at_local_pos(&mut self, force: Vec3, local_pos: Vec3, transform: &Transform) {

        let global_force_pos = transform.transform_point(local_pos);
        let global_force = transform.rotation * force;

        self.add_force_at_pos(global_force, global_force_pos, transform);
    }

    pub fn clear(&mut self) {
        self.force = Vec3::ZERO;
        self.torque = Vec3::ZERO;
    }

}


//todo: maybe add change filter
pub fn px_apply_forces(
    physx: Res<PhysX>,
    mut query: Query<(&PxRigidActorHandle, &mut PxExternalForce), (Changed<PxExternalForce>, Without<PxStaticActor>)>,
){

    unsafe {

        for (handle, mut force) in query.iter_mut() {

            let actor = *physx.handles.rigid_actors.get(handle.0).unwrap() as *mut physx_sys::PxRigidBody;

            physx_sys::PxRigidBody_addForce_mut(actor, physx_vec3(force.force).as_ptr(), physx_sys::PxForceMode::Force, true);
            physx_sys::PxRigidBody_addTorque_mut(actor, physx_vec3(force.torque).as_ptr(), physx_sys::PxForceMode::Force, true);

            force.bypass_change_detection().clear();
        }

    }


}
