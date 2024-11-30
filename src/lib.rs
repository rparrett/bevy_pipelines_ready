use bevy::{
    prelude::*,
    render::{
        render_resource::{CachedPipelineState, PipelineCache},
        MainWorld, RenderApp,
    },
};

pub struct PipelinesReadyPlugin;

/// A `Resource` in the main world that stores the number of pipelines that are ready.
#[derive(Resource, Default, Debug)]
pub struct PipelinesReady(usize);
impl PipelinesReady {
    /// Returns the number of pipelines that are ready.
    pub fn get(&self) -> usize {
        self.0
    }
}

impl Plugin for PipelinesReadyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PipelinesReady>();

        app.sub_app_mut(RenderApp)
            .add_systems(ExtractSchedule, update_pipelines_ready);
    }
}

fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, cache: Res<PipelineCache>) {
    if let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() {
        let count = cache
            .pipelines()
            .filter(|pipeline| matches!(pipeline.state, CachedPipelineState::Ok(_)))
            .count();

        if pipelines_ready.0 == count {
            return;
        }

        pipelines_ready.0 = count;
    }
}
