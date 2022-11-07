FROM wasmedge/slim:0.11.2
WORKDIR /app
COPY /target/wasm32-wasi/release/docker-wasm.wasm /app/docker-wasm.wasm

COPY --from=awsguru/aws-lambda-adapter:0.4.0-x86_64 /lambda-adapter /opt/extensions/lambda-adapter
ENV READINESS_CHECK_PORT=8888
ENV PORT=8888

CMD ["wasmedge", "/app/docker-wasm.wasm"]