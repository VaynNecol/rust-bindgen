#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(not(test))]

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct JNINativeInterface_ {
    pub GetVersion: ::std::option::Option<
        unsafe extern "stdcall" fn(
            env: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub __hack: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_JNINativeInterface_() {
    assert_eq!(
        ::std::mem::size_of::<JNINativeInterface_>(),
        16usize,
        concat!("Size of: ", stringify!(JNINativeInterface_))
    );
    assert_eq!(
        ::std::mem::align_of::<JNINativeInterface_>(),
        8usize,
        concat!("Alignment of ", stringify!(JNINativeInterface_))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<JNINativeInterface_>())).GetVersion
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JNINativeInterface_),
            "::",
            stringify!(GetVersion)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<JNINativeInterface_>())).__hack as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JNINativeInterface_),
            "::",
            stringify!(__hack)
        )
    );
}
extern "stdcall" {
    pub fn bar();
}
