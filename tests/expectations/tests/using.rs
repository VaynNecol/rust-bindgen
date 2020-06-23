#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Point<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type IntPoint2D = Point<::std::os::raw::c_int>;
pub type IntVec2D = Point<::std::os::raw::c_int>;
