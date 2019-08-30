#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::{c_int, c_uint};

    #[test]
    fn test_smfi_setconn() {
        let bind = CString::new("inet:55522@127.0.0.1").unwrap();
        let bind_ptr = bind.into_raw();
        let setconn_res = unsafe { smfi_setconn(bind_ptr) };
        let _bind = unsafe { CString::from_raw(bind_ptr) };

        assert_eq!(setconn_res, MI_SUCCESS as c_int)
    }

    #[test]
    fn test_smfi_settimeout() {
        let settimeout_res = unsafe { smfi_settimeout(12345 as c_int) };
        assert_eq!(settimeout_res, MI_SUCCESS as c_int)
    }

    #[test]
    fn test_smfi_setbacklog() {
        let setbacklog_res = unsafe { smfi_setbacklog(256 as c_int) };
        assert_eq!(setbacklog_res, MI_SUCCESS as c_int)
    }

    #[test]
    fn test_smfi_setbacklog_error() {
        let setbacklog_res = unsafe { smfi_setbacklog(0 as c_int) };

        assert_eq!(setbacklog_res, MI_FAILURE as c_int)
    }

    #[test]
    fn test_smfi_setdbg() {
        let setdbg_res = unsafe { smfi_setdbg(6 as c_int) };
        assert_eq!(setdbg_res, MI_SUCCESS as c_int)
    }

    #[test]
    fn test_smfi_register() {
        let xxfi_name = CString::new("Milter Name").unwrap();
        let xxfi_name = xxfi_name.into_raw();
        let smfi_desc = smfiDesc {
            xxfi_name,
            xxfi_version: SMFI_VERSION as c_int,
            xxfi_flags: 0,
            xxfi_connect: None,
            xxfi_helo: None,
            xxfi_envfrom: None,
            xxfi_envrcpt: None,
            xxfi_header: None,
            xxfi_eoh: None,
            xxfi_body: None,
            xxfi_eom: None,
            xxfi_abort: None,
            xxfi_close: None,
            xxfi_unknown: None,
            xxfi_data: None,
            xxfi_negotiate: None,
        };
        let _xxfi_name = unsafe { CString::from_raw(xxfi_name) };

        let register_res = unsafe { smfi_register(smfi_desc) };
        assert_eq!(register_res, MI_SUCCESS as c_int)
    }

    #[test]
    fn test_smfi_register_failure() {
        let xxfi_name = CString::new("Milter Name").unwrap();
        let xxfi_name = xxfi_name.into_raw();
        let smfi_desc = smfiDesc {
            xxfi_name,
            xxfi_version: -1 as c_int,
            xxfi_flags: 0,
            xxfi_connect: None,
            xxfi_helo: None,
            xxfi_envfrom: None,
            xxfi_envrcpt: None,
            xxfi_header: None,
            xxfi_eoh: None,
            xxfi_body: None,
            xxfi_eom: None,
            xxfi_abort: None,
            xxfi_close: None,
            xxfi_unknown: None,
            xxfi_data: None,
            xxfi_negotiate: None,
        };
        unsafe { CString::from_raw(xxfi_name) };

        let register_res = unsafe { smfi_register(smfi_desc) };
        assert_eq!(register_res, MI_FAILURE as c_int)
    }

    #[test]
    fn test_smfi_stop() {
        let stop_res = unsafe { smfi_stop() };
        assert_eq!(stop_res, MI_SUCCESS as c_int)
    }

    #[test]
    fn test_smfi_version() {
        let mut major: c_uint = 12345;
        let mut minor: c_uint = 12345;
        let mut patch_level: c_uint = 12345;

        let version_res = unsafe { smfi_version(&mut major, &mut minor, &mut patch_level) };

        assert_eq!(version_res, MI_SUCCESS as c_int);
        assert_eq!(major, 1);
        assert_eq!(minor, 0);
        assert_eq!(patch_level, 1);
    }
}
