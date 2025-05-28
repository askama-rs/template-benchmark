//! This quick-and-dirty implementation is propably error-ridden and unsafe!
//! DO NOT USE IN PRODUCTION!

#[doc(hidden)]
pub mod __reexport {
    pub use std::mem::transmute;
    pub use std::option::Option::Some;
    pub use std::sync::Arc;

    pub use parking_lot::Mutex;
    pub use scopeguard::guard;

    pub use super::Lifetimeless;
}

use std::ops::Deref;
use std::sync::Arc;

use parking_lot::{ArcMutexGuard, Mutex, RawMutex};

pub struct Lifetimeless<T: 'static>(ArcMutexGuard<RawMutex, Option<&'static T>>);

impl<T: 'static> Lifetimeless<T> {
    #[doc(hidden)]
    #[inline]
    #[track_caller]
    pub unsafe fn _new(arc: &Arc<Mutex<Option<&'static T>>>) -> Self {
        Self(arc.try_lock_arc().unwrap())
    }
}

impl<T: PartialEq + 'static> Deref for Lifetimeless<T> {
    type Target = T;

    #[inline]
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.0.as_deref().unwrap()
    }
}

impl<T: PartialEq + 'static> PartialEq for Lifetimeless<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self.0.as_deref(), other.0.as_deref()) {
            (Some(lhs), Some(rhs)) => lhs == rhs,
            (None, _) | (_, None) => false,
        }
    }
}

#[macro_export]
macro_rules! lifetimeless {
    ($ident:ident: &$ty:ty) => {
        let $ident: &$ty = $ident;
        let $ident = $crate::lifetimeless::__reexport::Arc::new(
            $crate::lifetimeless::__reexport::Mutex::new($crate::lifetimeless::__reexport::Some(
                unsafe {
                    $crate::lifetimeless::__reexport::transmute::<&$ty, &'static $ty>($ident)
                },
            )),
        );
        let $ident = $crate::lifetimeless::__reexport::guard($ident, |$ident| {
            let _: &$ty = $ident.try_lock().unwrap().take().unwrap();
        });
        let $ident = unsafe { $crate::lifetimeless::__reexport::Lifetimeless::_new(&$ident) };
    };
}
