use pyo3::{prelude::*, types::PyModule};

pub fn call_python(module: &str, _function: &str) {
    // TODO: handle args
    Python::with_gil(|py| {
        let module = PyModule::import(py, module).expect("error");
        let r = module.call0();
        println!("r: {:?}", r)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_python() {
        call_python("music.artists", "list")
    }
}
