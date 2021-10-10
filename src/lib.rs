
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

chugin::ctor! (ctor, DATA_OFFSET, {
    let obj = MyChugin::new(44100.0, 200.0, 1.0);
    obj
});

chugin::dtor! (dtor, DATA_OFFSET, MyChugin, obj, {
});

chugin::tick! (tick, DATA_OFFSET, MyChugin, obj, _inp, {
    obj.tick()
});

chugin::mfun_setter_getter_float! (
    set_freq, get_freq, DATA_OFFSET, 
    MyChugin, obj, freq, 
    { obj.set_freq(freq as f32); }, 
    { obj.get_freq() }
);

fn ck_query_impl(query: *mut chuck::DL_Query) -> chugin::CKResult {
    let q = chugin::Query::new(query)?;

    q.begin_class("RustOsc", "UGen")?;
    
    q.add_ctor(Some(ctor))?;
    q.add_dtor(Some(dtor))?;
    
    let offset = q.add_mvar("int", "@data", false)? as usize;
    unsafe { DATA_OFFSET = offset; }
    
    q.add_ugen_func(Some(tick), 0, 1)?;
    
    q.add_mfun(Some(set_freq), "float", "freq", &[(String::from("float"), String::from("f"))])?;
    
    q.end_class()?;

    Ok(())
}

chugin::query! (query, ck_query_impl(query));
