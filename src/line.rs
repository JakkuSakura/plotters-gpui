use gpui::{point, Hsla, Path, Pixels, Point, WindowContext};
use tracing::warn;

#[derive(Clone, Debug)]
pub struct Line {
    pub points: Vec<Point<Pixels>>,
    pub width: Pixels,
    pub color: Hsla,
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

    pub fn width(mut self, width: f64) -> Self {
        self.width = width.into();
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

        let first_point = self.points[0];
        let width = self.width;
        let mut angle = f32::atan2(
            self.points.first().unwrap().y.0 - self.points.last().unwrap().y.0,
            self.points.first().unwrap().x.0 - self.points.last().unwrap().x.0,
        );
        angle += std::f32::consts::FRAC_PI_2;
        let shift = point(width * f32::cos(angle), width * f32::sin(angle));
        let mut reversed_points = vec![first_point + shift];
        let mut path = Path::new(first_point);
        for p in self.points.iter().cloned().skip(1) {
            path.line_to(p);
            reversed_points.push(p + shift);
        }

        // now do the reverse to close the path
        for p in reversed_points.into_iter().rev() {
            path.line_to(p);
        }

        cx.paint_path(path, self.color);
    }
}
