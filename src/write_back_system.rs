use bevy::prelude::*;

use physx::prelude::*;

use crate::{PhysxRes, PxDynamicRigidBodyHandle, PxArticulationLinkHandle};

use super::{custom::{PxArticulationLink, PxRigidDynamic}};






//positions
pub fn px_sync_transforms(   
    physx: Res<PhysxRes>,
    mut query_dyn_transforms: Query<(&PxDynamicRigidBodyHandle, &mut Transform), Without<PxArticulationLinkHandle>>,
    mut query_link_transforms: Query<(&PxArticulationLinkHandle, &mut Transform), Without<PxDynamicRigidBodyHandle>>,
    // mut query_static_transforms: Query<(&PhysXStaticRigidBodyHandle, &mut Transform)>,
){
    
    //dyn Actors
    query_dyn_transforms.par_iter_mut().for_each_mut(|(handle, mut transform)| {

        let actor = physx.handles.dynamic_actors.get(handle.0)
            .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

        let position = Vec3::new(actor.get_global_position().x(), actor.get_global_position().y(), actor.get_global_position().z());

        let global_roation = actor.get_global_rotation();
        let rotation = Quat::from_xyzw(global_roation.x(), global_roation.y(), global_roation.z(), global_roation.w());

        *transform = Transform::from_translation(position).with_rotation(rotation);
        
    });


    //link
    query_link_transforms.par_iter_mut().for_each_mut(|(handle, mut transform)| {

        let actor = physx.handles.articulation_links.get(handle.0)
            .and_then(|actor| unsafe { (*actor as *mut PxArticulationLink).as_mut() }).unwrap();

        let position = Vec3::new(actor.get_global_position().x(), actor.get_global_position().y(), actor.get_global_position().z());

        let global_roation = actor.get_global_rotation();
        let rotation = Quat::from_xyzw(global_roation.x(), global_roation.y(), global_roation.z(), global_roation.w());

        *transform = Transform::from_translation(position).with_rotation(rotation);

    });



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



// //velocitys
// #[derive(Component, Default)]
// pub struct PxVelocity {
//     pub liniar: Vec3,
//     pub angular: Vec3,
// }

// impl PxVelocity {

//     pub fn get_velocity_at_pos(&self, pos: Vec3, transform: &Transform) -> Vec3 {

//         let point = pos - transform.translation;

//         let mut vel = self.liniar;
//         vel += self.angular.cross(point);

//         return vel;

//     }

// }


// pub fn px_write_velocitys(
//     physx: Res<PhysXRes>,
//     mut dyn_q: Query<(&PxDynamicRigidBodyHandle, &mut PxVelocity), Without<PxArticulationLinkHandle>>,
//     mut link_q: Query<(&PxArticulationLinkHandle, &mut PxVelocity), Without<PxDynamicRigidBodyHandle>>,
//     // time: Res<Time>,
// ){

//     //dyn Actors
//     for (handle, mut velocity) in dyn_q.iter_mut() {

//         let actor = physx.handles.dynamic_actors.get(handle.0)
//             .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

//         velocity.liniar = Vec3::new(actor.get_linear_velocity().x(), actor.get_linear_velocity().y(), actor.get_linear_velocity().z());
//         velocity.angular = Vec3::new(actor.get_angular_velocity().x(), actor.get_angular_velocity().y(), actor.get_angular_velocity().z());

//     }


//     //links Actors
//     for (handle, mut velocity) in link_q.iter_mut() {

//         let actor = physx.handles.articulation_links.get(handle.0)
//             .and_then(|actor| unsafe { (*actor as *mut PxRigidDynamic).as_mut() }).unwrap();

//             velocity.liniar = Vec3::new(actor.get_linear_velocity().x(), actor.get_linear_velocity().y(), actor.get_linear_velocity().z());
//             velocity.angular = Vec3::new(actor.get_angular_velocity().x(), actor.get_angular_velocity().y(), actor.get_angular_velocity().z());

//     }

// }

