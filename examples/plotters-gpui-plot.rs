use std::rc::Rc;

use gpui::{div, prelude::*, App, AppContext, View, ViewContext, WindowContext, WindowOptions};
use parking_lot::RwLock;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use plotters_gpui::backend::GpuiBackend;
use plotters_gpui::element::{PlottersChart, PlottersDrawAreaModel, PlottersDrawAreaViewer};

struct MainViewer {
    figure: View<PlottersDrawAreaViewer>,
    animation: bool,
}

impl MainViewer {
    fn new(model: Rc<RwLock<PlottersDrawAreaModel>>, cx: &mut WindowContext) -> Self {
        let figure = PlottersDrawAreaViewer::with_shared_model(model);

        Self {
            figure: cx.new_view(move |_| figure),
            animation: false,
        }
    }
}

impl Render for MainViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.defer(move |this, cx| {
            if this.animation {
                cx.notify();
            }
        });
        div()
            .size_full()
            .flex_col()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child(self.figure.clone())
    }
}
struct Animation {
    start: f64,
    end: f64,
    step: f64,
    time_start: std::time::Instant,
}
impl Animation {
    fn new(start: f64, end: f64, step: f64) -> Self {
        Self {
            start,
            end,
            step,
            time_start: std::time::Instant::now(),
        }
    }
    fn next_line(&mut self, shift: f64) -> Vec<(f64, f64)> {
        let mut line = Vec::new();
        let t = self.time_start.elapsed().as_secs_f64() * 10.0;
        let mut x = self.start;
        while x <= self.end {
            let y = (x + t).sin();
            line.push((x, y + shift));
            x += self.step;
        }
        line
    }
}

impl PlottersChart for Animation {
    fn plot(
        &mut self,
        root: &DrawingArea<GpuiBackend, Shift>,
    ) -> Result<(), plotters_gpui::DrawingErrorKind> {
        let mut chart = ChartBuilder::on(root)
            .caption("Animation", ("sans-serif", 24).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f64..100f64, 0f64..100f64)
            .unwrap();

        chart.configure_mesh().draw().unwrap();
        for shift in 0..20 {
            let line = self.next_line((shift * 5) as f64);

            chart.draw_series(LineSeries::new(line, &BLACK)).unwrap();
        }

        // chart
        //     .configure_series_labels()
        //     .background_style(&WHITE.mix(0.8))
        //     .border_style(&BLACK)
        //     .draw()
        //     .unwrap();
        Ok(())
    }
}

fn main_viewer(cx: &mut WindowContext) -> MainViewer {
    let figure = PlottersDrawAreaModel::new(Box::new(Animation::new(0.0, 100.0, 0.1)));
    let mut main_viewer = MainViewer::new(Rc::new(RwLock::new(figure)), cx);
    main_viewer.animation = true;

    main_viewer
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
