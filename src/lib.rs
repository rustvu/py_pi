//! Python module implemented in Rust for Monte-Carlo pi estimation.

use pyo3::prelude::*;
use rand::Rng;
use std::thread;

fn monte_carlo_pi(n: usize) -> usize {
    let mut rng = rand::thread_rng();
    let mut inside = 0;
    for _ in 0..n {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    return inside;
}

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
    let mut inside = 0;
    for handle in handles {
        inside += handle.join().unwrap();
    }
    return 4.0 * inside as f64 / n as f64;
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_pi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(single, m)?)?;
    m.add_function(wrap_pyfunction!(parallel, m)?)?;
    Ok(())
}
