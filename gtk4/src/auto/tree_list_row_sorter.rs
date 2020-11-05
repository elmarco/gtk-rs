// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Sorter;

glib_wrapper! {
    pub struct TreeListRowSorter(Object<gtk_sys::GtkTreeListRowSorter, gtk_sys::GtkTreeListRowSorterClass, TreeListRowSorterClass>) @extends Sorter;

    match fn {
        get_type => || gtk_sys::gtk_tree_list_row_sorter_get_type(),
    }
}

impl TreeListRowSorter {
    pub fn new<P: IsA<Sorter>>(sorter: Option<&P>) -> TreeListRowSorter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_list_row_sorter_new(
                sorter.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct TreeListRowSorterBuilder {
    sorter: Option<Sorter>,
}

impl TreeListRowSorterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> TreeListRowSorter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref sorter) = self.sorter {
            properties.push(("sorter", sorter));
        }
        let ret = glib::Object::new(TreeListRowSorter::static_type(), &properties)
            .expect("object new")
            .downcast::<TreeListRowSorter>()
            .expect("downcast");
        ret
    }

    pub fn sorter<P: IsA<Sorter>>(mut self, sorter: &P) -> Self {
        self.sorter = Some(sorter.clone().upcast());
        self
    }
}

pub const NONE_TREE_LIST_ROW_SORTER: Option<&TreeListRowSorter> = None;

pub trait TreeListRowSorterExt: 'static {
    fn get_sorter(&self) -> Option<Sorter>;

    fn set_sorter<P: IsA<Sorter>>(&self, sorter: Option<&P>);

    fn connect_property_sorter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeListRowSorter>> TreeListRowSorterExt for O {
    fn get_sorter(&self) -> Option<Sorter> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tree_list_row_sorter_get_sorter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_sorter<P: IsA<Sorter>>(&self, sorter: Option<&P>) {
        unsafe {
            gtk_sys::gtk_tree_list_row_sorter_set_sorter(
                self.as_ref().to_glib_none().0,
                sorter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_property_sorter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sorter_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkTreeListRowSorter,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TreeListRowSorter>,
        {
            let f: &F = &*(f as *const F);
            f(&TreeListRowSorter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sorter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sorter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TreeListRowSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeListRowSorter")
    }
}
