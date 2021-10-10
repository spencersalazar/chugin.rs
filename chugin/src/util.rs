// Utilities for working with Chugins

use crate::chuck;
use crate::cktype::CKType;
    
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
        
pub fn get_next_arg<T: CKType>(args: chuck::Args) -> (chuck::Args, T) {
    T::get_next_arg(args)
}
