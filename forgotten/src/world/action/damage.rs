use crate::{prelude::*, world::realtime};

const KNOCKBACK: usize = 3;

impl World {
    pub fn melee_attack(&mut self, attacker: Entity, victim: Entity, direction: CardinalDirection) {
        if self.components.player.get(attacker).is_some() {
            self.player_melee_attack(attacker, victim, direction);
        } else if self.components.player.get(victim).is_some() {
            self.npc_melee_attack(attacker, victim);
        }
    }

    pub fn player_melee_attack(&mut self, attacker: Entity, victim: Entity, direction: CardinalDirection) {
        let player = self.components.player.get_mut(attacker).unwrap();
        let remove = if let Some(ammo) = player.melee_weapon.ammo.as_mut() {
            ammo.current = ammo.current.saturating_sub(1);
            ammo.current == 0
        } else {
            false
        };

        // if player.melee_weapon.name == WeaponType::Chainsaw {
        //     println!("Chain Saw Sound");
        //     //     external_events.push(ExternalEvent::SoundEffect(SoundEffect::Chainsaw));
        // } else {
        //     println!("Punch Sound");
        //     //     external_events.push(ExternalEvent::SoundEffect(SoundEffect::Punch));
        // }

        // if let Some(enemy) = self.components.enemy.get(victim) {
        //     message_log.push(Message::PlayerHitEnemy { enemy: *enemy, weapon: player.melee_weapon.name });
        // }

        let pen = player.melee_pen();
        if pen >= self.components.armour.get(victim).expect("npc lacks armour").value {
            let dmg = player.melee_dmg();
            // if player.traits.double_damage {
            //     dmg *= 2;
            // }
            self.damage_character(victim, dmg);
        }

        let player = self.components.player.get_mut(attacker).unwrap();
        for ability in player.melee_weapon.abilities.iter() {
            use WeaponAbility::*;
            match ability {
                KnockBack => {
                    self.components.pushed_from.insert(victim, self.spatial_table.coord_of(victim).unwrap());
                    self.components.realtime.insert(victim, ());
                    self.realtime_components.movement.insert(
                        victim,
                        realtime::movement::spec::Movement {
                            path: direction.coord(),
                            repeat: realtime::movement::spec::Repeat::Steps(KNOCKBACK),
                            cardinal_step_duration: Duration::from_millis(50),
                        }
                        .build(),
                    );
                }
            }
        }

        if remove {
            player.melee_weapon = Weapon::new_bare_hands();
        }
    }

    pub fn npc_melee_attack(&mut self, attacker: Entity, victim: Entity) {}

    pub fn damage_character(&mut self, character: Entity, hit_points_to_lose: u32) {
        if self.components.dead.contains(character) {
            // prevent cascading damage on explosions
            return;
        }

        let hit_points = self.components.hp.get_mut(character).expect("character lacks hit_points");
        if hit_points_to_lose >= hit_points.current {
            hit_points.current = 0;
            self.character_die(character);
        } else {
            hit_points.current -= hit_points_to_lose;
        }
    }

    fn character_die(&mut self, character: Entity) {
        self.components.dead.insert(character, ());
    }
}

// Projectiles
impl World {
    pub fn apply_projectile_damage(
        &mut self,
        projectile_entity: Entity,
        mut projectile_damage: ProjectileDamage,
        projectile_movement_direction: Direction,
        entity_to_damage: Entity,
    ) {
        println!("apply_projectile_damage");
        if let Some(armour) = self.components.armour.get(entity_to_damage).cloned() {
            if let Some(remaining_pen) = projectile_damage.pen.checked_sub(armour.value) {
                if let Some(&enemy) = self.components.enemy.get(entity_to_damage) {
                    if let Some(weapon) = projectile_damage.weapon_name {
                        // message_log.push(Message::PlayerHitEnemy { enemy, weapon });
                    }
                }

                let damage = projectile_damage.hit_points;
                let victim_health =
                    self.components.hp.get(entity_to_damage).map(|hp| hp.current).unwrap_or(0);
                let actual_damage = damage.min(victim_health);
                self.damage_character(entity_to_damage, damage);

                // Get some health back
                if projectile_damage.life_steal {
                    if let Some(player) = self.components.player.entities().next() {
                        if let Some(hit_points) = self.components.hp.get_mut(player) {
                            hit_points.current = (hit_points.current + actual_damage).min(hit_points.max);
                        }
                    }
                }
                // Push them back!
                if projectile_damage.push_back {
                    self.components.realtime.insert(entity_to_damage, ());
                    self.realtime_components.movement.insert(
                        entity_to_damage,
                        realtime::movement::spec::Movement {
                            path: projectile_movement_direction.coord(),
                            repeat: realtime::movement::spec::Repeat::Steps(KNOCKBACK),
                            cardinal_step_duration: Duration::from_millis(100),
                        }
                        .build(),
                    );
                }

                if remaining_pen > 0 {
                    projectile_damage.pen = remaining_pen;
                    self.components.projectile_damage.insert(projectile_entity, projectile_damage);
                } else {
                    self.components.remove_entity(projectile_entity);
                }
            } else {
                self.components.remove_entity(projectile_entity);
            }
        }
    }
}
