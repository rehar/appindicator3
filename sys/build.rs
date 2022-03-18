#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    let mut config = pkg_config::Config::new();

    if cfg!(feature = "v0_5") {
        config.atleast_version("0.5");
    } else if cfg!(feature = "v0_4") {
        config.atleast_version("0.4");
    }

    let probe = if cfg!(feature = "original") {
        // use the original libappindicator3 library
        config.probe("appindicator3-0.1")
    } else {
        // default use libayatana-appinidcator3
        config.probe("ayatana-appindicator3-0.1")
    };
    if let Err(s) = probe {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}
