pub mod chuck;
pub mod fn_macros;

use std::result::Result;
use std::ffi;

pub use macros::{query_fn};

// chuck version is #define-d, so not supported by bindgen
// major version must be the same between chuck:chugin
const CK_DLL_VERSION_MAJOR: chuck::t_CKUINT = 0x0008;
// minor version of chugin must be less than or equal to chuck's
const CK_DLL_VERSION_MINOR: chuck::t_CKUINT = 0x0000;

fn ck_dll_version_make(maj: chuck::t_CKUINT, min: chuck::t_CKUINT) -> chuck::t_CKUINT {
    (maj << 16) | min
}

/// Return Chugin API version; to be returned from a public ck_version function
pub fn version() -> chuck::t_CKUINT {
    ck_dll_version_make(CK_DLL_VERSION_MAJOR, CK_DLL_VERSION_MINOR)
}

/// Chugin result type
pub type CKResult<T=(), E=&'static str> = Result<T, E>;

/// Wrapper to encapsulate ffi::CString + easily convert to byte representation
/// Essentially provides ?-compatible error handling for creation and a 
/// container to manage the data while passing the byte representation to C 
/// functions. 
struct CString {
    cstring: ffi::CString,
}

impl CString {
    fn new(s: &str) -> CKResult<CString> {
        let s = match ffi::CString::new(s) {
            Ok(s) => s,
            Err(_) => return Err("unable to convert C-string: name")
        };
        
        return Ok(CString { cstring: s });
    }
    
    fn c_str(&self) -> *const i8 {
        &self.cstring.to_bytes_with_nul()[0] as *const u8 as *const i8
    }
}

/// Chugin Query wrapper class
pub struct Query {
    query: *mut chuck::DL_Query,
}

/// Chugin Query wrapper class
impl Query {
    
    /// Create new wrapper from ChucK type
    pub fn new(query: *mut chuck::DL_Query) -> CKResult<Query> {
        if !query.is_null() {
            Ok(Query {
                query: query
            })
        } else {
            Err("invalid query object provided")
        }
    }
    
    /// Begin a new class
    pub fn begin_class(&self, name: &str, parent: &str) -> CKResult {
        
        let name = CString::new(name)?;
        let parent = CString::new(parent)?;
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let begin_class = match query.begin_class {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            begin_class(self.query, name.c_str(), parent.c_str());
        }
        
        Ok(())
    }
    
    /// Add a constructor for the class that is currently being constructed
    pub fn add_ctor(&self, ctor: chuck::f_ctor) -> CKResult {
                
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_ctor = match query.add_ctor {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            add_ctor(self.query, ctor);
        }
        
        Ok(())
    }
    
    /// Add a destructor for the class that is currently being constructed
    pub fn add_dtor(&self, dtor: chuck::f_dtor) -> CKResult {
                
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_dtor = match query.add_dtor {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            add_dtor(self.query, dtor);
        }
        
        Ok(())
    }
    
    /// Add a member variable for the class that is being constructed
    pub fn add_mvar(&self, type_: &str, name: &str, is_const: bool) -> CKResult<chuck::t_CKUINT> {
        
        let type_ = CString::new(type_)?;
        let name = CString::new(name)?;
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_mvar = match query.add_mvar {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        Ok(unsafe {
            add_mvar(self.query, type_.c_str(), name.c_str(), if is_const { 1 } else { 0 })
        })
    }
    
    /// Add a tick function for the class that is being constructed
    pub fn add_mfun(&self, 
        mfun: chuck::f_mfun, 
        type_: &str, name: &str,
        args: &[(String,String)]
    ) -> CKResult {
        
        let type_ = CString::new(type_)?;
        let name = CString::new(name)?;
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_mfun = match query.add_mfun {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        let add_arg = match query.add_arg {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            add_mfun(self.query, mfun, type_.c_str(), name.c_str());
        }
        
        for arg in args {
            let type_ = CString::new(&arg.0)?;
            let name = CString::new(&arg.1)?;
            
            unsafe {
                add_arg(self.query, type_.c_str(), name.c_str());
            }
        }
        
        Ok(())
    }
    
    /// Add a tick function for the class that is being constructed
    pub fn add_ugen_func(&self, tick: chuck::f_tick, num_in: u32, num_out: u32) -> CKResult {
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_ugen_func = match query.add_ugen_func {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            add_ugen_func(self.query, tick, None, num_in.into(), num_out.into());
        }
        
        Ok(())
    }
    
    /// End a class that is being constructed
    pub fn end_class(&self) -> CKResult {
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let end_class = match query.end_class {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        match unsafe { end_class(self.query) } {
            0 => Err("failed to end_class"),
            _ => Ok(()),
        }
    }
}

/// Utilities for working with Chugins
pub mod util {
    use crate::chuck;
    
    /// Set a data member variable in a ChucK object
    /// Note: the type in obj needs to be manually dropped/dealloced at some point
    pub unsafe fn set_object_data<T>(ck_obj: *mut chuck::Object, offset: usize, obj: Box<T>) {
        let data = (*ck_obj).data.offset(offset as isize);
        let ptr = data as *mut usize;
        *ptr = Box::into_raw(obj) as *mut T as usize;
    }
    
    /// Get a data member variable in a ChucK object
    /// Note: Box<T> will automatically drop/dealloc the object unless you call Box::into_raw on it
    pub unsafe fn get_object_data<T>(ck_obj: *const chuck::Object, offset: usize) -> Box<T> {
        let data = (*ck_obj).data.offset(offset as isize);
        let ptr = data as *const usize;
        Box::from_raw(*ptr as *mut T)
    }
    
    unsafe fn impl_get_next_arg<T: Copy>(args: chuck::Args) -> (T, chuck::Args) {
        let arg: T = *(args as *mut T);
        let args = args.offset(1);
        (arg, args)
    }
        
    pub unsafe fn get_next_float(args: chuck::Args) -> (chuck::Float, chuck::Args) {
        impl_get_next_arg(args)
    }
}

