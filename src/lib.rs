use pyo3::prelude::*;
use pyo3_asyncio::{tokio, TaskLocals};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn start_server(py: Python) -> PyResult<()> {
    let locals = TaskLocals::with_running_loop(py)?.copy_context(py)?;
    pyo3_asyncio::tokio::future_into_py_with_locals(py, locals.clone(), async move {
        println!("Starting server...");
        Ok(())
    });
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn axumapi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    Ok(())
}
