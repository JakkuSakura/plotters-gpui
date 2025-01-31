use gpui::{div, prelude::*, AppContext, WindowOptions};
use parking_lot::RwLock;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use plotters_gpui::backend::GpuiBackend;
use plotters_gpui::element::{PlottersChart, PlottersDrawAreaModel, PlottersDrawAreaViewer};
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::SystemTime;
use sysinfo::{Pid, ProcessesToUpdate, System};

struct MainViewer {
    figure: gpui::Entity<PlottersDrawAreaViewer>,
    animation: bool,
}

impl MainViewer {
    fn new(model: Rc<RwLock<PlottersDrawAreaModel>>, app: &mut gpui::App) -> Self {
        let figure = PlottersDrawAreaViewer::with_shared_model(model);

        Self {
            figure: app.new(move |_| figure),
            animation: false,
        }
    }
}

impl Render for MainViewer {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        if self.animation {
            let id = cx.entity_id();
            cx.defer(move |app| app.notify(id));
        }
        div()
            .size_full()
            .flex_col()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child(self.figure.clone())
    }
}
struct CpuUsage {
    system: System,
    pid: Pid,
    points: VecDeque<(SystemTime, f32)>,
}
impl CpuUsage {
    fn new() -> Self {
        Self {
            system: System::new_all(),
            pid: (std::process::id() as usize).into(),
            points: VecDeque::new(),
        }
    }
    fn should_sample(&self) -> bool {
        if self.points.is_empty() {
            return true;
        }
        let (time, _) = self.points.back().unwrap();
        time.elapsed().unwrap().as_secs_f32() > 0.1
    }
    fn try_sample(&mut self) {
        if !self.should_sample() {
            return;
        }
        self.system.refresh_processes(ProcessesToUpdate::All, false);
        let process = self.system.process(self.pid).unwrap();
        self.points
            .push_back((SystemTime::now(), process.cpu_usage()));
        if self.points.len() > 100 {
            self.points.pop_front();
        }
    }
    fn get_line(&mut self) -> Vec<(f32, f32)> {
        let mut line = Vec::new();
        let time = self.points.front().unwrap().0;
        for (x, y) in self.points.iter() {
            let seconds = x.duration_since(time).unwrap().as_secs_f32();
            line.push((seconds, *y));
        }
        line
    }
}

impl PlottersChart for CpuUsage {
    fn plot(
        &mut self,
        root: &DrawingArea<GpuiBackend, Shift>,
    ) -> Result<(), plotters_gpui::DrawingErrorKind> {
        self.try_sample();
        let line = self.get_line();
        let x_min = line.first().map(|(x, _)| *x).unwrap_or(0.0);
        let x_max = line.last().map(|(x, _)| *x).unwrap_or(1.0);
        let y_min = line.iter().map(|(_, y)| *y).fold(f32::INFINITY, f32::min);
        let y_max = line
            .iter()
            .map(|(_, y)| *y)
            .fold(f32::NEG_INFINITY, f32::max);
        let mut chart = ChartBuilder::on(root)
            .caption("CPU Usage", ("sans-serif", 24).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();
        chart.configure_mesh().draw().unwrap();
        println!("Plot line with {} points", line.len());
        chart.draw_series(LineSeries::new(line, &BLACK)).unwrap();

        Ok(())
    }
}

fn main_viewer(app: &mut gpui::App) -> MainViewer {
    let figure = PlottersDrawAreaModel::new(Box::new(CpuUsage::new()));
    let mut main_viewer = MainViewer::new(Rc::new(RwLock::new(figure)), app);
    main_viewer.animation = true;

    main_viewer
}

fn main() {
    gpui::Application::new().run(move |app: &mut gpui::App| {
        app.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            move |_window, app| {
                let view = main_viewer(app);
                app.new(|_| view)
            },
        )
        .unwrap();
        app.activate(true);
    });
}
