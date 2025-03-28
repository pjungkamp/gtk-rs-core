// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Drive, Mount, Volume};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GVolumeMonitor")]
    pub struct VolumeMonitor(Object<ffi::GVolumeMonitor, ffi::GVolumeMonitorClass>);

    match fn {
        type_ => || ffi::g_volume_monitor_get_type(),
    }
}

impl VolumeMonitor {
    pub const NONE: Option<&'static VolumeMonitor> = None;

    #[doc(alias = "g_volume_monitor_get")]
    pub fn get() -> VolumeMonitor {
        unsafe { from_glib_full(ffi::g_volume_monitor_get()) }
    }
}

pub trait VolumeMonitorExt: IsA<VolumeMonitor> + 'static {
    #[doc(alias = "g_volume_monitor_get_connected_drives")]
    #[doc(alias = "get_connected_drives")]
    fn connected_drives(&self) -> Vec<Drive> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_volume_monitor_get_connected_drives(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_volume_monitor_get_mount_for_uuid")]
    #[doc(alias = "get_mount_for_uuid")]
    fn mount_for_uuid(&self, uuid: &str) -> Option<Mount> {
        unsafe {
            from_glib_full(ffi::g_volume_monitor_get_mount_for_uuid(
                self.as_ref().to_glib_none().0,
                uuid.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_volume_monitor_get_mounts")]
    #[doc(alias = "get_mounts")]
    fn mounts(&self) -> Vec<Mount> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_volume_monitor_get_mounts(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_volume_monitor_get_volume_for_uuid")]
    #[doc(alias = "get_volume_for_uuid")]
    fn volume_for_uuid(&self, uuid: &str) -> Option<Volume> {
        unsafe {
            from_glib_full(ffi::g_volume_monitor_get_volume_for_uuid(
                self.as_ref().to_glib_none().0,
                uuid.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_volume_monitor_get_volumes")]
    #[doc(alias = "get_volumes")]
    fn volumes(&self) -> Vec<Volume> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_volume_monitor_get_volumes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "drive-changed")]
    fn connect_drive_changed<F: Fn(&Self, &Drive) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drive_changed_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Drive) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            drive: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(drive),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"drive-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drive_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drive-connected")]
    fn connect_drive_connected<F: Fn(&Self, &Drive) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drive_connected_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Drive) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            drive: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(drive),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"drive-connected".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drive_connected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drive-disconnected")]
    fn connect_drive_disconnected<F: Fn(&Self, &Drive) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drive_disconnected_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Drive) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            drive: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(drive),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"drive-disconnected".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drive_disconnected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drive-eject-button")]
    fn connect_drive_eject_button<F: Fn(&Self, &Drive) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drive_eject_button_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Drive) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            drive: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(drive),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"drive-eject-button".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drive_eject_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drive-stop-button")]
    fn connect_drive_stop_button<F: Fn(&Self, &Drive) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drive_stop_button_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Drive) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            drive: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(drive),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"drive-stop-button".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drive_stop_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mount-added")]
    fn connect_mount_added<F: Fn(&Self, &Mount) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn mount_added_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Mount) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            mount: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(mount),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"mount-added".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    mount_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mount-changed")]
    fn connect_mount_changed<F: Fn(&Self, &Mount) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn mount_changed_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Mount) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            mount: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(mount),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"mount-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    mount_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mount-pre-unmount")]
    fn connect_mount_pre_unmount<F: Fn(&Self, &Mount) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn mount_pre_unmount_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Mount) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            mount: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(mount),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"mount-pre-unmount".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    mount_pre_unmount_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mount-removed")]
    fn connect_mount_removed<F: Fn(&Self, &Mount) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn mount_removed_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Mount) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            mount: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(mount),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"mount-removed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    mount_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "volume-added")]
    fn connect_volume_added<F: Fn(&Self, &Volume) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn volume_added_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Volume) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            volume: *mut ffi::GVolume,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(volume),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"volume-added".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    volume_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "volume-changed")]
    fn connect_volume_changed<F: Fn(&Self, &Volume) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn volume_changed_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Volume) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            volume: *mut ffi::GVolume,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(volume),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"volume-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    volume_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "volume-removed")]
    fn connect_volume_removed<F: Fn(&Self, &Volume) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn volume_removed_trampoline<
            P: IsA<VolumeMonitor>,
            F: Fn(&P, &Volume) + 'static,
        >(
            this: *mut ffi::GVolumeMonitor,
            volume: *mut ffi::GVolume,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VolumeMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(volume),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"volume-removed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    volume_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<VolumeMonitor>> VolumeMonitorExt for O {}
