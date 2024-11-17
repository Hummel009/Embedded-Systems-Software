FROM ubuntu:22.04

RUN apt-get update

RUN apt-get install -y git

RUN apt-get install -y curl

RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get install -y build-essential

RUN apt-get install -y openocd

RUN apt-get install -y gdb-multiarch

RUN rustup target install thumbv7m-none-eabi

RUN cargo install cargo-binutils

RUN rustup component add llvm-tools-preview
