use gpui::{
    div, prelude::*, px, size, App, AppContext, Application, Bounds, Context, Entity, Window,
    WindowBounds, WindowOptions,
};
use parking_lot::RwLock;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_gpui::backend::GpuiBackend;
use plotters_gpui::element::*;
use rand::SeedableRng as _;
use rand_distr::{Distribution as _, Normal};
use rand_xorshift::XorShiftRng;
use std::rc::Rc;

struct MainViewer {
    figure: Entity<PlottersDrawAreaViewer>,
}

impl MainViewer {
    fn new(model: Rc<RwLock<PlottersDrawAreaModel>>, cx: &mut App) -> Self {
        let figure = PlottersDrawAreaViewer::with_shared_model(model);

        Self {
            figure: cx.new(move |_| figure),
        }
    }
}

impl Render for MainViewer {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex_col()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child(self.figure.clone())
    }
}

struct AreaChart {
    data: Vec<f64>,
}

impl AreaChart {
    fn new() -> Self {
        let data: Vec<_> = {
            let norm_dist = Normal::new(500.0, 100.0).unwrap();
            let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
            let x_iter = norm_dist.sample_iter(&mut x_rand);
            x_iter
                .filter(|x| *x < 1500.0)
                .take(100)
                .zip(0..)
                .map(|(x, b)| x + (b as f64).powf(1.2))
                .collect()
        };

        Self { data }
    }
}

impl PlottersChart for AreaChart {
    fn plot(
        &mut self,
        root: &DrawingArea<GpuiBackend, Shift>,
    ) -> Result<(), plotters_gpui::DrawingErrorKind> {
        let mut chart = ChartBuilder::on(root)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .caption("Area Chart Demo", ("sans-serif", 40))
            .build_cartesian_2d(0..(self.data.len() - 1), 0.0..1500.0)
            .unwrap();

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()
            .unwrap();

        chart
            .draw_series(
                AreaSeries::new(
                    (0..).zip(self.data.iter()).map(|(x, y)| (x, *y)),
                    0.0,
                    RED.mix(0.2),
                )
                .border_style(RED),
            )
            .unwrap();

        Ok(())
    }
}

fn main_viewer(cx: &mut App) -> MainViewer {
    let figure = PlottersDrawAreaModel::new(Box::new(AreaChart::new()));
    MainViewer::new(Rc::new(RwLock::new(figure)), cx)
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |_, cx| cx.new(move |cx| main_viewer(cx)),
        )
        .unwrap();
    });
}
