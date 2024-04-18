use pyo3::prelude::*;

use ::tokio::net::TcpListener;
use ::tokio::runtime::Builder;
use axum::{routing::get, Router};

#[pyfunction]
pub fn start_server(_py: Python) -> PyResult<()> {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    /*let locals = pyo3_asyncio::TaskLocals::with_running_loop(py)?.copy_context(py)?;
    // Convert the async move { } block to a Python awaitable
    pyo3_asyncio::tokio::future_into_py_with_locals(py, locals.clone(), async move {
        let py = Python::with_gil(|py| {
            pyo3_asyncio::into_future_with_locals(
                &locals,
                py.import("starbucks.menu")?
                    .call_method1("list_americano", (1,))?,
            )
        })?;
        let l = py.await?;
        println!("l: {:?}", l);
        Ok(())
    });*/

    rt.block_on(async move {
        serve().await;
    });

    Ok(())
}

async fn serve<'a>() {
    let app = Router::new().route("/", get(root));
    println!("Starting server...");
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    println!("In root handler...");
    Python::with_gil(|py| {
        // Import the desired Python module
        let sys = py.import("sys")?;

        // Call a method from the imported module
        let version: String = sys.call_method0("version")?.extract()?;
        println!("Python version: {}", version);

        Ok::<(), PyErr>(())
    });
    "hello"
}
