use bevy_log::{Level, LogPlugin};

#[allow(dead_code)]
pub fn log_plugin() -> LogPlugin {
    LogPlugin {
        filter: String::from("bevy_butler"),
        level: Level::DEBUG,
        ..Default::default()
    }
}
