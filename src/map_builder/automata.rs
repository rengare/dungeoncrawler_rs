use pipe_trait::Pipe;

use super::MapArchitect;
use crate::prelude::*;

pub struct CelluarAutomataArchitect {}

impl CelluarAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            rng.range(0, 100).pipe(|roll| {
                if roll > 55 {
                    *t = TileType::Floor;
                } else {
                    *t = TileType::Wall;
                }
            })
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        map.tiles = map.tiles.clone().pipe(|mut new_tiles| {
            for y in 1..SCREEN_HEIGHT - 1 {
                for x in 1..SCREEN_WIDTH - 1 {
                    let neighbors = self.count_neighbors(x, y, map);
                    let idx = map_idx(x, y);
                    if neighbors > 4 || neighbors == 0 {
                        new_tiles[idx] = TileType::Wall;
                    } else {
                        new_tiles[idx] = TileType::Floor;
                    }
                }
            }
            new_tiles
        })
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tileType)| **tileType == TileType::Floor)
            .map(|(index, _)| {
                (
                    index,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(index)),
                )
            })
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())
            .map(|(index, _)| index)
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for CelluarAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        }
        .pipe(|mb| mb)

        // self.random_noise_map(rng, &mut mb.map);
        // for _ in 0..10 {
        //     self.iteration(&mut mb.map);
        // }

        // let start = self.find_start(&mb.map);
        // mb.monster_spawns = mb.spawn_monsters(&start, rng);
        // mb.player_start = start;
        // mb.amulet_start = mb.find_most_distant();
    }
}
