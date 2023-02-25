use bevy::prelude::Vec3;
use physx::prelude::*;
use physx_sys::{PxFilterFlag, FilterShaderCallbackInfo, phys_PxFilterObjectIsTrigger, PxPairFlag};


//imports
pub type PxMaterial = physx::material::PxMaterial<()>;
pub type PxShape = physx::shape::PxShape<(), PxMaterial>;
pub type PxArticulationLink = physx::articulation_link::PxArticulationLink<(), PxShape>;
pub type PxRigidStatic = physx::rigid_static::PxRigidStatic<(), PxShape>;
pub type PxRigidDynamic = physx::rigid_dynamic::PxRigidDynamic<(), PxShape>;
pub type PxArticulation = physx::articulation::PxArticulation<(), PxArticulationLink>;
pub type PxArticulationReducedCoordinate = physx::articulation_reduced_coordinate::PxArticulationReducedCoordinate<(), PxArticulationLink>;

pub type PxScene = physx::scene::PxScene<
    (),
    PxArticulationLink,
    PxRigidStatic,
    PxRigidDynamic,
    PxArticulation,
    PxArticulationReducedCoordinate,
    OnCollision,
    OnTrigger,
    OnConstraintBreak,
    OnWakeSleep,
    OnAdvance,
>;




pub unsafe extern "C" fn costum_filter_shader(
    mut shader_cb_info: *mut FilterShaderCallbackInfo
) -> u16 {

    // let triggers through
    if phys_PxFilterObjectIsTrigger((*shader_cb_info).attributes0) || phys_PxFilterObjectIsTrigger((*shader_cb_info).attributes1) {
        (*(*shader_cb_info).pairFlags).mBits = PxPairFlag::eTRIGGER_DEFAULT as u16;
        return PxFilterFlag::eDEFAULT as u16;
    }

    // generate contacts for all that were not filtered above
    (*(*shader_cb_info).pairFlags).mBits = PxPairFlag::eCONTACT_DEFAULT as u16;

    

    // trigger the contact callback for pairs (A,B) where
    // the filtermask of A contains the ID of B and vice versa.
    if  ((*shader_cb_info).filterData0.word1) != 0 && ((*shader_cb_info).filterData0.word1 == (*shader_cb_info).filterData1.word0) ||
        ((*shader_cb_info).filterData1.word1) != 0 && ((*shader_cb_info).filterData1.word1 == (*shader_cb_info).filterData0.word0) 
    {
        return PxFilterFlag::eKILL as u16;
    }

    return PxFilterFlag::eDEFAULT as u16;
}


pub struct OnCollision;
impl CollisionCallback for OnCollision {
    fn on_collision(
        &mut self,
        _header: &physx_sys::PxContactPairHeader,
        _pairs: &[physx_sys::PxContactPair],
    ) {
    }
}


pub struct OnTrigger;
impl TriggerCallback for OnTrigger {
    fn on_trigger(&mut self, _pairs: &[physx_sys::PxTriggerPair]) {}
}


pub struct OnConstraintBreak;
impl ConstraintBreakCallback for OnConstraintBreak {
    fn on_constraint_break(&mut self, _constraints: &[physx_sys::PxConstraintInfo]) {}
}


pub struct OnWakeSleep;
impl WakeSleepCallback<PxArticulationLink, PxRigidStatic, PxRigidDynamic> for OnWakeSleep {
    fn on_wake_sleep(
        &mut self,
        _actors: &[&physx::actor::ActorMap<PxArticulationLink, PxRigidStatic, PxRigidDynamic>],
        _is_waking: bool,
    ) {
    }
}


pub struct OnAdvance;
impl AdvanceCallback<PxArticulationLink, PxRigidDynamic> for OnAdvance {
    fn on_advance(
        &self,
        _actors: &[&physx::rigid_body::RigidBodyMap<PxArticulationLink, PxRigidDynamic>],
        _transforms: &[PxTransform],
    ) {
    }
}


#[allow(dead_code)]
pub fn physx_pose(x: f32, y: f32, z: f32) -> physx::math::PxTransform{
    return physx::math::PxTransform::from_translation(&physx::math::PxVec3::new(x, y, z));
}

#[allow(dead_code)]
pub fn physx_vec3(vec: Vec3) -> physx::math::PxVec3{
    return physx::math::PxVec3::new(vec.x, vec.y, vec.z);
}

#[allow(dead_code)]
pub fn vec3_from_pxvec3(vec: physx_sys::PxVec3) -> Vec3{
    return Vec3::new(vec.x, vec.y, vec.z);
}