
#[macro_export]
macro_rules! query {
    ($ck_query:ident, $query:expr)=>{
        #[no_mangle]
        pub extern "C" fn ck_version() -> chuck::t_CKUINT {
            chugin::version()
        }
        
        #[no_mangle]
        pub extern "C" fn ck_query($ck_query: *mut chuck::DL_Query) -> chuck::t_CKBOOL {
            match $query {
                Ok(_) => chuck::CK_TRUE,
                Err(_) => chuck::CK_FALSE,
            }
        }
    }
}

#[macro_export]
macro_rules! ctor {
    ($ident:ident, $offset:expr, $obj:expr)=>{
        #[no_mangle]
        pub extern "C" fn $ident(ck_self: *mut chuck::Object,
                _args: *mut ::std::os::raw::c_void,
                _vm: *mut chuck::VM,
                _shred: *mut chuck::VM_Shred,
                _api: chuck::CK_DL_API) {
            let obj = Box::new($obj);
    
            unsafe {
                chugin::util::set_object_data(ck_self, $offset, obj);
            }
        }
    }
}

#[macro_export]
macro_rules! dtor {
    ($ident:ident, $offset:expr, $t:ty, $obj:ident, $code:stmt)=>{
        #[no_mangle]
        pub extern "C" fn $ident(ck_self: *mut chuck::Object,
                _vm: *mut chuck::VM,
                _shred: *mut chuck::VM_Shred,
                _api: chuck::CK_DL_API) {
    
            let $obj: Box<$t> = unsafe {
                chugin::util::get_object_data(ck_self, $offset)
            };
            
            $code
        }
    }
}

#[macro_export]
macro_rules! tick {
    ($ident:ident, $offset:expr, $t:ty, $obj:ident, $inp:ident, $out:expr)=>{
        #[no_mangle]
        extern "C" fn $ident(
            ck_self: *mut chuck::Object,
            $inp: f32,
            out: *mut f32,
            _api: chuck::CK_DL_API,
        ) -> chuck::t_CKBOOL {
    
            let mut $obj: Box<$t> = unsafe {
                chugin::util::get_object_data(ck_self, $offset)
            };
            
            let out_ = $out;
            unsafe {
                *out = out_;
            }
    
            Box::into_raw($obj);
    
            chuck::CK_TRUE
        }
    }
}
