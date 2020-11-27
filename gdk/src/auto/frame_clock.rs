// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::FrameClockPhase;
use crate::FrameTimings;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct FrameClock(Object<ffi::GdkFrameClock, ffi::GdkFrameClockClass>);

    match fn {
        get_type => || ffi::gdk_frame_clock_get_type(),
    }
}

impl FrameClock {
    pub fn begin_updating(&self) {
        unsafe {
            ffi::gdk_frame_clock_begin_updating(self.to_glib_none().0);
        }
    }

    pub fn end_updating(&self) {
        unsafe {
            ffi::gdk_frame_clock_end_updating(self.to_glib_none().0);
        }
    }

    pub fn get_current_timings(&self) -> Option<FrameTimings> {
        unsafe {
            from_glib_none(ffi::gdk_frame_clock_get_current_timings(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_frame_counter(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_counter(self.to_glib_none().0) }
    }

    pub fn get_frame_time(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_frame_time(self.to_glib_none().0) }
    }

    pub fn get_history_start(&self) -> i64 {
        unsafe { ffi::gdk_frame_clock_get_history_start(self.to_glib_none().0) }
    }

    pub fn get_timings(&self, frame_counter: i64) -> Option<FrameTimings> {
        unsafe {
            from_glib_none(ffi::gdk_frame_clock_get_timings(
                self.to_glib_none().0,
                frame_counter,
            ))
        }
    }

    pub fn request_phase(&self, phase: FrameClockPhase) {
        unsafe {
            ffi::gdk_frame_clock_request_phase(self.to_glib_none().0, phase.to_glib());
        }
    }

    pub fn connect_after_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn after_paint_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"after-paint\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    after_paint_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_before_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn before_paint_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"before-paint\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    before_paint_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_flush_events<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn flush_events_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"flush-events\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    flush_events_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_layout<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn layout_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"layout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    layout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn paint_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"paint\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    paint_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_resume_events<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resume_events_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"resume-events\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    resume_events_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_update<F: Fn(&FrameClock) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn update_trampoline<F: Fn(&FrameClock) + 'static>(
            this: *mut ffi::GdkFrameClock,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    update_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FrameClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FrameClock")
    }
}
