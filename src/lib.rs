use pyo3::{prelude::*, wrap_pyfunction};

mod server;
mod handler {
    pub mod helpers;
    pub mod router;
}

#[pymodule]
fn axumapi(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(server::start_server, m)?)?;
    Ok(())
}
