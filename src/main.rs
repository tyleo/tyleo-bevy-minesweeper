use bevy::{app::PluginGroupBuilder, prelude::*};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn make_window_plugin() -> WindowPlugin {
    let primary_window = Window {
        title: "Mine Sweeper!".into(),
        resolution: (700., 800.).into(),
        ..default()
    };

    WindowPlugin {
        primary_window: Some(primary_window),
        ..default()
    }
}

fn make_default_plugins() -> PluginGroupBuilder {
    let window_plugin = make_window_plugin();
    DefaultPlugins.set(window_plugin)
}

fn startup_camera_system(mut commands: Commands) {
    let camera_2d_bundle = Camera2dBundle {
        camera: Camera { ..default() },
        ..default()
    };

    commands.spawn(camera_2d_bundle);
}

fn main() {
    let mut app = App::new();

    app.add_plugins(make_default_plugins());

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, startup_camera_system);

    app.run();
}
