use ::ggez::{
    *,
    graphics::*,
    mint::Point2
};

const LINE_WIDTH: f32 = 2.0;

pub fn draw_rect(
    ctx: &mut Context,
    canvas: &mut Canvas,
    points: &[Point2<f32>],
    fg_color: Color,
    bg_color: Color,
) -> GameResult {
    let stroke_opts = StrokeOptions::DEFAULT
        .with_line_width(LINE_WIDTH)
        .with_line_cap(LineCap::Round);
    let fill_opts = FillOptions::DEFAULT.with_fill_rule(FillRule::NonZero);

    let fill_mesh = Mesh::new_polygon(
        ctx,
        DrawMode::Fill(fill_opts),
        &points,
        bg_color
    )?;
    canvas.draw(&fill_mesh, DrawParam::default());
    let stroke_mesh = Mesh::new_polygon(
        ctx,
        DrawMode::Stroke(stroke_opts),
        &points,
        fg_color
    )?;
    canvas.draw(&stroke_mesh, DrawParam::default());
    Ok(())
}

pub fn rect_points(rect: Rect) -> [Point2<f32>; 4] {
    [
        Point2 { x: rect.left(), y: rect.top() },
        Point2 { x: rect.right(), y: rect.top() },
        Point2 { x: rect.right(), y: rect.bottom() },
        Point2 { x: rect.left(), y: rect.bottom() }
    ]
}

pub fn complement_rects_points(rect: Rect) -> [[Point2<f32>; 4]; 4] {
    const BIG_NUMBER: f32 = 1000000.0;
    let base_rect_points = [
        Point2 { x: -BIG_NUMBER, y: -BIG_NUMBER },
        Point2 { x: BIG_NUMBER, y: -BIG_NUMBER },
        Point2 { x: BIG_NUMBER, y: BIG_NUMBER },
        Point2 { x: -BIG_NUMBER, y: BIG_NUMBER }
    ];
    let mut complement_rects_points = [base_rect_points; 4];
    complement_rects_points[0][1].x = rect.left() - 1.0;
    complement_rects_points[0][2].x = rect.left() - 1.0;
    complement_rects_points[1][2].y = rect.top() - 1.0;
    complement_rects_points[1][3].y = rect.top() - 1.0;
    complement_rects_points[2][0].x = rect.right() + 1.0;
    complement_rects_points[2][3].x = rect.right() + 1.0;
    complement_rects_points[3][0].y = rect.bottom() + 1.0;
    complement_rects_points[3][1].y = rect.bottom() + 1.0;
    complement_rects_points
}
