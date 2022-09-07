///! Rust safe encapsulation over libmercury C apis.
///! Current progress covers only a part of Mercury RPC Layer.
///! See https://mercury-hpc.github.io/user/hg for more information.
pub mod error;

use std::{ffi::CString, ptr};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

type TriggerResult = u32;
type TriggerCount = u32;

pub struct HG {
    hg_class: *mut mercury_sys::hg_class,
    hg_context: Option<*mut mercury_sys::hg_context>,
}

impl HG {
    pub fn init(na_info: &str, na_listen: bool) -> Result<HG> {
        unsafe {
            let na_info = CString::new(na_info).unwrap();
            let na_listen = if na_listen { 1 } else { 0 };
            let hg_class = mercury_sys::HG_Init(na_info.as_ptr(), na_listen);
            if hg_class.is_null() {
                Err(error::Error::InitFailed.into())
            } else {
                Ok(HG {
                    hg_class,
                    hg_context: None,
                })
            }
        }
    }

    pub fn target_addr(&self) -> Result<String> {
        unsafe {
            let mut addr = ptr::null_mut();
            let ret = mercury_sys::HG_Addr_self(self.hg_class, &mut addr);
            if ret != 0 {
                return Err(error::Error::HgAddrSelfErr(ret).into());
            }

            let addr_str = Vec::with_capacity(128).as_mut_ptr();
            let mut addr_str_size: u64 = 128;
            let ret = mercury_sys::HG_Addr_to_string(
                self.hg_class,
                addr_str,
                ptr::addr_of_mut!(addr_str_size),
                addr,
            );
            if ret != 0 {
                return Err(error::Error::HgAddrToStrErr(ret).into());
            }

            let addr_str = CString::from_raw(addr_str).into_string()?;
            let ret = mercury_sys::HG_Addr_free(self.hg_class, addr);
            if ret != 0 {
                return Err(error::Error::HgAddrFreeErr(ret).into());
            }

            Ok(addr_str)
        }
    }

    pub fn create_context(self) -> Result<HG> {
        unsafe {
            let hg_context = mercury_sys::HG_Context_create(self.hg_class);
            if hg_context.is_null() {
                return Err(error::Error::HgContextCreateFailed.into());
            }
            Ok(HG {
                hg_context: Some(hg_context),
                ..self
            })
        }
    }

    pub fn trigger(&self, timeout: u32, max_count: u32) -> Result<(TriggerResult, TriggerCount)> {
        let hg_context = match self.hg_context {
            Some(hg_context) => hg_context,
            None => return Err(error::Error::UninitializedContext.into()),
        };
        unsafe {
            let mut actual_count = 0;
            let ret = mercury_sys::HG_Trigger(
                hg_context,
                timeout,
                max_count,
                ptr::addr_of_mut!(actual_count),
            );
            Ok((ret, actual_count))
        }
    }

    pub fn progress(&self) -> Result<()> {
        let hg_context = match self.hg_context {
            Some(hg_context) => hg_context,
            None => return Err(error::Error::UninitializedContext.into()),
        };
        unsafe {
            let ret = mercury_sys::HG_Progress(hg_context, 100);
            if ret != 0 {
                return Err(error::Error::ProgressErr(ret).into());
            }
            Ok(())
        }
    }

    pub fn destroy_context(&self) {
        unsafe {
            if let Some(hg_context) = self.hg_context {
                let ret = mercury_sys::HG_Context_destroy(hg_context);
                assert_eq!(0, ret);
            }
        }
    }

    pub fn finalize(&self) {
        unsafe {
            let ret = mercury_sys::HG_Finalize(self.hg_class);
            assert_eq!(0, ret);
        }
    }
}
