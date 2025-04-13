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

use std::env;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        PhysicsPlugins::default().with_length_unit(20.0),
        PhysicsDiagnosticsPlugin,
        PhysicsDiagnosticsUiPlugin,
    ))
    .add_plugins(LdtkPlugin)
    .insert_resource(Gravity(Vector::NEG_Y * 100.0))
    .insert_resource(LevelSelection::index(0))
    .register_ldtk_entity::<PlayerBundle>("Player")
    .register_ldtk_entity::<GoalBundle>("Goal")
    .register_ldtk_int_cell::<WallBundle>(1)
    .add_systems(Startup, setup)
    .add_systems(Update, init_added_player);
    if env::var("R").as_deref() == Ok("1") {
        app.add_plugins({
            let rec = revy::RecordingStreamBuilder::new("bevy-platformer")
                .save("log.rrd")
                .unwrap();
            revy::RerunPlugin { rec }
        });
    }
    app.run();
}

#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

fn init_added_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<Assets<Image>>,
    player: Single<(Entity, &Sprite, &Transform), Added<Player>>,
) {
    let (player_entity, player_sprite, sprite_transform) = (player.0, player.1, player.2);
    let player_size = Vec2::new(1.0, 1.0);
    //assets.get(&player_sprite.image).unwrap().size_f32() * sprite_transform.scale.truncate();
    let player_mesh = meshes.add(Rectangle::from_size(player_size));
    let player_material = materials.add(Color::NONE);
    commands.entity(player_entity).insert((
        RigidBody::Dynamic,
        //LockedAxes::ROTATION_LOCKED,
        //Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        Mesh2d(player_mesh),
        Transform::from_scale(Vec3::new(0.5, 0.5, 0.5))
            .with_translation(Vec3::new(400.0, 500.0, 10.0)),
        //MeshMaterial2d(player_material),
        Collider::rectangle(player_size.x, player_size.y),
    ));

    //
    //commands.spawn((
    //    Player,
    //));
}
