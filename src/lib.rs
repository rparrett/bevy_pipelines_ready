use bevy::{
    prelude::*,
    render::{
        render_resource::{CachedPipelineState, PipelineCache},
        Render, RenderApp,
    },
};
use crossbeam_channel::Receiver;

pub struct PipelinesReadyPlugin;

#[derive(Resource)]
struct PipelinesReadyChannel(Receiver<usize>);

/// A `Resource` in the main world that stores the number of pipelines that are ready.
#[derive(Resource, Default)]
pub struct PipelinesReady(usize);
impl PipelinesReady {
    /// Returns the number of pipelines that are ready.
    pub fn get(&self) -> usize {
        self.0
    }
}

impl Plugin for PipelinesReadyPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = crossbeam_channel::bounded(1);

        app.init_resource::<PipelinesReady>();

        app.add_systems(Update, move |mut ready: ResMut<PipelinesReady>| {
            let Ok(num) = rx.try_recv() else {
                return;
            };

            ready.0 = num;
        });

        let renderer_app = app.sub_app_mut(RenderApp);

        let mut current = 0;
        renderer_app.add_systems(Render, move |cache: Res<PipelineCache>| {
            let count = cache
                .pipelines()
                .filter(|pipeline| matches!(pipeline.state, CachedPipelineState::Ok(_)))
                .count();

            if current == count {
                return;
            }

            let _ = tx.send(count);

            current = count;
        });
    }
}
