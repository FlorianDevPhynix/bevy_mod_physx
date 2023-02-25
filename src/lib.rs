use bevy::prelude::*;

use physx::prelude::*;
use physx::scene::Scene;
use physx::{physics::PhysicsFoundationBuilder, foundation::DefaultAllocator};

use slotmap::{SlotMap, new_key_type};

pub mod custom;
pub use custom::*;

pub mod sync_systems;
pub use sync_systems::*;

pub mod write_back_system;
pub use write_back_system::*;


pub struct PhysxPlugin;

impl Plugin for PhysxPlugin {
    fn build(&self, app: &mut App) {
        app 

            .add_startup_system(setup_physx)
            //sync physx
            .add_systems((
                new_dyn_actor.in_base_set(PhysxSet::NewActors), 
                apply_system_buffers.in_base_set(PhysxSet::NewActorsFlush), //clear commands for new components
                new_collider.in_base_set(PhysxSet::NewColliders),//px_apply_forces, px_set_joints
            ).before(px_step_simulation).chain())
            //run physx
            .add_system(px_step_simulation.in_base_set(CoreSet::PostUpdate) )//.after(px_set_joints))
            //write back
            .add_systems((
                px_sync_transforms, //px_write_velocitys
            ).in_base_set(CoreSet::PostUpdate).after(px_step_simulation).chain())
            ;

    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum PhysxSet {
    /// Runs before all other members of this set.
    NewActors,
    NewActorsFlush,
    NewColliders,
    UpdatePhysx,
    SyncBevy,
    // LastFlush,
}


#[derive(Resource)]
pub struct PhysxRes {
    pub foundation: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>,
    pub scene: Owner<PxScene>,
    pub handles: Handels,
}
unsafe impl Send for PhysxRes {}
unsafe impl Sync for PhysxRes {}


#[derive(Default)]
pub struct Handels {
    // pub static_actors: SlotMap<RigidStaticHandle, *mut physx_sys::PxRigidStatic>,
    pub dynamic_actors: SlotMap<RigidDynamicHandle, *mut physx_sys::PxRigidDynamic>,
    pub articulation_links: SlotMap<ArticulationLinkHandle, *mut physx_sys::PxArticulationLink>,
}


new_key_type! {
    pub struct RigidStaticHandle;
    pub struct RigidDynamicHandle;
    pub struct ArticulationLinkHandle;
}


#[derive(Component)]
pub struct PxStaticRigidBodyHandle(pub RigidStaticHandle);
#[derive(Component)]
pub struct PxDynamicRigidBodyHandle(pub RigidDynamicHandle);
#[derive(Component)]
pub struct PxArticulationLinkHandle(pub ArticulationLinkHandle);


const PHYSXSTEP: f32 = 1.0 / 60.0;

//run physx
fn px_step_simulation(   
    mut physx: ResMut<PhysxRes>,
    time: Res<Time>,
    mut accumilator: Local<f32>,
){

    *accumilator += time.delta_seconds();

    if *accumilator >= PHYSXSTEP {
        *accumilator -= PHYSXSTEP;

        physx.scene.simulate(PHYSXSTEP, None, None);

        physx.scene.fetch_results(true).expect("PhysX simulation failed");
    }
}



fn setup_physx(   
    mut commands: Commands,
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

    
    let mut scene: Owner<PxScene> = foundation
        .create(SceneDescriptor {
            gravity: PxVec3::new(0.0, -9.81, 0.0),
            on_advance: Some(OnAdvance),
            simulation_filter_shader: FilterShaderDescriptor::Custom(costum_filter_shader),
            thread_count: 4, //todo get cpu count
            ..SceneDescriptor::new(()) 
        })
        .unwrap();



    let handles = Handels::default();


    //spawn ground plane
    let mut material = foundation.physics_mut().create_material(0.4, 0.4, 0.4, ()).unwrap();

    let ground_plane = foundation.physics_mut()
        .create_plane(PxVec3::new(0.0, 1.0, 0.0), 0.0, material.as_mut(), ())
        .unwrap();

    scene.add_static_actor(ground_plane);


    commands.insert_resource(PhysxRes{ foundation, scene, handles });
}



