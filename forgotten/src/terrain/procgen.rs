use super::builders;
use crate::prelude::*;
use gridbugs::grid_2d::Grid;

const DISTANCE_FOR_STAIRS: u32 = 125;

fn choose_stairs_coord(map: &mut Grid<LevelCell>, player_coord: Coord) {
    let mut possible_stairs = map
        .enumerate()
        .filter(|(_, cell)| **cell == LevelCell::Floor || **cell == LevelCell::CaveFloor)
        .filter_map(
            |(coord, _)| if coord.distance2(player_coord) > DISTANCE_FOR_STAIRS { Some(coord) } else { None },
        )
        .collect::<Vec<_>>();

    crate::rng::shuffle(&mut possible_stairs);
    let stairs_coord = possible_stairs.pop().expect("No stairs spots");
    *map.get_checked_mut(stairs_coord) = LevelCell::Stairs;
}

pub fn generate(size: Size, level: u8) -> Grid<LevelCell> {
    let RoomsAndCorridorsLevel { map: rooms_and_corridors_map, player_spawn } =
        RoomsAndCorridorsLevel::generate(size);
    let cave_map = builders::generate_cave_map(size);
    let mut combined_map = combine_rooms_and_corridors_level_with_cave(&rooms_and_corridors_map, &cave_map);

    *combined_map.get_checked_mut(player_spawn) = LevelCell::PlayerSpawn;

    let mut water_map = make_water_map(size);
    remove_unreachable_floor(&mut combined_map, &mut water_map, player_spawn);
    remove_invalid_doors(&mut combined_map);
    add_grass(&mut combined_map);

    if level != FINAL_LEVEL {
        choose_stairs_coord(&mut combined_map, player_spawn);
    }

    for (coord, cell) in combined_map.enumerate_mut() {
        use LevelCell::*;

        if *water_map.get_checked(coord) {
            match cell {
                Water | PlayerSpawn => (),
                Grass => *cell = Water,
                Floor | Door => *cell = Water,
                CaveFloor | CaveWall => *cell = Water,
                Wall => {
                    if crate::rng::range(0..=100) > 75 {
                        *cell = Water
                    }
                }
                Stairs => todo!(),
            }
        }
    }

    print_map(&combined_map);

    combined_map
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Item generation
//////////////////////////////////////////////////////////////////////////////////////////

pub fn generate_items(
    level: u8,
    world: &mut World,
    terrain_state: &mut TerrainState,
    empty_coords: &mut Vec<Coord>,
) {
    crate::rng::shuffle(empty_coords);

    // for _ in 0..2 {
    //     if let Some(coord) = empty_coords.pop() {
    //         world.spawn_credit(coord, 2);
    //     }
    // }
    // for _ in 0..4 {
    //     if let Some(coord) = empty_coords.pop() {
    //         world.spawn_credit(coord, 1);
    //     }
    // }
    for _ in 0..1 {
        if let Some(coord) = empty_coords.pop() {
            world.spawn_medkit(coord);
        }
    }

    for _ in 0..2 {
        if let Some(coord) = empty_coords.pop() {
            let wpn = terrain_state.ranged_weapons.pop().unwrap();
            world.spawn_weapon(coord, wpn);
        }
    }

    if terrain_state.cattle_prod_floors.contains(&level) {
        if let Some(coord) = empty_coords.pop() {
            world.spawn_weapon(coord, WeaponType::CattleProd);
        }
    }

    if terrain_state.chainsaw_floors.contains(&level) {
        if let Some(coord) = empty_coords.pop() {
            world.spawn_weapon(coord, WeaponType::Chainsaw);
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Npc generation
//////////////////////////////////////////////////////////////////////////////////////////
use crate::{Agent, NpcType, World};
use gridbugs::{entity_table::ComponentTable, spatial_table::Coord};

pub struct EnemyCounts {
    mini: Vec<usize>,
    sec: Vec<usize>,
    sentry: Vec<usize>,
    doom: Vec<usize>,
}

impl EnemyCounts {
    fn new() -> Self {
        Self {
            mini: vec![8, 10, 10, 12, 12],
            sec: vec![2, 2, 4, 6, 6],
            sentry: vec![2, 3, 3, 4, 4],
            doom: vec![0, 0, 1, 2, 4],
        }
    }
}

pub fn generate_npcs(
    level: u8,
    world: &mut World,
    npc_candidates: &mut Vec<Coord>,
    agents: &mut ComponentTable<Agent>,
) {
    crate::rng::shuffle(npc_candidates);

    let index = level as usize - 1;
    let enemy_count = EnemyCounts::new();

    for _ in 0..enemy_count.mini[index] {
        if let Some(coord) = npc_candidates.pop() {
            let mini = world.spawn_minibot(coord);
            agents.insert(mini, Agent::new(world.size(), NpcType::MiniBot));
        }
    }
    for _ in 0..enemy_count.sec[index] {
        if let Some(coord) = npc_candidates.pop() {
            let sec_bot = world.spawn_secbot(coord);
            agents.insert(sec_bot, Agent::new(world.size(), NpcType::SecBot));
        }
    }
    for _ in 0..enemy_count.sentry[index] {
        if let Some(coord) = npc_candidates.pop() {
            println!("Spawning sentry at {:?}", coord);
            // let sentry = world.spawn_sentry(coord);
            // agents.insert(sentry, Agent::new(world.size()));
        }
    }
    for _ in 0..enemy_count.doom[index] {
        if let Some(coord) = npc_candidates.pop() {
            println!("Spawning doom at {:?}", coord);
            // let doom = world.spawn_doom(coord);
            // agents.insert(doom, Agent::new(world.size()));
        }
    }
}