
// include chuck_dl.h and associated bindings 
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::result::Result;
use std::ffi::{CString, CStr};

// basic ChucK types not automatically imported by bindgen (due to #define)
pub type t_CKUINT = ::std::os::raw::c_ulong;
pub type t_CKBOOL = ::std::os::raw::c_ulong;

pub const CK_TRUE: t_CKBOOL = 1;
pub const CK_FALSE: t_CKBOOL = 0;

// chuck version is #define-d, so not supported by bindgen
// major version must be the same between chuck:chugin
const CK_DLL_VERSION_MAJOR: t_CKUINT = 0x0008;
// minor version of chugin must be less than or equal to chuck's
const CK_DLL_VERSION_MINOR: t_CKUINT = 0x0000;

fn CK_DLL_VERSION_MAKE(maj: t_CKUINT, min: t_CKUINT) -> t_CKUINT {
    (maj << 16) | min
}

/// Return Chugin API version; to be returned from a public ck_version function
pub fn version() -> t_CKUINT {
    CK_DLL_VERSION_MAKE(CK_DLL_VERSION_MAJOR, CK_DLL_VERSION_MINOR)
}

/// Chugin result type
pub type CKResult<T=(), E=&'static str> = Result<T, E>;

/// internal function to convert CStr/CString -> array of bytes C-string
fn c_str(s: &CStr) -> *const i8 {
    &s.to_bytes_with_nul()[0] as *const u8 as *const i8
}

/// Chugin Query wrapper class
pub struct Query {
    query: *mut Chuck_DL_Query,
}

/// Chugin Query wrapper class
impl Query {
    
    /// Create new wrapper from ChucK type
    pub fn new(QUERY: *mut Chuck_DL_Query) -> CKResult<Query> {
        if !QUERY.is_null() {
            Ok(Query {
                query: QUERY
            })
        } else {
            Err("invalid query object provided")
        }
    }
    
    /// Begin a new class
    pub fn begin_class(&self, name: &str, parent: &str) -> CKResult {
        
        let name = match CString::new(name) {
            Ok(s) => s,
            Err(_) => return Err("unable to convert C-string: name")
        };
        
        let parent = match CString::new(parent) {
            Ok(s) => s,
            Err(_) => return Err("unable to convert C-string: parent")
        };
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let begin_class = match query.begin_class {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            begin_class(self.query, c_str(&name), c_str(&parent));
        }
        
        Ok(())
    }
    
    /// Add a constructor for the class that is currently being constructed
    pub fn add_ctor(&self, ctor: f_ctor) -> CKResult {
                
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
    pub fn add_dtor(&self, dtor: f_dtor) -> CKResult {
                
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
    pub fn add_mvar(&self, type_: &str, name: &str, is_const: bool) -> CKResult<t_CKUINT> {
        
        let type_ = match CString::new(type_) {
            Ok(s) => s,
            Err(_) => return Err("unable to convert C-string: type")
        };
        
        let name = match CString::new(name) {
            Ok(s) => s,
            Err(_) => return Err("unable to convert C-string: name")
        };
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_mvar = match query.add_mvar {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        Ok(unsafe {
            add_mvar(self.query, c_str(&type_), c_str(&name), if is_const { 1 } else { 0 })
        })
    }
    
    /// Add a tick function for the class that is being constructed
    pub fn add_ugen_func(&self, tick: f_tick, num_in: u32, num_out: u32) -> CKResult {
        
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
    use super::Chuck_Object;
    
    /// Set a data member variable in a ChucK object
    /// Note: the type in obj needs to be manually dropped/dealloced at some point
    pub unsafe fn set_object_data<T>(ck_obj: *mut Chuck_Object, offset: usize, obj: Box<T>) {
        let data = (*ck_obj).data.offset(offset as isize);
        let ptr = data as *mut usize;
        *ptr = Box::into_raw(obj) as *mut T as usize;
    }
    
    /// Get a data member variable in a ChucK object
    /// Note: Box<T> will automatically drop/dealloc the object unless you call Box::into_raw on it
    pub unsafe fn get_object_data<T>(ck_obj: *const Chuck_Object, offset: usize) -> Box<T> {
        let data = (*ck_obj).data.offset(offset as isize);
        let ptr = data as *const usize;
        Box::from_raw(*ptr as *mut T)
    }
}

