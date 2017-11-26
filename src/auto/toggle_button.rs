// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ToggleButton(Object<ffi::GtkToggleButton, ffi::GtkToggleButtonClass>): Button, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_toggle_button_get_type(),
    }
}

impl ToggleButton {
    pub fn new() -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for ToggleButton {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ToggleButtonExt {
    fn get_active(&self) -> bool;

    fn get_inconsistent(&self) -> bool;

    fn get_mode(&self) -> bool;

    fn set_active(&self, is_active: bool);

    fn set_inconsistent(&self, setting: bool);

    fn set_mode(&self, draw_indicator: bool);

    fn toggled(&self);

    fn get_property_draw_indicator(&self) -> bool;

    fn set_property_draw_indicator(&self, draw_indicator: bool);

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_draw_indicator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToggleButton> + IsA<glib::object::Object>> ToggleButtonExt for O {
    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_active(self.to_glib_none().0))
        }
    }

    fn get_inconsistent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_inconsistent(self.to_glib_none().0))
        }
    }

    fn get_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_mode(self.to_glib_none().0))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    fn set_inconsistent(&self, setting: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_inconsistent(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_mode(&self, draw_indicator: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_mode(self.to_glib_none().0, draw_indicator.to_glib());
        }
    }

    fn toggled(&self) {
        unsafe {
            ffi::gtk_toggle_button_toggled(self.to_glib_none().0);
        }
    }

    fn get_property_draw_indicator(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "draw-indicator".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_draw_indicator(&self, draw_indicator: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "draw-indicator".to_glib_none().0, Value::from(&draw_indicator).to_glib_none().0);
        }
    }

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggled",
                transmute(toggled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_draw_indicator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::draw-indicator",
                transmute(notify_draw_indicator_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inconsistent",
                transmute(notify_inconsistent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn toggled_trampoline<P>(this: *mut ffi::GtkToggleButton, f: glib_ffi::gpointer)
where P: IsA<ToggleButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToggleButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GtkToggleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToggleButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToggleButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_draw_indicator_trampoline<P>(this: *mut ffi::GtkToggleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToggleButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToggleButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_inconsistent_trampoline<P>(this: *mut ffi::GtkToggleButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToggleButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToggleButton::from_glib_borrow(this).downcast_unchecked())
}
