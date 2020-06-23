#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type MaybeWrapped<A> = A;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rooted<T> {
    pub ptr: MaybeWrapped<T>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Rooted<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
