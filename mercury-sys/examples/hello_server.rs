use std::{ffi::CString, ptr};

const TOTAL_RPCS: u32 = 2;
static mut NUM_RPCS: u32 = 0;

fn main() {
    unsafe {
        // Initialize Mercury and get an hg_class handle.
        let na_info = CString::new("tcp").unwrap();
        let hg_class = mercury_sys::HG_Init(na_info.as_ptr(), mercury_sys::HG_TRUE as u8);
        assert!(!hg_class.is_null());

        // Get the address of the server.
        let mut addr = ptr::null_mut();
        let ret = mercury_sys::HG_Addr_self(hg_class, &mut addr);
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        let hostname = Vec::with_capacity(128).as_mut_ptr();
        let mut hostname_size: u64 = 128;
        let ret = mercury_sys::HG_Addr_to_string(
            hg_class,
            hostname,
            ptr::addr_of_mut!(hostname_size),
            addr,
        );
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        let hostname = CString::from_raw(hostname);
        println!(
            "Server running at address {}",
            hostname.into_string().unwrap()
        );
        let ret = mercury_sys::HG_Addr_free(hg_class, addr);
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        // Creates a Mercury context from the Mercury class.
        let hg_context = mercury_sys::HG_Context_create(hg_class);
        assert!(!hg_context.is_null());

        let func_name = CString::new("hello").unwrap();
        let rpc_id = mercury_sys::HG_Register_name(
            hg_class,
            func_name.as_ptr(),
            None,
            None,
            Some(hello_world),
        );
        println!("RPC id: {}", rpc_id);

        let ret = mercury_sys::HG_Registered_disable_response(
            hg_class,
            rpc_id,
            mercury_sys::HG_TRUE as u8,
        );
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        // Progress loop.
        loop {
            let mut count = 0;
            loop {
                let ret = mercury_sys::HG_Trigger(hg_context, 0, 1, ptr::addr_of_mut!(count));
                if ret == mercury_sys::hg_return_HG_SUCCESS && count == 1 {
                    break;
                }
            }

            // Make progress on receiving/sending data.
            let ret = mercury_sys::HG_Progress(hg_context, 100);
            assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

            if NUM_RPCS == TOTAL_RPCS {
                break;
            }
        }

        // Destroys the Mercury context.
        let ret = mercury_sys::HG_Context_destroy(hg_context);
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

        // Finalize Mercury.
        let ret = mercury_sys::HG_Finalize(hg_class);
        assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);
    }
}

#[no_mangle]
unsafe extern "C" fn hello_world(handle: mercury_sys::hg_handle_t) -> mercury_sys::hg_return_t {
    println!("RPC server side is called.");
    NUM_RPCS += 1;

    let ret = mercury_sys::HG_Destroy(handle);
    assert_eq!(mercury_sys::hg_return_HG_SUCCESS, ret);

    mercury_sys::hg_return_HG_SUCCESS
}
