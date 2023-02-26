use bevy::prelude::*;

use physx::prelude::*;

use crate::{PhysxRes, PxDynamicRigidBodyHandle, PxArticulationLinkHandle, PxRigidDynamic};




//velocitys
#[derive(Component, Default)]
pub struct PxVelocity {
    pub liniar: Vec3,
    pub angular: Vec3,
}

impl PxVelocity {

    pub fn get_velocity_at_pos(&self, pos: Vec3, transform: &Transform) -> Vec3 {

        let point = pos - transform.translation;

        let mut vel = self.liniar;
        vel += self.angular.cross(point);

        return vel;

    }

}


pub fn px_write_velocitys(
    physx: Res<PhysxRes>,
    mut dyn_q: Query<(&PxDynamicRigidBodyHandle, &mut PxVelocity), Without<PxArticulationLinkHandle>>,
    mut link_q: Query<(&PxArticulationLinkHandle, &mut PxVelocity), Without<PxDynamicRigidBodyHandle>>,
    // time: Res<Time>,
){

    //dyn Actors
    for (handle, mut velocity) in dyn_q.iter_mut() {

        let actor = physx.handles.dynamic_actors.get(handle.0)
            .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

        velocity.liniar = Vec3::new(actor.get_linear_velocity().x(), actor.get_linear_velocity().y(), actor.get_linear_velocity().z());
        velocity.angular = Vec3::new(actor.get_angular_velocity().x(), actor.get_angular_velocity().y(), actor.get_angular_velocity().z());

    }


    //links Actors
    for (handle, mut velocity) in link_q.iter_mut() {

        let actor = physx.handles.articulation_links.get(handle.0)
            .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

            velocity.liniar = Vec3::new(actor.get_linear_velocity().x(), actor.get_linear_velocity().y(), actor.get_linear_velocity().z());
            velocity.angular = Vec3::new(actor.get_angular_velocity().x(), actor.get_angular_velocity().y(), actor.get_angular_velocity().z());

    }

}

