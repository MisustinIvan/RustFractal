use crate::complex::Complex;
#[cfg(test)]
use crate::mandelbrot::MandelbrotSet;

#[test]
fn test_mandelbrot_point_calc() {
    // how to specify the step of the iteration?
    // i want 100..1000 with 10 steps

    for mut max_iter in 1..10 {
        max_iter *= 100;
        let width = 1200;
        let height = 800;
        let mandelbrot = MandelbrotSet::new(width, height, max_iter);
        assert_eq!(
            mandelbrot.calculate_point(Complex::new(-0.5, 0.01), mandelbrot.max_iter),
            mandelbrot.max_iter
        );
        assert_eq!(
            mandelbrot.calculate_point(Complex::new(-0.5, 0.5), mandelbrot.max_iter),
            mandelbrot.max_iter,
        );
        assert_eq!(
            mandelbrot.calculate_point(Complex::new(0.0, 0.0), mandelbrot.max_iter),
            mandelbrot.max_iter
        );
        assert_ne!(
            mandelbrot.calculate_point(Complex::new(0.5, 0.5), mandelbrot.max_iter),
            mandelbrot.max_iter
        );
    }
}
