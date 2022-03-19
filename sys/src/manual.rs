use glib::GType;

extern "C" {
    // Enum related type function which are getting generated at compile time of libappindicator3
    pub fn app_indicator_category_get_type() -> GType;
    pub fn app_indicator_status_get_type() -> GType;
}