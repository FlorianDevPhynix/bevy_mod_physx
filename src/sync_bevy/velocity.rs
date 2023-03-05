use bevy::prelude::*;

use physx::prelude::*;

use crate::{PhysX, PxRigidActorHandle, PxRigidDynamic, PxDynamicActor};
use crate::sync_physx::articulations::PxArticulationLink;



//velocitys
//read only for now
//setLinearVelocity is missing in physx-sys
#[derive(Component, Default)] 
pub struct PxVelocity {
    liniar: Vec3,
    angular: Vec3,
}

impl PxVelocity {

    pub fn get_linear_velocity(&self) -> Vec3 {
        return self.liniar;
    }

    pub fn get_angular_velocity(&self) -> Vec3 {
        return self.angular;
    }

    pub fn get_velocity_at_pos(&self, pos: Vec3, transform: &Transform) -> Vec3 {

        let point = pos - transform.translation;

        let mut vel = self.liniar;
        vel += self.angular.cross(point);

        return vel;

    }

}


pub fn sync_velocitys(
    physx: Res<PhysX>,
    mut query: Query<(&PxRigidActorHandle, &mut PxVelocity), Or<(With<PxDynamicActor>, With<PxArticulationLink>)>>,
){

    for (handle, mut velocity) in query.iter_mut() {

        let actor = physx.handles.rigid_actors.get(handle.0)
            .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

        velocity.liniar = Vec3::new(actor.get_linear_velocity().x(), actor.get_linear_velocity().y(), actor.get_linear_velocity().z());
        velocity.angular = Vec3::new(actor.get_angular_velocity().x(), actor.get_angular_velocity().y(), actor.get_angular_velocity().z());

    }

}

