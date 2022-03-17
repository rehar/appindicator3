use std::{cell::Cell, rc::Rc, path::Path};

use appindicator3::{Indicator, prelude::AppIndicatorExt, IndicatorStatus, IndicatorCategory};
use gdk::gio::{Icon};
use gtk::{prelude::*, MenuItem};

const CAN_HAZ_LABEL: bool = true;

fn image_menu_item(icon: &str, label: &str) -> gtk::MenuItem{
    let g_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    let icon = gtk::Image::from_gicon(&Icon::for_string(icon).unwrap(), gtk::IconSize::Menu);
    let label = gtk::Label::new(Some(label));
    let menu_item = gtk::MenuItem::new();
    g_box.add(&icon);
    g_box.add(&label);
  
    menu_item.add(&g_box);
    menu_item.show_all();
  
    menu_item
}

fn item_clicked(item: &MenuItem) {
    println!("{} clicked!", item.label().unwrap());
}

fn append_submenu (parent: &gtk::MenuItem) {
    let menu = gtk::Menu::new();

    let mi = gtk::MenuItem::with_label("Sub 1");
    mi.connect_activate(item_clicked);
    menu.append(&mi);

    let prev_mi = mi;
    let mi = gtk::MenuItem::with_label("Sub 2");
    
    mi.connect_activate(glib::clone!(@weak prev_mi => move |_| {
        toggle_sensitivity(&prev_mi.upcast::<gtk::Widget>());
    }));

    menu.append(&mi);

    let mi = gtk::MenuItem::with_label("Sub 3");
    mi.connect_activate(item_clicked);
    menu.append(&mi);

    menu.show_all();

    parent.set_submenu(Some(&menu));
}
fn toggle_sensitivity (widget: &gtk::Widget) {
    widget.set_sensitive(!widget.is_sensitive());
}
fn main() -> Result<(), glib::BoolError> {
    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");

    gtk::init()?;

    let can_haz_label = Rc::new(Cell::new(CAN_HAZ_LABEL));
    let mut percentage = 0.0;
    let indicator = Indicator::new("example-simple-client", "indicator-messages", IndicatorCategory::ApplicationStatus);
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());

    indicator.set_status(IndicatorStatus::Active);
    
    indicator.set_attention_icon_full("indicator-messages-new", "Messages Icon Highlighted");
    indicator.set_label("1%", "100%");
    indicator.set_title(Some("Test Indicator"));

    indicator.connect_scroll_event(|_, delta, direction| {
        println!("Got scroll event! delta: {}, direction: {}", delta, direction);
    });

    indicator.connect_label_notify(move |ind|{
        if let Some(label) = ind.label() {
            println!("Label changed! {}", label);
        }
    });

    glib::timeout_add_seconds_local(1, glib::clone!(@weak indicator, @weak can_haz_label => @default-return glib::Continue(false), move || {
        percentage = (percentage + 1.0) % 100.0;
        if can_haz_label.get() {
            let percentstr = format!("{:.2}", percentage + 1.0);
            indicator.set_label(&percentstr, "100%");
        } else {
             indicator.set_label("", "");
        }
        glib::Continue(true)
    }));


    let menu = gtk::Menu::new();
    let item = gtk::CheckMenuItem::with_label("1");
    item.connect_activate(|item| {
        item_clicked(item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&item);
    item.show();


    let item = gtk::RadioMenuItem::with_label("2");
    item.connect_activate(|item| {
        item_clicked(item.upcast_ref::<gtk::MenuItem>())
    });
    menu.append(&item);
    item.show();

    let item = gtk::MenuItem::with_label("3");
    append_submenu(&item);
    menu.append(&item);
    item.show();

    let toggle = gtk::MenuItem::with_label("Toggle 3");
    toggle.connect_activate( |item| {
        toggle_sensitivity(item.upcast_ref::<gtk::Widget>());
    });
    menu.append(&toggle);
    toggle.show();

    let image = image_menu_item("gtk-new", "Icon");
    image.connect_activate(|item| {
        let g_box = item.child().unwrap().downcast::<gtk::Box>().unwrap();
        for ref item in g_box.children() {
            if let Some(image) = item.downcast_ref::<gtk::Image>() {
                image.set_from_gicon(&Icon::for_string("gtk-open").unwrap(), gtk::IconSize::Menu);
            }
        }
    });
    menu.append(&image);
    image.show();
    

    let item = gtk::MenuItem::with_label("Get Attention");
    let mut active_flag = Rc::new(Cell::new(false));
    item.connect_activate(glib::clone!(@weak indicator, @weak active_flag => move |item| {
        if active_flag.get() {
            indicator.set_status(IndicatorStatus::Attention);
            item.set_label("I'm okay now");
            active_flag.set(false);
        } else {
            indicator.set_status(IndicatorStatus::Active);
            item.set_label("Get Attention");
            active_flag.set(true);
        }
    }));
    menu.append(&item);
    item.show();
    indicator.set_secondary_activate_target(Some(&item));

    let item = gtk::MenuItem::with_label("Hide label");
    item.connect_activate(glib::clone!(@weak can_haz_label => move |item| {
        can_haz_label.set(!can_haz_label.get());

        if can_haz_label.get() {
            item.set_label("Hide label");
        } else {
            item.set_label("Show label");
        }
    }));
    menu.append(&item);
    item.show();

    let item = gtk::CheckMenuItem::with_label("Switch Icon");
    item.connect_activate(glib::clone!(@weak indicator => move |item| {
        if item.is_active() {
            indicator.set_icon_full("simple-client-test-icon", "Client Icon")
        } else {
            indicator.set_icon_full("indicator-messages", "Messages Icon")
        }
    }));
    menu.append(&item);
    item.show();

    indicator.set_menu(Some(&menu));
    gtk::main();

    Ok(())
}