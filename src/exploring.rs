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
    ExploreAnimation::*
};

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
pub const FLOOR_INTENSITY: f32 = 0.1;
pub const LINE_VANISH: f32 = 0.8;

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
    level: &mut Level,
    pos: &mut Position,
    dir: &mut Direction,
    anim: &mut Option<ExploreAnimation>
) -> GameResult {
    finish_anim(pos, dir, anim)?;
    match input.keycode {
        Some(KeyCode::Up) => {
            if level.wall_towards(*pos, *dir).is_passable() {
                *anim = Some(StepForward(STEP_DURATION));
            }
        },
        Some(KeyCode::Down) => {
            if level.wall_towards(*pos, dir.rev()).is_passable() {
                *pos = pos.move_by(*dir, -1);
                *anim = Some(StepBackward(STEP_DURATION));
            }
        },
        Some(KeyCode::Left) => *dir = dir.left(),
        Some(KeyCode::Right) => *dir = dir.right(),
        _ => {}
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
        _ => {}
    }
    *anim = None;
    Ok(())
}

pub fn update(
    ctx: &mut Context,
    _level: &mut Level,
    pos: &mut Position,
    dir: &mut Direction,
    anim: &mut Option<ExploreAnimation>
) -> GameResult {
    match anim {
        Some(StepBackward(dur) | StepForward(dur)) => {
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

fn draw_rect(
    ctx: &mut Context,
    canvas: &mut Canvas,
    points: &[Point2<f32>],
    color: Color,
    z: i32
) -> GameResult {
    let stroke_opts = StrokeOptions::DEFAULT
        .with_line_width(LINE_WIDTH)
        .with_line_cap(LineCap::Round);
    let fill_opts = FillOptions::DEFAULT.with_fill_rule(FillRule::NonZero);

    let fill_mesh = Mesh::new_polygon(
        ctx,
        DrawMode::Fill(fill_opts),
        &points,
        Color::BLACK
    )?;
    canvas.draw(&fill_mesh, DrawParam::default().z(z));
    let stroke_mesh = Mesh::new_polygon(
        ctx,
        DrawMode::Stroke(stroke_opts),
        &points,
        color
    )?;
    canvas.draw(&stroke_mesh, DrawParam::default().z(z));
    Ok(())
}

fn draw_wall_rect(
    ctx: &mut Context,
    canvas: &mut Canvas,
    base_points: &[(f32, f32)],
    x: f32,
    y: f32,
    front: bool,
    anim: &mut Option<ExploreAnimation>
) -> GameResult {
    let intensity = LINE_VANISH.powi(y as i32);
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
            _ => {}
        };
        viewport_point(dx, dy, dz)
    }).collect::<Vec<_>>();
    draw_rect(ctx, canvas, &points, color, -1)
}

fn draw_floor_rect(
    ctx: &mut Context,
    canvas: &mut Canvas,
    base_points: &[(f32, f32)],
    x: f32,
    y: f32,
    dz: f32,
    anim: &mut Option<ExploreAnimation>
) -> GameResult {
    let intensity = FLOOR_INTENSITY * LINE_VANISH.powi(y as i32);
    let color = Color::new(intensity, intensity, intensity, 1.0);

    let points = base_points.into_iter().map(|&(dx, mut dy)| {
        match anim {
            Some(StepBackward(_) | StepForward(_)) => dy -= 0.5,
            _ => {}
        };
        viewport_point(x + dx, y + dy, dz)
    }).collect::<Vec<_>>();
    draw_rect(ctx, canvas, &points, color, -2)
}

fn draw_wall(
    ctx: &mut Context,
    canvas: &mut Canvas,
    wall: Wall,
    x: f32,
    y: f32,
    front: bool,
    anim: &mut Option<ExploreAnimation>
) -> GameResult {
    if wall == Wall::Some || wall == Wall::Door {
        draw_wall_rect(
            ctx, canvas,
            WALL_BASE_POINTS,
            x as f32, y as f32 + 0.5,
            front,
            anim
        )?;
    }
    if wall == Wall::Door {
        draw_wall_rect(
            ctx, canvas,
            DOOR_BASE_POINTS,
            x as f32, y as f32 + 0.5,
            front,
            anim
        )?;
    }
    Ok(())
}

pub fn draw(
    ctx: &mut Context,
    resources: &Resources,
    level: &mut Level,
    pos: &mut Position,
    dir: &mut Direction,
    anim: &mut Option<ExploreAnimation>
) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.set_sampler(Sampler::nearest_clamp());

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
            ctx, &mut canvas,
            FLOOR_BASE_POINTS,
            x as f32, y as f32 - 0.5,
            0.5,
            anim
        )?;

        let wall_pos = pos.translate(*dir, x, y);
        draw_wall(
            ctx, &mut canvas,
            level.wall_towards(wall_pos, *dir),
            x as f32, y as f32 + 0.5,
            true,
            anim
        )?;
        draw_wall(
            ctx, &mut canvas,
            level.wall_towards(wall_pos, dir.left()),
            x as f32 - 0.5, y as f32,
            false,
            anim
        )?;
        draw_wall(
            ctx, &mut canvas,
            level.wall_towards(wall_pos, dir.right()),
            x as f32 + 0.5, y as f32,
            false,
            anim
        )?;
    }
    canvas.draw(&resources.game_frame, DrawParam::default());
    canvas.finish(ctx)
}
