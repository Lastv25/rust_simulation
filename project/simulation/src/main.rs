use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn map_center(x: u32, y: u32) -> (u32, u32) {
    let midpoint_x = x as f64 / 2.0;
    let midpoint_y = y as f64 / 2.0;
    let x = midpoint_x as u32;
    let y = midpoint_y as u32;
    return (x, y);
}

fn circle_coordinates(x_center: u32, y_center: u32, radius: u32) -> Vec<(u32, u32)> {
    let number_hexa: u32 = radius * 6;
    let mut vec = Vec::with_capacity(number_hexa.try_into().unwrap());
    for r in 1..radius {
        vec.push((x_center + r, y_center));
        vec.push((x_center + r, y_center - r));
        vec.push((x_center, y_center + r));
        vec.push((x_center - r, y_center));
        vec.push((x_center - r, y_center + r));
        vec.push((x_center, y_center - r));
    }
    return vec;
}

fn spiral_coordinates(
    x_translation: i32,
    y_translation: i32,
    radius_max: f64,
    hexagons: u32,
) -> Vec<(u32, u32)> {
    let mut radius: f64 = 0.0;
    let mut angle: f64 = 0.0;
    let step = 1.0 * std::f64::consts::PI / hexagons as f64;

    let mut vec = Vec::with_capacity(hexagons as usize);

    let mut x = 0;
    let mut y = 0;

    for _i in 0..hexagons {
        let new_x = x + (radius * angle.cos()) as i32;
        let new_y = y + (radius * angle.sin()) as i32;

        vec.push((
            (new_x + x_translation).try_into().unwrap(),
            (new_y + y_translation).try_into().unwrap(),
        ));
        angle += step;
        radius += step;

        if radius > radius_max {
            return vec;
        }
    }

    return vec;
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("flat_hex_tiles.png");

    let map_size = TilemapSize { x: 10, y: 10 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // let (midpoint_x, midpoint_y) = map_center(map_size.x, map_size.y);
    let mut coords = spiral_coordinates(5, 5, 10.0, 5);
    // let mut coords = circle_coordinates(midpoint_x, midpoint_y, 4);
    // coords.push((midpoint_x, midpoint_y));
    for coord in coords.into_iter() {
        let (x, y) = coord;
        println!("Points: ({}, {})", x, y);
        let tile_pos = TilePos { x, y };
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            })
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Hexagon(HexCoordSystem::Column);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    // Add atlas to array texture loader so it's preprocessed before we need to use it.
    // Only used when the atlas feature is off and we are using array textures.
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("tiles.png")),
            tile_size,
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Simulation Fun"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .run();
}
