#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::result::Result;
use std::ffi::{CString, CStr};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// basic ChucK types not automatically imported by bindgen (due to #define)
type t_CKUINT = ::std::os::raw::c_ulong;
type t_CKBOOL = ::std::os::raw::c_ulong;

const CK_TRUE: t_CKBOOL = 1;
const CK_FALSE: t_CKBOOL = 0;

// chuck version is #define-d, so not supported by bindgen
// major version must be the same between chuck:chugin
const CK_DLL_VERSION_MAJOR: t_CKUINT = 0x0008;
// minor version of chugin must be less than or equal to chuck's
const CK_DLL_VERSION_MINOR: t_CKUINT = 0x0000;

fn CK_DLL_VERSION_MAKE(maj: t_CKUINT, min: t_CKUINT) -> t_CKUINT {
    (maj << 16) | min
}

type CKResult<T=(), E=&'static str> = Result<T, E>;

fn c_str(s: &CStr) -> *const i8 {
    &s.to_bytes_with_nul()[0] as *const u8 as *const i8
}

static mut data_offset: usize = 0;

struct MyChugin {
    freq: f32,
    amp: f32,
    phase: f32,
    srate: f32,
    phase_update: f32,
}

impl MyChugin {
    pub fn new(srate: f32, freq: f32, amp: f32) -> MyChugin {
        MyChugin {
            srate: srate,
            freq: freq,
            amp: amp,
            phase: 0.0,
            phase_update: freq/srate,
        }
    }
    
    fn update_phase_update(&mut self) {
        self.phase_update = self.freq/self.srate;
    }
    
    pub fn set_freq(&mut self, freq: f32) -> f32 {
        self.freq = freq;
        self.update_phase_update();
        self.freq
    }
    
    pub fn get_freq(&self) -> f32 {
        self.freq
    }
    
    pub fn tick(&mut self) -> f32 {
        let y = -1.0+self.phase;
        self.phase += self.phase_update;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        y*self.amp
    }
}

/// Chugin Query wrapper class
struct ChuginQuery {
    query: *mut Chuck_DL_Query,
}

/// Chugin Query wrapper class
impl ChuginQuery {
    pub fn new(QUERY: *mut Chuck_DL_Query) -> CKResult<ChuginQuery> {        
        if !QUERY.is_null() {
            Ok(ChuginQuery {
                query: QUERY
            })
        } else {
            Err("invalid query object provided")
        }
    }
    
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

fn ck_query_impl(QUERY: *mut Chuck_DL_Query) -> CKResult {
    let q = ChuginQuery::new(QUERY)?;
    
    q.begin_class("RustOsc", "UGen")?;
    q.add_ctor(Some(ctor))?;
    q.add_dtor(Some(dtor))?;
    let offset = q.add_mvar("int", "@data", false)? as usize;
    unsafe { 
        data_offset = offset;
    }
    q.add_ugen_func(Some(tick), 0, 1)?;
    q.end_class()?;
    
    Ok(())
}

#[no_mangle]
pub extern "C" fn ck_version() -> t_CKUINT {
    CK_DLL_VERSION_MAKE(CK_DLL_VERSION_MAJOR, CK_DLL_VERSION_MINOR)
}

#[no_mangle]
pub extern "C" fn ck_query(QUERY: *mut Chuck_DL_Query) -> t_CKBOOL {
    println!("hello, chuck!");
    
    match ck_query_impl(QUERY) {
        Ok(_) => CK_TRUE,
        Err(_) => CK_FALSE,
    }
}

#[no_mangle]
pub extern "C" fn ctor(SELF: *mut Chuck_Object,
        _ARGS: *mut ::std::os::raw::c_void,
        _VM: *mut Chuck_VM,
        _SHRED: *mut Chuck_VM_Shred,
        _API: CK_DL_API) {
    let mut chugin = Box::new(MyChugin::new(44100.0, 200.0, 1.0));
    
    let data_offset_ = unsafe { data_offset } as isize;
    let data = (*SELF).data.offset(data_offset_);
    let obj = data as *mut MyChugin;
    *obj = Box::leak(chugin) as *mut MyChugin;
}

#[no_mangle]
pub extern "C" fn dtor(_SELF: *mut Chuck_Object,
        _VM: *mut Chuck_VM,
        _SHRED: *mut Chuck_VM_Shred,
        _API: CK_DL_API) {
}

#[no_mangle]
extern "C" fn tick(
    _SELF: *mut Chuck_Object,
    _in_: f32,
    out: *mut f32,
    _API: CK_DL_API,
) -> t_CKBOOL {
    unsafe {
        *out = 0.0;
    }
    
    CK_TRUE
}
