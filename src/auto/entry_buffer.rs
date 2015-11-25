// This file was generated by gir (b85b267) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct EntryBuffer(Object<ffi::GtkEntryBuffer>);

    match fn {
        get_type => || ffi::gtk_entry_buffer_get_type(),
    }
}

impl EntryBuffer {
    pub fn new(initial_chars: Option<&str>, n_initial_chars: i32) -> EntryBuffer {
        unsafe {
            from_glib_full(ffi::gtk_entry_buffer_new(initial_chars.to_glib_none().0, n_initial_chars))
        }
    }

    pub fn delete_text(&self, position: u32, n_chars: i32) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_delete_text(self.to_glib_none().0, position, n_chars)
        }
    }

    pub fn emit_deleted_text(&self, position: u32, n_chars: u32) {
        unsafe {
            ffi::gtk_entry_buffer_emit_deleted_text(self.to_glib_none().0, position, n_chars);
        }
    }

    pub fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32) {
        unsafe {
            ffi::gtk_entry_buffer_emit_inserted_text(self.to_glib_none().0, position, chars.to_glib_none().0, n_chars);
        }
    }

    //pub fn get_bytes(&self) -> Fundamental: Size {
    //    unsafe { TODO: call ffi::gtk_entry_buffer_get_bytes() }
    //}

    pub fn get_length(&self) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_get_length(self.to_glib_none().0)
        }
    }

    pub fn get_max_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_buffer_get_max_length(self.to_glib_none().0)
        }
    }

    pub fn get_text(&self) -> String {
        unsafe {
            from_glib_none(ffi::gtk_entry_buffer_get_text(self.to_glib_none().0))
        }
    }

    pub fn insert_text(&self, position: u32, chars: &str, n_chars: i32) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_insert_text(self.to_glib_none().0, position, chars.to_glib_none().0, n_chars)
        }
    }

    pub fn set_max_length(&self, max_length: i32) {
        unsafe {
            ffi::gtk_entry_buffer_set_max_length(self.to_glib_none().0, max_length);
        }
    }

    pub fn set_text(&self, chars: &str, n_chars: i32) {
        unsafe {
            ffi::gtk_entry_buffer_set_text(self.to_glib_none().0, chars.to_glib_none().0, n_chars);
        }
    }

}
