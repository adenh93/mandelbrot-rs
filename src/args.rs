use clap::Parser;

/// Generates a mandelbrot set.
/// Uses sane defaults for all parameters, while allowing customisation.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Maximum amount of iterations
    #[clap(long, value_parser, default_value_t = 1_000)]
    pub max_iters: usize,

    /// Minimum horizontal bounds
    #[clap(long, value_parser, default_value_t = -2.0)]
    pub x_min: f64,

    /// Maximum horizontal bounds
    #[clap(long, value_parser, default_value_t = 1.0)]
    pub x_max: f64,

    /// Minimum vertical bounds
    #[clap(long, value_parser, default_value_t = -1.0)]
    pub y_min: f64,

    /// Maximum vertical bounds
    #[clap(long, value_parser, default_value_t = 1.0)]
    pub y_max: f64,

    /// Total width of generated set
    #[clap(short, long, value_parser, default_value_t = 100)]
    pub width: usize,

    /// Total height of generated set
    #[clap(short, long, value_parser, default_value_t = 24)]
    pub height: usize,
}
