use gpui::{div, prelude::*, App, AppContext, View, ViewContext, WindowContext, WindowOptions};
use parking_lot::RwLock;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use plotters_gpui::backend::GpuiBackend;
use plotters_gpui::element::{PlottersChart, PlottersDrawAreaModel, PlottersDrawAreaViewer};
use std::rc::Rc;

fn pdf(x: f64, y: f64) -> f64 {
    const SDX: f64 = 0.1;
    const SDY: f64 = 0.1;
    const A: f64 = 5.0;
    let x = x / 10.0;
    let y = y / 10.0;
    A * (-x * x / 2.0 / SDX / SDX - y * y / 2.0 / SDY / SDY).exp()
}

struct MainViewer {
    figure: View<PlottersDrawAreaViewer>,
}

impl MainViewer {
    fn new(model: Rc<RwLock<PlottersDrawAreaModel>>, cx: &mut WindowContext) -> Self {
        let figure = PlottersDrawAreaViewer::with_shared_model(model);

        Self {
            figure: cx.new_view(move |_| figure),
        }
    }
}

impl Render for MainViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.defer(move |_, cx| {
            cx.notify();
        });

        div()
            .size_full()
            .flex_col()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child(self.figure.clone())
    }
}

struct MyChart {
    pitch: usize,
    last: std::time::Instant,
}

impl MyChart {
    fn new() -> Self {
        Self {
            pitch: 0,
            last: std::time::Instant::now(),
        }
    }

    fn next(&mut self) {
        if self.last.elapsed().as_millis() < 25 {
            return;
        }

        self.last = std::time::Instant::now();
        self.pitch += 1;
        if self.pitch >= 157 {
            self.pitch = 0;
        }
    }
}

impl PlottersChart for MyChart {
    fn plot(
        &mut self,
        root: &DrawingArea<GpuiBackend, Shift>,
    ) -> Result<(), plotters_gpui::DrawingErrorKind> {
        root.fill(&WHITE).unwrap();

        self.next();

        let mut chart = ChartBuilder::on(root)
            .caption("2D Gaussian PDF", ("sans-serif", 20))
            .build_cartesian_3d(-3.0..3.0, 0.0..6.0, -3.0..3.0)
            .unwrap();
        chart.with_projection(|mut p| {
            p.pitch = 1.57 - (1.57 - self.pitch as f64 / 50.0).abs();
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()
            .unwrap();

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (-15..=15).map(|x| x as f64 / 5.0),
                    (-15..=15).map(|x| x as f64 / 5.0),
                    pdf,
                )
                .style_func(&|&v| (VulcanoHSL::get_color(v / 5.0)).into()),
            )
            .unwrap();

        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

        Ok(())
    }
}

fn main_viewer(cx: &mut WindowContext) -> MainViewer {
    let figure = PlottersDrawAreaModel::new(Box::new(MyChart::new()));
    MainViewer::new(Rc::new(RwLock::new(figure)), cx)
}

fn main() {
    App::new().run(move |cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            move |cx| {
                let view = main_viewer(cx);
                cx.new_view(move |_| view)
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
