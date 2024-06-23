
use bevy::core_pipeline::bloom::BloomPrefilterSettings;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

//use shape::Circle;
use rand::prelude::*;
use rand::{Rng, thread_rng};

use crate::Diameter;
use crate::signal::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Startup, create_particle); 
        app.add_systems(Update, create_particle);        
    }
}


fn create_particle(
    mut commands: Commands, 
    meshes: ResMut<Assets<Mesh>>, 
    materials: ResMut<Assets<ColorMaterial>>, 
    //time: Res<Time>,
    diameter: Res<Diameter>,
    mut add_particle: ResMut<AddParticle>,
) {
    //if thread_rng().gen_bool(time.delta_seconds_f64()*3.0) {
    if add_particle.is_high() {
        add_particle.set_low();
        let colors = [
            Color::GREEN,
            Color::BLUE,
            Color::RED,
            Color::YELLOW,
            Color::PINK,
            Color::CYAN,
            Color::VIOLET,
            Color::TURQUOISE,
            Color::YELLOW_GREEN,
            Color::AZURE,
            Color::CYAN,
        ];
        //colors.sample();
        let color = colors[thread_rng().gen_range(0..colors.len())];
        let particle = ParticleBundle::new(meshes, materials, color, diameter);
        commands.spawn((particle)).id();
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
    friction: Friction,
    restitution: Restitution,
    dumping: Damping,
    velocity: Velocity,
    // mass_properties: MassProperties,
}

impl ParticleBundle {

    fn new(
        mut meshes: ResMut<Assets<Mesh>>, 
        mut materials: ResMut<Assets<ColorMaterial>>, 
        color: Color, 
        diameter: Res<Diameter>,
        
    ) -> Self {
        let x = thread_rng().gen_range(diameter.w.clone());
        let y = thread_rng().gen_range(diameter.h.clone());
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
            gravity: GravityScale(0.0),
            friction: Friction::coefficient(0.0),
            restitution: Restitution::coefficient(1.0),
            dumping: Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            velocity: Velocity::linear(vec2(thread_rng().gen_range(-1.0..=1.0), thread_rng().gen_range(-1.0..=1.0)) *500.0),
        }
    }

}