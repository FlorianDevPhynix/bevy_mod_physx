use std::f32::consts::FRAC_PI_4;

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


    let part_mesh = meshes.add(Mesh::from(shape::UVSphere{ radius: 0.2, ..default() }));
    let part_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());


    let mut articulation = PxArticulation::new();

    let root = articulation.create_link(&mut commands, None, Transform::from_xyz(0.0, 0.5, 0.0));
    let part_1 = articulation.create_link(&mut commands, Some(root), Transform::from_xyz(0.0, 1.5, 0.0));
    let part_2_1 = articulation.create_link(&mut commands, Some(part_1), Transform::from_xyz(0.0, 2.0, 0.0));
    let part_2_2 = articulation.create_link(&mut commands, Some(part_1), Transform::from_xyz(0.0, 2.0, 0.0));

 
    commands.entity(root).insert((
        PxMassProperties::Density(100.0),
        PxCollider::Box { size: Vec3::splat(0.5)},
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 0.5, 0.5))),
            material: part_material.clone(),
            ..default()
        }
    ));


    commands.entity(part_1).insert((
        PxCollider::Sphere { radius: 0.2 },
        PbrBundle { mesh: part_mesh.clone(), material: part_material.clone(), ..default() },
        PxArticulationJoint::new(PxJointType::Spherical)
            .parent_pose(Transform::from_xyz(0.0, 0.3, 0.0))
            .child_pose(Transform::from_xyz(0.0, -0.3, 0.0))
            .motions(PxJointMotion::Limited(PxJointLimit::new(-FRAC_PI_4, FRAC_PI_4)))
            .drives(PxJointDrive { target: 0.0, stiffness: 1000.0, damping: 100.0, force_limit: 100.0 }),
    ));


    commands.entity(part_2_1).insert((
        PxMassProperties::Density(20.0),
        PxCollider::Sphere { radius: 0.2 },
        PbrBundle { mesh: part_mesh.clone(), material: part_material.clone(), ..default() },
        PxArticulationJoint::new(PxJointType::Spherical)
            .parent_pose(Transform::from_xyz(-0.3, 0.3, 0.0))
            .child_pose(Transform::from_xyz(0.0, -0.3, 0.0))
            .motions(PxJointMotion::Limited(PxJointLimit::new(-FRAC_PI_4, FRAC_PI_4)))
            .drives(PxJointDrive { target: 0.0, stiffness: 100.0, damping: 10.0, force_limit: 100.0 }),
    ));


    commands.entity(part_2_2).insert((
        PxMassProperties::Density(0.5),
        PxCollider::Sphere { radius: 0.2 },
        PbrBundle { mesh: part_mesh.clone(), material: part_material.clone(), ..default() },
        PxArticulationJoint::new(PxJointType::Spherical)
            .parent_pose(Transform::from_xyz(0.3, 0.3, 0.0))
            .child_pose(Transform::from_xyz(0.0, -0.3, 0.0))
            .motions(PxJointMotion::Limited(PxJointLimit::new(-FRAC_PI_4, FRAC_PI_4)))
            .drives(PxJointDrive { target: 0.0, stiffness: 100.0, damping: 10.0, force_limit: 100.0 }),
    ));


    let _articulation_entity = commands.spawn(articulation);

 
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

