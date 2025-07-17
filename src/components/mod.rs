//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a
//! RadarGraph component to be used in our app.

mod radar_axis;
mod radar_curve;
mod radar_graph;
mod radar_grid;
mod radar_legend;
mod utils;

// Export the main component and data structures
pub use radar_graph::{DataPoint, RadarCurve, RadarGraph};

// Export individual components for advanced usage
pub mod radar_components {
    pub use super::radar_axis::RadarAxis;
    pub use super::radar_curve::RadarCurveVisual;
    pub use super::radar_grid::RadarGrid;
    pub use super::radar_legend::RadarLegend;
}
