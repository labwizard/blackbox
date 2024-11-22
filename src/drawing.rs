use ::ggez::{
    Context,
    GameError,
    GameResult,
    graphics::{
        Canvas,
        Color,
        DrawMode,
        DrawParam,
        FillOptions,
        FillRule,
        Image,
        LineCap,
        Mesh,
        Rect,
        StrokeOptions
    },
    mint::Point2
};

const LINE_WIDTH: f32 = 2.0;

pub struct BitmapFont {
    source: Image,
    pub width: u32,
    pub height: u32,
    chars_per_row: u32,
    rows: u32
}

pub struct BitmapText<'text, 'font> {
    pub text: &'text [u8],
    pub font: &'font BitmapFont
}

impl BitmapFont {
    pub fn from_bytes(
        ctx: &mut Context,
        data: &[u8],
        width: u32,
        height: u32
    ) -> GameResult<BitmapFont> {
        let source = Image::from_bytes(ctx, data)?;
        let chars_per_row = source.width() / width;
        let rows = source.height() / height;
        if chars_per_row < 1 {
            return Err(GameError::CustomError(
                "character width in bitmap font too large".to_string()
            ));
        } else if rows < 1 {
            return Err(GameError::CustomError(
                "character height in bitmap font too large".to_string()
            ));
        }
        return Ok(Self { source, width, height, chars_per_row, rows })
    }

    fn source_rect(&self, ch: u8) -> Option<Rect> {
        let i = ch as u32 % self.chars_per_row;
        let j = ch as u32 / self.chars_per_row;
        if i < self.chars_per_row && j < self.rows {
            Some(Rect {
                x: (i * self.width) as f32 / self.source.width() as f32,
                y: (j * self.height) as f32 / self.source.height() as f32,
                w: self.width as f32 / self.source.width() as f32,
                h: self.height as f32 / self.source.height() as f32
            })
        } else {
            None
        }
    }
}

pub fn draw_bitmap_text(
    canvas: &mut Canvas,
    text: impl AsRef<str>,
    font: &BitmapFont,
    color: Color,
    x: f32,
    y: f32
) {
    for (i, ch) in text.as_ref().as_bytes().iter().cloned().enumerate() {
        if let Some(rect) = font.source_rect(ch) {
            let dest = [x + i as f32 * font.width as f32, y];
            canvas.draw(
                &font.source,
                DrawParam::new().src(rect).dest(dest).color(color)
            );
        }
    }
}

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
