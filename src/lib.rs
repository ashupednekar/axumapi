use pyo3::{prelude::*, wrap_pyfunction};

mod server;

#[pymodule]
fn axumapi(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(server::start_server, m)?)?;
    Ok(())
}

