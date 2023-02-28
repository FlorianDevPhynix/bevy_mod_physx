use bevy::prelude::*;
use physx::{prelude::*, traits::Class};


use crate::PhysXRes;
use crate::PxRigidActorHandle;
use crate::trans_to_physx;


#[derive(Component)]
pub enum PxCollider {
    Box{half_extents: Vec3},
    Sphere{radius: f32},
    Capsule{radius: f32, depth: f32},
}



pub fn new_collider(
    mut physx: ResMut<PhysXRes>,
    query: Query<(&PxCollider, &PxRigidActorHandle), Added<PxCollider>>,
){ 


    unsafe {

        let px_material = physx_sys::PxPhysics_createMaterial_mut(physx.foundation.physics_mut().as_mut_ptr(),0.6, 0.6, 0.4);


        for (collider, handle) in query.iter() {

            
            match collider {
                PxCollider::Box{ half_extents } => {

                    let geom = PxBoxGeometry::new(half_extents.x, half_extents.y, half_extents.z);

                    let actor = *physx.handles.rigid_actors.get(handle.0).unwrap();

                    physx_sys::PxRigidActorExt_createExclusiveShape_mut_1(actor, geom.as_ptr(), px_material, physx_sys::PxShapeFlags{ mBits: 1u64 as u8 });

                },
                PxCollider::Sphere { radius } => {

                    let geom = PxSphereGeometry::new(*radius);

                    let actor = *physx.handles.rigid_actors.get(handle.0).unwrap();

                    physx_sys::PxRigidActorExt_createExclusiveShape_mut_1(actor, geom.as_ptr(), px_material, physx_sys::PxShapeFlags{ mBits: 1u64 as u8 });

                },
                PxCollider::Capsule { radius, depth } => {
                        
                    let geom = PxCapsuleGeometry::new(*radius, *depth / 2.0);

                    let actor = *physx.handles.rigid_actors.get(handle.0).unwrap();

                    let shape = physx_sys::PxRigidActorExt_createExclusiveShape_mut_1(actor, geom.as_ptr(), px_material, physx_sys::PxShapeFlags{ mBits: 1u64 as u8 });

                    //rotate capsule upright to fit with bevy's coordinate system
                    let local_pose = Transform::from_rotation(Quat::from_rotation_z((90.0 as f32).to_radians()));

                    physx_sys::PxShape_setLocalPose_mut(shape, trans_to_physx(local_pose).as_ptr());
                },
    
            }
        }
    }

}





// unsafe fn get_actor(physx: &mut PhysXRes, opt_static: Option<&PxStaticRigidBodyHandle>, opt_dyn: Option<&PxDynamicRigidBodyHandle>, opt_link: Option<&PxArticulationLinkHandle>) -> *mut physx_sys::PxRigidActor {
    
//         if let Some(handle) = opt_static {
//             return *physx.handles.static_actors.get(handle.0).unwrap() as *mut physx_sys::PxRigidActor;
//         }
    
//         if let Some(handle) = opt_dyn {
//             return *physx.handles.dynamic_actors.get(handle.0).unwrap() as *mut physx_sys::PxRigidActor;
//         }
    
//         if let Some(handle) = opt_link {
//             return *physx.handles.articulation_links.get(handle.0).unwrap() as *mut physx_sys::PxRigidActor;
//         }
    
//         panic!("No actor found for collider!");
// }