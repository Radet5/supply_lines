use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum StartupSet {
    StartupRoundA,
    StartupRoundB,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(
            Startup,
            (StartupSet::StartupRoundA, StartupSet::StartupRoundB).chain(),
        );
    }
}
