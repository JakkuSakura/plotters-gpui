use crate::backend::GpuiBackend;
use gpui::{canvas, Bounds, IntoElement, Pixels, Render, Styled, ViewContext, WindowContext};
use parking_lot::RwLock;
use plotters::coord::Shift;
use plotters::drawing::{DrawingArea, IntoDrawingArea};
use plotters::prelude::*;
use std::rc::Rc;
use tracing::error;

pub struct PlottersDrawAreaModel {
    pub backend_color: RGBColor,
    pub chart: Box<dyn PlottersChart>,
}

impl PlottersDrawAreaModel {
    pub fn new(chart: Box<dyn PlottersChart>) -> Self {
        Self {
            backend_color: WHITE,
            chart,
        }
    }
}
#[derive(Clone)]
pub struct PlottersDrawAreaViewer {
    model: Rc<RwLock<PlottersDrawAreaModel>>,
}

impl PlottersDrawAreaViewer {
    pub fn new(model: PlottersDrawAreaModel) -> Self {
        let model = Rc::new(RwLock::new(model));
        Self { model }
    }

    pub fn with_shared_model(model: Rc<RwLock<PlottersDrawAreaModel>>) -> Self {
        Self { model }
    }

    pub fn plot(
        &self,
        bounds: Bounds<Pixels>,
        cx: &mut WindowContext,
    ) -> Result<(), DrawingAreaErrorKind<crate::Error>> {
        let mut model = self.model.write();
        let root = GpuiBackend::new(bounds, cx).into_drawing_area();
        root.fill(&model.backend_color)?;
        model
            .chart
            .plot(&root)
            .map_err(DrawingAreaErrorKind::BackendError)?;
        root.present()?;
        Ok(())
    }
}

impl Render for PlottersDrawAreaViewer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let this = self.clone();
        canvas(
            |_, _| {},
            move |bounds, _, cx| {
                if let Err(err) = this.plot(bounds, cx) {
                    error!("failed to plot: {}", err);
                }
            },
        )
        .size_full()
    }
}

pub trait PlottersChart {
    fn plot(
        &mut self,
        area: &DrawingArea<GpuiBackend, Shift>,
    ) -> Result<(), crate::DrawingErrorKind>;
}

impl PlottersChart for () {
    fn plot(&mut self, _: &DrawingArea<GpuiBackend, Shift>) -> Result<(), crate::DrawingErrorKind> {
        Ok(())
    }
}

impl<F> PlottersChart for F
where
    F: FnMut(&DrawingArea<GpuiBackend, Shift>) -> Result<(), crate::DrawingErrorKind>,
{
    fn plot(
        &mut self,
        area: &DrawingArea<GpuiBackend, Shift>,
    ) -> Result<(), crate::DrawingErrorKind> {
        self(area)
    }
}

macro_rules! impl_plotters_char_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($name),*> PlottersChart for ($($name,)*)
        where
            $($name: PlottersChart,)*
        {
            fn plot(&mut self, area: &DrawingArea<GpuiBackend, Shift>) -> Result<(), crate::DrawingErrorKind> {
                let ($($name,)*) = self;
                $($name.plot(area)?;)*
                Ok(())
            }
        }
    };
}

macro_rules! impl_plotters_char_for_all_tuples {
    ($first:ident, $($rest:ident),*) => {
        impl_plotters_char_for_tuple!($first $(, $rest)*);
        impl_plotters_char_for_all_tuples!($($rest),*);
    };
    ($last:ident) => {
        impl_plotters_char_for_tuple!($last);
    };
}

impl_plotters_char_for_all_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
