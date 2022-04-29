prebuild:
	DOCKER_BUILDKIT=1 docker build -t ghostscript-rust --progress=plain --output type=local,dest=./gs-lib .

# build:
    # LOCAL_LIB=/Users/christophe/Development/solana/aws-test/target/lib cargo lambda build

build-GhostscriptRust:

deploy:
	sam build
	LOCAL_LIB=/Users/christophe/Development/solana/aws-test/target/lib cargo lambda build --release --package main --manifest-path main/Cargo.toml
	mv ./target/lambda/bootstrap/bootstrap ./.aws-sam/build/GhostscriptRust/bootstrap
	sam deploy