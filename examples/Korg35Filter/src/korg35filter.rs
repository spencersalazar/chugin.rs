
use crate::vaonepole::VAOnePole;

pub type Float = f32;
pub type Sample = Float;
const PI: Float = std::f32::consts::PI;

/// Built-in or custom saturation in the filter's feedback loop
#[derive(Debug)]
pub enum Saturator {
    None,
    Tanh,
    Custom(fn (x: Float)->Float),
}

/// Virtual analog Korg35 filter, after Pirkle
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Korg35Filter {
    srate: Float,
    freq: Float,
    K: Float,
    lpf1: VAOnePole,
    lpf2: VAOnePole,
    hpf: VAOnePole,
    a0: Float,
    K_norm: Float,
    saturation: Float,
    saturator: Saturator,
}

/// Virtual analog Korg35 filter, after Pirkle
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
            saturation: 1.0,
            saturator: Saturator::Tanh,
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
    
    #[allow(non_snake_case)]
    pub fn get_freq(&self) -> Float {
        self.freq
    }
    
    #[allow(non_snake_case)]
    pub fn get_K(&self) -> Float {
        self.K
    }
    
    pub fn tick(&mut self, xn: Sample) -> Sample {
        let y1 = self.lpf1.tick(xn);
        
        #[allow(non_snake_case)]
        let S35 = self.hpf.get_feedback() + self.lpf2.get_feedback();
        
        let u = self.a0*(y1+S35);
        
        let u = match self.saturator {
            Saturator::None => u,
            Saturator::Tanh => (u*self.saturation).tanh(),
            Saturator::Custom(f) => f(u*self.saturation),
        };
        
        let y = self.K*self.lpf2.tick(u);
        
        self.hpf.tick(y);
        
        let y = self.K_norm*y;
        
        return y;
    }
}
