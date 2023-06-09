use crate::commands::TileEntity;
use crate::datatypes::Direction;
use crate::datatypes::Position;
use crate::serialization::SourceMap;
use crate::serialization::TileEntitySerialize;
use crate::serialization::TileSerialize;
use crate::serialization::Tileset;
use std::fs;

pub fn load() -> Vec<TileSerialize> {
    let mut content = fs::read_to_string(String::from(
        "src\\tiles.json",
    ))
    .unwrap();
    let tileset: Tileset = serde_json::from_str(content.as_str()).unwrap();

    content = fs::read_to_string("src\\CanneryRow.json").unwrap();
    let raw_map: SourceMap = serde_json::from_str(content.as_str()).unwrap();

    let mut map: Vec<TileSerialize> = Vec::new();

    for i in raw_map.layers.iter().enumerate() {
        map.append(&mut parse(&i.1.data, &tileset));
    }

    map
}

fn parse(data: &[usize], tileset: &Tileset) -> Vec<TileSerialize> {
    let mut tiles = Vec::new();

    for x in 0..12 {
        for y in 0..12 {
            let mut gl_tile_id = data[(11 - y) * 12 + x];

            let fh = (gl_tile_id & 0x80000000) > 0;
            let fv = (gl_tile_id & 0x40000000) > 0;
            let fd = (gl_tile_id & 0x20000000) > 0;

            gl_tile_id &= !(0x80000000 | 0x40000000 | 0x20000000 | 0x10000000);

            if gl_tile_id == 0 {
                continue;
            }

            gl_tile_id -= 1;

            if tileset.tiles[gl_tile_id].properties.is_empty() {
                continue;
            }

            for prop in tileset.tiles[gl_tile_id].properties[0].value.iter() {
                tiles.push(TileSerialize {
                    entity: prop.clone(),
                    position: Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    direction: Direction {
                        ordinal: resolve_orientation(
                            fh,
                            fv,
                            fd,
                            matches!(prop, TileEntitySerialize::Wall),
                        ),
                    },
                    type_id: gl_tile_id
                });
            }
        }
    }

    tiles
}

fn resolve_orientation(fh: bool, fv: bool, fd: bool, iswall: bool) -> i8 {
    let mut walloffset = 0;

    if iswall {
        walloffset += 2;
    }

    if fh && fv {
        return (2 + walloffset) % 4;
    }
    if fh && fd {
        return (1 + walloffset) % 4;
    }
    if fv && fd {
        return (3 + walloffset) % 4;
    }

    walloffset % 4
}
