use chugin;
use chugin::chuck;
use num::Integer;

pub type Float = f32;
pub type Sample = Float;
const PI: Float = std::f32::consts::PI;
const EPSILON: Float = f32::EPSILON;

trait DSPUtil {
    fn wrap_under(&self, val: Self) -> Self;
    
    fn floor_odd(&self) -> Self;
}

impl DSPUtil for f32 {
    fn wrap_under(&self, val: f32) -> f32 {
        let mut x = *self;
        // todo: closed form/non-branching version
        while x > val { x -= val; }
        x
    }

    fn floor_odd(&self) -> f32 {
        2.0*(self/2.0).floor()+1.0
    }
}

#[derive(Debug)]
enum BlitHarmonics {
    Num(i32),
    Max,
}

#[derive(Debug)]
#[allow(non_snake_case)]
struct Blit {
    srate: Float,
    freq: Float,
    harmonics: BlitHarmonics,
    /// period
    P: Float,
    /// number of harmonics
    M: Float,
    /// phase update
    phase: Float,
    /// phase update
    update: Float,
}

impl Blit {
    pub fn new(srate: Float) -> Blit {
        let mut blit = Blit {
            srate: srate,
            freq: 200.0,
            harmonics: BlitHarmonics::Max,
            P: 1.0,
            M: 1.0,
            phase: 0.0,
            update: 0.1,
        };
        
        // sane default
        blit.set_freq(220.0);
        
        blit
    }
    
    pub fn set_freq(&mut self, freq: Float) {
        self.freq = freq;
        self.P = self.srate/freq;
        self.update = 1.0/self.P;
        self.M = self.P.floor_odd()
    }
    
    pub fn tick(&mut self) -> Float {
        let denom = (PI*self.phase).sin();
        let y = if denom < EPSILON {
            1.0
        } else {
            (self.M*PI*self.phase).sin() / (self.P*denom)
        };
        
        self.phase = (self.phase+self.update).wrap_under(1.0);
        
        y
    }
}

static mut DATA_OFFSET: usize = 0;

chugin::ctor!(ctor, DATA_OFFSET, {
    let obj = Blit::new(44100.0);
    obj
});

chugin::dtor!(dtor, DATA_OFFSET, Blit, _obj, {});

chugin::mfun_setter_getter_float!(
    set_freq,
    get_freq,
    DATA_OFFSET,
    Blit,
    blit,
    freq,
    {
        blit.set_freq(freq as Float);
    },
    { blit.freq }
);

chugin::tick!(tick, DATA_OFFSET, Blit, obj, _inp, { 
    obj.tick() as f32
});

fn ck_query_impl(query: *mut chuck::DL_Query) -> chugin::CKResult {
    let q = chugin::Query::new(query)?;

    q.begin_class("RustBlit", "UGen")?;

    q.add_ctor(Some(ctor))?;
    q.add_dtor(Some(dtor))?;

    let offset = q.add_mvar("int", "@data", false)? as usize;
    unsafe { DATA_OFFSET = offset; }

    q.add_ugen_func(Some(tick), 0, 1)?;

    q.add_mfun(
        Some(set_freq),
        "float",
        "freq",
        &[(String::from("float"), String::from("f"))],
    )?;

    q.end_class()?;

    Ok(())
}

chugin::query!(query, ck_query_impl(query));
