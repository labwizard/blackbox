use ::ggez::{
    Context,
    GameResult,
    event::EventHandler,
    input::keyboard::KeyInput
};
use ::std::time::Duration;
use crate::*;
use GameState::*;

pub const VIEWPORT_WIDTH: f32 = 400.0;
pub const VIEWPORT_HEIGHT: f32 = 300.0;
pub const VIEWPORT_LEFT: f32 = 16.0;
pub const VIEWPORT_TOP: f32 = 16.0;
pub const INITIAL_WIDTH: f32 = 400.0;
pub const INITIAL_HEIGHT: f32 = 300.0;
pub const HORIZ_VANISH_RATE: f32 = 0.6;
pub const VERT_VANISH_RATE: f32 = 0.6;
pub const MAX_VANISH_DIST: isize = 5;
pub const LINE_WIDTH: f32 = 2.0;
pub const LINE_VANISH: f32 = 0.8;

pub struct Game {
    pub resources: Resources,
    pub state: GameState
}

pub enum GameState {
    Exploring {
        level: Level,
        pos: Position,
        dir: Direction,
        anim: Option<ExploreAnimation>
    }
}

pub enum ExploreAnimation {
    StepBackward(Duration),
    StepForward(Duration)
}

impl EventHandler for Game {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        repeated: bool,
    ) -> GameResult {
        match &mut self.state {
            Exploring { level, pos, dir, anim } => {
                exploring::key_down_event(
                    ctx, input, repeated,
                    level, pos, dir, anim
                )
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.state {
            Exploring { level, pos, dir, anim } => {
                exploring::update(
                    ctx,
                    level, pos, dir, anim
                )
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.state {
            Exploring { level, pos, dir, anim } => {
                exploring::draw(
                    ctx,
                    &self.resources,
                    level, pos, dir, anim
                )
            }
        }
    }
}
