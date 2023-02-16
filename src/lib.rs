mod reader;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn read_datetime(path: &str) -> PyResult<String> {
    Ok(reader::read_datetime(path).unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
fn datetime_reader(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_datetime, m)?)?;
    Ok(())
}
