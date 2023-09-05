use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


fn radius_concentric(x: u32, y: u32) -> (f64, f64) {
    // Midpoint formula
    let midpoint_x = x as f64 / 2.0;
    let midpoint_y = y as f64 / 2.0;
    return (midpoint_x, midpoint_y);
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
    // Alternatively, you can use helpers::filling::fill_tilemap.
    // for x in 0..map_size.x {
    //     for y in 0..map_size.y {
    //         let tile_pos = TilePos { x, y };
    //         println!("x:{},y:{}", x,y);
    //         let tile_entity = commands
    //             .spawn(TileBundle {
    //                 position: tile_pos,
    //                 tilemap_id: TilemapId(tilemap_entity),                 
    //                 ..Default::default()
    //             })
    //             .id();
    //         tile_storage.set(&tile_pos, tile_entity);
    //     }
    // }

    let (midpoint_x, midpoint_y) = radius_concentric(map_size.x, map_size.y);
    println!("Midpoint: ({}, {})", midpoint_y, midpoint_y);
    let x = midpoint_x as u32;
    let y = midpoint_y as u32;
    let tile_pos: TilePos = TilePos { x, y };
    let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),                 
                    ..Default::default()
                })
                .id();
    tile_storage.set(&tile_pos, tile_entity);

    let x_2 = x+1;
    let y_2 = y;
    println!("New point: ({}, {})", x_2, y_2);

    let tile_pos_2 = TilePos { x:x_2, y:y_2 };
    let tile_entity_2: Entity = commands
                .spawn(TileBundle {
                    position: tile_pos_2,
                    tilemap_id: TilemapId(tilemap_entity),                 
                    ..Default::default()
                })
                .id();
    tile_storage.set(&tile_pos_2, tile_entity_2);

    let x_3 = x;
    let y_3 = y+1;
    println!("New point: ({}, {})", x_3, y_3);

    let tile_pos_3 = TilePos { x:x_3, y:y_3 };
    let tile_entity_3: Entity = commands
                .spawn(TileBundle {
                    position: tile_pos_3,
                    tilemap_id: TilemapId(tilemap_entity),                 
                    ..Default::default()
                })
                .id();
    tile_storage.set(&tile_pos_3, tile_entity_3);

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Hexagon(HexCoordSystem::Column);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture:  TilemapTexture::Single(texture_handle),
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
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Simulation Fun",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true,KeyCode::Escape)))
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup).run();
}

