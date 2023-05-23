use std::env;
use std::fs;
use std::os::raw;
use serde_json::Result;

use crate::commands::TileEntity;
use crate::datatypes::Direction;
use crate::datatypes::Position;
use crate::serialization::SourceMap;
use crate::serialization::TileEntitySerialize;
use crate::serialization::TileSerialize;
use crate::serialization::Tileset;
use crate::serialization::TilesetTile;
use crate::serialization::TilesetTileProperty;

pub fn load() -> Vec<TileEntity> {
    let mut content = fs::read_to_string(String::from("C:\\repos\\swp-rust\\src\\tiles.json")).ok().unwrap();
    let tileset: Tileset = serde_json::from_str(content.as_str()).unwrap();

    content = fs::read_to_string("C:\\repos\\swp-rust\\src\\CanneryRow.json").ok().unwrap();
    let raw_map: SourceMap = serde_json::from_str(content.as_str()).unwrap();
    
    let mut map: Vec<TileEntity> = Vec::new();

    for i in raw_map.layers.iter().enumerate() {
        println!("layer {}, data len: {}", i.0, i.1.data.len());
        map.append(&mut parse(&i.1.data, &tileset));
    }

    map
}

fn parse(data: &[usize], tileset: &Tileset) -> Vec<TileEntity>{
    let mut tiles = Vec::new();

    for x in 0..12 {
        for y in 0..12 {
            println!("x: {}, y: {}", x, y);
            let mut gl_tile_id = data[x * 12 + y];

            let fh = (gl_tile_id & 0x80000000) > 0;
            let fv = (gl_tile_id & 0x40000000) > 0;
            let fd = (gl_tile_id & 0x20000000) > 0;
            let rotated_hex120 = (gl_tile_id & 0x10000000) > 0;

            gl_tile_id &= !(0x80000000 |
                0x40000000 |
                0x20000000 |
                0x10000000);

            if(gl_tile_id == 0) {
                return vec![];
            }

            gl_tile_id -= 1;

            if tileset.tiles[gl_tile_id].properties.is_empty() {
                return vec![];
            }

            for prop in tileset.tiles[gl_tile_id].properties[0].value.iter() {
                tiles.push(TileEntity::from(TileSerialize{entity: prop.clone(), position: Position { x: x as i32, y: y as i32 }, direction: Direction { ordinal: resolve_orientation(fh, fv, fd, matches!(prop, TileEntitySerialize::Wall)) }}));
            }
        }
    }

    tiles
}

fn resolve_orientation(fh:bool, fv:bool, fd:bool, iswall:bool) -> i8{
    let mut walloffset = 0;

    if iswall{
        walloffset += 2;
    }

    if (fh && fv)
    {
        return (2 + walloffset) % 4;
    }
    if (fh && fd)
    {
        return (1 + walloffset) % 4;
    }
    if (fv && fd)
    {
        return (3 + walloffset) % 4;
    }


    walloffset % 4
}