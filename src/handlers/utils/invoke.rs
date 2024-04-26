use pyo3::{prelude::*, types::PyModule};

pub fn call_python(module_path: String, method: String) {
    // TODO: handle args
    Python::with_gil(|py| {
        let module = PyModule::import(py, "music.artists").expect("No flying for you.");
    });
}
