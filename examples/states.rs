use bevy::prelude::*;
use bevy_pipelines_ready::{PipelinesReady, PipelinesReadyPlugin};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Ready,
}

#[derive(Component)]
struct LoadingOnly;

// This value should be found experimentally by logging `PipelinesReady` in your app
// during normal use and noting the maximum value.
#[cfg(not(target_arch = "wasm32"))]
const EXPECTED_PIPELINES: usize = 10;
// The value will likely differ on the web due to different implementations of some
// render features.
#[cfg(all(target_arch = "wasm32", feature = "webgpu", not(feature = "webgl2")))]
const EXPECTED_PIPELINES: usize = 8;
// Note: you must add these features to your app. See `Cargo.toml`.
#[cfg(all(target_arch = "wasm32", feature = "webgl2", not(feature = "webgpu")))]
const EXPECTED_PIPELINES: usize = 6;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PipelinesReadyPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup_loading_screen)
        .add_systems(Update, print.run_if(resource_changed::<PipelinesReady>))
        .add_systems(
            Update,
            transition
                .run_if(in_state(GameState::Loading))
                .run_if(resource_changed::<PipelinesReady>),
        )
        .add_systems(OnExit(GameState::Loading), cleanup::<LoadingOnly>)
        .run();
}

// Your loading screen should include all of the cameras, lights, and other elements that cause
// pipelines to be built in your app.
fn setup_loading_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        TextBundle::from_section("Pipelines loading...".to_string(), TextStyle::default()),
        LoadingOnly,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 1_000_000.,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 6.0, 5.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::default()),
        material: materials.add(Color::from(bevy::color::palettes::tailwind::PINK_500)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
        material: materials.add(Color::from(Srgba::gray(0.6))),
        transform: Transform::from_xyz(0., -0.5, 0.),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn print(ready: Res<PipelinesReady>) {
    info!("Pipelines Ready: {}/{}", ready.get(), EXPECTED_PIPELINES);
}

fn transition(ready: Res<PipelinesReady>, mut next_state: ResMut<NextState<GameState>>) {
    if ready.get() >= EXPECTED_PIPELINES {
        next_state.set(GameState::Ready);
    }
}

fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
