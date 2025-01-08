pub const GRASS: usize = 1;
pub const FARM: usize = 2;
pub const LAKE: usize = 3;
pub const FOREST: usize = 4;
pub const SWAMP: usize = 5;
pub const MINE: usize = 6;
pub const CASTLE: usize = 7;

pub struct Square {
    pub biome: usize,
    pub crowns: usize,
}

pub const DOMINOS: [[Square; 2]; 48] = [
    [Square {biome: FARM, crowns: 0}, Square {biome: FARM, crowns: 0}],
    [Square {biome: FARM, crowns: 0}, Square {biome: FARM, crowns: 0}],
    [Square {biome: FOREST, crowns: 0}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: FOREST, crowns: 0}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: FOREST, crowns: 0}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: FOREST, crowns: 0}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: LAKE, crowns: 0}, Square {biome: LAKE, crowns: 0}],
    [Square {biome: LAKE, crowns: 0}, Square {biome: LAKE, crowns: 0}],
    [Square {biome: LAKE, crowns: 0}, Square {biome: LAKE, crowns: 0}],
    [Square {biome: GRASS, crowns: 0}, Square {biome: GRASS, crowns: 0}],
    [Square {biome: GRASS, crowns: 0}, Square {biome: GRASS, crowns: 0}],
    [Square {biome: SWAMP, crowns: 0}, Square {biome: SWAMP, crowns: 0}],
    [Square {biome: FARM, crowns: 0}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: FARM, crowns: 0}, Square {biome: LAKE, crowns: 0}],
    [Square {biome: FARM, crowns: 0}, Square {biome: GRASS, crowns: 0}],
    [Square {biome: FARM, crowns: 0}, Square {biome: SWAMP, crowns: 0}],
    [Square {biome: FOREST, crowns: 0}, Square {biome: LAKE, crowns: 0}],
    [Square {biome: FOREST, crowns: 0}, Square {biome: GRASS, crowns: 0}],
    [Square {biome: FARM, crowns: 1}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: FARM, crowns: 1}, Square {biome: LAKE, crowns: 0}],
    [Square {biome: FARM, crowns: 1}, Square {biome: GRASS, crowns: 0}],
    [Square {biome: FARM, crowns: 1}, Square {biome: SWAMP, crowns: 0}],
    [Square {biome: FARM, crowns: 1}, Square {biome: MINE, crowns: 0}],
    [Square {biome: FOREST, crowns: 1}, Square {biome: FARM, crowns: 0}],
    [Square {biome: FOREST, crowns: 1}, Square {biome: FARM, crowns: 0}],
    [Square {biome: FOREST, crowns: 1}, Square {biome: FARM, crowns: 0}],
    [Square {biome: FOREST, crowns: 1}, Square {biome: FARM, crowns: 0}],
    [Square {biome: FOREST, crowns: 1}, Square {biome: LAKE, crowns: 0}],
    [Square {biome: FOREST, crowns: 1}, Square {biome: GRASS, crowns: 0}],
    [Square {biome: LAKE, crowns: 1}, Square {biome: FARM, crowns: 0}],
    [Square {biome: LAKE, crowns: 1}, Square {biome: FARM, crowns: 0}],
    [Square {biome: LAKE, crowns: 1}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: LAKE, crowns: 1}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: LAKE, crowns: 1}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: LAKE, crowns: 1}, Square {biome: FOREST, crowns: 0}],
    [Square {biome: FARM, crowns: 0}, Square {biome: GRASS, crowns: 1}],
    [Square {biome: LAKE, crowns: 0}, Square {biome: GRASS, crowns: 1}],
    [Square {biome: FARM, crowns: 0}, Square {biome: SWAMP, crowns: 1}],
    [Square {biome: GRASS, crowns: 0}, Square {biome: SWAMP, crowns: 1}],
    [Square {biome: MINE, crowns: 1}, Square {biome: FARM, crowns: 0}],
    [Square {biome: FARM, crowns: 0}, Square {biome: GRASS, crowns: 2}],
    [Square {biome: LAKE, crowns: 0}, Square {biome: GRASS, crowns: 2}],
    [Square {biome: FARM, crowns: 0}, Square {biome: SWAMP, crowns: 2}],
    [Square {biome: GRASS, crowns: 0}, Square {biome: SWAMP, crowns: 2}],
    [Square {biome: MINE, crowns: 2}, Square {biome: FARM, crowns: 0}],
    [Square {biome: SWAMP, crowns: 0}, Square {biome: MINE, crowns: 2}],
    [Square {biome: SWAMP, crowns: 0}, Square {biome: MINE, crowns: 2}],
    [Square {biome: FARM, crowns: 0}, Square {biome: MINE, crowns: 3}],
];