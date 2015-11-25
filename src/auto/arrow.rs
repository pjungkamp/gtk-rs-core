// This file was generated by gir (b85b267) from gir-files (11e0e6d)
// DO NOT EDIT

use ArrowType;
use Buildable;
use Misc;
use ShadowType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Arrow(Object<ffi::GtkArrow>): Widget, Misc, Buildable;

    match fn {
        get_type => || ffi::gtk_arrow_get_type(),
    }
}

impl Arrow {
    pub fn new(arrow_type: ArrowType, shadow_type: ShadowType) -> Arrow {
        unsafe {
            Widget::from_glib_none(ffi::gtk_arrow_new(arrow_type, shadow_type)).downcast_unchecked()
        }
    }

    pub fn set(&self, arrow_type: ArrowType, shadow_type: ShadowType) {
        unsafe {
            ffi::gtk_arrow_set(self.to_glib_none().0, arrow_type, shadow_type);
        }
    }

}
