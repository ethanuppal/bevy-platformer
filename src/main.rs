// Copyright (C) 2025 Ethan Uppal.
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3 of the License only.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(20.0),
            PhysicsDiagnosticsPlugin,
            PhysicsDiagnosticsUiPlugin,
        ))
        .add_plugins(LdtkPlugin)
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tile-based-game.ldtk").into(),
        ..Default::default()
    });

    let player_size = Vector::new(20.0, 20.0);
    let player_mesh = meshes.add(Rectangle::from_size(player_size.f32()));
    let player_material = materials.add(Color::srgb(0.0, 1.0, 0.5));

    commands.spawn((
        Mesh2d(player_mesh),
        MeshMaterial2d(player_material),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        Collider::rectangle(player_size.x, player_size.y),
        Position::from_xy(500.0, 200.0),
        Player,
    ));
}
