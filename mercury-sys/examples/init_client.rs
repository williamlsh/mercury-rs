use std::ffi::CString;

fn main() {
    unsafe {
        // Initialize an hg_class.
        let na_info = CString::new("tcp").unwrap();
        let hg_class = mercury_sys::HG_Init(na_info.as_ptr(), mercury_sys::HG_FALSE as u8);
        assert!(!hg_class.is_null());

        // Creates a context for the hg_class.
        let hg_context = mercury_sys::HG_Context_create(hg_class);
        assert!(!hg_context.is_null());

        // Destroy the context.
        let ret = mercury_sys::HG_Context_destroy(hg_context);
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        // Finalize the hg_class.
        let ret = mercury_sys::HG_Finalize(hg_class);
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);
    }
}
