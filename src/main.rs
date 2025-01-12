use ::ggez::{
    ContextBuilder,
    GameResult,
    conf::{
        FullscreenType::*,
        WindowMode,
        WindowSetup
    },
    event::run,
    graphics::Color
};

pub mod drawing;
mod game;
pub mod model;
mod resources;
pub mod scene;

pub use game::*;
pub use resources::*;

pub const TITLE: &str = "blackbox";
pub const AUTHOR: &str = "Studio Stardust";

pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;

pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new(TITLE, AUTHOR)
        .window_mode(WindowMode {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            fullscreen_type: Windowed,
            resizable: false,
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
