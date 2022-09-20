use crate::Args;

pub struct MandelbrotConfig {
    pub max_iters: usize,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub width: usize,
    pub height: usize,
}

impl MandelbrotConfig {
    pub fn from_args(args: &Args) -> Self {
        MandelbrotConfig {
            max_iters: args.max_iters,
            x_min: args.x_min,
            x_max: args.x_max,
            y_min: args.y_min,
            y_max: args.y_max,
            width: args.width,
            height: args.height,
        }
    }
}
