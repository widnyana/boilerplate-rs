VERSION  := latest
IMG_NAME := username/appname
IMAGE    := ${IMG_NAME}:${VERSION}

default: help


test: ## Run all tests
	@cargo test


test-cov:  ## Run all tests with coverage- `cargo install cargo-tarpaulin`
	@cargo tarpaulin

build: ## Builds the app for current os-arch
	@make test && cargo build --release

run: ## Runs the app
	@CARGO_INCREMENTAL=1 cargo run --bin backend

lint: ## Run clippy
	@find . -type f | grep '\/src\/.*\.rs'| xargs touch && cargo clippy --all-targets --workspace

lint-fix: ## Fix lint
	@cargo fix

fmt: ## Run format
	@cargo +nightly fmt

docker: ## Build a Docker Image
	@DOCKER_BUILDKIT=1 docker build --rm -t ${IMAGE} .

docker-run: ## Run Docker Image locally
	@docker run --rm ${IMAGE}

analyse: ## Analyse for unsafe usage - `cargo install cargo-geiger`
	@cargo geiger

## Release tag
release: ## create new git tag and push it to remote
	@git tag -a ${V} -m "Release ${V}" && git push origin ${V}

## Delete tag
delete-tag: ## delete git tag, both local and remote
	@git tag -d ${V} && git push --delete origin ${V}


unused-deps:  ## Find unused dependencies
	@cargo +nightly udeps


.PHONY: help
.DEFAULT_GOAL := help
help:
	@echo  "[!] Available Command: "
	@echo  "-----------------------"
	@grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' | sort
