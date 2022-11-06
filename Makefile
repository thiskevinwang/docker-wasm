# load .env
ifneq (,$(wildcard ./.env))
  include .env
  export
endif

# $(error|info ...) must be indented with spaces instead of tabs
ifndef REGISTRY
  $(error REGISTRY is not set; Add a `.env` file with REGISTRY=your-registry)
else
  $(info REGISTRY: $(REGISTRY))
endif

ifndef TAG
  $(error TAG is not set; Add a `.env` file with TAG=your-tag)
else
  $(info TAG: $(TAG))
endif

cmd-exists-%:
	@hash $(*) > /dev/null 2>&1 || \
		(echo "ERROR: '$(*)' must be installed and available on your PATH."; exit 1)

build: cmd-exists-cargo
	@echo "==> Building with cargo"
	@cargo build --release --target wasm32-wasi

run: cmd-exists-wasmedge
	@echo "==> Running with cargo"
	@wasmedge target/wasm32-wasi/release/docker-wasm.wasm

docker-build: cmd-exists-docker build
	@echo "==> Building with docker"
	@docker buildx build --platform wasi/wasm32 -t $(TAG) .

docker-tag: cmd-exists-docker
	@echo "==> Tagging docker image"
	@docker image rm $(REGISTRY)/$(TAG)
	@docker tag $(TAG) $(REGISTRY)/$(TAG)

docker-push: cmd-exists-docker
	@echo "==> Pushing docker image"
	@docker push $(REGISTRY)/$(TAG)

docker-run: cmd-exists-docker
	@echo "==> Running with docker"
	@docker run -p 8888:8888 \
	  --rm \
		--runtime=io.containerd.wasmedge.v1 \
		--platform=wasi/wasm32 \
		$(TAG)