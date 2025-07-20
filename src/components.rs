//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a
//! RadarGraph component to be used in our app.

pub mod radar;
pub mod theme_buttons;
pub mod tooltip;
pub mod utils;

// We don't need to re-export components here since they're exported by the radar module
