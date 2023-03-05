#![allow(dead_code)]

use bevy::prelude::*;
use physx::prelude::*;





pub fn physx_pose(x: f32, y: f32, z: f32) -> physx::math::PxTransform{
    return physx::math::PxTransform::from_translation(&physx::math::PxVec3::new(x, y, z));
}


pub fn vec3_to_physx(vec: Vec3) -> physx::math::PxVec3{
    return physx::math::PxVec3::new(vec.x, vec.y, vec.z);
}


pub fn vec3_from_physx(vec: physx_sys::PxVec3) -> Vec3{
    return Vec3::new(vec.x, vec.y, vec.z);
}


pub fn trans_to_physx(
    trans: bevy::prelude::Transform,
) -> PxTransform {
    let mut px_trans = PxTransform::default();

    *px_trans.translation_mut() = PxVec3::new(trans.translation.x, trans.translation.y, trans.translation.z);

    *px_trans.rotation_mut() = PxQuat::new(trans.rotation.x, trans.rotation.y, trans.rotation.z, trans.rotation.w);

    return px_trans;
}


// pub fn trans_from_physx(
//     trans: PxTransform,
// ) -> bevy::prelude::Transform {

//     let translation = vec3_from_pxvec3(vec3_from_pxvec3(*trans.translation()));
//     let quat = Quat::fro
    
//     return Transform::from_matrix(matrix);
// }