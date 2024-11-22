use ::ggez::{
    ContextBuilder,
    GameResult,
    conf::{
        FullscreenType::*,
        WindowMode,
        WindowSetup
    },
    event::run
};

mod character;
mod direction;
mod drawing;
mod exploring;
mod game;
mod level;
mod position;
mod resources;
mod viewing_character;
pub use character::*;
pub use direction::*;
pub use drawing::*;
pub use game::*;
pub use level::*;
pub use position::*;
pub use resources::*;
pub use viewing_character::*;

pub const TITLE: &str = "blackbox";
pub const AUTHOR: &str = "Studio Stardust";

pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new(TITLE, AUTHOR)
        .window_mode(WindowMode {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            fullscreen_type: Windowed,
            .. WindowMode::default()
        })
        .window_setup(WindowSetup {
            title: TITLE.to_string(),
            .. WindowSetup::default()
        })
        .build()?;
    let game = Game::new(&mut ctx)?;
    run(ctx, event_loop, game);
}
