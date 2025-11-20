//! Python module implemented in Rust for Monte-Carlo pi estimation.

// The Rust module name has to match the name of the dynamic library.
#[pyo3::pymodule]
mod py_pi {
    use pyo3::prelude::*;
    use rand::Rng;
    use std::thread;

    /// Simple (single-threaded) Monte-Carlo pi estimation.
    #[pyfunction]
    fn single(n: usize) -> f64 {
        let inside = monte_carlo_pi(n);
        return 4.0 * inside as f64 / n as f64;
    }

    // Parallel Monte-Carlo pi estimation.
    #[pyfunction]
    fn parallel(n: usize) -> f64 {
        let n_workers = thread::available_parallelism().unwrap().get();

        let mut handles = vec![];
        for _ in 0..n_workers {
            let handle = thread::spawn(move || {
                let inside = monte_carlo_pi(n / n_workers);
                return inside;
            });
            handles.push(handle);
        }

        let inside = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum::<usize>();

        return 4.0 * inside as f64 / n as f64;
    }

    fn monte_carlo_pi(n: usize) -> usize {
        let mut rng = rand::rng();
        (0..n)
            .filter(|_| {
                let x: f64 = rng.random();
                let y: f64 = rng.random();
                x * x + y * y <= 1.0
            })
            .count()
    }
}
