use bevy::prelude::*;
use bevy_physx::prelude::*;


fn main() {

    App::new()
    
        .add_plugins(DefaultPlugins)

        .add_plugin(PhysXPlugin)

        .add_startup_system(setup)
        
        .run();

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

    //high mass
    commands.spawn((
        PxDynamicActor,
        PxCollider::Box { size: Vec3::ONE },
        PxMassProperties::Density(1000.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0) )),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(-1.2, 5.0, 0.0),
            ..default()
        }
    ));
 

    //low mass
    commands.spawn((
        PxDynamicActor,
        PxCollider::Box { size: Vec3::ONE },
        PxMassProperties::Density(0.1),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0) )),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(1.2, 5.0, 0.0),
            ..default() 
        } 
    ));


    
    //ball
    commands.spawn((
        PxDynamicActor,
        PxCollider::Sphere { radius: 1.0 },
        PxMassProperties::Density(1.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 1.0, ..default() } )),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
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

