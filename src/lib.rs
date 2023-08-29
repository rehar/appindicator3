#![cfg_attr(feature = "dox", feature(doc_cfg))]
//! Rust bindings for the AppIndicator and AyatanaAppIndicator libraries. 
//! 
//! Allows applications to export a menu into the an Application Indicators aware menu bar. 
//! Based on KSNI it also works in KDE and will fallback to generic Systray support if none 
//! of those are available. 
//! 
//! # Usage
//! 
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! gtk = "0.18"
//! appindicator3 = "0.3"
//! ```
//! 
//! Next:
//! 
//! ```rust
//! use gtk::prelude::*;
//! use appindicator3::{Indicator, IndicatorCategory, IndicatorStatus, prelude::*};
//! 
//! fn main() {
//!     gtk::init().unwrap();
//!
//!     let m = gtk::Menu::new();
//!     let mi = gtk::MenuItem::with_label("Hello World");
//!     mi.connect_activate(|_| {
//!         gtk::main_quit();
//!     });
//!     m.add(&mi);
//!     mi.show_all();
//! 
//!     let _indicator = Indicator::builder("Example")
//!         .category(IndicatorCategory::ApplicationStatus)
//!         .menu(&m)
//!         .icon("face-smile", "icon")
//!         .status(IndicatorStatus::Active)
//!         .build();
//! 
//!     gtk::main();
//! }
//! ```
pub use ffi;
pub use glib;

#[allow(unused_imports)]
mod auto;
mod builder;
pub mod prelude;
pub mod enums;

pub use crate::auto::*;
pub use crate::builder::IndicatorBuilder;