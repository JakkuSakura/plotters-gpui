use crate::line::Line;
use crate::utils::{color_to_hsla, coord_to_point};
use gpui::{bounds, fill, point, px, App, Bounds, Pixels, SharedString, Size, TextRun, Window};
use plotters_backend::{
    text_anchor::{HPos, VPos},
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
};

/// The embedded backend for plotters in gpui
pub struct GpuiBackend<'a> {
    bounds: Bounds<Pixels>,
    window: &'a mut Window,
    cx: &'a mut App,
}

impl<'a> GpuiBackend<'a> {
    /// Create a new embedded backend
    pub fn new(bounds: Bounds<Pixels>, window: &'a mut Window, cx: &'a mut App) -> Self {
        Self { bounds, window, cx }
    }
}

impl DrawingBackend for GpuiBackend<'_> {
    type ErrorType = crate::Error;

    fn get_size(&self) -> (u32, u32) {
        let size = self.bounds.size;
        (size.width.0 as u32, size.height.0 as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = color_to_hsla(color);
        let point = coord_to_point(self.bounds.origin, point);
        let size = Size::new(Pixels(1.0), Pixels(1.0));
        let bounds = bounds(point, size);
        let quad = fill(bounds, color);
        self.window.paint_quad(quad);

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut line = Line::between_points(
            coord_to_point(self.bounds.origin, from),
            coord_to_point(self.bounds.origin, to),
        )
        .width(px(style.stroke_width() as _))
        .color(color_to_hsla(style.color()));

        line.render_pixels(self.window);
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let upper_left = coord_to_point(self.bounds.origin, upper_left);
        let bottom_right = coord_to_point(self.bounds.origin, bottom_right);
        let color = color_to_hsla(style.color());

        if fill {
            let mut builder = gpui::PathBuilder::fill();
            builder.move_to(upper_left);
            builder.line_to(point(upper_left.x, bottom_right.y));
            builder.line_to(bottom_right);
            builder.line_to(point(bottom_right.x, upper_left.y));
            builder.line_to(upper_left);
            let path = builder.build().map_err(|err| {
                DrawingErrorKind::DrawingError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err.to_string(),
                ))
            })?;

            self.window.paint_path(path, color);
        } else {
            for (p1, p2) in [
                (upper_left, point(bottom_right.x, upper_left.y)),
                (point(bottom_right.x, upper_left.y), bottom_right),
                (bottom_right, point(upper_left.x, bottom_right.y)),
                (point(upper_left.x, bottom_right.y), upper_left),
            ] {
                Line::between_points(p1, p2)
                    .color(color)
                    .render_pixels(self.window);
            }
        }

        Ok(())
    }

    // path in plotters does not close the shape
    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let iter = path.into_iter();
        let mut points = Vec::with_capacity(iter.size_hint().0 * 2);
        for point in iter {
            points.push(coord_to_point(self.bounds.origin, point));
        }

        if points.is_empty() {
            return Ok(());
        }

        let mut line = Line::new();
        line.points = points;
        line.width = px(style.stroke_width() as _);
        line.color = color_to_hsla(style.color());
        line.render_pixels(self.window);

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut iter = vert.into_iter();
        let start = match iter.next() {
            Some(start) => start,
            None => return Ok(()),
        };

        let mut builder = gpui::PathBuilder::fill();
        builder.move_to(coord_to_point(self.bounds.origin, start));
        for point in iter {
            builder.line_to(coord_to_point(self.bounds.origin, point));
        }

        let path = builder.build().map_err(|err| {
            DrawingErrorKind::DrawingError(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            ))
        })?;

        let color = color_to_hsla(style.color());
        self.window.paint_path(path, color);
        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let layout = style
            .layout_box(text)
            .map_err(|e| DrawingErrorKind::FontError(Box::new(e)))?;
        let ((min_x, min_y), (max_x, max_y)) = layout;
        let width = max_x - min_x;
        let height = max_y - min_y;
        let dx = match style.anchor().h_pos {
            HPos::Left => 0,
            HPos::Right => -width,
            HPos::Center => -width / 2,
        };
        let dy = match style.anchor().v_pos {
            VPos::Top => 0,
            VPos::Center => -height / 2,
            VPos::Bottom => -height,
        };
        let color = color_to_hsla(style.color());
        let point =
            coord_to_point(self.bounds.origin, pos) + gpui::point(px(dx as _), px(dy as _)) * 1.24;
        let font = self.window.text_style().font();
        let len = text.len();
        let size = px(style.size() as _);

        let shaped_line = self
            .window
            .text_system()
            .shape_line(
                SharedString::from(text.to_string()),
                size,
                &[TextRun {
                    len,
                    font,
                    color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
            )
            .map_err(|err| DrawingErrorKind::FontError(err.to_string().into()))?;
        shaped_line
            .paint(point, size, self.window, self.cx)
            .map_err(|err| {
                DrawingErrorKind::DrawingError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err.to_string(),
                ))
            })?;

        Ok(())
    }
}
