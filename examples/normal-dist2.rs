use gpui::{
    div, prelude::*, px, size, App, AppContext, Application, Bounds, Context, Entity, Window,
    WindowBounds, WindowOptions,
};
use parking_lot::RwLock;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use plotters_gpui::backend::GpuiBackend;
use plotters_gpui::element::{PlottersChart, PlottersDrawAreaModel, PlottersDrawAreaViewer};
use rand::SeedableRng as _;
use rand_distr::num_traits::Float as _;
use rand_distr::{Distribution, Normal};
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

struct MyChart;
impl PlottersChart for MyChart {
    fn plot(
        &mut self,
        root: &DrawingArea<GpuiBackend, Shift>,
    ) -> Result<(), plotters_gpui::DrawingErrorKind> {
        let sd = 0.60;

        let random_points: Vec<f64> = {
            let norm_dist = Normal::new(0.0, sd).unwrap();
            let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
            let x_iter = norm_dist.sample_iter(&mut x_rand);
            x_iter.take(5000).filter(|x| x.abs() <= 4.0).collect()
        };

        let mut chart = ChartBuilder::on(root)
            .margin(5)
            .caption("1D Gaussian Distribution Demo", ("sans-serif", 30))
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .set_label_area_size(LabelAreaPosition::Right, 60)
            .build_cartesian_2d(-4f64..4f64, 0f64..0.1)
            .unwrap()
            .set_secondary_coord(
                (-4f64..4f64).step(0.1).use_round().into_segmented(),
                0u32..500u32,
            );

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_label_formatter(&|y| format!("{:.0}%", *y * 100.0))
            .y_desc("Percentage")
            .draw()
            .unwrap();

        chart
            .configure_secondary_axes()
            .y_desc("Count")
            .draw()
            .unwrap();

        let actual = Histogram::vertical(chart.borrow_secondary())
            .style(GREEN.filled())
            .margin(3)
            .data(random_points.iter().map(|x| (*x, 1)));

        chart
            .draw_secondary_series(actual)
            .unwrap()
            .label("Observed")
            .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.filled()));

        let pdf = LineSeries::new(
            (-400..400).map(|x| x as f64 / 100.0).map(|x| {
                (
                    x,
                    (-x * x / 2.0 / sd / sd).exp() / (2.0 * std::f64::consts::PI * sd * sd).sqrt()
                        * 0.1,
                )
            }),
            &RED,
        );

        chart
            .draw_series(pdf)
            .unwrap()
            .label("PDF")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.filled()));

        chart.configure_series_labels().draw().unwrap();

        Ok(())
    }
}

fn main_viewer(cx: &mut App) -> MainViewer {
    let figure = PlottersDrawAreaModel::new(Box::new(MyChart));
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
