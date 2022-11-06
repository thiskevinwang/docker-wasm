FROM scratch
WORKDIR /app
COPY /target/wasm32-wasi/release/docker-wasm.wasm /app/docker-wasm.wasm


ENTRYPOINT ["/app/docker-wasm.wasm"]