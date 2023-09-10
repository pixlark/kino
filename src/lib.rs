#![warn(
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::clone_on_ref_ptr
)]
#![allow(
    clippy::cast_precision_loss,
    clippy::must_use_candidate,
    clippy::missing_panics_doc, // May re-enable this when things are more stable.
)]
#![cfg_attr(feature = "ci", deny(warnings))]

mod canvas;
mod colors;
mod context;
mod gl;
pub mod prelude;
mod shaders;
