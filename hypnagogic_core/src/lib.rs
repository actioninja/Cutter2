#![warn(clippy::pedantic, clippy::cargo)]
// too many lines is a dumb metric
#![allow(clippy::too_many_lines)]
// as is fine, clippy is silly
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
// Not actually going to be a published crate, useless to add
#![allow(clippy::cargo_common_metadata)]
// Annoying
#![allow(clippy::module_name_repetitions)]
// allow this for now, but it's probably a bad idea
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
// sign conversion is fine
#![allow(clippy::cast_sign_loss)]

pub mod config;
pub mod generation;
pub mod operations;
pub mod util;
