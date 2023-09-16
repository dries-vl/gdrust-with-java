/* automatically generated by rust-bindgen 0.66.1 */

pub const NO_PROTECTION_DOMAIN: u32 = 0;
pub const NEW_PROTECTION_DOMAIN: i32 = -1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __graal_isolate_t {
    _unused: [u8; 0],
}
pub type graal_isolate_t = __graal_isolate_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __graal_isolatethread_t {
    _unused: [u8; 0],
}
pub type graal_isolatethread_t = __graal_isolatethread_t;
pub type __graal_uword = ::std::os::raw::c_ulonglong;
pub const __graal_create_isolate_params_version: _bindgen_ty_1 = 4;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __graal_create_isolate_params_t {
    pub version: ::std::os::raw::c_int,
    pub reserved_address_space_size: __graal_uword,
    pub auxiliary_image_path: *const ::std::os::raw::c_char,
    pub auxiliary_image_reserved_space_size: __graal_uword,
    pub _reserved_1: ::std::os::raw::c_int,
    pub _reserved_2: *mut *mut ::std::os::raw::c_char,
    pub pkey: ::std::os::raw::c_int,
    pub _reserved_3: ::std::os::raw::c_char,
    pub _reserved_4: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout___graal_create_isolate_params_t() {
    const UNINIT: ::std::mem::MaybeUninit<__graal_create_isolate_params_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__graal_create_isolate_params_t>(),
        56usize,
        concat!("Size of: ", stringify!(__graal_create_isolate_params_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__graal_create_isolate_params_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__graal_create_isolate_params_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reserved_address_space_size) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(reserved_address_space_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).auxiliary_image_path) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(auxiliary_image_path)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).auxiliary_image_reserved_space_size) as usize - ptr as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(auxiliary_image_reserved_space_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._reserved_1) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(_reserved_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._reserved_2) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(_reserved_2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pkey) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(pkey)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._reserved_3) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(_reserved_3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._reserved_4) as usize - ptr as usize },
        53usize,
        concat!(
            "Offset of field: ",
            stringify!(__graal_create_isolate_params_t),
            "::",
            stringify!(_reserved_4)
        )
    );
}
pub type graal_create_isolate_params_t = __graal_create_isolate_params_t;
extern "C" {
    pub fn graal_create_isolate(
        params: *mut graal_create_isolate_params_t,
        isolate: *mut *mut graal_isolate_t,
        thread: *mut *mut graal_isolatethread_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn graal_attach_thread(
        isolate: *mut graal_isolate_t,
        thread: *mut *mut graal_isolatethread_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn graal_get_current_thread(isolate: *mut graal_isolate_t) -> *mut graal_isolatethread_t;
}
extern "C" {
    pub fn graal_get_isolate(thread: *mut graal_isolatethread_t) -> *mut graal_isolate_t;
}
extern "C" {
    pub fn graal_detach_thread(thread: *mut graal_isolatethread_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn graal_tear_down_isolate(
        isolateThread: *mut graal_isolatethread_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn graal_detach_all_threads_and_tear_down_isolate(
        isolateThread: *mut graal_isolatethread_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn run_main(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn filter_env(
        arg1: *mut graal_isolatethread_t,
        arg2: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
