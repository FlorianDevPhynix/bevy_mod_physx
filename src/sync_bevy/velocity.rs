use bevy::prelude::*;

use physx::prelude::*;

use crate::{PhysXRes, PxRigidActorHandle, PxRigidDynamic, DynamicActor};




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
    physx: Res<PhysXRes>,
    mut query: Query<(&PxRigidActorHandle, &mut PxVelocity), Or<(With<DynamicActor>, With<crate::sync_physx::articulation::ArticulationLink>)>>,
    // time: Res<Time>,
){

    for (handle, mut velocity) in query.iter_mut() {

        let actor = physx.handles.rigid_actors.get(handle.0)
            .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

        velocity.liniar = Vec3::new(actor.get_linear_velocity().x(), actor.get_linear_velocity().y(), actor.get_linear_velocity().z());
        velocity.angular = Vec3::new(actor.get_angular_velocity().x(), actor.get_angular_velocity().y(), actor.get_angular_velocity().z());

    }

}
