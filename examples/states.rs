use bevy::prelude::*;
use bevy_pipelines_ready::{PipelinesReady, PipelinesReadyPlugin};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Ready,
}

// This value should be found experimentally by logging `PipelinesReady` in your app
// during normal use and noting the maximum value.
#[cfg(not(target_arch = "wasm32"))]
const EXPECTED_PIPELINES: usize = 35;
// The value will likely differ on the web due to different implementations of some
// render features.
#[cfg(all(target_arch = "wasm32", feature = "webgpu", not(feature = "webgl2")))]
const EXPECTED_PIPELINES: usize = 20;
// Note: These features can be simplified if your app only builds for one of either
// webgpu or webgl2. Simply use `#[cfg(target_arch = "wasm32")]`. If your app builds
// for both, you must add these features (or similar) to your app. See `Cargo.toml`.
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
        Text::new("Pipelines loading...".to_string()),
        DespawnOnExit(GameState::Loading),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 1_000_000.,
            ..default()
        },
        Transform::from_xyz(3.0, 6.0, 5.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::default())),
        MeshMaterial3d(materials.add(Color::from(bevy::color::palettes::tailwind::PINK_500))),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::from(Srgba::gray(0.6)))),
        Transform::from_xyz(0., -0.5, 0.),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn print(ready: Res<PipelinesReady>) {
    info!("Pipelines Ready: {}/{}", ready.get(), EXPECTED_PIPELINES);
}

fn transition(ready: Res<PipelinesReady>, mut next_state: ResMut<NextState<GameState>>) {
    if ready.get() >= EXPECTED_PIPELINES {
        // Note: you may want to wait an additional period of time or number
        // of frames after this.
        //
        // In my experience, Bevy's framerate seems to take a few seconds to
        // fully recover after pipelines are compiled when running in Firefox.
        next_state.set(GameState::Ready);
    }
}
