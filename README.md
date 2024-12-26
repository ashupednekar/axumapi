
AxumAPI

> unmaintained: pls look at https://github.com/sparckles/Robyn

This is aimed to be the fastest api mux for python packages with axum as the underlying server, while being ridiculously easy to use as a web server/framework

> If someone wants to dive in and build this out as a lightweight API/ws only counterpart to Robyn, ping me on `https://linkedin.com/in/ashupednekar`.

### current caveats
- **lack of application state in python - (can be solved by simply having a discovery function and an `Arc<Mutex<HashMap<Py<PyAny>>>>`, done it elsewhere... works great.
- ideally this should be inverted to be a python initiated server that takes in the routes and corresponding functions into a `matchit` compatible type for better compatibility and developer experience, again... happy to dive in if there's interest

