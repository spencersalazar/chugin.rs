use std::ffi;
use crate::CKResult;

/// Wrapper to encapsulate ffi::CString + easily convert to byte representation
/// Essentially provides ?-compatible error handling for creation and a 
/// container to manage the data while passing the byte representation to C 
/// functions. 
pub struct CString {
    cstring: ffi::CString,
}

impl CString {
    pub fn new(s: &str) -> CKResult<CString> {
        let s = match ffi::CString::new(s) {
            Ok(s) => s,
            Err(_) => return Err("unable to convert C-string: name")
        };
        
        return Ok(CString { cstring: s });
    }
    
    pub fn c_str(&self) -> *const i8 {
        &self.cstring.to_bytes_with_nul()[0] as *const u8 as *const i8
    }
}
