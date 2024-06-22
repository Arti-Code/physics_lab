
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
//use shape::Circle;
use rand::prelude::*;

use crate::Diameter;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_particle); 
        app.add_systems(Update, create_particle);        
    }
}


fn create_particle(
    mut commands: Commands, 
    meshes: ResMut<Assets<Mesh>>, 
    materials: ResMut<Assets<ColorMaterial>>, 
    time: Res<Time>,
    diameter: Res<Diameter>,
) {
    if thread_rng().gen_bool(time.delta_seconds_f64()) {
        let particle = ParticleBundle::new(meshes, materials, Color::GREEN, diameter);
        commands.spawn((particle));
        info!("particle!");
    }
}



#[derive(Component, Debug)]
struct Particle;

#[derive(Bundle)]
struct ParticleBundle {
    particle: Particle,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    rigidbody: RigidBody,
    gravity: GravityScale,
    // velocity: Velocity,
    // mass_properties: MassProperties,
}

impl ParticleBundle {

    fn new(mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, color: Color, diameter: Res<Diameter>) -> Self {
        let x = thread_rng().gen_range(diameter.w.clone());
        let y = thread_rng().gen_range(diameter.h.clone());
        ParticleBundle {
            particle: Particle,
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(20.0))),
                material: materials.add(color),
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            collider: Collider::ball(20.0),
            rigidbody: RigidBody::Dynamic,
            gravity: GravityScale(0.05),
        }
    }

}