
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
//use shape::Circle;
use rand::prelude::*;

use crate::Diameter;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (create_wall0, create_wall1, create_wall2, create_wall3)); 
    }
}


fn create_wall0(
    mut commands: Commands, 
    meshes: ResMut<Assets<Mesh>>, 
    materials: ResMut<Assets<ColorMaterial>>, 
) {
    let wall0 = WallBundle::new(
        meshes, 
        materials, 
        Color::GRAY, 
        vec2(0.0, -300.0), 
        400.0, 
        5.0
    );
    commands.spawn((wall0));
}
fn create_wall1(
    mut commands: Commands, 
    meshes: ResMut<Assets<Mesh>>, 
    materials: ResMut<Assets<ColorMaterial>>, 
) {
    let wall0 = WallBundle::new(
        meshes, 
        materials, 
        Color::GRAY, 
        vec2(0.0, 300.0), 
        400.0, 
        5.0
    );
    commands.spawn((wall0));
}
fn create_wall2(
    mut commands: Commands, 
    meshes: ResMut<Assets<Mesh>>, 
    materials: ResMut<Assets<ColorMaterial>>, 
) {
    let wall0 = WallBundle::new(
        meshes, 
        materials, 
        Color::GRAY, 
        vec2(-400.0, 0.0), 
        5.0, 
        300.0
    );
    commands.spawn((wall0));
}
fn create_wall3(
    mut commands: Commands, 
    meshes: ResMut<Assets<Mesh>>, 
    materials: ResMut<Assets<ColorMaterial>>, 
) {
    let wall0 = WallBundle::new(
        meshes, 
        materials, 
        Color::GRAY, 
        vec2(400.0, 0.0), 
        5.0, 
        300.0
    );
    commands.spawn((wall0));
}


#[derive(Component, Debug)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
    wall: Wall,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    rigidbody: RigidBody,
    friction: Friction,
    restitution: Restitution,
}

impl WallBundle {

    fn new(
        mut meshes: ResMut<Assets<Mesh>>, 
        mut materials: ResMut<Assets<ColorMaterial>>, 
        color: Color, 
        pos: Vec2, 
        half_x: f32, 
        half_y: f32
    ) -> Self {
        WallBundle {
            wall: Wall,
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(half_x*2.0, half_y*2.0))),
                material: materials.add(color),
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(half_x, half_y),
            rigidbody: RigidBody::Fixed,
            friction: Friction::coefficient(0.0),
            restitution: Restitution::coefficient(1.0),
        }
    }

}