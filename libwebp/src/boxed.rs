use std::borrow::{Borrow, BorrowMut};
use std::cmp;
use std::fmt;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::mem::forget;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};
use std::slice;

pub struct WebpBox<T: ?Sized> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T: ?Sized> WebpBox<T> {
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Self {
            ptr: NonNull::new_unchecked(raw),
            _marker: PhantomData,
        }
    }

    pub fn into_raw(b: Self) -> *mut T {
        let ptr = b.ptr;
        forget(b);
        ptr.as_ptr()
    }

    pub fn leak<'a>(b: Self) -> &'a mut T {
        unsafe { &mut *Self::into_raw(b) }
    }
}

impl<T> WebpBox<[T]> {
    pub(crate) unsafe fn from_raw_parts(data: *mut T, len: usize) -> Self {
        let raw = slice::from_raw_parts_mut(data as *mut T, len) as *mut [T];
        Self::from_raw(raw)
    }
}

// unsafe impl<#[may_dangle] T: ?Sized> Drop for WebpBox<T> {
impl<T: ?Sized> Drop for WebpBox<T> {
    #[cfg(not(feature = "0.5"))]
    fn drop(&mut self) {
        use libc::c_void as libc_void;
        use libc::free;
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            free(self.ptr.as_ptr() as *mut libc_void);
        }
    }

    #[cfg(feature = "0.5")]
    fn drop(&mut self) {
        use libwebp_sys::WebPFree;
        use std::os::raw::*;
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
            WebPFree(self.ptr.as_ptr() as *mut c_void);
        }
    }
}

impl<T: ?Sized> Deref for WebpBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for WebpBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> AsRef<T> for WebpBox<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsMut<T> for WebpBox<T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized> Borrow<T> for WebpBox<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> BorrowMut<T> for WebpBox<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

// impl<T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<WebpBox<U>> for WebpBox<T> {}

impl<T: PartialEq<T> + ?Sized> PartialEq<WebpBox<T>> for WebpBox<T> {
    fn eq(&self, other: &WebpBox<T>) -> bool {
        T::eq(self, other)
    }
    fn ne(&self, other: &WebpBox<T>) -> bool {
        T::ne(self, other)
    }
}

impl<T: Eq + ?Sized> Eq for WebpBox<T> {}

impl<T: PartialOrd<T> + ?Sized> PartialOrd<WebpBox<T>> for WebpBox<T> {
    fn partial_cmp(&self, other: &WebpBox<T>) -> Option<cmp::Ordering> {
        T::partial_cmp(self, other)
    }
    fn lt(&self, other: &WebpBox<T>) -> bool {
        T::lt(self, other)
    }
    fn le(&self, other: &WebpBox<T>) -> bool {
        T::le(self, other)
    }
    fn gt(&self, other: &WebpBox<T>) -> bool {
        T::gt(self, other)
    }
    fn ge(&self, other: &WebpBox<T>) -> bool {
        T::ge(self, other)
    }
}

impl<T: Ord + ?Sized> Ord for WebpBox<T> {
    fn cmp(&self, other: &WebpBox<T>) -> cmp::Ordering {
        T::cmp(self, other)
    }
}

impl<T: Iterator + ?Sized> Iterator for WebpBox<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        T::next(self)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        T::size_hint(self)
    }
}

impl<T: ExactSizeIterator + ?Sized> ExactSizeIterator for WebpBox<T> {
    fn len(&self) -> usize {
        T::len(self)
    }
    // fn is_empty(&self) -> bool {
    //     T::is_empty(self)
    // }
}

impl<T: FusedIterator + ?Sized> FusedIterator for WebpBox<T> {}

impl<T: DoubleEndedIterator + ?Sized> DoubleEndedIterator for WebpBox<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        T::next_back(self)
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for WebpBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self as &T, f)
    }
}

impl<T: fmt::Display + ?Sized> fmt::Display for WebpBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self as &T, f)
    }
}

impl<T: ?Sized> fmt::Pointer for WebpBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}
