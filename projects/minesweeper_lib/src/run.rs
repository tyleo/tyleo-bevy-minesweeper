use crate::{
    components::*, config::GameConfig, ext::*, resources::*, util::*, BoardPlugin, TypeRegistry,
};
use bevy::{
    app::PluginGroupBuilder,
    input::keyboard::{Key, KeyboardInput},
    log,
    prelude::*,
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

fn set_window_plugin(
    plugin_group: impl PluginGroup,
    canvas_id_selector: Option<String>,
    resolution: Option<Vec2>,
) -> PluginGroupBuilder {
    let primary_window = Window {
        canvas: canvas_id_selector,
        title: "Mine Sweeper!".into(),
        resolution: resolution.unwrap_or(Vec2::new(700., 800.)).into(),
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
    resolution: Option<Vec2>,
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
    board_options: Res<BoardOptions>,
    asset_server: Res<AssetServer>,
) {
    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: board_options.colors.padding_color,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: board_options.colors.revealed_tile_color,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: board_options.colors.unknown_tile_color,
            ..Default::default()
        },
        pending_tile_material: SpriteMaterial {
            color: board_options.colors.highlighted_tile_color,
            ..Default::default()
        },
        bomb_number_font: asset_server.load("fonts/arial-rounded-mt-regular.ttf"),
        bomb_number_colors: board_options.colors.number_colors.clone(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: board_options.colors.flag_color,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: board_options.colors.bomb_color,
        },
    });
    // Plugin activation
    next_state.set(AppState::Loaded);
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn run(config: GameConfig) {
    let mut app = App::new();

    app.register_types(TypeRegistry);

    app.add_plugins(make_default_plugins(
        config.canvas_id_selector,
        config.resolution.map(|resolution| resolution.into()),
    ));
    app.add_plugins(BoardPlugin {
        running_state: AppState::InGame,
    });

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.insert_state(AppState::Loading);

    let colors: BoardColors = config
        .color_config
        .map(|color_config| color_config.into())
        .unwrap_or_default();

    app.insert_resource(ClearColor(colors.background_color));
    app.insert_resource(BoardOptions {
        bomb_count: config.bomb_count.unwrap_or(40),
        map_size: config
            .tile_count
            .map(|tile_count| (tile_count.x, tile_count.y))
            .unwrap_or((20, 20)),
        safe_start: true,
        tile_size: config
            .tile_size
            .map(|tile_size| TileSizeOption::Adaptive {
                min: tile_size.min,
                max: tile_size.max,
            })
            .unwrap_or(default()),
        tile_padding: config.tile_padding_size.unwrap_or(3.0),
        colors,
        ..default()
    });

    app.add_systems(Startup, setup_board);
    app.add_systems(Startup, startup_camera_system);
    app.add_systems(Startup, iteration_system);
    app.add_systems(FixedUpdate, state_handler);

    app.run();
}
