#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

mod chugin;

static mut data_offset: usize = 0;

/// data for the actual object itself
#[derive(Debug)]
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

fn ck_query_impl(QUERY: *mut chugin::Chuck_DL_Query) -> chugin::CKResult {
    let q = chugin::Query::new(QUERY)?;
    
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
pub extern "C" fn ck_version() -> chugin::t_CKUINT {
    chugin::version()
}

#[no_mangle]
pub extern "C" fn ck_query(QUERY: *mut chugin::Chuck_DL_Query) -> chugin::t_CKBOOL {
    println!("hello, chuck!");
    
    match ck_query_impl(QUERY) {
        Ok(_) => chugin::CK_TRUE,
        Err(_) => chugin::CK_FALSE,
    }
}

#[no_mangle]
pub extern "C" fn ctor(SELF: *mut chugin::Chuck_Object,
        _ARGS: *mut ::std::os::raw::c_void,
        _VM: *mut chugin::Chuck_VM,
        _SHRED: *mut chugin::Chuck_VM_Shred,
        _API: chugin::CK_DL_API) {
    let chugin = Box::new(MyChugin::new(44100.0, 200.0, 1.0));
    println!("MyChugin: {:?}", chugin);
    
    unsafe {
        chugin::util::set_object_data(SELF, data_offset, chugin);
    }
}

#[no_mangle]
pub extern "C" fn dtor(SELF: *mut chugin::Chuck_Object,
        _VM: *mut chugin::Chuck_VM,
        _SHRED: *mut chugin::Chuck_VM_Shred,
        _API: chugin::CK_DL_API) {
    
    let chugin: Box<MyChugin> = unsafe {
        chugin::util::get_object_data(SELF, data_offset)
    };
    
    println!("MyChugin: {:?}", chugin);
}

#[no_mangle]
extern "C" fn tick(
    SELF: *mut chugin::Chuck_Object,
    _in_: f32,
    out: *mut f32,
    _API: chugin::CK_DL_API,
) -> chugin::t_CKBOOL {
    
    let mut chugin: Box<MyChugin> = unsafe {
        chugin::util::get_object_data(SELF, data_offset)
    };
    
    unsafe {
        *out = chugin.tick();
    }
    
    Box::into_raw(chugin);
    
    chugin::CK_TRUE
}
