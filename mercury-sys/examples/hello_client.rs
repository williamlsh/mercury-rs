use std::{env, ffi::CString, ptr};

static mut COMPLETED: bool = false;
static mut HG_CONTEXT: *mut mercury_sys::hg_context = ptr::null_mut();
static mut RPC_ID: u64 = 0;

fn main() {
    let arg = env::args().nth(1).unwrap();
    unsafe {
        // Initialize an hg_class.
        let na_info = CString::new("tcp").unwrap();
        let hg_class = mercury_sys::HG_Init(na_info.as_ptr(), mercury_sys::HG_FALSE as u8);
        assert!(!hg_class.is_null());

        // Creates a context for the hg_class.
        HG_CONTEXT = mercury_sys::HG_Context_create(hg_class);
        assert!(!HG_CONTEXT.is_null());

        let func_name = CString::new("hello").unwrap();
        RPC_ID = mercury_sys::HG_Register_name(hg_class, func_name.as_ptr(), None, None, None);
        println!("RPC id: {}", RPC_ID);

        let ret = mercury_sys::HG_Registered_disable_response(
            hg_class,
            RPC_ID,
            mercury_sys::HG_TRUE as u8,
        );
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        let server_addr = CString::new(arg).unwrap();
        let ret = mercury_sys::HG_Addr_lookup1(
            HG_CONTEXT,
            Some(lookup_callback),
            ptr::null_mut(),
            server_addr.as_ptr(),
            ptr::null_mut(),
        );
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        // Progress loop.
        loop {
            let mut count = 0;
            loop {
                let ret = mercury_sys::HG_Trigger(HG_CONTEXT, 0, 1, ptr::addr_of_mut!(count));
                if ret == mercury_sys::hg_return_HG_SUCCESS && COMPLETED {
                    break;
                }
            }

            // Make progress on receiving/sending data.
            let ret = mercury_sys::HG_Progress(HG_CONTEXT, 100);
            assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

            if COMPLETED {
                break;
            }
        }

        // Destroy the context.
        let ret = mercury_sys::HG_Context_destroy(HG_CONTEXT);
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        // Finalize the hg_class.
        let ret = mercury_sys::HG_Finalize(hg_class);
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);
    }
}

#[no_mangle]
unsafe extern "C" fn lookup_callback(
    callback_info: *const mercury_sys::hg_cb_info,
) -> mercury_sys::hg_return_t {
    assert_eq!(0, (*callback_info).ret);

    let addr = (*callback_info).info.lookup.addr;
    let mut handle = ptr::null_mut();
    let ret = mercury_sys::HG_Create(HG_CONTEXT, addr, RPC_ID, &mut handle);
    assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

    let ret = mercury_sys::HG_Forward(handle, None, ptr::null_mut(), ptr::null_mut());
    assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

    let ret = mercury_sys::HG_Destroy(handle);
    assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

    COMPLETED = true;
    mercury_sys::hg_return_HG_SUCCESS
}
