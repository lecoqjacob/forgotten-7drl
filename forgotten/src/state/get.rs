use crate::TurnState;

use super::*;

#[derive(Debug)]
pub struct GetRangedWeapon(pub(crate) Private);

impl GetRangedWeapon {
    pub fn commit(self, scope: &mut StateScope, slot: RangedWeaponSlot) -> GameState {
        scope.0.world.equip_ranged_weapon_from_ground(scope.0.player_entity, slot);
        scope.0.turn_state = TurnState::EnemyTurn;
        GameState::Running(Running(self.0))
    }

    pub const fn cancel(self) -> GameState {
        GameState::Running(Running(self.0))
    }
}

#[derive(Debug)]
pub struct GetMeleeWeapon(pub(crate) Private);

impl GetMeleeWeapon {
    pub fn commit(self, scope: &mut StateScope) -> GameState {
        scope.0.world.equip_melee_weapon_from_ground(scope.0.player_entity);
        scope.0.turn_state = TurnState::EnemyTurn;
        GameState::Running(Running(self.0))
    }

    pub const fn cancel(self) -> GameState {
        GameState::Running(Running(self.0))
    }
}
