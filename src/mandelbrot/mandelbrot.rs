use super::MandelbrotConfig;
use num::complex::Complex;
use std::io;

type Rows = Vec<Vec<usize>>;

pub struct Mandelbrot {
    rows: Rows,
}

impl Mandelbrot {
    pub fn new(config: &MandelbrotConfig) -> Self {
        let rows = Mandelbrot::calculate(config);
        Mandelbrot { rows }
    }

    pub fn render(&self, out: &mut impl io::Write) {
        for row in &self.rows {
            let mut line = String::with_capacity(row.len());

            for column in row {
                let val = match column {
                    0..=2 => ' ',
                    3..=5 => '.',
                    6..=10 => '•',
                    11..=30 => '*',
                    31..=100 => '+',
                    101..=200 => 'x',
                    201..=400 => '$',
                    401..=700 => '#',
                    _ => '%',
                };

                line.push(val);
            }

            write!(out, "{}\n", line).expect("Failed to write to stream");
        }
    }

    fn calculate(config: &MandelbrotConfig) -> Rows {
        (0..config.height)
            .map(|img_y| {
                (0..config.width)
                    .map(|img_x| Mandelbrot::calculate_point(config, img_x, img_y))
                    .collect()
            })
            .collect()
    }

    fn calculate_point(config: &MandelbrotConfig, img_x: usize, img_y: usize) -> usize {
        let x_percent = img_x as f64 / config.width as f64;
        let y_percent = img_y as f64 / config.height as f64;
        let cx = config.x_min + (config.x_max - config.x_min) * x_percent;
        let cy = config.y_min + (config.y_max - config.y_min) * y_percent;

        Mandelbrot::at_point(cx, cy, config.max_iters)
    }

    fn at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
        let mut z = Complex::new(0.0, 0.0);
        let c = Complex::new(cx, cy);

        for i in 0..=max_iters {
            if z.norm() > 2.0 {
                return i;
            }
            z = z * z + c;
        }

        max_iters
    }
}
