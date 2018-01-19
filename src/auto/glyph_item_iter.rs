// This file was generated by gir (c6b70b0) from gir-files (469db10)
// DO NOT EDIT

use GlyphItem;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GlyphItemIter(Boxed<ffi::PangoGlyphItemIter>);

    match fn {
        copy => |ptr| ffi::pango_glyph_item_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_glyph_item_iter_free(ptr),
        get_type => || ffi::pango_glyph_item_iter_get_type(),
    }
}

impl GlyphItemIter {
    pub fn init_end(&mut self, glyph_item: &mut GlyphItem, text: &str) -> bool {
        unsafe {
            from_glib(ffi::pango_glyph_item_iter_init_end(self.to_glib_none_mut().0, glyph_item.to_glib_none_mut().0, text.to_glib_none().0))
        }
    }

    pub fn init_start(&mut self, glyph_item: &mut GlyphItem, text: &str) -> bool {
        unsafe {
            from_glib(ffi::pango_glyph_item_iter_init_start(self.to_glib_none_mut().0, glyph_item.to_glib_none_mut().0, text.to_glib_none().0))
        }
    }

    pub fn next_cluster(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_glyph_item_iter_next_cluster(self.to_glib_none_mut().0))
        }
    }

    pub fn prev_cluster(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_glyph_item_iter_prev_cluster(self.to_glib_none_mut().0))
        }
    }
}
