use std::path::Path;

use appindicator3::prelude::*;
use appindicator3::{traits::AppIndicatorExt, Indicator, IndicatorCategory, IndicatorStatus};
use gtk::prelude::*;

const APP_NAME: &str = "AppIndicator Builder Example";

fn main() -> () {
    gtk::init();

    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let mut m = gtk::Menu::builder()
        .tooltip_text("tooltip_text")
        .opacity(0.5)
        .name("name")
        .width_request(500)
        .build();

    let mi = gtk::CheckMenuItem::with_label("Hello Rust!");
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    m.append(&mi);
    m.show_all();
        
    let indicator = Indicator::builder(APP_NAME)
        .category(IndicatorCategory::ApplicationStatus)
        .menu(&m)
        .icon("rust-logo", "icon")
        .icon_theme_path(icon_path.to_str().unwrap())
        .status(IndicatorStatus::Active)
        .title(APP_NAME)
        .label("My label")
        .build();

    gtk::main();
    ()
}
