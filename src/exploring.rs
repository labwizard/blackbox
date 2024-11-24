use ::ggez::{
    Context,
    GameResult,
    graphics::*,
    input::keyboard::{
        KeyCode,
        KeyInput
    },
    mint::Point2
};
use ::std::time::Duration;
use crate::{
    *,
    ExploreAnimation::*,
    GameState::*,
    ItemSlot::Weapon
};

pub const VIEWPORT_LEFT: f32 = 16.0;
pub const VIEWPORT_TOP: f32 = 16.0;
pub const VIEWPORT_WIDTH: f32 = 400.0;
pub const VIEWPORT_HEIGHT: f32 = 300.0;
pub const VIEWPORT_RIGHT: f32 = VIEWPORT_LEFT + VIEWPORT_WIDTH;
pub const VIEWPORT_BOTTOM: f32 = VIEWPORT_TOP + VIEWPORT_HEIGHT;

pub const INITIAL_WIDTH: f32 = VIEWPORT_WIDTH;
pub const INITIAL_HEIGHT: f32 = VIEWPORT_HEIGHT;
pub const HORIZ_VANISH_RATE: f32 = 0.6;
pub const VERT_VANISH_RATE: f32 = 0.6;
pub const MAX_VANISH_DIST: isize = 6;
pub const FLOOR_INTENSITY: f32 = 0.1;
pub const INTENSITY_VANISH: f32 = 0.8;

pub const PARTYLIST_LEFT: f32 = VIEWPORT_LEFT;
pub const PARTYLIST_TOP: f32 = 340.0;
pub const PARTYLIST_WIDTH: f32 = VIEWPORT_WIDTH;
pub const PARTYLIST_HEIGHT: f32 = 128.0;
pub const PARTYLIST_RIGHT: f32 = PARTYLIST_LEFT + PARTYLIST_WIDTH;
pub const PARTYLIST_BOTTOM: f32 = PARTYLIST_TOP + PARTYLIST_HEIGHT;
pub const PARTYLIST_RECT_POINTS: &[Point2<f32>] = &[
    Point2 { x: PARTYLIST_LEFT - 1.0, y: PARTYLIST_TOP - 1.0 },
    Point2 { x: PARTYLIST_RIGHT + 1.0, y: PARTYLIST_TOP - 1.0 },
    Point2 { x: PARTYLIST_RIGHT + 1.0, y: PARTYLIST_BOTTOM + 1.0, },
    Point2 { x: PARTYLIST_LEFT - 1.0, y: PARTYLIST_BOTTOM + 1.0, }
];

pub const CONTROLS_LEFT: f32 = 432.0;
pub const CONTROLS_TOP: f32 = PARTYLIST_TOP;
pub const CONTROLS_WIDTH: f32 = 192.0;
pub const CONTROLS_HEIGHT: f32 = PARTYLIST_HEIGHT;
pub const CONTROLS_RIGHT: f32 = CONTROLS_LEFT + CONTROLS_WIDTH;
pub const CONTROLS_BOTTOM: f32 = CONTROLS_TOP + CONTROLS_HEIGHT;
pub const CONTROLS_RECT_POINTS: &[Point2<f32>] = &[
    Point2 { x: CONTROLS_LEFT - 1.0, y: CONTROLS_TOP - 1.0 },
    Point2 { x: CONTROLS_RIGHT + 1.0, y: CONTROLS_TOP - 1.0 },
    Point2 { x: CONTROLS_RIGHT + 1.0, y: CONTROLS_BOTTOM + 1.0, },
    Point2 { x: CONTROLS_LEFT - 1.0, y: CONTROLS_BOTTOM + 1.0, }
];

pub const WALL_BASE_POINTS: &[(f32, f32)] = &[
    (0.5, 0.5),
    (-0.5, 0.5),
    (-0.5, -0.5),
    (0.5, -0.5)
];
pub const DOOR_BASE_POINTS: &[(f32, f32)] = &[
    (0.3, 0.5),
    (-0.3, 0.5),
    (-0.3, -0.3),
    (0.3, -0.3)
];
pub const FLOOR_BASE_POINTS: &[(f32, f32)] = &[
    (0.5, 0.5),
    (-0.5, 0.5),
    (-0.5, -0.5),
    (0.5, -0.5)
];

