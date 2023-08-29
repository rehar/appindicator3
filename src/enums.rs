use glib::{translate::*, StaticType, Type, ToValue, value::FromValue, EnumClass};
use crate::{IndicatorCategory, IndicatorStatus};
use self::traits::AppIndicatorEnumNickname;

impl StaticType for IndicatorCategory {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::app_indicator_category_get_type()) }
    }
}

impl ToValue for IndicatorCategory {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}


impl glib::value::ValueType for IndicatorCategory {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for IndicatorCategory {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl AppIndicatorEnumNickname for IndicatorCategory {
    fn nick(&self) -> String {
        EnumClass::with_type(self.value_type()).unwrap().value(self.into_glib()).unwrap().nick().to_owned()
    }
}

impl StaticType for IndicatorStatus {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::app_indicator_status_get_type()) }
    }
}

impl ToValue for IndicatorStatus {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}


impl glib::value::ValueType for IndicatorStatus {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for IndicatorStatus {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl AppIndicatorEnumNickname for IndicatorStatus {
    fn nick(&self) -> String {
        EnumClass::with_type(self.value_type()).unwrap().value(self.into_glib()).unwrap().nick().to_owned()
    }
}

#[doc(hidden)]
pub mod traits {
    /// Trait for retrieving an enum nickname.
    pub trait AppIndicatorEnumNickname {
        /// Get nick corresponding to the enum.
        fn nick(&self) -> String;
    }
}