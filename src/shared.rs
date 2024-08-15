use bevy::{
    app::PluginGroupBuilder,
    input::keyboard::{Key, KeyboardInput},
    log,
    prelude::*,
};
use board_plugin_lib::{components::*, ext::*, resources::*, BoardPlugin, TypeRegistry};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Loading,
    Loaded,
    Out,
    InGame,
}

fn set_window_plugin(
    plugin_group: impl PluginGroup,
    canvas_id_selector: Option<String>,
    resolution: Option<(f32, f32)>,
) -> PluginGroupBuilder {
    let primary_window = Window {
        canvas: canvas_id_selector,
        title: "Mine Sweeper!".into(),
        resolution: resolution.unwrap_or((700., 800.)).into(),
        ..default()
    };

    let window_plugin = WindowPlugin {
        primary_window: Some(primary_window),
        ..default()
    };

    plugin_group.set(window_plugin)
}

#[cfg(feature = "process_assets")]
fn set_asset_plugin(plugin_group: impl PluginGroup) -> PluginGroupBuilder {
    let asset_plugin = AssetPlugin {
        mode: AssetMode::Processed,
        ..default()
    };

    plugin_group.set(asset_plugin)
}

fn make_default_plugins(
    canvas_id_selector: Option<String>,
    resolution: Option<(f32, f32)>,
) -> PluginGroupBuilder {
    let default_plugins = DefaultPlugins;

    let default_plugins = set_window_plugin(default_plugins, canvas_id_selector, resolution);

    #[cfg(feature = "process_assets")]
    let default_plugins = set_asset_plugin(default_plugins);

    default_plugins
}

fn startup_camera_system(mut commands: Commands) {
    let camera_2d_bundle = Camera2dBundle {
        camera: Camera { ..default() },
        ..default()
    };

    commands.spawn(camera_2d_bundle);
}

fn iteration_system(mut commands: Commands) {
    commands.spawn(Coordinates { x: 1, y: 1 });
}

fn state_handler(
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut key_event_reader: EventReader<KeyboardInput>,
) {
    if state.get() == &AppState::Loaded {
        next_state.set(AppState::InGame)
    }

    for event in key_event_reader.read() {
        if let Key::Character(character) = &event.logical_key {
            match character.as_str() {
                "c" => {
                    log::debug!("clearing detected");
                    if event.state.is_pressed() && *state.get() == AppState::InGame {
                        log::info!("clearing game");
                        next_state.set(AppState::Out);
                    }
                }
                "g" => {
                    log::debug!("loading detected");
                    if event.state.is_pressed() && *state.get() == AppState::Out {
                        log::info!("loading game");
                        next_state.set(AppState::InGame);
                    }
                }
                _ => {}
            }
        }
    }
}

fn setup_board(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: true,
        ..Default::default()
    });
    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::srgb_u8(40, 40, 40),
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::srgb_u8(100, 100, 100),
            ..Default::default()
        },
        bomb_number_font: asset_server.load("fonts/arial-rounded-mt-regular.ttf"),
        bomb_number_colors: BoardAssets::default_bomb_number_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: Color::WHITE,
        },
    });
    // Plugin activation
    next_state.set(AppState::Loaded);
}

pub fn run(canvas_id_selector: Option<String>, resolution: Option<(f32, f32)>) {
    let mut app = App::new();

    app.register_types(TypeRegistry);

    app.add_plugins(make_default_plugins(canvas_id_selector, resolution));
    app.add_plugins(BoardPlugin {
        running_state: AppState::InGame,
    });

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.insert_state(AppState::Loading);

    app.insert_resource(BoardOptions {
        bomb_count: 40,
        map_size: (20, 20),
        safe_start: true,
        tile_padding: 3.0,
        ..default()
    });

    app.add_systems(Startup, setup_board);
    app.add_systems(Startup, startup_camera_system);
    app.add_systems(Startup, iteration_system);
    app.add_systems(FixedUpdate, state_handler);

    app.run();
}
