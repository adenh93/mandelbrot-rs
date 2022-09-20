mod args;
use clap::Parser;
use mandelbrot_rs::{run, Args};

fn main() {
    let args = Args::parse();
    run(&args);
}
