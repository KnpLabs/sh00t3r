# see https://github.com/liuchong/docker-rustup/blob/master/dockerfiles/nightly/Dockerfile
FROM debian:stretch

RUN apt-get update \
    && apt-get install --no-install-recommends -y \
        autoconf \
        automake \
        autotools-dev \
        build-essential \
        ca-certificates \
        curl \
        file \
        libtool \
        xutils-dev \
    && rm -rf /var/lib/apt/lists/*

ENV SSL_VERSION=1.0.2o

RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O \
    && tar -xzf openssl-$SSL_VERSION.tar.gz \
    && cd openssl-$SSL_VERSION && ./config && make depend && make install \
    && cd .. && rm -rf openssl-$SSL_VERSION*

ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1

RUN mkdir /home/rust \
    && useradd --uid=1000 -d /home/rust rust \
    && chown 1000:1000 /home/rust

USER rust

# install toolchain
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y

ENV PATH=/home/rust/.cargo/bin:$PATH

RUN rustup target add wasm32-unknown-unknown --toolchain nightly \
    && rustup target add wasm32-unknown-emscripten --toolchain nightly

RUN cargo install --git https://github.com/alexcrichton/wasm-gc
RUN cargo install wasm-bindgen-cli

WORKDIR /app
