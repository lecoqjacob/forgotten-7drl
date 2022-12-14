use crate::prelude::*;
use gridbugs::{
    entity_table_realtime::{RealtimeComponent, RealtimeComponentApplyEvent},
    line_2d::{InfiniteStepIter, StepIter},
};

pub mod spec {
    pub use gridbugs::coord_2d::Coord;
    pub use std::time::Duration;

    pub enum Repeat {
        Once,
        Forever,
        Steps(usize),
    }

    pub struct Movement {
        pub path: Coord,
        pub repeat: Repeat,
        pub cardinal_step_duration: Duration,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Path {
    Once(StepIter),
    Forever(InfiniteStepIter),
    Steps { infinite_step_iter: InfiniteStepIter, remaining_steps: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementState {
    path: Path,
    cardinal_step_duration: Duration,
    ordinal_step_duration: Duration,
}

const fn ordinal_duration_from_cardinal_duration(duration: Duration) -> Duration {
    const SQRT_2_X_1_000_000: u64 = 1_414_214;
    let ordinal_micros = (duration.as_micros() as u64 * SQRT_2_X_1_000_000) / 1_000_000;
    Duration::from_micros(ordinal_micros)
}

impl spec::Movement {
    pub fn build(self) -> MovementState {
        MovementState {
            cardinal_step_duration: self.cardinal_step_duration,
            ordinal_step_duration: ordinal_duration_from_cardinal_duration(self.cardinal_step_duration),
            path: match self.repeat {
                spec::Repeat::Forever => Path::Forever(InfiniteStepIter::new(self.path)),
                spec::Repeat::Once => Path::Once(StepIter::new(self.path)),
                spec::Repeat::Steps(n) => {
                    Path::Steps { infinite_step_iter: InfiniteStepIter::new(self.path), remaining_steps: n }
                }
            },
        }
    }
}

impl MovementState {
    pub const fn cardinal_step_duration(&self) -> Duration {
        self.cardinal_step_duration
    }
}

impl RealtimeComponent for MovementState {
    type Event = Option<Direction>;

    fn tick(&mut self) -> (Self::Event, Duration) {
        let event = match self.path {
            Path::Forever(ref mut path) => path.next(),
            Path::Once(ref mut path) => path.next(),
            Path::Steps { ref mut infinite_step_iter, ref mut remaining_steps } => {
                remaining_steps.checked_sub(1).and_then(|next_remaining_steps| {
                    *remaining_steps = next_remaining_steps;
                    infinite_step_iter.next()
                })
            }
        };

        let until_next_event = if let Some(direction) = event {
            if direction.is_cardinal() {
                self.cardinal_step_duration
            } else {
                self.ordinal_step_duration
            }
        } else {
            self.cardinal_step_duration
        };

        (event, until_next_event)
    }
}

impl<'a> RealtimeComponentApplyEvent<Context<'a>> for MovementState {
    fn apply_event(event: Option<Direction>, entity: Entity, context: &mut Context<'a>) {
        match event {
            None => context.world.projectile_stop(entity),
            Some(direction) => context.world.projectile_move(entity, direction),
        }
    }
}
