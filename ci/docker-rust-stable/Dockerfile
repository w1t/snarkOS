FROM buildpack-deps:buster


FROM cimg/rust:1.43.0

RUN sudo apt-get update && sudo apt-get install -y clang llvm-dev llvm pkg-config xz-utils make libssl-dev libssl-dev
RUN cargo install sccache
