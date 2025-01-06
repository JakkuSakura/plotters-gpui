pub mod backend;
#[cfg(feature = "plotters")]
pub mod element;
pub mod line;
mod utils;

pub type Error = std::io::Error;
pub type DrawingErrorKind = plotters_backend::DrawingErrorKind<Error>;
