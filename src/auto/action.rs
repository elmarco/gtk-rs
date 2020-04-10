// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Action(Interface<gio_sys::GAction>);

    match fn {
        get_type => || gio_sys::g_action_get_type(),
    }
}

impl Action {
    pub fn name_is_valid(action_name: &str) -> bool {
        unsafe {
            from_glib(gio_sys::g_action_name_is_valid(
                action_name.to_glib_none().0,
            ))
        }
    }

    pub fn parse_detailed_name(
        detailed_name: &str,
    ) -> Result<(GString, glib::Variant), glib::Error> {
        unsafe {
            let mut action_name = ptr::null_mut();
            let mut target_value = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_action_parse_detailed_name(
                detailed_name.to_glib_none().0,
                &mut action_name,
                &mut target_value,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(action_name), from_glib_full(target_value)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn print_detailed_name(
        action_name: &str,
        target_value: Option<&glib::Variant>,
    ) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_action_print_detailed_name(
                action_name.to_glib_none().0,
                target_value.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_ACTION: Option<&Action> = None;

pub trait ActionExt: 'static {
    fn activate(&self, parameter: Option<&glib::Variant>);

    fn change_state(&self, value: &glib::Variant);

    fn get_enabled(&self) -> bool;

    fn get_name(&self) -> Option<GString>;

    fn get_parameter_type(&self) -> Option<glib::VariantType>;

    fn get_state(&self) -> Option<glib::Variant>;

    fn get_state_hint(&self) -> Option<glib::Variant>;

    fn get_state_type(&self) -> Option<glib::VariantType>;

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parameter_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Action>> ActionExt for O {
    fn activate(&self, parameter: Option<&glib::Variant>) {
        unsafe {
            gio_sys::g_action_activate(self.as_ref().to_glib_none().0, parameter.to_glib_none().0);
        }
    }

    fn change_state(&self, value: &glib::Variant) {
        unsafe {
            gio_sys::g_action_change_state(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_enabled(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_action_get_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe { from_glib_none(gio_sys::g_action_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn get_parameter_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(gio_sys::g_action_get_parameter_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_state(&self) -> Option<glib::Variant> {
        unsafe { from_glib_full(gio_sys::g_action_get_state(self.as_ref().to_glib_none().0)) }
    }

    fn get_state_hint(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(gio_sys::g_action_get_state_hint(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_state_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(gio_sys::g_action_get_state_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enabled\0".as_ptr() as *const _,
                Some(*(&notify_enabled_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(*(&notify_name_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parameter_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_parameter_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parameter-type\0".as_ptr() as *const _,
                Some(*(&notify_parameter_type_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(*(&notify_state_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state-type\0".as_ptr() as *const _,
                Some(*(&notify_state_type_trampoline::<Self, F> as *const _ as *const _)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}
