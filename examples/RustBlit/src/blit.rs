use crate::util::DSPUtil;

pub type Float = f32;
const PI: Float = std::f32::consts::PI;
const EPSILON: Float = f32::EPSILON;

#[derive(Debug)]
pub enum BlitHarmonics {
    Num(i32),
    Max,
}

/** Bandlimited impulse train after Stilson and Smith. 
 cf. T. Stilson and J. O. Smith. 
 Alias-free digital synthesis of classic analog waveforms. 
 In Proc. 1996 Int. Computer Music Conf., Hong Kong, 1996. Comp. Music Assoc. 
 and https://www.music.mcgill.ca/~gary/307/week5/node14.html 
 */
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Blit {
    /// sample rate
    srate: Float,
    /// frequency
    pub freq: Float,
    /// number of harmonics setting (max or specific number)
    harmonics: BlitHarmonics,
    /// period
    P: Float,
    /// number of harmonics (used in DSP)
    M: Float,
    /// phase update
    phase: Float,
    /// per-sample phase update
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
        self.M = self.P.floor_odd();
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