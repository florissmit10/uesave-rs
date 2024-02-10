mod error;

use std::fs::File;
use std::io::BufReader;

use pyo3::prelude::*;
use uesave::{Error, Save};
use crate::error::PyParseError;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pyclass]
struct PySave {
    save: Save,
}

#[pyfunction]
fn read_sav(path: &str) -> Result<PySave, PyParseError> {
    let file = File::open(path)?;

    let mut reader =BufReader::new(file);

    let save = Save::read(&mut reader)?;

    Ok(PySave{save})
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "uesave")]
fn py_uesave(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(read_sav, m)?)?;
    Ok(())
}
