prepare-image:
	DOCKER_BUILDKIT=1 docker build -f Dockerfile_Prepare_Image -t ghostscript_rust --progress=plain .

get-ghostscript-libs:
	DOCKER_BUILDKIT=1 docker build -f Dockerfile_Extract_Gs_Libs --progress=plain --output type=local,dest=./ghostscript/gs-lib/lib .

build-cache-%:
	DOCKER_BUILDKIT=1 docker build -f ./app-$*/Dockerfile_Cache_Build -t ghostscript_rust_cached --progress=plain .

build-%:
	DOCKER_BUILDKIT=1 docker build -f ./app-$*/Dockerfile_Build -t ghostscript_rust_latest --progress=plain --output type=local,dest=./.aws-sam/build/GhostscriptRust .

deploy-%:
	sam build
	@make build-$*
	sam deploy --no-confirm-changeset

local-%:
	cargo run --bin local-$*

local-sam:
	sam local invoke -e event.json

build-GhostscriptRust:
	@echo done