pub const STEP_DURATION: Duration = Duration::from_millis(200);

fn viewport_point(dx: f32, dy: f32, dz: f32) -> Point2<f32> {
    let w = INITIAL_WIDTH * HORIZ_VANISH_RATE.powf(dy);
    let h = INITIAL_HEIGHT * VERT_VANISH_RATE.powf(dy);
    Point2 {
        x: dx * w + VIEWPORT_WIDTH / 2.0 + VIEWPORT_LEFT,
        y: dz * h + VIEWPORT_HEIGHT / 2.0 + VIEWPORT_TOP
    }
}

pub fn key_down_event(
    _ctx: &mut Context,
    input: KeyInput,
    _repeated: bool,
    game: &mut Game
) -> GameResult {
    let (anim, selected) = match &mut game.state {
        Exploring { anim, selected }
            => (anim, selected),
        _   => unimplemented!()
    };
    let (level, pos, dir) = (&mut game.level, &mut game.pos, &mut game.dir);
    finish_anim(pos, dir, anim)?;
    if let Some(i) = selected.as_mut() {
        match input.keycode {
            Some(KeyCode::Return) => game.state = ViewingCharacter {
                i: *i,
                selected: Weapon
            },
            Some(KeyCode::Escape)
                => *selected = None,
            Some(KeyCode::Up)
                => *i = (*i + game.party.len() - 1) % game.party.len(),
            Some(KeyCode::Down)
                => *i = (*i + 1) % game.party.len(),
            _   => {}
        }
    } else {
        match input.keycode {
            Some(KeyCode::Up | KeyCode::W) => {
                if level.wall_towards(*pos, *dir).is_passable() {
                    *anim = Some(StepForward(STEP_DURATION));
                }
            },
            Some(KeyCode::S) => {
                if level.wall_towards(*pos, dir.rev()).is_passable() {
                    *pos = pos.move_by(*dir, -1);
                    *anim = Some(StepBackward(STEP_DURATION));
                }
            },
            Some(KeyCode::A) => {
                if level.wall_towards(*pos, dir.left()).is_passable() {
                    *anim = Some(StepLeft(STEP_DURATION));
                }
            },
            Some(KeyCode::D) => {
                if level.wall_towards(*pos, dir.right()).is_passable() {
                    *anim = Some(StepRight(STEP_DURATION));
                }
            },
            Some(KeyCode::P) => *selected = Some(0),
            Some(KeyCode::I) => {
                game.state = GameState::ViewingInventory {
                    i: 0,
                    requester: Box::new(::std::mem::take(&mut game.state)),
                    condition: Box::new(|_, _| true)
                };
            }
            Some(KeyCode::Down) => *dir = dir.rev(),
            Some(KeyCode::Left) => *dir = dir.left(),
            Some(KeyCode::Right) => *dir = dir.right(),
            _ => {}
        }
    }
    Ok(())
}

fn finish_anim(
    pos: &mut Position,
    dir: &mut Direction,
    anim: &mut Option<ExploreAnimation>
) -> GameResult {
    match anim {
        Some(StepBackward(_)) => {},
        Some(StepForward(_)) => {
            *pos = pos.move_by(*dir, 1);
        },
        Some(StepLeft(_)) => {
            *pos = pos.move_by(dir.left(), 1);
        },
        Some(StepRight(_)) => {
            *pos = pos.move_by(dir.right(), 1);
        },
        _ => {}
    }
    *anim = None;
    Ok(())
}

