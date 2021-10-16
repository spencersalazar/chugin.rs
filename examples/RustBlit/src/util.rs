/// DSP utilities
pub trait DSPUtil {
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
