
use bevy::core_pipeline::bloom::BloomPrefilterSettings;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::hashbrown::HashMap;
use bevy_rapier2d::prelude::*;

use rand::prelude::*;
use rand::{Rng, thread_rng};

use crate::Diameter;
use crate::signal::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (create_particles));
        app.add_systems(Update, (
            create_particle,
            //collision_events_system,
            //draw_primitives2d_system,
            react,
        ));        
    }
}


fn create_particles(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    diameter: Res<Diameter>,
    //mut add_particle: ResMut<AddParticle>,
) {
    for _ in 0..12 {
        let particle = ParticleBundle::new_random(
            &mut meshes, 
            &mut materials, 
            &diameter
        );
        commands.spawn(particle)
        .with_children(|child_builder| {
            child_builder.spawn((
                Collider::ball(40.0),
                Sensor,
                Sleeping::disabled(),
                ActiveCollisionTypes::all(),
                //ReactionField,
            ));
        });
    }
}

fn create_particle(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    diameter: Res<Diameter>,
    mut add_particle: ResMut<AddParticle>,
) {
    if add_particle.is_high() {
        add_particle.set_low();
        /* let colors = [
            Color::GREEN,
            Color::BLUE,
            Color::RED,
            Color::YELLOW,
            Color::ORANGE,
        ]; */
        //let color = colors[thread_rng().gen_range(0..colors.len())];
        let particle = ParticleBundle::new_random(
            &mut meshes, 
            &mut materials, 
            &diameter
        );
        commands.spawn(particle)
        .with_children(|child_builder| {
            child_builder.spawn((
                Collider::ball(40.0),
                Sensor,
                Sleeping::disabled(),
                ActiveCollisionTypes::all(),
                //ReactionField,
            ));
        });
    }
}

fn react(
    mut particles: Query<(Entity, &mut ExternalImpulse, &ForceField, &ParticleType), With<Particle>>,
    physics: ResMut<RapierContext>,
) {
    let mut particles_color: HashMap<Entity, ParticleType> = HashMap::new();
    for (e, _, _, typo) in particles.iter() {
        particles_color.insert(e, typo.clone());
    }

    for (e, mut imp, field, typo) in particles.iter_mut() {
            let mut impulse = Vec2::ZERO;
            for (e1, e2, _) in physics.intersection_pairs_with(e) {
                let (en0, en1, ok) = if e == e1 {
                    let p = physics.collider_parent(e2).unwrap();
                    (e1, p, true)
                } else if e == e2 {
                    let p = physics.collider_parent(e1).unwrap();
                    (e2, p, true)
                } else {
                    (Entity::PLACEHOLDER, Entity::PLACEHOLDER, false)
                };
                if !ok {
                    continue;
                }
                match field {
                    ForceField::Magnetic { f, r, max } => {
                        let range = r;
                        let force = *f;
                        let typo0 = particles_color.get(&en0).unwrap();
                        let typo1 = particles_color.get(&en1).unwrap();
                        let rbh0 = physics.entity2body().get(&en0).unwrap();
                        let rbh1 = physics.entity2body().get(&en1).unwrap();
                        let rb0 = physics.bodies.get(*rbh0).unwrap();
                        let rb1 = physics.bodies.get(*rbh1).unwrap();
                        let pos0 = vec2(rb0.translation().x, rb0.translation().y);
                        let pos1 = vec2(rb1.translation().x, rb1.translation().y);
                        let dist = pos0.distance(pos1);
                        let rel_dist = dist/range;
                        let x = (rel_dist-(range/2.0))*2.0;
                        //let y = 1.0-x.powi(2);
                        //let y = -(x*x.abs());
                        let y = x;
                        let mut dir = pos1-pos0;
                        let r = ((0.5-typo0.color.r()) * 2.0) * typo1.color.r();
                        let g = ((0.5-typo0.color.g()) * 2.0) * typo1.color.g();
                        let b = ((0.5-typo0.color.b()) * 2.0) * typo1.color.b();
                        let action = vec![r, g, b].iter().sum::<f32>(); 
                        dir = dir.normalize_or_zero();
                        let mut i = (dir * action * force) * y;
                        
                        //if rel_dist <= *max {
                        //    i = -force * (rel_dist/max);
                        //} else {
                        //    i = force * ((rel_dist-max)/(1.0-max));
                        //}
                        impulse += i;       
                    },
                    _ => {},
                }
            }
            imp.impulse = impulse;
        }
    }

