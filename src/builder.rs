// rustdoc-stripper-ignore-next
//! Builder pattern types.

use glib::{ToValue, IsA};
use crate::{Indicator, prelude::*, IndicatorCategory, IndicatorEnumNickname, IndicatorStatus};



#[derive(Clone)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Indicator`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct IndicatorBuilder {
    attention_icon_desc: Option<String>,
    attention_icon_name: Option<String>,
    category: String,
    icon_desc: Option<String>,
    icon_name: Option<String>,
    icon_theme_path: Option<String>,
    id: String,
    label: Option<String>,
    label_guide: Option<String>,
    ordering_index: Option<u32>,
    status: Option<String>,
    title: Option<String>,
    menu: Option<gtk::Menu>,
    secondary_widget: Option<gtk::Widget>,
}

impl Default for IndicatorBuilder {
    fn default() -> Self {
        Self {
            attention_icon_desc: None,
            attention_icon_name: None,
            category: IndicatorCategory::Other.nick(),
            icon_desc: None,
            icon_name: None,
            icon_theme_path: None,
            id: String::new(),
            label: None,
            label_guide: None,
            ordering_index: None,
            status: None,
            title: None,
            menu: None,
            secondary_widget: None,
             
        }
    }
}

impl IndicatorBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`IndicatorBuilder`] using the unique `id` of the indicator
    ///
    ///
    /// ## `id`
    /// The unique id of the indicator to create.
    /// 
    /// # Returns
    /// 
    /// a new instance of [`IndicatorBuilder`](crate::builders::IndicatorBuilder)
    ///
    pub fn new(id: &str) -> Self {
        Self::default().id(id)
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Indicator`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Indicator {
        // properties initalize with mandatory ones
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![("id", &self.id), ("category", &self.category)];

        
        if let Some(ref attention_icon_desc) = self.attention_icon_desc {
            properties.push(("attention-icon-desc", attention_icon_desc));
        }
        if let Some(ref attention_icon_name) = self.attention_icon_name {
            properties.push(("attention-icon-name", attention_icon_name));
        }
        if let Some(ref icon_desc) = self.icon_desc {
            properties.push(("icon-desc", icon_desc));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref icon_theme_path) = self.icon_theme_path {
            properties.push(("icon-theme-path", icon_theme_path));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref label_guide) = self.label_guide {
            properties.push(("label-guide", label_guide));
        }
        if let Some(ref ordering_index) = self.ordering_index {
            properties.push(("ordering-index", ordering_index));
        }
        if let Some(ref status) = self.status {
            properties.push(("status", status));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        let object = glib::Object::new::<Indicator>(&properties)
            .expect("Failed to create an instance of Indicator");

        if let Some(ref menu) = self.menu {
            object.set_menu(Some(menu));
        }

        if let Some(ref widget) = self.secondary_widget {
            object.set_secondary_activate_target(Some(widget));
        }

        object
    }

    #[doc(hidden)]
    /// Function to set the internal indicator id
    fn id(mut self, id: &str) -> Self{
        self.id = id.to_string();
        self
    }

    /// Wrapper function for property `property::Indicator::attention-icon-name`.
    /// ## `icon_name`
    /// The name of the attention icon to set for this indicator
    /// ## `icon_desc`
    /// A textual description of the icon
    pub fn attention_icon(mut self, icon_name: &str, icon_desc: &str) -> Self {
        self.attention_icon_desc = Some(icon_desc.to_string());
        self.attention_icon_name =  Some(icon_name.to_string());
        self
    }

    /// Wrapper function for property `property::Indicator::category`.
    /// Defaults to [`Other`](crate::auto::IndicatorCategory)
    /// ## `category`
    /// The category of indicator.
    pub fn category(mut self, category: IndicatorCategory) -> Self {
        self.category = category.nick();
        self
    }

    /// Sets the default icon to use when the status is active but
    /// not set to attention. In most cases, this should be the
    /// application icon for the program.
    ///
    /// Wrapper function for property `property::Indicator::icon-name` and
    /// `signal::Indicator::icon-desc`.
    /// ## `icon_name`
    /// The icon name to set.
    /// ## `icon_desc`
    /// A textual description of the icon for accessibility
    pub fn icon(mut self, icon_name: &str, icon_desc: &str) -> Self {
        self.icon_desc = Some(icon_desc.to_string());
        self.icon_name = Some(icon_name.to_string());
        self
    }

    /// Sets the path to use when searching for icons.
    /// ## `icon_theme_path`
    /// The icon theme path to set.
    pub fn icon_theme_path(mut self, icon_theme_path: &str) -> Self {
        self.icon_theme_path = Some(icon_theme_path.to_string());
        self
    }

    /// This is a wrapper function for the `property::Indicator::label`
    /// ## `label`
    /// The label to show next to the icon.
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// This is a wrapper function for the `property::Indicator::guide` properties.
    /// ## `guide`
    /// A guide to size the label correctly.
    pub fn label_guide(mut self, label_guide: &str) -> Self {
        self.label_guide = Some(label_guide.to_string());
        self
    }

    /// Sets the ordering index for the app indicator which effects the
    /// placement of it on the panel. For almost all app indicator
    /// this is not the function you're looking for.
    ///
    /// Wrapper function for property `property::Indicator::ordering-index`.
    /// ## `ordering_index`
    /// A value for the ordering of this app indicator
    pub fn ordering_index(mut self, ordering_index: u32) -> Self {
        self.ordering_index = Some(ordering_index);
        self
    }

    /// Wrapper function for property `property::Indicator::status`.
    /// ## `status`
    /// The status to set for this indicator
    pub fn status(mut self, status: IndicatorStatus) -> Self {
        self.status = Some(status.nick());
        self
    }

    /// Sets the title of the application indicator, or how it should be referred
    /// in a human readable form. This string should be UTF-8 and localized as it
    /// expected that users will set it.
    ///
    /// In the Unity desktop the most prominent place that this is show will be
    /// in the HUD. HUD listings for this application indicator will start with
    /// the title as the first part of the line for the menu items.
    ///
    /// ## `title`
    /// Title of the app indicator
    #[cfg(any(feature = "v0_5", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_5")))]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the menu that should be shown when the Application Indicator
    /// is clicked on in the panel. An application indicator will not
    /// be rendered unless it has a menu.
    ///
    /// Wrapper function for property `property::Indicator::menu`.
    /// ## `menu`
    /// A [`gtk::Menu`][crate::gtk::Menu] to set
    pub fn menu(mut self, menu: &impl IsA<gtk::Menu>) -> Self {
        self.menu = Some(menu.as_ref().clone());
        self
    }

    /// Set the `menuitem` to be activated when a secondary activation event (i.e. a
    /// middle-click) is emitted over the [`Indicator`][crate::Indicator] icon/label.
    ///
    /// The `menuitem` can be also a complex [`gtk::Widget`][crate::gtk::Widget], but to get activated when
    /// a secondary activation occurs in the `Appindicator`, it must be a visible and
    /// active child (or inner-child) of the `property::Indicator::menu`.
    ///
    /// ## `menuitem`
    /// A [`gtk::Widget`][crate::gtk::Widget] to be activated on secondary activation
    pub fn secondary_activate_target(mut self, menuitem: &impl IsA<gtk::Widget>) -> Self {
         self.secondary_widget = Some(menuitem.as_ref().clone());
         self
    }
}

impl<O: IsA<Indicator>> AppIndicatorBuilderExt for O {
    fn builder(id: &str) -> IndicatorBuilder {
        IndicatorBuilder::default().id(id)
    }
}
#[doc(hidden)]
pub mod traits {
    /// Trait containing [`IndicatorBuilder`](crate::builders::IndicatorBuilder) constructor methods.
    pub trait AppIndicatorBuilderExt {
        // rustdoc-stripper-ignore-next
        /// Creates a new builder-pattern struct instance to construct [`Indicator`](crate::auto::Indicator) objects.
        ///
        /// This method returns an instance of [`IndicatorBuilder`](crate::builders::IndicatorBuilder) which can be used to create [`Indicator`](crate::auto::Indicator) objects.
        ///
        /// ## `id`
        /// The unique id of the indicator to create.
        /// 
        /// # Returns
        /// 
        /// a new instance of [`IndicatorBuilder`](crate::builders::IndicatorBuilder)
        ///
        fn builder(id: &str) -> crate::IndicatorBuilder;
    }
}
