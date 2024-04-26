use pyo3::{prelude::*, types::PyModule};

pub fn call_python(path: String, class: String, method: String) {
    Python::with_gil(|py| {
        let module = PyModule::import(py, "antigravity").expect("No flying for you.");
    });
}
