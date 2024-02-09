use tiled::{ChunkData, LayerType, Loader, TileLayer};
use macroquad::prelude::*;

use std::path::Path;
use std::collections::HashMap;


#[derive(Clone)]
struct Tileset {
    image: Texture2D,
    tiles: HashMap<u32, DrawTextureParams>,
}


impl Tileset {
    fn new(image: Texture2D) -> Self {
        Self {
            image,
            tiles: HashMap::new(),
        }
    }
}


#[derive(Copy, Clone, Debug)]
struct Tile {
    x: i32,
    y: i32,
    tileset: usize,
    id: u32,
}


struct Layer {
    tiles: Vec<Tile>,
}

impl Layer {
    fn new() -> Self {
        Self {
            tiles: Vec::new(),
        }
    }
}


pub struct Tilemap {
    tile_width: u32,
    tile_height: u32,
    tilesets: Vec<Tileset>,
    layers: Vec<Layer>,
}

impl Tilemap {
    pub async fn new(path: String) -> Option<Self> {
        println!("Loading tilemap {path}");

        let file_path = Path::new("resources/tilemaps/").join(path + ".tmx");

        let mut loader = Loader::new();
        let map = loader.load_tmx_map(file_path);

        if map.is_err() {
            error!("Error loading tilemap: {}", map.unwrap_err().to_string());
            return None;
        }

        let map = map.unwrap();

        let mut tilesets = Vec::new();

        for load_tileset in map.tilesets() {
            let load_tileset_image = load_tileset.image.clone().unwrap();
            let tileset_image_path = load_tileset_image.source;
            let tileset_image_path_str = tileset_image_path.to_str().unwrap();

            println!("Loading tileset with image from path {tileset_image_path_str}");

            let tileset_image = load_texture(tileset_image_path_str);

            let tile_width = load_tileset.tile_width;
            let tile_height = load_tileset.tile_height;
            let tiles_per_row = load_tileset_image.width as u32 / tile_width;

            let tileset_image = tileset_image.await.unwrap();
            tileset_image.set_filter(FilterMode::Nearest);

            let mut tileset = Tileset::new(tileset_image);

            for (id, _) in load_tileset.tiles() {
                let draw_params = DrawTextureParams{
                    source: Some(Rect::new((id % tiles_per_row * tile_width) as f32, (id / tiles_per_row * tile_height) as f32, tile_width as f32, tile_height as f32)),
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                    dest_size: None,
                };

                tileset.tiles.insert(id, draw_params);
            }

            tilesets.push(tileset);
        }

        let mut layers = Vec::new();
        
        for load_layer in map.layers() {
            match load_layer.layer_type() {
                LayerType::Tiles(load_tile_layer) => {
                    let mut layer = Layer::new();
                    match load_tile_layer {
                        TileLayer::Finite(finite_tile_layer) => {
                            for y in 0..finite_tile_layer.height() as i32 {
                                for x in 0..finite_tile_layer.width() as i32 {
                                    if let Some(load_tile) = finite_tile_layer.get_tile(x, y) {
                                        let tile = Tile {
                                            x,
                                            y,
                                            tileset: load_tile.tileset_index(),
                                            id: load_tile.id(),
                                        };

                                        layer.tiles.push(tile);
                                    }
                                }
                            }
                        }
                        TileLayer::Infinite(infinite_tile_layer) => {
                            for ((x, y), chunk) in infinite_tile_layer.chunks() {
                                let off_x = x * ChunkData::WIDTH as i32;
                                let off_y = y * ChunkData::HEIGHT as i32;

                                for y in 0..ChunkData::HEIGHT as i32 {
                                    for x in 0..ChunkData::WIDTH as i32 {
                                        if let Some(load_tile) = chunk.get_tile(x, y) {
                                            let tile = Tile {
                                                x: off_x + x,
                                                y: off_y + y,
                                                tileset: load_tile.tileset_index(),
                                                id: load_tile.id(),
                                            };

                                            layer.tiles.push(tile);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    layers.push(layer);
                } // LayerType::Tiles
                _ => ()
            }
        }


        println!("Successfully loaded tilemap.");

        Some(Self {
            tile_width: map.tile_width,
            tile_height: map.tile_height,
            tilesets,
            layers,
        })
    }

    pub fn draw(&self) {
        for layer in &self.layers {
            for tile in &layer.tiles {
                let tileset = &self.tilesets[tile.tileset];
                draw_texture_ex(
                    &tileset.image, 
                    (tile.x * self.tile_width as i32) as f32, 
                    (tile.y * self.tile_height as i32) as f32, 
                    WHITE,
                    tileset.tiles.get(&tile.id).unwrap().clone(),
                );
            }
        }
    }
}
