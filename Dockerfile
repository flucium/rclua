FROM ubuntu:23.04
RUN apt update && apt upgrade -y && apt install -y curl git vim build-essential && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
source "$HOME/.cargo/env" && \
rustup toolchain add 1.77.2 && \
rustup default 1.77.2