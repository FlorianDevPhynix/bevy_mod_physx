use bevy::prelude::*;
use bevy_mod_physx::prelude::*;


fn main() {

    App::new()
    
        .add_plugins(DefaultPlugins)

        .add_plugin(PhysXPlugin)

        .add_startup_system(setup)

        .add_system(raycast_system)
        
        .run();

}


pub fn raycast_system(
    mut physx_res: ResMut<PhysX>,
) {

    let ray_cast_result = physx_res.raycast(Vec3::Y * 100.0, Vec3::NEG_Y, 200.0);
    println!("{:?}", ray_cast_result);

}



pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    // plane
    commands.spawn((
        PxPlane::default(),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(1000.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        }
    ));

    // Dynamic Actor to raycast against
    commands.spawn((
        PxDynamicActor,
        PxCollider::Box { size: Vec3::splat(1.0) },
        PxMaterial{restitution: 1.0, ..default()},
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube{ size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 50.0, 0.0),
            ..default()
        }
    ));
 
 
    // camera 
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 10.0),
        ..default()
    }); 

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        ..default()
    });

}

