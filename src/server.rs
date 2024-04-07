use pyo3::prelude::*;

use ::tokio::net::TcpListener;
use ::tokio::runtime::Builder;
use axum::{routing::get, Router};
use std::future::Future;
/*fn call_python(py: Python<'static>) -> Result<&PyAny, PyErr> {
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
}*/

#[pyfunction]
pub fn start_server(py: Python) -> PyResult<()> {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    rt.block_on(async move {
        serve().await;
    });

    Ok(())
}

async fn serve() {
    let app = Router::new().route("/", get(root));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
