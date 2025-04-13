# Journal

`avian2d` and `bevy_ecs_ldtk` are really annoying to work together

I managed to fork them to get them to work on bevy 0.16.0-rc.2

| Physics plugin | Components added to player | Outcome |
| --- | --- | --- |
| Disabled | Anything | None |
| `PhysicsPlugins::default().with_length_unit(20.0)` | Nothing | Normal |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`RigidBody` | Normal |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`Collider` | Normal |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`RigidBody, Collider` | Player sprite disappears |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`RigidBody, Collider` | Player sprite disappears |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`RigidBody, Collider, Mesh2d` | Player sprite falls, but level is gray while it is on screen (I position it off screen) |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`RigidBody, Collider, Transform`, positioned above level | Everything is normal but you have to explicitly set a large z component for the player to be in front |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`RigidBody, Collider, Transform` but positioned off to the side | Player sprite falls and everything is normal |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`RigidBody, Collider, Mesh2d, Transform`, positioned above level | Player disappears, level flashes gray quickly for no reason |
| `PhysicsPlugins::default().with_length_unit(20.0)` |`RigidBody, Collider, Mesh2d, Transform` but positioned off to the side | Player sprite falls and everything is normal |

Note that setting gravity to `0.0` doesn't affect any of these outcomes.
