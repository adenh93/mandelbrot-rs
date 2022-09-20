mod args;
mod mandelbrot;

pub use args::Args;
use mandelbrot::Mandelbrot;
pub use mandelbrot::MandelbrotConfig;
use std::io;

pub fn run(args: &Args) {
    let config = MandelbrotConfig::from_args(args);
    let mandelbrot = Mandelbrot::new(&config);
    mandelbrot.render(&mut io::stdout());
}
