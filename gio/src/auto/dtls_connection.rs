// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

#[cfg(feature = "v2_70")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
use crate::TlsProtocolVersion;
use crate::{
    ffi, AsyncResult, Cancellable, DatagramBased, TlsCertificate, TlsCertificateFlags, TlsDatabase,
    TlsInteraction, TlsRehandshakeMode,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GDtlsConnection")]
    pub struct DtlsConnection(Interface<ffi::GDtlsConnection, ffi::GDtlsConnectionInterface>) @requires DatagramBased;

    match fn {
        type_ => || ffi::g_dtls_connection_get_type(),
    }
}

impl DtlsConnection {
    pub const NONE: Option<&'static DtlsConnection> = None;
}

pub trait DtlsConnectionExt: IsA<DtlsConnection> + 'static {
    #[doc(alias = "g_dtls_connection_close")]
    fn close(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_dtls_connection_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dtls_connection_close_async")]
    fn close_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn close_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let _ = ffi::g_dtls_connection_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = close_async_trampoline::<P>;
        unsafe {
            ffi::g_dtls_connection_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.close_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_dtls_connection_emit_accept_certificate")]
    fn emit_accept_certificate(
        &self,
        peer_cert: &impl IsA<TlsCertificate>,
        errors: TlsCertificateFlags,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_dtls_connection_emit_accept_certificate(
                self.as_ref().to_glib_none().0,
                peer_cert.as_ref().to_glib_none().0,
                errors.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_dtls_connection_get_certificate")]
    #[doc(alias = "get_certificate")]
    fn certificate(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_dtls_connection_get_certificate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_dtls_connection_get_ciphersuite_name")]
    #[doc(alias = "get_ciphersuite_name")]
    #[doc(alias = "ciphersuite-name")]
    fn ciphersuite_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_dtls_connection_get_ciphersuite_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dtls_connection_get_database")]
    #[doc(alias = "get_database")]
    fn database(&self) -> Option<TlsDatabase> {
        unsafe {
            from_glib_none(ffi::g_dtls_connection_get_database(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dtls_connection_get_interaction")]
    #[doc(alias = "get_interaction")]
    fn interaction(&self) -> Option<TlsInteraction> {
        unsafe {
            from_glib_none(ffi::g_dtls_connection_get_interaction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_dtls_connection_get_negotiated_protocol")]
    #[doc(alias = "get_negotiated_protocol")]
    #[doc(alias = "negotiated-protocol")]
    fn negotiated_protocol(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_dtls_connection_get_negotiated_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dtls_connection_get_peer_certificate")]
    #[doc(alias = "get_peer_certificate")]
    #[doc(alias = "peer-certificate")]
    fn peer_certificate(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_dtls_connection_get_peer_certificate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dtls_connection_get_peer_certificate_errors")]
    #[doc(alias = "get_peer_certificate_errors")]
    #[doc(alias = "peer-certificate-errors")]
    fn peer_certificate_errors(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_dtls_connection_get_peer_certificate_errors(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_dtls_connection_get_protocol_version")]
    #[doc(alias = "get_protocol_version")]
    #[doc(alias = "protocol-version")]
    fn protocol_version(&self) -> TlsProtocolVersion {
        unsafe {
            from_glib(ffi::g_dtls_connection_get_protocol_version(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_64", deprecated = "Since 2.64")]
    #[allow(deprecated)]
    #[doc(alias = "g_dtls_connection_get_rehandshake_mode")]
    #[doc(alias = "get_rehandshake_mode")]
    #[doc(alias = "rehandshake-mode")]
    fn rehandshake_mode(&self) -> TlsRehandshakeMode {
        unsafe {
            from_glib(ffi::g_dtls_connection_get_rehandshake_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dtls_connection_get_require_close_notify")]
    #[doc(alias = "get_require_close_notify")]
    #[doc(alias = "require-close-notify")]
    fn requires_close_notify(&self) -> bool {
        unsafe {
            from_glib(ffi::g_dtls_connection_get_require_close_notify(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dtls_connection_handshake")]
    fn handshake(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_dtls_connection_handshake(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dtls_connection_handshake_async")]
    fn handshake_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn handshake_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let _ =
                ffi::g_dtls_connection_handshake_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = handshake_async_trampoline::<P>;
        unsafe {
            ffi::g_dtls_connection_handshake_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn handshake_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.handshake_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_dtls_connection_set_advertised_protocols")]
    #[doc(alias = "advertised-protocols")]
    fn set_advertised_protocols(&self, protocols: &[&str]) {
        unsafe {
            ffi::g_dtls_connection_set_advertised_protocols(
                self.as_ref().to_glib_none().0,
                protocols.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_dtls_connection_set_certificate")]
    #[doc(alias = "certificate")]
    fn set_certificate(&self, certificate: &impl IsA<TlsCertificate>) {
        unsafe {
            ffi::g_dtls_connection_set_certificate(
                self.as_ref().to_glib_none().0,
                certificate.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_dtls_connection_set_database")]
    #[doc(alias = "database")]
    fn set_database(&self, database: Option<&impl IsA<TlsDatabase>>) {
        unsafe {
            ffi::g_dtls_connection_set_database(
                self.as_ref().to_glib_none().0,
                database.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_dtls_connection_set_interaction")]
    #[doc(alias = "interaction")]
    fn set_interaction(&self, interaction: Option<&impl IsA<TlsInteraction>>) {
        unsafe {
            ffi::g_dtls_connection_set_interaction(
                self.as_ref().to_glib_none().0,
                interaction.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_60", deprecated = "Since 2.60")]
    #[allow(deprecated)]
    #[doc(alias = "g_dtls_connection_set_rehandshake_mode")]
    #[doc(alias = "rehandshake-mode")]
    fn set_rehandshake_mode(&self, mode: TlsRehandshakeMode) {
        unsafe {
            ffi::g_dtls_connection_set_rehandshake_mode(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "g_dtls_connection_set_require_close_notify")]
    #[doc(alias = "require-close-notify")]
    fn set_require_close_notify(&self, require_close_notify: bool) {
        unsafe {
            ffi::g_dtls_connection_set_require_close_notify(
                self.as_ref().to_glib_none().0,
                require_close_notify.into_glib(),
            );
        }
    }

    #[doc(alias = "g_dtls_connection_shutdown")]
    fn shutdown(
        &self,
        shutdown_read: bool,
        shutdown_write: bool,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_dtls_connection_shutdown(
                self.as_ref().to_glib_none().0,
                shutdown_read.into_glib(),
                shutdown_write.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dtls_connection_shutdown_async")]
    fn shutdown_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        shutdown_read: bool,
        shutdown_write: bool,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn shutdown_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let _ =
                ffi::g_dtls_connection_shutdown_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = shutdown_async_trampoline::<P>;
        unsafe {
            ffi::g_dtls_connection_shutdown_async(
                self.as_ref().to_glib_none().0,
                shutdown_read.into_glib(),
                shutdown_write.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn shutdown_future(
        &self,
        shutdown_read: bool,
        shutdown_write: bool,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.shutdown_async(
                    shutdown_read,
                    shutdown_write,
                    io_priority,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    #[doc(alias = "advertised-protocols")]
    fn advertised_protocols(&self) -> Vec<glib::GString> {
        ObjectExt::property(self.as_ref(), "advertised-protocols")
    }

    #[doc(alias = "base-socket")]
    fn base_socket(&self) -> Option<DatagramBased> {
        ObjectExt::property(self.as_ref(), "base-socket")
    }

    #[doc(alias = "accept-certificate")]
    fn connect_accept_certificate<
        F: Fn(&Self, &TlsCertificate, TlsCertificateFlags) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_certificate_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P, &TlsCertificate, TlsCertificateFlags) -> bool + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            peer_cert: *mut ffi::GTlsCertificate,
            errors: ffi::GTlsCertificateFlags,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                DtlsConnection::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(peer_cert),
                from_glib(errors),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"accept-certificate".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    accept_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    #[doc(alias = "advertised-protocols")]
    fn connect_advertised_protocols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_advertised_protocols_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::advertised-protocols".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_advertised_protocols_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "certificate")]
    fn connect_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_certificate_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::certificate".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "ciphersuite-name")]
    fn connect_ciphersuite_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ciphersuite_name_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::ciphersuite-name".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_ciphersuite_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "database")]
    fn connect_database_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_database_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::database".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_database_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "interaction")]
    fn connect_interaction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interaction_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::interaction".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_interaction_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    #[doc(alias = "negotiated-protocol")]
    fn connect_negotiated_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_negotiated_protocol_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::negotiated-protocol".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_negotiated_protocol_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "peer-certificate")]
    fn connect_peer_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_certificate_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::peer-certificate".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_peer_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "peer-certificate-errors")]
    fn connect_peer_certificate_errors_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_certificate_errors_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::peer-certificate-errors".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_peer_certificate_errors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "protocol-version")]
    fn connect_protocol_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_protocol_version_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::protocol-version".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_protocol_version_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v2_60", deprecated = "Since 2.60")]
    #[doc(alias = "rehandshake-mode")]
    fn connect_rehandshake_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rehandshake_mode_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::rehandshake-mode".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_rehandshake_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "require-close-notify")]
    fn connect_require_close_notify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_require_close_notify_trampoline<
            P: IsA<DtlsConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDtlsConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DtlsConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::require-close-notify".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_require_close_notify_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DtlsConnection>> DtlsConnectionExt for O {}
