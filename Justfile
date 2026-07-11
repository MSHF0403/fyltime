git_revision := `git rev-parse --short HEAD`
app_version := `awk -F'"' '/^\[package\]/{p=1} p && /^version *=/{print $2; exit}' Cargo.toml`
build_date := `date -u +%Y-%m-%dT%H:%M:%SZ`

local_image := "fyt"
container_image := "ghcr.io/mshf0403/fyt"
container_runner := "docker"

test:
    cargo llvm-cov

build: test
    cargo build --release

container-local:
    {{ container_runner }} build \
        --build-arg GIT_REVISION={{ git_revision }} \
        --build-arg BUILD_DATE={{ build_date }} \
        --build-arg VERSION={{ app_version }} \
        -t {{ local_image }}:latest \
        -t {{ local_image }}:{{ app_version }} \
        -f Containerfile \
        .

container:
    {{ container_runner }} buildx build --push \
        --platform linux/amd64,linux/arm64 \
        --build-arg GIT_REVISION={{ git_revision }} \
        --build-arg BUILD_DATE={{ build_date }} \
        --build-arg VERSION={{ app_version }} \
        -t {{ container_image }}:latest \
        -t {{ container_image }}:{{ app_version }} \
        -f Containerfile \
        .