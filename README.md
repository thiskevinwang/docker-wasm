# docker-wasm

This is a proof of concept: a simple HTTP server, written in `rust` and compiled to `wasm`.

It should be run using the new [Docker WASM](https://www.docker.com/blog/docker-wasm-technical-preview/)
technical preview.

See [`Makefile`](./Makefile) for various commands to run.

### Live Demo

The project is currently deployed to AWS Lambda + Function URL

> **Warning**: This is not intended to be a stable URL and may change in the future.

```
curl https://x2swn6lrdcbwuq6fkkxvo2ugpm0esgja.lambda-url.us-east-1.on.aws/echo -XPOST -d "hello world"

curl https://x2swn6lrdcbwuq6fkkxvo2ugpm0esgja.lambda-url.us-east-1.on.aws/echo/reversed -XPOST -d "hello world"
```

Educational links

- https://www.freecodecamp.org/news/edge-cloud-microservices-with-wasmedge-and-rust/
- https://wasmedge.org/book/en/quick_start/install.html
- https://github.com/second-state/wasmedge_tensorflow_interface
- https://github.com/WasmEdge/wasmedge_hyper_demo/tree/main/server
