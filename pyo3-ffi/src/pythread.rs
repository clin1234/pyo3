use core::ffi::c_ulong;
use core::num::NonZero;

extern_libpython! {
    pub fn PyThread_get_thread_ident() -> NonZero<c_ulong>;
}