pub fn update(ctx: &mut Context, game: &mut Game) -> GameResult {
    let anim = match &mut game.state {
        Exploring { anim, .. }
            => anim,
        _   => unimplemented!()
    };
    let (pos, dir) = (&mut game.pos, &mut game.dir);
    match anim {
        Some(
            StepBackward(dur)
            | StepForward(dur)
            | StepLeft(dur)
            | StepRight(dur)
        ) => {
            if ctx.time.delta() >= *dur {
                finish_anim(pos, dir, anim)?;
            } else {
                *dur -= ctx.time.delta();
            }
        },
        _ => {}
    }
    Ok(())
}

fn draw_wall_rect(
    ctx: &mut Context,
    canvas: &mut Canvas,
    base_points: &[(f32, f32)],
    x: f32,
    y: f32,
    front: bool,
    anim: &Option<ExploreAnimation>
) -> GameResult {
    let intensity = INTENSITY_VANISH.powi(y as i32);
    let color = Color::new(intensity, intensity, intensity, 1.0);

    let points = base_points.into_iter().map(|&(du, dz)| {
        let (mut dx, mut dy) = (x, y);
        if front {
            dx += du;
        } else {
            dy += du;
        }
        match anim {
            Some(StepBackward(_) | StepForward(_)) => dy -= 0.5,
            Some(StepLeft(_)) => dx += 0.5,
            Some(StepRight(_)) => dx -= 0.5,
            _ => {}
        };
        viewport_point(dx, dy, dz)
    }).collect::<Vec<_>>();
    draw_rect(ctx, canvas, &points, color, Color::BLACK)
}

fn draw_floor_rect(
    ctx: &mut Context,
    canvas: &mut Canvas,
    base_points: &[(f32, f32)],
    x: f32,
    y: f32,
    dz: f32,
    anim: &Option<ExploreAnimation>
) -> GameResult {
    let intensity = FLOOR_INTENSITY * INTENSITY_VANISH.powi(y as i32);
    let color = Color::new(intensity, intensity, intensity, 1.0);

    let points = base_points.into_iter().map(|&(mut dx, mut dy)| {
        match anim {
            Some(StepBackward(_) | StepForward(_)) => dy -= 0.5,
            Some(StepLeft(_)) => dx += 0.5,
            Some(StepRight(_)) => dx -= 0.5,
            _ => {}
        };
        viewport_point(x + dx, y + dy, dz)
    }).collect::<Vec<_>>();
    draw_rect(ctx, canvas, &points, color, Color::BLACK)
}

fn draw_wall(
    ctx: &mut Context,
    canvas: &mut Canvas,
    wall: Wall,
    x: f32,
    y: f32,
    front: bool,
    anim: &Option<ExploreAnimation>
) -> GameResult {
    let dx = x;
    let dy = y as f32 + 0.5;
    if wall == Wall::Some || wall == Wall::Door {
        draw_wall_rect(ctx, canvas, WALL_BASE_POINTS, dx, dy, front, anim)?;
    }
    if wall == Wall::Door {
        draw_wall_rect(ctx, canvas, DOOR_BASE_POINTS, dx, dy, front, anim)?;
    }
    Ok(())
}

pub fn draw_viewport(
    ctx: &mut Context,
    canvas: &mut Canvas,
    resources: &Resources,
    level: &Level,
    pos: &Position,
    dir: &Direction,
    anim: &Option<ExploreAnimation>
) -> GameResult {
    let mut render_points = Vec::new();
    for x in (-MAX_VANISH_DIST)..=MAX_VANISH_DIST {
        for y in 0..=MAX_VANISH_DIST {
            render_points.push((x, y));
        }
    }
    render_points.sort_by(|&(x1, y1), &(x2, y2)| {
        let d1 = (x1 as f32).powi(2) + (y1 as f32 + 0.5).powi(2);
        let d2 = (x2 as f32).powi(2) + (y2 as f32 + 0.5).powi(2);
        d2.partial_cmp(&d1).unwrap() // reversed!
    });
    for &(x, y) in render_points.iter() {
        draw_floor_rect(
            ctx, canvas,
            FLOOR_BASE_POINTS,
            x as f32, y as f32 - 0.5,
            0.5,
            anim
        )?;

        let wall_pos = pos.translate(*dir, x, y);
        draw_wall(
            ctx, canvas,
            level.wall_towards(wall_pos, *dir),
            x as f32, y as f32 + 0.5,
            true,
            anim
        )?;
        draw_wall(
            ctx, canvas,
            level.wall_towards(wall_pos, dir.left()),
            x as f32 - 0.5, y as f32,
            false,
            anim
        )?;
        draw_wall(
            ctx, canvas,
            level.wall_towards(wall_pos, dir.right()),
            x as f32 + 0.5, y as f32,
            false,
            anim
        )?;
    }

    canvas.draw(&resources.viewport_mask, DrawParam::default());
    Ok(())
}

