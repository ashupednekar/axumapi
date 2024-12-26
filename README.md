
AxumAPI

> unmaintained: pls look at https://github.com/sparckles/Robyn

This is aimed to be the fastest api mux for python packages with axum as the underlying server, while being ridiculously easy to use as a web server/framework

### caveats
- lack of application state (in python)This is aimed to be the fastest api mux for python packages with axum as the underlying server, while being ridiculously easy to use as a web server/framework caveats - lack of application state (in python) 
        (can be solved by simply having a discovery function and an `Arc<Mutex<HashMap<Py<PyAny>>>>`, did it elsewhere... works great.

If someone wants to dive in and build this out as a lightweight API/ws counterpart to Robyn, ping me on https://linkedin.com/in/ashupednekar.
