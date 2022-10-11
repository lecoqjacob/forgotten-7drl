use super::explosion;
use crate::{prelude::*, world::realtime};
use gridbugs::{
    coord_2d::Coord,
    entity_table::{entity_data, Entity},
    visible_area_detection::{vision_distance, Light, Rational},
};
use vector::Radians;

impl World {
    /// Helper method to spawn an entity at a location
    fn spawn_entity<L: Into<Location>>(&mut self, location: L, entity_data: EntityData) -> Entity {
        let entity = self.entity_allocator.alloc();
        let location @ Location { layer, coord } = location.into();
        if let Err(e) = self.spatial_table.update(entity, location) {
            panic!("{:?}: There is already a {:?} at {:?}", e, layer, coord);
        }
        self.components.insert_entity_data(entity, entity_data);
        entity
    }

    // Terrain

    pub fn spawn_wall(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::Wall,
                solid: (),
                opacity: 255,
            },
        );
    }

    pub fn spawn_cave_wall(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::CaveWall,
                solid: (),
                opacity: 255,
            },
        );
    }

    pub fn spawn_door(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::DoorClosed,
                door_state: DoorState::Closed,
                solid: (),
                opacity: 255,
            },
        );
    }

    pub fn spawn_floor(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Floor),
            entity_data! {
                tile: Tile::Floor,
            },
        );
    }

    pub fn spawn_cave_floor(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Floor),
            entity_data! {
                tile: Tile::CaveFloor,
            },
        );
    }

    pub fn spawn_grass(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Feature),
            entity_data! {
                tile: Tile::Grass,
                opacity: 128,
                grass_state: GrassState::Normal,
            },
        );
    }

    pub fn spawn_water(&mut self, coord: Coord) {
        self.spawn_entity(
            (coord, Layer::Floor),
            entity_data! {
                tile: Tile::Water,
            },
        );
    }

    // Entities

    pub fn spawn_player(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                character: (),
                tile: Tile::Player,
                player: Player::new(),
                hp: HitPoints::new_full(100),
                vision: vision_distance::Circle::new(200),
                light: Light {
                    colour: Rgb24::new_grey(200),
                    vision_distance: vision_distance::Circle::new_squared(200),
                    diminish: Rational {numerator: 1, denominator: 8},
                },
            },
        )
    }

    pub fn spawn_minibot(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 1,
                character: (),
                armour: Armour::new(1),
                hp: HitPoints::new_full(4),
                tile: Tile::Npc(NpcType::MiniBot),
                npc: Npc { disposition: Disposition::Hostile, npc_type: NpcType::MiniBot },
            },
        )
    }

    pub fn spawn_secbot(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 2,
                character: (),
                armour: Armour::new(3),
                hp: HitPoints::new_full(5),
                tile: Tile::Npc(NpcType::SecBot),
                npc: Npc { disposition: Disposition::Hostile, npc_type: NpcType::SecBot },
            },
        )
    }

    pub fn spawn_robocop(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 3,
                character: (),
                armour: Armour::new(5),
                hp: HitPoints::new_full(10),
                tile: Tile::Npc(NpcType::RoboCop),
                npc: Npc { disposition: Disposition::Hostile, npc_type: NpcType::RoboCop },
            },
        )
    }

    pub fn spawn_doombot(&mut self, coord: Coord) -> Entity {
        self.spawn_entity(
            (coord, Layer::Character),
            entity_data! {
                damage: 5,
                character: (),
                armour: Armour::new(10),
                hp: HitPoints::new_full(30),
                tile: Tile::Npc(NpcType::DoomBot),
                npc: Npc { disposition: Disposition::Hostile, npc_type: NpcType::DoomBot },
            },
        )
    }

    // Items
    pub fn spawn_weapon(&mut self, coord: Coord, ranged_weapon: WeaponType) {
        self.spawn_entity(
            (coord, Layer::Item),
            entity_data! {
                tile:  ranged_weapon.tile(),
                item: Item::Weapon(ranged_weapon),
                weapon: ranged_weapon.new_weapon(),
            },
        );
    }

    // Effects
    pub fn spawn_flash(&mut self, coord: Coord, colour: Option<Rgb24>) -> Entity {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord, layer: None }).unwrap();

        self.components.insert_entity_data(
            entity,
            entity_data!(
                realtime: (),
                light: Light {
                    colour: colour.unwrap_or(Rgb24::new_grey(100)),
                    vision_distance: vision_distance::Circle::new_squared(90),
                    diminish: Rational {numerator: 1, denominator: 20},
                },
            ),
        );
        self.realtime_components.fade.insert(entity, FadeState::new(Duration::from_millis(100)));

        entity
    }

    pub fn spawn_explosion_emitter(
        &mut self,
        coord: Coord,
        spec: &explosion::spec::ParticleEmitter,
    ) -> Entity {
        let emitter_entity = self.entity_allocator.alloc();
        self.spatial_table.update(emitter_entity, Location { coord, layer: None }).unwrap();

        self.components.insert_entity_data(
            emitter_entity,
            entity_data!(realtime: (), light: Light {
                colour: Rgb24::new(255, 187, 63),
                diminish: Rational {numerator: 1, denominator: 100},
                vision_distance: vision_distance::Circle::new_squared(420),
            }),
        );

        self.realtime_components.fade.insert(emitter_entity, FadeState::new(spec.duration));
        self.realtime_components.particle_emitter.insert(emitter_entity, {
            use crate::world::realtime::particle::spec::*;
            ParticleEmitter {
                emit_particle_every_period: realtime::period_per_frame(spec.num_particles_per_frame),
                fade_out_duration: Some(spec.duration),
                particle: Particle {
                    tile: None,
                    movement: Some(Movement {
                        angle_range: Radians::uniform_range_all(),
                        cardinal_period_range: UniformInclusiveRange {
                            low: spec.min_step,
                            high: spec.max_step,
                        },
                    }),
                    fade_duration: Some(spec.fade_duration),
                    colour_hint: Some(UniformInclusiveRange {
                        low: Rgb24::new(255, 17, 0),
                        high: Rgb24::new(255, 255, 63),
                    }),
                    possible_particle_emitter: Some(Possible {
                        chance: rational::Rational { numerator: 1, denominator: 20 },
                        value: Box::new(ParticleEmitter {
                            emit_particle_every_period: spec.min_step,
                            fade_out_duration: None,
                            particle: Particle {
                                tile: None,
                                movement: Some(Movement {
                                    angle_range: Radians::uniform_range_all(),
                                    cardinal_period_range: UniformInclusiveRange {
                                        low: Duration::from_millis(200),
                                        high: Duration::from_millis(500),
                                    },
                                }),
                                fade_duration: Some(Duration::from_millis(1000)),
                                ..Default::default()
                            },
                        }),
                    }),
                    ..Default::default()
                },
            }
            .build()
        });

        self.realtime_components.light_colour_fade.insert(
            emitter_entity,
            realtime::light_colour_fade::LightColourFadeState {
                fade_state: realtime::fade::FadeState::new(spec.fade_duration),
                from: Rgb24::new(255, 187, 63),
                to: Rgb24::new(0, 0, 0),
            },
        );

        emitter_entity
    }

    pub fn spawn_bullet(&mut self, start: Coord, target: Coord, weapon: &Weapon) {
        let entity = self.entity_allocator.alloc();
        self.spatial_table.update(entity, Location { coord: start, layer: None }).unwrap();

        self.components.insert_entity_data(
            entity,
            entity_data!(
                realtime: (),
                on_collision: weapon.on_collision.unwrap_or_default(),
                collides_with: CollidesWith {
                    solid: true,
                    character: false,
                },
                tile: Tile::Bullet,
                projectile_damage: ProjectileDamage {
                    hit_points: weapon.dmg,
                    push_back: weapon
                        .abilities
                        .iter()
                        .any(|a| *a ==WeaponAbility::KnockBack),
                    pen: weapon.pen,
                    hull_pen_percent: weapon.hull_pen_percent,
                    life_steal: weapon
                        .abilities
                        .iter()
                        .any(|a| *a == WeaponAbility::LifeSteal),
                    weapon_name: Some(weapon.name),
                },
            ),
        );

        self.realtime_components.movement.insert(
            entity,
            realtime::movement::spec::Movement {
                path: target - start,
                cardinal_step_duration: Duration::from_millis(25),
                repeat: realtime::movement::spec::Repeat::Once,
            }
            .build(),
        );

        let particle_emitter_ = if let Some(light_colour) = weapon.light_colour {
            use realtime::particle::spec::*;

            if weapon.bright {
                ParticleEmitter {
                    emit_particle_every_period: Duration::from_millis(8),
                    fade_out_duration: None,
                    particle: Particle {
                        tile: None,
                        movement: None,
                        fade_duration: Some(Duration::from_millis(200)),
                        possible_light: Some(Possible {
                            chance: rational::Rational { numerator: 1, denominator: 1 },
                            value: Light {
                                colour: light_colour,
                                vision_distance: vision_distance::Circle::new_squared(50),
                                diminish: Rational { numerator: 10, denominator: 1 },
                            },
                        }),
                        ..Default::default()
                    },
                }
            } else {
                ParticleEmitter {
                    emit_particle_every_period: Duration::from_millis(1),
                    fade_out_duration: None,
                    particle: Particle {
                        tile: None,
                        movement: None,
                        fade_duration: Some(Duration::from_millis(100)),
                        possible_light: Some(Possible {
                            chance: rational::Rational { numerator: 1, denominator: 1 },
                            value: Light {
                                colour: light_colour,
                                vision_distance: vision_distance::Circle::new_squared(7),
                                diminish: Rational { numerator: 100, denominator: 1 },
                            },
                        }),
                        ..Default::default()
                    },
                }
            }
        } else {
            use realtime::particle::spec::*;

            ParticleEmitter {
                emit_particle_every_period: Duration::from_micros(2000),
                fade_out_duration: None,
                particle: Particle {
                    tile: None,
                    movement: Some(Movement {
                        angle_range: Radians::uniform_range_all(),
                        cardinal_period_range: UniformInclusiveRange {
                            low: Duration::from_millis(200),
                            high: Duration::from_millis(500),
                        },
                    }),
                    fade_duration: Some(Duration::from_millis(1000)),
                    possible_light: None,
                    ..Default::default()
                },
            }
        }
        .build();

        self.realtime_components.particle_emitter.insert(entity, particle_emitter_);
    }
}