pub fn draw_partylist(
    ctx: &mut Context,
    canvas: &mut Canvas,
    resources: &Resources,
    party: &Vec<Character>,
    _anim: &Option<ExploreAnimation>,
    selected: Option<usize>
) -> GameResult {
    draw_rect(
        ctx, canvas,
        PARTYLIST_RECT_POINTS,
        Color::WHITE,
        Color::BLACK
    )?;

    draw_bitmap_text(
        canvas,
        "# NAME      HP  MP",
        &resources.font_bold,
        Color::WHITE,
        VIEWPORT_LEFT + 8.0,
        PARTYLIST_TOP + 8.0
    );
    for (i, character) in party.iter().enumerate() {
        let x = VIEWPORT_LEFT + 8.0;
        let y = PARTYLIST_TOP + 8.0 + 16.0 * (i + 1) as f32;
        if selected == Some(i) {
            draw_bitmap_text(
                canvas,
                ">",
                &resources.font_bold,
                Color::WHITE,
                x, y
            );
        } else {
            draw_bitmap_text(
                canvas,
                (i + 1).to_string(),
                &resources.font_regular,
                Color::WHITE,
                x, y
            );
        }
        let line = format!(
            "{:9} {:<3} {:<3}",
            character.name,
            character.hp,
            character.mp
        );
        draw_bitmap_text(
            canvas,
            &line,
            &resources.font_regular,
            Color::WHITE,
            x + 32.0, y
        );
    }
    Ok(())
}

pub fn draw_controls(
    ctx: &mut Context,
    canvas: &mut Canvas,
    resources: &Resources,
    controls: &[(&str, &str)]
) -> GameResult {
    draw_rect(
        ctx, canvas,
        CONTROLS_RECT_POINTS,
        Color::WHITE,
        Color::BLACK
    )?;

    for (i, (key, line)) in controls.iter().enumerate() {
        let x1 = CONTROLS_LEFT + 8.0;
        let x2 = x1 + 64.0;
        let y = CONTROLS_TOP + 8.0 + 16.0 * i as f32;
        draw_bitmap_text(
            canvas,
            key,
            &resources.font_bold,
            Color::WHITE,
            x1, y
        );
        draw_bitmap_text(
            canvas,
            line,
            &resources.font_regular,
            Color::WHITE,
            x2, y
        )
    }
    Ok(())
}

pub fn draw(ctx: &mut Context, game: &Game) -> GameResult {
    let (anim, selected) = match &game.state {
        Exploring { anim, selected }
            => (anim, selected),
        _   => unimplemented!()
    };

    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.set_sampler(Sampler::nearest_clamp());

    draw_viewport(
        ctx, &mut canvas,
        &game.resources,
        &game.level, &game.pos, &game.dir,
        anim
    )?;
    draw_partylist(
        ctx, &mut canvas,
        &game.resources,
        &game.party,
        anim,
        *selected
    )?;

    if selected.is_some() {
        draw_controls(
            ctx, &mut canvas,
            &game.resources,
            &[("[\n]", "DETAILS"), ("[\x1b]", "BACK")]
        )?;
    } else {
        draw_controls(
            ctx, &mut canvas,
            &game.resources,
            &[("[P]", "PARTY"), ("[I]", "ITEMS")]
        )?;
    }

    canvas.finish(ctx)
}
