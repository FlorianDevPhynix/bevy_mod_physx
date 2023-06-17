use std::collections::HashMap;

use bevy::prelude::*;
use physx::{prelude::*, physics::PhysicsFoundationBuilder, foundation::DefaultAllocator, traits::Class, scene::Scene};


mod helpers;
use helpers::*;

mod custom;
use custom::*;

mod sync_physx;
use sync_physx::*;

mod sync_bevy;
use sync_bevy::*;

mod handles;
use handles::*;


// mod debug_render;
// use debug_render::*;


pub mod prelude {
    pub use crate::PhysXPlugin;
    pub use crate::PhysX;
    pub use crate::RaycastHit;
    pub use crate::sync_bevy::*;
    pub use crate::sync_physx::*;
}

pub mod unsafe_handles {
    pub use crate::handles::PxRigidActorHandle;
}




pub struct PhysXPlugin;

impl Plugin for PhysXPlugin {
    fn build(&self, app: &mut App) {
        app 
            //register
            .register_type::<PxPlane>()
            // settings
            .register_type::<PhysXSettings>()
            .init_resource::<PhysXSettings>()

            //setup physx
            .register_type::<PhysXFixedTime>()
            .init_resource::<PhysXFixedTime>()
            .add_startup_system(setup_physx)

            //sync Physx
            .add_systems((
                new_ground_plane,
                new_static_actor, 
                new_dyn_actor, 
                new_articulation,
            ).in_base_set(PhysXPipelineSet::BeforeFlush).chain())

            .add_system(apply_system_buffers.in_base_set(PhysXPipelineSet::Flush).after(PhysXPipelineSet::BeforeFlush))

            .add_systems((
                px_update_time,
                update_gravity
            ).in_base_set(PhysXPipelineSet::AfterFlush).after(PhysXPipelineSet::Flush))
            .add_systems(( //after flush
                new_articulation_joint,
                new_collider,
                // update_changed_transform,
                update_mass_properties_system,
                update_damping_system,
                update_articulation_joint_drive,
                add_articulation_system,
                px_apply_forces, 
            ).in_base_set(PhysXPipelineSet::AfterFlush).after(PhysXPipelineSet::Flush).chain())

            //run physx
            .add_system(px_step_simulation.in_base_set(PhysXPipelineSet::RunPhysx).after(PhysXPipelineSet::AfterFlush) )

            //sync bevy
            .add_systems(( //todo: run if changed maybe
                sync_transforms, 
                sync_velocitys
            ).in_base_set(PhysXPipelineSet::SyncBevy).after(PhysXPipelineSet::RunPhysx))
            ;

    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
enum PhysXPipelineSet {
    BeforeFlush, //
    Flush,       //  Sync PhysX
    AfterFlush,  //
    RunPhysx,
    SyncBevy,
}


#[derive(Debug, Clone)]
pub struct RaycastHit {
    pub entity: Entity,
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

#[derive(Resource)]
pub struct PhysX {
    foundation: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>,
    scene: Owner<PxScene>,
    actor_to_entity: HashMap<*mut physx_sys::PxRigidActor, Entity>,
    handles: Handels, 
}
unsafe impl Send for PhysX {}
unsafe impl Sync for PhysX {}


impl Drop for PhysX {
    fn drop(&mut self) {
        unsafe {
            // physx_sys::phys_PxCloseExtensions();
            physx_sys::PxScene_release_mut(self.scene.as_mut_ptr());
            physx_sys::PxPhysics_release_mut(self.foundation.physics_mut().as_mut_ptr());
            physx_sys::PxFoundation_release_mut(self.foundation.foundation_mut().as_mut_ptr());
        }

    }
}


impl PhysX {

    fn insert_rigid_actor(&mut self, entity: Entity, actor: *mut physx_sys::PxRigidActor) -> PxRigidActorHandle {

        let handle = self.handles.rigid_actors.insert(actor);
        self.actor_to_entity.insert(actor, entity);

        return PxRigidActorHandle(handle);

    }

    // /// unsafe fuction to get a PxRigidActor from a PxRigidActorHandle
    // pub unsafe fn get_rigid_actor(&self, handle: &PxRigidActorHandle) -> Option<&*mut physx_sys::PxRigidActor> {

    //     return self.handles.rigid_actors.get(handle.0);

    // }
    
    /// Raycast from origin in direction, returns entity, distance, position, normal
    /// returns None if no hit
    /// returns Some((entity, distance, position, normal)) if hit
    pub fn raycast(&mut self, origin: Vec3, direction: Vec3, max_distance: f32) -> Option<RaycastHit> {

        unsafe {

            let filter_data = physx_sys::PxQueryFilterData_new();
            let mut hit = std::mem::MaybeUninit::uninit();
 
            if physx_sys::PxSceneQueryExt_raycastSingle(
                self.scene.as_mut_ptr(),
                vec3_to_physx(origin).as_ptr(),
                vec3_to_physx(direction).as_ptr(),
                max_distance,
                physx_sys::PxHitFlags::Default,
                hit.as_mut_ptr(),
                &filter_data,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ) {

                let hit = hit.assume_init();

                match self.actor_to_entity.get(&hit.actor) {
                    Some(entity) => {
                        return Some(RaycastHit { 
                            entity: *entity, 
                            distance: hit.distance, 
                            position: vec3_from_physx(hit.position), 
                            normal: vec3_from_physx(hit.normal), 
                        });
                    }
                    None => {
                        panic!("Error: Raycast hit actor without entity");
                    }
                }

            } else {
                return None;
            }
 
        }

    }

}

#[derive(Resource, Reflect)]
pub struct PhysXFixedTime {
    #[reflect_value]
    pub step: f32
}

impl Default for PhysXFixedTime {
    fn default() -> Self {
        Self { step: 1.0 / 60.0 }
    }
}

pub fn px_update_time(
    bevy_time: Res<FixedTime>,
    mut physx_time: ResMut<PhysXFixedTime>
) {
    if bevy_time.is_changed() {
        physx_time.step = bevy_time.period.as_secs_f32();
    }
}

//run physx
fn px_step_simulation(
    mut physx: ResMut<PhysX>,
    time: Res<Time>,
    update: Res<PhysXFixedTime>,
    mut accumilator: Local<f32>,
){

    *accumilator += time.delta_seconds();

    if *accumilator >= update.step {
        *accumilator -= update.step;

        physx.scene.simulate(update.step, None, None);

        physx.scene.fetch_results(true).expect("PhysX simulation failed");
    }
}

#[derive(Resource, Reflect)]
pub struct PhysXSettings {
    #[reflect_value]
    pub gravity: Vec3,
    /// can't be changed after PhysX initialization
    pub thread_count: u32,
}

impl Default for PhysXSettings {
    fn default() -> Self {
        Self {
            gravity: Vec3::new( 0.0, -9.81, 0.0 ),
            thread_count: std::thread::available_parallelism().unwrap().get() as u32,
        }
    }
}

// get starting value from world so this can be used for detecting changes
/* impl FromWorld for PhysXSettings {
    fn from_world(world: &mut World) -> Self {
        if let Some(value) = world.get_resource::<Self>() {
            *value.to_owned()
        } else {
            Self {
                gravity: Vec3::new( 0.0, -9.81, 0.0 ),
                thread_count: std::thread::available_parallelism().unwrap().get() as u32,
            }
        }
    }
} */

// 15.06.2023: not perfect change detection, runs once on first call
// setting gravity once on first FixedUpdate is useless, does not seem like a big problem;
// perfect detection possibly with Local resource that holds old value,
// needs default value, implementing FromWorld to assign the resource value before the first execution
fn update_gravity(
    mut physx: ResMut<PhysX>,
    settings: Res<PhysXSettings>,
    //mut old_setting: Local<PhysXSettings>
) {

    if settings.is_changed() {
        physx.scene.set_gravity( settings.gravity.x, settings.gravity.y, settings.gravity.z );
    }
}

fn setup_physx(   
    mut commands: Commands,
    settings: Res<PhysXSettings>
){

    let mut foundation: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>;

    let mut builder = PhysicsFoundationBuilder::<physx::foundation::DefaultAllocator>::new(DefaultAllocator);

    builder.enable_visual_debugger(true);
    builder.with_extensions(true);
    builder.set_pvd_host("127.0.0.1");


    //with or without PVD
    foundation = match builder.build::<PxShape>() {
        Some(found) => found,
        None => Default::default(),
    };

    let scene: Owner<PxScene> = foundation
        .create(SceneDescriptor {
            gravity: vec3_to_physx( settings.gravity ),
            on_advance: Some(OnAdvance),
            simulation_filter_shader: FilterShaderDescriptor::Custom(costum_filter_shader),
            thread_count: settings.thread_count,
            ..SceneDescriptor::new(()) 
        })
        .unwrap();

    // unsafe { physx_sys::PxScene_setVisualizationParameter_mut(scene.as_mut_ptr(), 0u32, 1.0); }

    let handles = Handels::default();

    commands.insert_resource(PhysX{ foundation, scene, handles, actor_to_entity: HashMap::new() });
}



