use gpui::{px, Hsla, PathBuilder, Pixels, Point, Window};
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

    pub fn render_pixels(&mut self, window: &mut Window) {
        if self.points.is_empty() {
            warn!("Line must have at least 1 points to render");
            return;
        }

        let mut builder = PathBuilder::stroke(px(self.width.0));
        let Some(first_p) = self.points.first() else {
            return;
        };

        builder.move_to(*first_p);
        for p in self.points.iter().skip(1) {
            builder.line_to(*p);
        }

        if let Ok(path) = builder.build() {
            window.paint_path(path, self.color);
        }
    }
}
