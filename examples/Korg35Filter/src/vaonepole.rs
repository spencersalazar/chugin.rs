
pub type Float = f32;
pub type Sample = Float;
const PI: Float = std::f32::consts::PI;

/// Virtual analog one-pole filter
#[derive(Debug)]
pub struct VAOnePole {
    pub a: Float,
    pub b: Float,
    z1: Sample,
    srate: Float,
    pub is_lpf: bool,
}

/// Virtual analog one-pole filter
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
