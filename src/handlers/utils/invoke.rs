use pyo3::{prelude::*, types::PyList, types::PyModule};
use std::fs;
use std::path::Path;

pub fn call_python(_module: &str, _function: &str) -> Result<Py<PyAny>, PyErr> {
    // TODO: handle args
    println!("invoking python...");
    let path = Path::new("examples/music");
    let py_app = fs::read_to_string(path.join("songs.py"))?;
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath = py
            .import_bound("sys")?
            .getattr("path")?
            .downcast_into::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code_bound(py, &py_app, "", "")?
            .getattr("list")?
            .into();
        app.call0(py).unwrap().extract(py)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_python() {
        let r = call_python("music.artists", "list");
        println!("r: {:?}", r.unwrap().to_string());
    }
}