#[derive(Component, Debug)]
struct Particle;

#[derive(Component, Debug, Clone)]
struct ParticleType {
    pub color: Color,
}

impl ParticleType {

    pub fn random() -> Self {
        let r = thread_rng().gen_range(0..=1) as f32 * 1.0;
        let g = thread_rng().gen_range(0..=1) as f32 * 1.0;
        let b = thread_rng().gen_range(0..=1) as f32 * 1.0;
        ParticleType {
            color: Color::rgb(r, g, b),
        }
    }

}


#[derive(Bundle)]
struct ParticleBundle {
    particle: Particle,
    particle_type: ParticleType,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    active_events: ActiveEvents,
    rigidbody: RigidBody,
    ccd: Ccd,
    gravity: GravityScale,
    friction: Friction,
    restitution: Restitution,
    dumping: Damping,
    velocity: Velocity,
    impulse: ExternalImpulse,
    field: ForceField,
    collisions: ActiveCollisionTypes,
}

impl ParticleBundle {

    fn new_random(
        meshes: &mut ResMut<Assets<Mesh>>, 
        materials: &mut ResMut<Assets<ColorMaterial>>, 
        diameter: &Res<Diameter>,
    ) -> Self {
        let x = thread_rng().gen_range(diameter.w.clone());
        let y = thread_rng().gen_range(diameter.h.clone());
        let particle_type = ParticleType::random();
        ParticleBundle {
            particle: Particle,
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(5.0))),
                material: materials.add(particle_type.color),
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            collider: Collider::ball(5.0),
            active_events: ActiveEvents::COLLISION_EVENTS,
            rigidbody: RigidBody::Dynamic,
            ccd: Ccd::enabled(),
            gravity: GravityScale(0.0),
            friction: Friction::coefficient(0.0),
            restitution: Restitution::coefficient(1.0),
            dumping: Damping {
                linear_damping: 0.6,
                angular_damping: 0.0,
            },
            velocity: Velocity::linear(
        vec2(
                thread_rng().gen_range(-1.0..=1.0), 
                thread_rng().gen_range(-1.0..=1.0)) * 0.0
            ),
            impulse: ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            },
            field: ForceField::Magnetic { f: 10.0, r: 40.0, max: 0.2 },
            collisions: ActiveCollisionTypes::DYNAMIC_STATIC,
            particle_type,
        }
    }

}


pub struct Magnetic {
    pub strength: f32,
    pub maximum: f32,
    pub range: f32,
}

pub struct Neutral;

#[derive(Component, Debug)]
pub enum ForceField {
    Neutral,
    Magnetic{f: f32, r: f32, max: f32},
}

#[derive(Bundle)]
pub struct ForceFieldBundle {
    field: ForceField,
    collider: Collider,
    sensor: Sensor,
}

#[derive(Component, Debug)]
pub struct ReactionField;

impl ForceFieldBundle {

    pub fn new_magnetic(
        strength: f32, 
        range: f32, 
        maximum: f32
    ) -> Self {
        let field = ForceField::Magnetic { f: strength, r: range, max: maximum };
        ForceFieldBundle {
            field,
            collider: Collider::ball(range),
            sensor: Sensor,
        }        
    }

    pub fn new_neutral() -> Self {
        ForceFieldBundle {
            field: ForceField::Neutral,
            collider: Collider::ball(0.0),
            sensor: Sensor,
        }
    }
}

/* fn draw_primitives2d_system(fields: Query<(&ForceField, &GlobalTransform), With<ForceField>>, mut gizmos: Gizmos) {
    for (field, gtf) in fields.iter() {
        match field {
            ForceField::Magnetic { f, r, max } => {
                gizmos.primitive_2d(Circle::new(*r), gtf.translation().xy(), 0.0, Color::rgba(1.0, 1.0, 1.0, 0.01));
            },
            ForceField::Neutral => {},
        }
    }
} */