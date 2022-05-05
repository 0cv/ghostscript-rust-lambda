prebuild:
	DOCKER_BUILDKIT=1 docker build -t ghostscript-rust --progress=plain --output type=local,dest=./gs-lib .

# used as placeholder for SAM, i.e. the template.yaml which requires a Make target called as shown here. Shall
# not be removed
build-GhostscriptRust:

build:
	sam build
	LOCAL_LIB=/Users/christophe/Development/solana/aws-test/target/lib cargo lambda build --release --package main --manifest-path main/Cargo.toml
	mv ./target/lambda/bootstrap/bootstrap ./.aws-sam/build/GhostscriptRust/bootstrap

deploy:
	@make build
	sam deploy

local:
	@make build
	sam local invoke -e event.json