
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
//use shape::Circle;
use rand::prelude::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_particle);        
    }
}


fn create_particle(mut commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<ColorMaterial>>) {
    let particle = ParticleBundle::new(meshes, materials, Color::GREEN);
    commands.spawn((particle));
}



#[derive(Component, Debug)]
struct Particle;

#[derive(Bundle)]
struct ParticleBundle {
    particle: Particle,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    rigidbody: RigidBody,
    // velocity: Velocity,
    // mass_properties: MassProperties,
}

impl ParticleBundle {

    fn new(mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, color: Color) -> Self {
        let x = thread_rng().gen_range(-200.0..=200.0);
        let y = thread_rng().gen_range(-200.0..=200.0);
        ParticleBundle {
            particle: Particle,
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(5.0))),
                material: materials.add(color),
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            collider: Collider::ball(5.0),
            rigidbody: RigidBody::Dynamic,
        }
    }

}