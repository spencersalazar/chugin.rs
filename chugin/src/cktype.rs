use crate::chuck;

/// trait for working with ChucK types (int, float, etc.)
pub trait CKType {
    fn get_next_arg(args: chuck::Args) -> (chuck::Args, Self);
}

/// CKType impl for ChucK float (f64)
impl CKType for chuck::Float {
    fn get_next_arg(args: chuck::Args) -> (chuck::Args, Self) {
        // convert to array of arg type
        let args = args as *const chuck::Float;
        // capture arg value at current pointer position
        let arg = unsafe { *args };
        // advance by one element of arg type
        let args = unsafe { args.offset(1) as chuck::Args };
        // return
        (args, arg)
    }
}
