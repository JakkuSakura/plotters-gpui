use gpui::{point, px, Hsla, Path, Pixels, Point, WindowContext};
use tracing::warn;

#[derive(Clone, Debug)]
pub struct Line {
    pub points: Vec<Point<Pixels>>,
    pub width: Pixels,
    pub color: Hsla,
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

impl Line {
    pub fn new() -> Self {
        Self {
            points: vec![],
            width: 1.0.into(),
            color: gpui::black(),
        }
    }

    pub fn between_points(start: Point<Pixels>, end: Point<Pixels>) -> Self {
        let mut line = Self::new();
        line.add_point(start);
        line.add_point(end);
        line
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = color.into();
        self
    }

    pub fn add_point(&mut self, point: Point<Pixels>) {
        self.points.push(point);
    }

    pub fn render_pixels(&mut self, cx: &mut WindowContext) {
        if self.points.is_empty() {
            warn!("Line must have at least 1 points to render");
            return;
        }
      
        let stroke = tiny_skia::Stroke {
            width: self.width.0,
            ..Default::default()
        };
        let mut builder = tiny_skia::PathBuilder::new();
        let Some(first_p) = self.points.first() else {
            return;
        };


        builder.move_to(first_p.x.0, first_p.y.0);
        for p in self.points.iter().skip(1) {
            builder.line_to(p.x.0, p.y.0);
        }
        let Some(path) = stroke_path(builder, &stroke, cx) else {
            return;
        };
        cx.paint_path(path, self.color);
    }
}

/// Build tiny-skia PathBuilder into a Path with stroke
fn stroke_path(
    builder: tiny_skia::PathBuilder,
    stroke: &tiny_skia::Stroke,
    cx: &WindowContext,
) -> Option<Path<Pixels>> {
    let skia_path = builder.finish()?;
    let skia_path = skia_path.stroke(stroke, cx.scale_factor())?;
    let first_p = skia_path.points().first()?;

    let mut path = Path::new(point(px(first_p.x), px(first_p.y)));
    for i in 1..skia_path.len() - 1 {
        let verb = skia_path.verbs()[i];
        let Some(p) = skia_path.points().get(i) else {
            continue;
        };

        match verb {
            tiny_skia_path::PathVerb::Move => path.move_to(point(px(p.x), px(p.y))),
            tiny_skia_path::PathVerb::Line => path.line_to(point(px(p.x), px(p.y))),
            _ => {}
        }
    }

    Some(path)
}
