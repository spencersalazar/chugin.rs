#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// include chuck_dl.h and associated bindings 
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub type DL_Query = Chuck_DL_Query;
pub type DL_Api = Chuck_DL_Api_Api;
pub type VM = Chuck_VM;
pub type VM_Shred = Chuck_VM_Shred;
pub type Object = Chuck_Object;

// basic ChucK types not automatically imported by bindgen (due to #define)
pub type t_CKUINT = ::std::os::raw::c_ulong;
pub type t_CKBOOL = ::std::os::raw::c_ulong;
pub type Float = f64;
pub type Args = *mut ::std::os::raw::c_void;

pub const CK_TRUE: t_CKBOOL = 1;
pub const CK_FALSE: t_CKBOOL = 0;
