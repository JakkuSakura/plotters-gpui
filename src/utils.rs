use gpui::{point, px, Hsla, Pixels, Point, Rgba};
use plotters_backend::{BackendColor, BackendCoord};

pub fn coord_to_point(origin: Point<Pixels>, coord: BackendCoord) -> Point<Pixels> {
    origin + point(px(coord.0 as f32), px(coord.1 as f32))
}

pub fn color_to_hsla(color: BackendColor) -> Hsla {
    Rgba {
        r: color.rgb.0 as f32 / 255.0,
        g: color.rgb.1 as f32 / 255.0,
        b: color.rgb.2 as f32 / 255.0,
        a: color.alpha as f32,
    }
    .into()
}
