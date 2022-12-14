use crate::world::explosion;

use super::*;

impl World {
    pub fn projectile_move(&mut self, projectile_entity: Entity, movement_direction: Direction) {
        if let Some(current_coord) = self.spatial_table.coord_of(projectile_entity) {
            let next_coord = current_coord + movement_direction.coord();
            let collides_with =
                self.components.collides_with.get(projectile_entity).cloned().unwrap_or_default();

            if let Some(&spatial_cell) = self.spatial_table.layers_at(next_coord) {
                // Is there a character here?
                if let Some(character_entity) = spatial_cell.character {
                    if let Some(&projectile_damage) = self.components.projectile_damage.get(projectile_entity)
                    {
                        self.apply_projectile_damage(
                            projectile_entity,
                            projectile_damage,
                            movement_direction,
                            character_entity,
                        );
                    }
                }

                // Is there a feature here?
                if let Some(entity_in_cell) = spatial_cell.feature.or(spatial_cell.character) {
                    if (collides_with.solid && self.components.solid.contains(entity_in_cell))
                        || (collides_with.character && self.components.character.contains(entity_in_cell))
                    {
                        if let Some(&projectile_damage) =
                            self.components.projectile_damage.get(projectile_entity)
                        {
                            if self.components.hp.contains(entity_in_cell) {
                                self.apply_projectile_damage(
                                    projectile_entity,
                                    projectile_damage,
                                    movement_direction,
                                    entity_in_cell,
                                );
                            }

                            self.projectile_stop(projectile_entity);
                        }

                        // Slammed against a wall
                        if self.realtime_components.movement.contains(projectile_entity)
                            && self.components.character.contains(projectile_entity)
                        {
                            if let Some(from_coord) = self.components.pushed_from.get(projectile_entity) {
                                let distance = current_coord.manhattan_distance(*from_coord);
                                let dmg = if distance >= 2 { 2 } else { distance };

                                if let Some(npc) = self.components.npc.get_mut(projectile_entity) {
                                    crate::log::append_entry(Message::EnemySlammedIntoWall(npc.npc_type));
                                }

                                self.damage_character(projectile_entity, dmg)
                            }
                        }

                        self.projectile_stop(projectile_entity);
                        return;
                    }
                }

                let _ignore_err = self.spatial_table.update_coord(projectile_entity, next_coord);
            } else {
                self.projectile_stop(projectile_entity);
            }
        } else {
            self.clear_entity(projectile_entity);
        }
    }

    pub fn projectile_stop(&mut self, projectile_entity: Entity) {
        if let Some(current_coord) = self.spatial_table.coord_of(projectile_entity) {
            if let Some(on_collision) = self.components.on_collision.get(projectile_entity) {
                match on_collision {
                    OnCollision::Remove => {
                        self.spatial_table.remove(projectile_entity);
                        self.components.remove_entity(projectile_entity);
                        self.entity_allocator.free(projectile_entity);
                        self.realtime_components.remove_entity(projectile_entity);
                    }
                    OnCollision::RemoveRealtime => {
                        self.realtime_components.remove_entity(projectile_entity);
                        self.components.realtime.remove(projectile_entity);
                        self.components.blocks_gameplay.remove(projectile_entity);
                    }
                    OnCollision::Explode(explosion_spec) => {
                        explosion::explode(self, current_coord, *explosion_spec);
                        self.spatial_table.remove(projectile_entity);
                        self.components.remove_entity(projectile_entity);
                        self.entity_allocator.free(projectile_entity);
                        self.realtime_components.remove_entity(projectile_entity);
                    }
                }
            }
        }

        // TODO: This might cause issues?
        self.components.realtime.remove(projectile_entity);
        self.realtime_components.movement.remove(projectile_entity);
    }

    pub fn alert_nearby_entities(
        &mut self,
        entity: Entity,
        direction: CardinalDirection,
        player_entity: Entity,
    ) {
        let robo_cop_coord = self.entity_coord(entity).unwrap();
        let spatial = &self.spatial_table;
        for (e, npc) in self.components.npc.iter_mut() {
            let current_coord = spatial.coord_of(e).unwrap();
            if current_coord.distance2(robo_cop_coord) <= 100 {
                npc.move_to = Some(spatial.coord_of(player_entity).unwrap());
            } else {
                npc.move_to = None;
            }
        }

        crate::log::append_entry(Message::AlertedNearbyEntities);

        let _ = self.character_walk_in_direction(entity, direction);
    }
}
