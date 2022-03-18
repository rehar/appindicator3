# appindicator3-rs
[![LGPL-2.1 licensed](https://img.shields.io/badge/license-MIT-green)](./LICENSE)

Rust bindings for the AppIndicator and AyatanaAppIndicator library.

##  [Ayatana](https://ayatanaindicators.github.io/) Application Indicator Library

Allows applications to export a menu into the an Application
Indicators aware menu bar. Based on KSNI it also works in KDE and will
fallback to generic Systray support if none of those are available.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
appindicator3 = "0.2.0"
```

By default `appindicator3-rs` links against the libayatana-appindicator3 library. To link against the original libappindicator3 library enable the original feature:

```toml
[dependencies]
appindicator3 = { version = "0.2.0", feature = ["original", "v0_5"] }

```

### Example Program

```rust
use appindicator3::{prelude::*, IndicatorStatus};
use appindicator3::{Indicator, IndicatorCategory};
use gtk::prelude::*;

fn main() {
    gtk::init().unwrap();

    let m = gtk::Menu::new();
    let mi = gtk::MenuItem::with_label("Hello World");
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    m.add(&mi);
    mi.show_all();

    let _indicator = Indicator::builder("Example")
        .category(IndicatorCategory::ApplicationStatus)
        .menu(&m)
        .icon("face-smile", "icon")
        .status(IndicatorStatus::Active)
        .build();

    gtk::main();
}
```

Additonal examples can be found in [examples](./examples).

## Bindings

A large portion of the bindings is auto generated by GObject Introspection and [gir](https://github.com/gtk-rs/gir/) using [this](./Gir.toml) configuration file.

More details on how to use gir can be found [here](https://gtk-rs.org/gir/book).

## License

appindicator3-rs is available under the MIT License. Please refer to the [`LICENSE`](LICENSE) file. 
This project provides bindings to the Application Indicator library but doesn't distribute any parts of it. Distributing compiled libraries and executable that link to this library may be subject to other licenses. 
