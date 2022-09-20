# mandelbrot-rs

Generates a mandelbrot set based on CLI arguments

## Arguments

| Argument    | Alias | Description                   | Type  | Default |
|-------------|-------|-------------------------------|-------|---------|
| --max-iters | -m    | Maximum amount of iterations  | usize | 1,000   |
| --x-min     |       | Minimum horizontal bounds     | f64   | -2.0    |
| --x-max     |       | Maximum horizontal bounds     | f64   | 1.0     |
| --y-min     |       | Minimum vertical bounds       | f64   | -1.0    |
| --y-max     |       | Maximum vertical bounds       | f64   | 1.0     |
| --width     | -w    | Total width of generated set  | usize | 100     |
| --height    | -h    | Total height of generated set | usize | 24      |