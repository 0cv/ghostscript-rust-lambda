FROM public.ecr.aws/lambda/provided:al2 AS build-stage

# install ziglang (required by cargo lambda)
RUN yum -y install python3-pip
RUN python3 -m pip install ziglang

# install C compiler
RUN yum groupinstall "Development Tools" -y
# RUN yum install ghostscript -y

# install rustc / cargo
# RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# make cargo available on the path
# ENV PATH="/root/.cargo/bin:${PATH}"

# install cargo lambda for compilation
# RUN cargo install cargo-lambda

# copy source code to the image
# COPY ghostscript-rs /src/ghostscript-rs
# COPY ghostscript-sys /src/ghostscript-sys
# COPY main /src/main
# COPY Cargo.toml /src
COPY ghostscript-9.56.1.tar.gz /src/

# set the compilation directory
WORKDIR "/src"
RUN tar -zxf ghostscript-9.56.1.tar.gz
RUN mv ghostscript-9.56.1 ghostscript
WORKDIR "/src/ghostscript"
RUN ./configure
RUN make
RUN make so
# WORKDIR "/src/main"

# RUN echo $(gs --version)
# RUN echo $(ls /src/ghostscript)
# RUN cargo lambda build --release

# ENTRYPOINT ["/bin/bash"]

FROM scratch AS export-stage
# COPY --from=build-stage /src/target/x86_64-unknown-linux-gnu/release/ghostscript-rust ./bootstrap
COPY --from=build-stage /src/ghostscript/sobin ./lib/