use axum::Router;
use pyo3::prelude::*;
use pyo3_asyncio::*;

#[pyfunction]
pub fn start_server(py: Python) -> PyResult<&PyAny> {
    // Construct the task locals structure with the current running loop and context
    let locals = pyo3_asyncio::TaskLocals::with_running_loop(py)?.copy_context(py)?;

    // Convert the async move { } block to a Python awaitable
    pyo3_asyncio::tokio::future_into_py_with_locals(py, locals.clone(), async move {
        let py_sleep = Python::with_gil(|py| {
            pyo3_asyncio::into_future_with_locals(
                &locals,
                py.import("asyncio")?.call_method1("sleep", (1,))?,
            )
        })?;

        py_sleep.await?;

        Ok(())
    })
}
