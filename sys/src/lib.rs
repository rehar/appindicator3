#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub mod auto;
pub mod manual;

#[allow(unused_imports)]
pub use crate::auto::*;

#[allow(unused_imports)]
pub use crate::manual::*;
