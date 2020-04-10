// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use Action;

glib_wrapper! {
    pub struct SimpleAction(Object<gio_sys::GSimpleAction, SimpleActionClass>) @implements Action;

    match fn {
        get_type => || gio_sys::g_simple_action_get_type(),
    }
}

impl SimpleAction {
    pub fn new(name: &str, parameter_type: Option<&glib::VariantTy>) -> SimpleAction {
        unsafe {
            from_glib_full(gio_sys::g_simple_action_new(
                name.to_glib_none().0,
                parameter_type.to_glib_none().0,
            ))
        }
    }

    pub fn new_stateful(
        name: &str,
        parameter_type: Option<&glib::VariantTy>,
        state: &glib::Variant,
    ) -> SimpleAction {
        unsafe {
            from_glib_full(gio_sys::g_simple_action_new_stateful(
                name.to_glib_none().0,
                parameter_type.to_glib_none().0,
                state.to_glib_none().0,
            ))
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            gio_sys::g_simple_action_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_state(&self, value: &glib::Variant) {
        unsafe {
            gio_sys::g_simple_action_set_state(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub fn set_state_hint(&self, state_hint: Option<&glib::Variant>) {
        unsafe {
            gio_sys::g_simple_action_set_state_hint(
                self.to_glib_none().0,
                state_hint.to_glib_none().0,
            );
        }
    }

    pub fn connect_activate<F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<
            F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static,
        >(
            this: *mut gio_sys::GSimpleAction,
            parameter: *mut glib_sys::GVariant,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Variant>::from_glib_borrow(parameter)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(*(&activate_trampoline::<F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_change_state<F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn change_state_trampoline<
            F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static,
        >(
            this: *mut gio_sys::GSimpleAction,
            value: *mut glib_sys::GVariant,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Variant>::from_glib_borrow(value)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-state\0".as_ptr() as *const _,
                Some(*(&change_state_trampoline::<F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_enabled_notify<F: Fn(&SimpleAction) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<F: Fn(&SimpleAction) + 'static>(
            this: *mut gio_sys::GSimpleAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enabled\0".as_ptr() as *const _,
                Some(*(&notify_enabled_trampoline::<F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_state_type_notify<F: Fn(&SimpleAction) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_type_trampoline<F: Fn(&SimpleAction) + 'static>(
            this: *mut gio_sys::GSimpleAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state-type\0".as_ptr() as *const _,
                Some(*(&notify_state_type_trampoline::<F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SimpleAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleAction")
    }
}
