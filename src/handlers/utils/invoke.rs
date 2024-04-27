use pyo3::{prelude::*, types::PyList, types::PyModule};
use std::fs;
use std::path::Path;

pub fn call_python(invokation_path: &str) -> Result<Py<PyAny>, PyErr> {
    // TODO: handle args
    let mut l = invokation_path.split(":");
    println!("invoking python...");
    let path = Path::new(l.next().unwrap());
    let py_app = fs::read_to_string(path.join(l.next().unwrap()))?;
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath = py
            .import_bound("sys")?
            .getattr("path")?
            .downcast_into::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code_bound(py, &py_app, "", "")?
            .getattr(l.next().unwrap())?
            .into();
        app.call0(py).unwrap().extract(py)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_python() {
        let r = call_python("examples/music:songs.py:list");
        println!("r: {:?}", r.unwrap().to_string());
    }
}
