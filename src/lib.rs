
use chugin;
use chugin::chuck as chuck;

static mut DATA_OFFSET: usize = 0;

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
        let y = -1.0+self.phase*2.0;
        self.phase += self.phase_update;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        y*self.amp
    }
}

#[chugin::query_fn]
fn ck_query_impl(query: *mut chuck::DL_Query) -> chugin::CKResult {
    let q = chugin::Query::new(query)?;
    
    q.begin_class("RustOsc", "UGen")?;
    q.add_ctor(Some(ctor))?;
    q.add_dtor(Some(dtor))?;
    let offset = q.add_mvar("int", "@data", false)? as usize;
    unsafe { 
        DATA_OFFSET = offset;
    }
    q.add_ugen_func(Some(tick), 0, 1)?;
    q.end_class()?;
    
    Ok(())
}

#[no_mangle]
pub extern "C" fn ctor(ck_self: *mut chuck::Object,
        _args: *mut ::std::os::raw::c_void,
        _vm: *mut chuck::VM,
        _shred: *mut chuck::VM_Shred,
        _api: chuck::CK_DL_API) {
    let chugin = Box::new(MyChugin::new(44100.0, 200.0, 1.0));
    println!("MyChugin: {:?}", chugin);
    
    unsafe {
        chugin::util::set_object_data(ck_self, DATA_OFFSET, chugin);
    }
}

#[no_mangle]
pub extern "C" fn dtor(ck_self: *mut chuck::Object,
        _vm: *mut chuck::VM,
        _shred: *mut chuck::VM_Shred,
        _api: chuck::CK_DL_API) {
    
    let chugin: Box<MyChugin> = unsafe {
        chugin::util::get_object_data(ck_self, DATA_OFFSET)
    };
    
    println!("MyChugin: {:?}", chugin);
}

#[no_mangle]
extern "C" fn tick(
    ck_self: *mut chuck::Object,
    _inp: f32,
    out: *mut f32,
    _api: chuck::CK_DL_API,
) -> chuck::t_CKBOOL {
    
    let mut chugin: Box<MyChugin> = unsafe {
        chugin::util::get_object_data(ck_self, DATA_OFFSET)
    };
    
    unsafe {
        *out = chugin.tick();
    }
    
    Box::into_raw(chugin);
    
    chuck::CK_TRUE
}
