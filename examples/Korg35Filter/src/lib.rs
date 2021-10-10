use chugin;
use chugin::chuck;

static mut DATA_OFFSET: usize = 0;

type Float = f32;
type Sample = Float;
const PI: Float = std::f32::consts::PI;

#[derive(Debug)]
struct VAOnePole {
    a: Float,
    b: Float,
    z1: Sample,
    srate: Float,
    is_lpf: bool,
}

impl VAOnePole {
    pub fn new(srate: Float) -> VAOnePole {
        VAOnePole {
            srate: srate,
            a: 1.0,
            b: 1.0,
            z1: 0.0,
            is_lpf: true,
        }
    }

    pub fn tick(&mut self, xn: Sample) -> Sample {
        // calculate v(n)
        let vn = (xn - self.z1)*self.a;
        
        // form LP output
        let lpf = vn + self.z1;
        
        // update memory
        self.z1 = vn + lpf;
        
        let hpf = xn - lpf;
    
        if self.is_lpf {
            return lpf;
        } else {
            return hpf;            
        }
    }
    
    pub fn get_feedback(&self) -> Sample {
        self.z1*self.b
    }

    pub fn set_freq(&mut self, freq: Float) {
        let freq = freq.clamp(10.0, self.srate*0.5);
        
        #[allow(non_snake_case)]
        let T = 1.0/self.srate;
        
        let wd = 2.0*PI*freq;
        let wa = (2.0/T)*(wd*T/2.0).tan();
        let g = wa*T/2.0;
        
        self.a = g/(1.0 + g);
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
struct Korg35Filter {
    srate: Float,
    freq: Float,
    K: Float,
    lpf1: VAOnePole,
    lpf2: VAOnePole,
    hpf: VAOnePole,
    a0: Float,
    K_norm: Float,
}

impl Korg35Filter {
    pub fn new(srate: Float) -> Korg35Filter {
        let mut k35 = Korg35Filter {
            srate: srate,
            freq: 100.0,
            K: 1.0,
            lpf1: VAOnePole::new(srate),
            lpf2: VAOnePole::new(srate),
            hpf: VAOnePole::new(srate),
            a0: 1.0,
            K_norm: 1.0,
        };
        
        // sane defaults
        k35.set(500.0, 1.5);
        
        k35
    }
    
    #[allow(non_snake_case)]
    pub fn set(&mut self, freq: Float, K: Float) {
        let freq = freq.clamp(10.0, self.srate/2.0);
        
        self.freq = freq;
        self.K = K;
        
        #[allow(non_snake_case)]
        let T = 1.0/self.srate;
        
        let wd = 2.0*PI*freq;
        let wa = (2.0/T)*(wd*T/2.0).tan();
        let g = wa*T/2.0;
        
        #[allow(non_snake_case)]
        let G = g/(1.0 + g);
        
        self.lpf1.a = G;
        self.lpf2.a = G;
        self.hpf.a = G;
        
        self.lpf2.b = (K - K*G)/(1.0 + g);
        self.hpf.b = -1.0/(1.0 + g);
        
        self.a0 = 1.0/(1.0-K*G+K*G*G);
        
        if K > 0.0 {
            self.K_norm = 1.0/K;
        } else {
            self.K_norm = 1.0;
        }
    }
    
    pub fn tick(&mut self, xn: Sample) -> Sample {
        let y1 = self.lpf1.tick(xn);
        #[allow(non_snake_case)]
        let S35 = self.hpf.get_feedback() + self.lpf2.get_feedback();
        
        let u = self.a0*(y1+S35);
        
        let y = self.K*self.lpf2.tick(u);
        
        self.hpf.tick(y);
        
        let y = self.K_norm*y;
        
        return y;
    }
}


chugin::ctor!(ctor, DATA_OFFSET, {
    let obj = Korg35Filter::new(44100.0);
    obj
});

chugin::dtor!(dtor, DATA_OFFSET, Korg35Filter, _obj, {});

chugin::tick!(tick, DATA_OFFSET, Korg35Filter, obj, inp, { obj.tick(inp) });

chugin::mfun_setter_getter_float!(
    set_freq,
    get_freq,
    DATA_OFFSET,
    Korg35Filter,
    k35,
    freq,
    {
        k35.set(freq as Float, k35.K);
    },
    { k35.freq }
);

chugin::mfun_setter_getter_float!(
    set_k,
    get_k,
    DATA_OFFSET,
    Korg35Filter,
    k35,
    k,
    {
        k35.set(k35.freq, k as Float );
    },
    { k35.K }
);

fn ck_query_impl(query: *mut chuck::DL_Query) -> chugin::CKResult {
    let q = chugin::Query::new(query)?;

    q.begin_class("Korg35", "UGen")?;

    q.add_ctor(Some(ctor))?;
    q.add_dtor(Some(dtor))?;

    let offset = q.add_mvar("int", "@data", false)? as usize;
    unsafe { DATA_OFFSET = offset; }

    q.add_ugen_func(Some(tick), 1, 1)?;

    q.add_mfun(
        Some(set_freq),
        "float",
        "freq",
        &[(String::from("float"), String::from("f"))],
    )?;

    q.add_mfun(
        Some(set_k),
        "float",
        "K",
        &[(String::from("float"), String::from("K"))],
    )?;

    q.end_class()?;

    Ok(())
}

chugin::query!(query, ck_query_impl(query));
